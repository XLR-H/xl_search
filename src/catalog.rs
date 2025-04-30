use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::collections::HashMap;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Product {
    pub id: usize,
    pub product_type: String,
    pub brand: String,
    pub vol: String,
    pub flavor: String,
    pub group: String,
}

#[derive(Debug, Clone)]
pub struct ProductCatalog {
    pub products: Vec<Product>,
    pub by_id: HashMap<usize, Product>,
}

impl ProductCatalog {
    pub fn from_csv(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(BufReader::new(file));

        let mut products = Vec::new();
        let mut by_id = HashMap::new();

        for result in reader.records() {
            let record = result?;
            let id: usize = record[0].parse()?;
            let product = Product {
                id,
                product_type: record[1].to_string(),
                brand: record[2].to_string(),
                vol: record[3].to_string(),
                flavor: record[4].to_string(),
                group: record[5].to_string(),
            };
            by_id.insert(id, product.clone());
            products.push(product);
        }

        Ok(ProductCatalog { products, by_id })
    }

    pub fn new(products: Vec<Product>) -> Self {
        let by_id = products
            .iter()
            .map(|p| (p.id, p.clone()))
            .collect();

        Self { products, by_id }
    }



    pub fn get_by_id(&self, id: usize) -> Option<&Product> {
        self.by_id.get(&id)
    }
}
