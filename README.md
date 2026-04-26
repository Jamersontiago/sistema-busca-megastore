# 🔎 Sistema de Busca de Produtos - Megastore

> Sistema de busca de produtos via terminal, desenvolvido em Rust, com foco em performance, organização de dados e boas práticas de desenvolvimento backend.

---

## 📸 Exemplo de execução

bash Digite o nome do produto: > notebook  Buscando produto...  Produto encontrado: Nome: Notebook Dell Preço: R$ 3500,00 

---

## 🧠 Como funciona a busca

A busca de produtos é realizada utilizando uma abordagem de busca linear em uma coleção de dados armazenada em memória.

O sistema percorre os itens sequencialmente até encontrar uma correspondência com o termo pesquisado, retornando as informações do produto.

Essa abordagem foi escolhida pela simplicidade e eficiência em conjuntos de dados pequenos e médios.

---

## 🧠 Decisões Técnicas

- Implementação de busca linear para garantir simplicidade e clareza do algoritmo  
- Estrutura organizada para facilitar manutenção e evolução do sistema  
- Separação de responsabilidades entre entrada, processamento e saída de dados  

---

## 🚀 Tecnologias utilizadas

- 🦀 Rust  
- ⚙️ Cargo  
- 🗂️ Git  
- 🌐 GitHub  

---

## ⚙️ Como executar o projeto

### Pré-requisitos
- Ter o Rust instalado (cargo)

### Passos

bash git clone https://github.com/Jamersontiago/sistema-busca-megastore.git cd sistema-busca-megastore cargo run 

O sistema será executado diretamente no terminal.

---

## 📂 Estrutura do projeto

sistema-busca-megastore/ ├── src/        # Código-fonte principal ├── Cargo.toml  # Configuração do projeto Rust ├── .gitignore  # Arquivos ignorados pelo Git └── README.md   # Documentação do projeto

---

## 📈 Melhorias futuras

- Implementação de busca binária para melhor performance  
- Utilização de HashMap para buscas em tempo O(1)  
- Persistência de dados com banco de dados  
- Criação de API para integração com outros sistemas  

---

## 👨‍💻 Autor

Jamerson Tiago  
📌 Estudante de Análise e Desenvolvimento de Sistemas  
💡 Interesse em Backend, Dados e Automação  
🔗 GitHub: https://github.com/Jamersontiago  

---

## 📌 Status do projeto

✅ Funcional  
🚧 Em evolução
