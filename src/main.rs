use clap::{arg, builder::OsStringValueParser, command, Parser};
use reqwest::Url;
use std::collections::HashMap;

fn main() {
    let opts = Options::parse();
    println!("{:?}", opts);
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
        if let Ok(content) = val {
            content.split(",").for_each(|s| {
                let mut kv = s.split("=");
                if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                    map.insert(k.to_string(), v.to_string());
                }
            });
        } else {
            return Err(clap::Error::new(clap::error::ErrorKind::InvalidValue));
        }

        Ok(map)
    }
}
impl HashMapValueParser {
    fn new() -> Self {
        HashMapValueParser
    }
}
