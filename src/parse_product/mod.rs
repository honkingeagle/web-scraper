use scraper::{Selector, html::Select};
use super::build_csv;
use crate::Product;

pub fn parse_product(html_products: Select){
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let h2_selector = Selector::parse("h2").unwrap();
    let price_selector = Selector::parse(".price").unwrap();

    let products = html_products
        .map(|product| Product {
            url: product
                .select(&a_selector)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string(),
            image: product
                .select(&img_selector)
                .next()
                .unwrap()
                .value()
                .attr("src")
                .unwrap()
                .to_string(),
            name: product
                .select(&h2_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>(),
            price: product
                .select(&price_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>(),
        })
        .collect::<Vec<Product>>();

        build_csv::build_csv_file(products);
}