use anyhow::Ok;
use clap::{arg, builder::OsStringValueParser, command, Parser};
use colored::*;
use reqwest::{header::HeaderMap, Client, Response, Url};
use std::collections::HashMap;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Options::parse();
    let client = Client::new();
    let result = match opts.opts {
        Method::Get(ref args) => get(client, args).await?,
        Method::Post(ref args) => post(client, args).await?,
    };
    print_response(result).await?;
    Ok(())
}
async fn print_response(res: Response) -> anyhow::Result<()> {
    print_header(&res).await?;
    print_headers(&res).await;
    let headers = &res.headers().to_owned();
    let body = res.text().await?;
    print_body(headers.to_owned(), body).await?;
    Ok(())
}
async fn print_header(res: &Response) -> anyhow::Result<()> {
    let header = format!("{} {:?}", res.status(), res.version()).blue();
    println!("{}", header);
    Ok(())
}
async fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n");
}
async fn print_body(headers: HeaderMap, body: String) -> anyhow::Result<()> {
    let mime: Option<mime::Mime> = headers
        .get(reqwest::header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap());
    match mime {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(&body).unwrap())
        }
        _ => println!("{}", body),
    }
    Ok(())
}
async fn get(client: Client, get: &Get) -> anyhow::Result<Response> {
    let res = client.get(get.url.clone()).send().await?;
    Ok(res)
}

async fn post(client: Client, post: &Post) -> anyhow::Result<Response> {
    let res = client
        .post(post.url.clone())
        .json(&post.body)
        .send()
        .await?;
    Ok(res)
}
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Options {
    #[command(subcommand)]
    opts: Method,
}
#[derive(Debug, Clone, Parser)]
enum Method {
    Get(Get),
    Post(Post),
}

// http get method
#[derive(Debug, Clone, Parser)]
struct Get {
    // request url
    #[arg(short, long, value_parser(parse_url))]
    url: String,
}

// http post method
#[derive(Debug, Clone, Parser)]
struct Post {
    // request url
    #[arg(short, long, value_parser(parse_url))]
    url: String,
    // request body
    #[arg(short, long,value_parser=HashMapValueParser::new())]
    body: HashMap<String, String>,
}
// parse url
fn parse_url(url: &str) -> anyhow::Result<String> {
    let _url: Url = Url::parse(url)?;
    Ok(url.to_string())
}

#[derive(Clone)]
struct HashMapValueParser;
impl clap::builder::TypedValueParser for HashMapValueParser {
    type Value = HashMap<String, String>;
    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = OsStringValueParser::new().parse(cmd, arg, value.to_os_string())?;
        let val = inner.into_string();
        let mut map: HashMap<String, String> = HashMap::new();
        if let std::result::Result::Ok(content) = val {
            content.split(",").for_each(|s| {
                let mut kv = s.split("=");
                if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                    map.insert(k.to_string(), v.to_string());
                }
            });
        } else {
            return Err(clap::Error::new(clap::error::ErrorKind::InvalidValue));
        }

        std::result::Result::Ok(map)
    }
}
impl HashMapValueParser {
    fn new() -> Self {
        HashMapValueParser
    }
}
