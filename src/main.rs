mod catalog;
mod search;
mod cache;

use catalog::{ProductCatalog, Product};
use search::SearchIndex;
use crate::cache::SearchCache;
// use serde::Deserialize;   não está usando esta linha,pq?


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = ProductCatalog::from_csv("data/produtos.csv")?;
    let search_index = SearchIndex::from_catalog(&catalog);

    let mut cache = SearchCache::new();

    use std::io::{self, Write};

    loop {
        print!("Digite um termo de busca (ou 'sair'): ");
        io::stdout().flush()?; // exibe o prompt sem esperar nova linha

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let term = input.trim().to_lowercase();

        if term == "sair" {
            break;
        }

        if let Some(result) = cache.get(&term) {
            println!("(cache) Resultados encontrados:");
            for id in result {
                if let Some(product) = catalog.by_id.get(id) {
                    println!("- {} {}", product.brand, product.product_type);
                }
            }
        } else {
            match search_index.search(&term) {
                Some(result) => {
                    println!("(busca direta) Resultados encontrados:");
                    for id in result {
                        if let Some(product) = catalog.by_id.get(id) {
                            println!("- {} {}", product.brand, product.product_type);
                        }
                    }
                    cache.insert(term.clone(), result.clone());
                }
                None => println!("Nenhum resultado encontrado."),
            }
        }
    }

    Ok(())
}
