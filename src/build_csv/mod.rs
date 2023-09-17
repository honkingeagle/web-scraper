use crate::Product;

pub fn build_csv_file(products: Vec<Product>) {
    println!("{:?}", products);
}

#[cfg(test)]
mod tests {
    use csv::Writer;

    #[test]
    fn csv_writer_creates_file() {
        let wtr = Writer::from_path("products.csv");
        assert!(wtr.is_ok());
    }
}