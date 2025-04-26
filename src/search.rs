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
                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }

    
}