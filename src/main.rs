mod catalog;
mod search;
mod cache;

use std::io::{self, Write};
use crate::catalog::ProductCatalog;
use crate::search::SearchIndex;
use crate::cache::SearchCache;

fn main() {
    let catalog = match ProductCatalog::from_csv("data/catalog.csv") {
        Ok(c) => c,
        Err(e) => {
            println!("Erro carregando catálogo: {}", e);
            return;
        }
    };

    let index = SearchIndex::from_catalog(&catalog);
    let mut cache = SearchCache::new();

    loop {
        print!("\nDigite o termo de busca (ou 'sair' para terminar): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Erro lendo entrada.");
            continue;
        }

        let input = input.trim().to_lowercase();
        if input == "sair" {
            break;
        }

        let results = if let Some(cached) = cache.get(&input) {
            Some(cached.clone())
        } else {
            let result = index.search_multiple(&input);
            if let Some(ref ids) = result {
                cache.insert(input.clone(), ids.clone());
            }
            result
        };

        match results {
            Some(ids) => {
                println!("\nProdutos encontrados:");
                for id in ids {
                    if let Some(product) = catalog.get_by_id(id) {
                        println!(
                            "- {} {} ({}, {}, {})",
                            product.product_type, product.brand, product.vol, product.flavor, product.group
                        );
                    }
                }
            }
            None => {
                println!("\nNenhum produto encontrado.");
            }
        }
    }

    println!("\nPrograma finalizado!");
}
