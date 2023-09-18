use crate::Product;
use csv::Writer;
pub fn build_csv_file(products: Vec<Product>) {
    let mut wtr = Writer::from_path("products.csv").unwrap();
    for product in products {
        wtr.serialize(product).unwrap_or_else(|err| {
            eprintln!("Unable to serialize product: {}", err);
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::Product;
    use csv::Writer;

    #[test]
    #[ignore]
    fn csv_writer_creates_file() {
        let wtr = Writer::from_path("products.csv");
        assert!(wtr.is_ok());
    }

    #[test]
    #[ignore]
    fn csv_writer_create_headers_example() {
        let product = Product {
            url: "https://example.com".to_string(),
            image: "https://imageurl/product.png".to_string(),
            name: "Pikachu".to_string(),
            price: "100".to_string()
        };
        
        let mut wtr = Writer::from_path("products.csv").unwrap();
        assert!(wtr.serialize(product).is_ok());
    }
}