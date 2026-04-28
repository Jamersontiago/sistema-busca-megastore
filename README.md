# 🛒 Sistema de Busca de Produtos - MegaStore

Sistema desenvolvido em Rust com foco em desempenho e organização de dados, utilizando estruturas de dados eficientes para realizar buscas rápidas em um catálogo de produtos.

---

## 📌 Objetivo

Implementar um sistema de busca otimizado capaz de localizar produtos por:

- Nome  
- Categoria  
- Marca  
- Faixa de preço  

Utilizando HashMap (tabelas hash) para garantir alta performance.

---

## ⚙️ Tecnologias utilizadas

- Rust  
- HashMap (estrutura de dados)  
- Testes automatizados com cargo test  

---

## 🧠 Estrutura do sistema

O sistema utiliza múltiplos índices para otimizar as buscas:

- indice_nome: HashMap<String, Produto>  
- indice_categoria: HashMap<String, Vec<Produto>>  
- indice_marca: HashMap<String, Vec<Produto>>  

Isso permite acesso direto aos dados sem necessidade de percorrer toda a lista.

---

## 🚀 Como executar o projeto

bash cargo run 

---

## 💻 Exemplo real de uso (terminal)

Ao executar o sistema, o usuário pode buscar um produto digitando seu nome:

text Digite o nome do produto: notebook dell 

### 📤 Saída:

text Produto encontrado: Nome: notebook dell Categoria: eletrônico Marca: dell Preço: R$ 3500.00 

✔ Esse exemplo demonstra a busca eficiente por nome utilizando HashMap (complexidade (ok)).

---

## 🧪 Testes automatizados

O sistema possui testes reais cobrindo todas as funcionalidades principais:

- Busca por nome  
- Busca por categoria  
- Busca por marca  
- Busca por preço  

Para executar:

bash cargo test 

### ✔ Resultado esperado:

text running 4 tests test teste_busca_nome ... ok test teste_busca_categoria ... ok test teste_busca_marca ... ok test teste_busca_preco ... ok  test result: ok. 4 passed; 0 failed 

---

## 📊 Complexidade e desempenho

| Tipo de busca        | Complexidade |
|---------------------|-------------|
| Busca por nome      | (ok)        |
| Busca por categoria | (ok)        |
| Busca por marca     | (ok)        |
| Busca por preço     | (ok)        |

- (ok) → acesso direto via HashMap  
- (ok) → número de itens encontrados  
- (ok) → varredura completa da lista  

---

## 🏗️ Arquitetura do projeto

src/  ├── main.rs        → interação com o usuário  ├── lib.rs         → exportação dos módulos  ├── produto.rs     → definição da struct Produto  ├── busca.rs       → lógica de indexação e busca  tests/  └── busca_test.rs  → testes automatizados

---

## 🔍 Funcionalidades implementadas

- Inserção de produtos  
- Busca por nome (instantânea)  
- Busca por categoria  
- Busca por marca  
- Busca por preço mínimo  
- Testes automatizados  
- Execução interativa via terminal  

---

## 📈 Possíveis melhorias

- Indexação por faixa de preço com estruturas ordenadas (ex: BTreeMap)  
- Interface gráfica (GUI)  
- API REST para integração com frontend  
- Persistência em banco de dados  

---

## 👨‍💻 Autor

Jamerson Tiago da Silva Leite  
Estudante de Análise e Desenvolvimento de Sistemas  

---

## 📌 Considerações finais

Este projeto demonstra na prática a aplicação de estruturas de dados eficientes, integrando teoria e implementação real, com foco em desempenho, organização e escalabilida.