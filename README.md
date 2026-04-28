#  Sistema de Busca de Produtos - MegaStore

Sistema desenvolvido em Rust com foco em desempenho, organização e eficiência na busca de produtos em grandes catálogos, utilizando estruturas de dados otimizadas como HashMap para garantir consultas rápidas.

---

##  Objetivo

Implementar um sistema de busca otimizado capaz de localizar produtos por:

- Nome  
- Categoria  
- Marca  
- Faixa de preço  

Utilizando tabelas hash para maximizar a performance.

---

##  Tecnologias utilizadas

- Rust  
- HashMap (tabelas hash)  
- Testes automatizados com cargo test  
- Git/GitHub para versionamento  
- Documentação técnica acadêmica  

---

##  Estrutura do sistema

O sistema utiliza múltiplos índices para otimizar as buscas:

- indice_nome: HashMap<String, Produto>  
- indice_categoria: HashMap<String, Vec<Produto>>  
- indice_marca: HashMap<String, Vec<Produto>>  

Essa abordagem elimina a necessidade de percorrer toda a coleção de produtos para buscas indexadas, proporcionando respostas rápidas e eficientes.

---

##  Como executar o projeto

bash cargo run 

---

##  Exemplo real de uso (terminal)

Ao executar o sistema, o usuário pode buscar um produto digitando seu nome:

text Digite o nome do produto: notebook dell 

###  Saída:

text Produto encontrado: Nome: notebook dell Categoria: eletrônico Marca: dell Preço: R$ 3500.00 

✔ Esse exemplo demonstra a busca eficiente por nome utilizando HashMap com complexidade média O(1).

---

##  Testes automatizados

O sistema possui testes reais cobrindo todas as funcionalidades principais:

- Busca por nome  
- Busca por categoria  
- Busca por marca  
- Busca por preço  

### Para executar:

bash cargo test 

### ✔ Resultado esperado:

text running 4 tests test teste_busca_nome ... ok test teste_busca_categoria ... ok test teste_busca_marca ... ok test teste_busca_preco ... ok  test result: ok. 4 passed; 0 failed 

---

##  Complexidade e desempenho

| Tipo de busca        | Complexidade |
|---------------------|-------------|
| Busca por nome      | O(1)        |
| Busca por categoria | O(1)        |
| Busca por marca     | O(1)        |
| Busca por preço     | O(n)        |

- O(1) → acesso direto via HashMap  
- O(n) → varredura linear sobre os produtos  

### Análise:
O uso de HashMap garante desempenho significativamente superior para buscas indexadas em comparação à busca linear tradicional, tornando o sistema escalável para catálogos maiores.

---

##  Arquitetura do projeto

bash src/ ├── main.rs         # interação com o usuário ├── lib.rs          # exportação dos módulos ├── produto.rs      # definição da struct Produto ├── busca.rs        # lógica de indexação e busca  tests/ └── busca_test.rs   # testes automatizados 

---

##  Funcionalidades implementadas

- Inserção de produtos  
- Busca por nome  
- Busca por categoria  
- Busca por marca  
- Busca por preço mínimo  
- Normalização de strings com to_lowercase()  
- Testes automatizados  
- Execução interativa via terminal  

---

##  Possíveis melhorias

- Indexação por faixa de preço com estruturas ordenadas (ex: BTreeMap)  
- Interface gráfica (GUI)  
- API REST para integração externa  
- Persistência em banco de dados  
- Escalabilidade para grandes volumes de produtos  

---

##  Autor

Jamerson Tiago da Silva Leite  
Estudante de Análise e Desenvolvimento de Sistemas  

---

##  Considerações finais

Este projeto demonstra na prática a aplicação de estruturas de dados eficientes em Rust, integrando teoria acadêmica e implementação real, com foco em desempenho, organização, manutenção e escalabilidade.

A solução evidencia como o uso estratégico de HashMap pode otimizar sistemas de busca, oferecendo respostas rápidas, código modular e base sólida para futuras expansões.