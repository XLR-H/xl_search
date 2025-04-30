use crate::catalog::ProductCatalog;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SearchIndex {
    pub index: HashMap<String, Vec<usize>>, //chave = termo, valor = lista de id
}

impl SearchIndex {
    pub fn from_catalog(catalog: &ProductCatalog) -> Self {
        let mut index: HashMap<String, Vec<usize>> = HashMap::new();

        for product in &catalog.products {
            let id: usize = product.id;
            let terms: Vec<&String> = vec![
                &product.product_type,
                &product.brand,
                &product.vol,
                &product.flavor,
                &product.group,
            ];

            //para cada palavra em cada campo, adicionar ao indice
            for term in terms {
                for word in term.to_lowercase().split_whitespace() {
                    index.entry(word.to_string())
                        .or_default()
                        .push(id);
                }
            }
        }

        Self { index }
    }

    pub fn search(&self, term: &str) -> Option<&Vec<usize>> {
        self.index.get(&term.to_lowercase())
    }

    pub fn search_multiple(&self, query: &str) -> Option<Vec<usize>> {
        let query_lower = query.to_lowercase();
        let mut words = query_lower.split_whitespace();

        if let Some(first) = words.next() {
            if let Some(mut result) = self.index.get(first).cloned() {
                for word in words {
                    if let Some(ids) = self.index.get(word) {
                        result.retain(|id| ids.contains(id)); //interseção
                    } else {
                        return None; //se um dos termos não foi encontrado
                    }
                }
                result.sort_by(|a, b| a.cmp(b));
                
                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }

    
}

#[derive(Debug)]
pub struct SearchCache {
    cache: HashMap<String, Vec<usize>>,
}

impl SearchCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, query: &str) -> Option<&Vec<usize>> {
        self.cache.get(&query.to_lowercase())
    }

    pub fn insert(&mut self, query: &str, results: Vec<usize>) {
        self.cache.insert(query.to_lowercase(), results);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{Product, ProductCatalog};

    fn setup_catalog() -> ProductCatalog {
        let products = vec![
            Product {
                id: 1,
                brand: "Ypê".to_string(),
                vol: "1L".to_string(),
                flavor: "".to_string(),
                product_type: "Limpeza".to_string(),
                group: "Sabão".to_string(),
            },
            Product {
                id: 2,
                brand: "Ypê".to_string(),
                vol: "500ml".to_string(),
                flavor: "".to_string(),
                product_type: "Limpeza".to_string(),
                group: "Detergente".to_string(),
            },
            Product {
                id: 3,
                brand: "Omo".to_string(),
                vol: "1.2L".to_string(),
                flavor: "".to_string(),
                product_type: "sabao po".to_string(),
                group: "lava roupas".to_string(),
            },
        ];
        ProductCatalog::new(products)
    }


    #[test]
    fn test_search_single_term() {
        let catalog = setup_catalog();
        let index = SearchIndex::from_catalog(&catalog);
        let results = index.search("ypê");
        assert_eq!(results, Some(vec![1, 2]).as_ref());
    }

    #[test]
    fn test_search_multiple_terms() {
        let catalog = setup_catalog();
        let index = SearchIndex::from_catalog(&catalog);
        let results = index.search_multiple("lava roupas");
        assert_eq!(results, Some(vec![3]));
    }

    #[test]
    fn test_search_no_results() {
        let catalog = setup_catalog();
        let index = SearchIndex::from_catalog(&catalog);
        let results = index.search("inexistente");
        assert_eq!(results, None);
    }
}

