# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
Este é um sistema de busca eficiente desenvolvido para a MegaStore, que permite localizar produtos em um catálogo com base em diversos atributos como tipo, marca, volume, sabor e grupo. O objetivo é oferecer uma ferramenta rápida, responsiva e fácil de usar, mesmo com grandes volumes de dados.

## Tecnologias Utilizadas
- **Rust** (linguagem principal)
- **Crates**:
  - `csv` (leitura de arquivos CSV)
  - `serde` com `derive` (serialização e desserialização dos dados)
- **Ferramentas de Teste**:
  - `cargo test` (execução dos testes unitários e de integração)

## Instruções de Execução do Sistema
1. Clone o repositório:
   ```bash
   git clone <url-do-repositorio>
   cd xl_search
   ```
2. Compile o projeto:
   ```bash
   cargo build
   ```
3. Certifique-se de que o arquivo `catalog.csv` está na pasta `data/`, ou atualize o caminho no `main.rs`.
4. Execute o sistema:
   ```bash
   cargo run
   ```

## Instruções de Execução dos Testes
1. Para rodar todos os testes:
   ```bash
   cargo test
   ```
2. Os testes estão localizados na pasta `/tests`, em `main_test.rs`, cobrindo funcionalidades do catálogo e sistema de busca.

## Exemplos de Uso
- Busca simples:
  ```
  Entrada: morango
  Resultado: Produtos com "morango" em qualquer campo
  ```
- Busca composta:
  ```
  Entrada: morango frios
  Resultado: Produtos que contenham ambos os termos
  ```
- Busca que retorna vazia:
  ```
  Entrada: inexistente
  Resultado: Nenhum produto encontrado
  ```

## Arquitetura do Sistema
O sistema é dividido em três módulos principais:
- `catalog.rs`: Carregamento e representação dos produtos.
- `search.rs`: Indexação e busca utilizando tabelas hash.
- `main.rs`: Interface de linha de comando e controle da aplicação.

## Algoritmos e Estruturas de Dados Utilizados
- **HashMap**: Utilizado para criar um índice invertido que associa cada palavra-chave a uma lista de IDs de produtos.
- **Busca por interseção de conjuntos**: Em buscas com múltiplos termos, apenas produtos que aparecem em todas as listas são retornados.

## Considerações sobre Desempenho e Escalabilidade
- O uso de tabelas hash permite buscas em tempo quase constante (O(1)) por termo.
- O sistema foi testado com centenas de registros e apresentou desempenho instantâneo.
- O modelo permite escalabilidade para milhares de produtos com mínima perda de performance.

## Contribuições
Até o momento, este é um projeto acadêmico individual. Para colaborações futuras, abra uma *issue* ou *pull request* no repositório.

## Licença
Este projeto está licenciado sob a Licença MIT. Consulte o arquivo `LICENSE` para mais informações.