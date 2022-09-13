use std::{error::Error, net::AddrParseError};
use serde::Deserialize;
use colour::{dark_green, yellow};

use dotenv::dotenv;


#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,

}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>>
{
    let response: String = ureq::get(url)
        .call()?
        .into_string()?;

        let articles : Articles = serde_json::from_str(&response)?;

        Ok(articles)
}

fn render_articles(articles : &Articles) {

    for a  in &articles.articles {
        dark_green!("> {}\n", a.title);
        yellow!(" > {}\n\n", a.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;

    let url: &str = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";

    let url = format!("{}{}", url, api_key);

    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}
