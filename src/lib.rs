mod scrape;
mod parse_product;
pub mod build_csv;

use parse_product::parse_product;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Product {
    pub url: String,
    pub image: String,
    pub name: String,
    pub price: String,
}

pub async fn parse_document(html: String) {
    let document = Html::parse_document(&html);
    let li_selector = Selector::parse("li.product").unwrap();
    let html_products = document.select(&li_selector);

    parse_product(html_products);
}
