mod catalog;
mod search;

use catalog::{ProductCatalog, Product};
use search::SearchIndex;
// use serde::Deserialize;   não está usando esta linha,pq?


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // carregar os produtos do csv
    let catalog = ProductCatalog::from_csv("data/produtos.csv")?;

    // criar o indice de busca
    let index = SearchIndex::from_catalog(&catalog);

    //buscar por um termo pra testar
    let termo = "morango";
    if let Some(resultados) = index.search(termo) {
        println!("Encontrado para '{}': ", termo);
        for id in resultados {
            if let Some(produto) = catalog.get_by_id(*id) {
                println!("{:?}", produto);
            }
        }
    } else {
        println!("Nunhum resultado encontrado para '{}'", termo);
    }

    Ok(())
}
