use clap::{Command, Arg};
use reqwest;
use serde::{Deserialize, Serialize};
use sled::Db;
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct UrlMapping {
    long_url: String,
    short_url: String,
}

fn get_db() -> Db {
    sled::open("url_db").expect("Failed to open database")
}

fn store_url(short_url: &str, long_url: &str) {
    let db = get_db();
    db.insert(short_url, long_url).expect("Failed to store URL");
}

fn retrieve_url(short_url: &str) -> Option<String> {
    let db = get_db();
    db.get(short_url).unwrap().map(|v| String::from_utf8(v.to_vec()).unwrap())
}

async fn shorten_url(long_url: String) -> Result<String, warp::Rejection> {
    let api_url = format!("https://tinyurl.com/api-create.php?url={}", long_url);
    match reqwest::get(&api_url).await {
        Ok(response) => match response.text().await {
            Ok(short_url) => {
                store_url(&short_url, &long_url);
                Ok(short_url)
            }
            Err(_) => Err(warp::reject::not_found()),
        },
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn expand_url(short_url: String) -> Result<String, warp::Rejection> {
    match retrieve_url(&short_url) {
        Some(long_url) => Ok(long_url),
        None => Err(warp::reject::not_found()),
    }
}

#[tokio::main]
async fn main() {
    let matches = Command::new("URL Shortener")
        .version("1.0")
        .author("Your Name")
        .about("Shortens URLs and stores them")
        .subcommand(Command::new("shorten")
            .about("Shortens a URL")
            .arg(Arg::new("URL")
                .required(true)
                .index(1)))
        .subcommand(Command::new("expand")
            .about("Expands a shortened URL")
            .arg(Arg::new("SHORT_URL")
                .required(true)
                .index(1)))
        .get_matches();

    match matches.subcommand() {
        Some(("shorten", sub_m)) => {
            // Create a longer-lived `url` value
            let url = sub_m.get_one::<String>("URL").map(|s| s.clone()).unwrap_or_else(|| "".to_string());
            match shorten_url(url).await {
                Ok(short_url) => {
                    println!("Shortened URL: {}", short_url);
                }
                Err(_) => eprintln!("Error shortening URL"),
            }
        }
        Some(("expand", sub_m)) => {
            // Create a longer-lived `short_url` value
            let short_url = sub_m.get_one::<String>("SHORT_URL").map(|s| s.clone()).unwrap_or_else(|| "".to_string());
            match expand_url(short_url).await {
                Ok(long_url) => {
                    println!("Original URL: {}", long_url);
                }
                Err(_) => eprintln!("URL not found"),
            }
        }
        _ => eprintln!("Invalid command"),
    }

    let shorten = warp::path!("shorten" / String)
        .and_then(|url| async move {
            match shorten_url(url).await {
                Ok(short_url) => Ok(warp::reply::html(short_url)),
                Err(_) => Err(warp::reject::not_found()),
            }
        });
    
    let expand = warp::path!("expand" / String)
        .and_then(|short_url| async move {
            match expand_url(short_url).await {
                Ok(long_url) => Ok(warp::reply::html(long_url)),
                Err(_) => Err(warp::reject::not_found()),
            }
        });

    let routes = shorten.or(expand);

    println!("Server running on http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
