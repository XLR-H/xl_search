use xl_search::catalog::ProductCatalog;
use xl_search::search::SearchIndex;

#[test]
fn test_load_catalog() {
    let catalog = ProductCatalog::from_csv("data/catalog.csv");
    assert!(catalog.is_ok(), "Falha ao carregar o catalogo!");
}

#[test]
fn test_search_existing_term() {
    let catalog = ProductCatalog::from_csv("data/catalog.csv").unwrap();
    let index = SearchIndex::from_catalog(&catalog);
    let results = index.search("morango");
    assert!(results.is_some(), "Busca por 'morango' não retornou resultados");
}

#[test]
fn test_search_multiple_terms_result() {
    let catalog = ProductCatalog::from_csv("data/catalog.csv").unwrap();
    let index = SearchIndex::from_catalog(&catalog);
    let results = index.search_multiple("sabao omo");
    assert_eq!(results, Some(vec![10]));
}

