use std::io;
use megastore_busca::{Sistema, Produto};

fn main() {
    let mut sistema = Sistema::novo();

    // Inserindo produtos
    sistema.adicionar_produto(Produto {
        nome: "arroz".to_string(),
        categoria: "alimento".to_string(),
        marca: "tio joão".to_string(),
        preco: 20.0,
    });

    sistema.adicionar_produto(Produto {
        nome: "feijão".to_string(),
        categoria: "alimento".to_string(),
        marca: "camil".to_string(),
        preco: 10.0,
    });

    sistema.adicionar_produto(Produto {
        nome: "notebook dell".to_string(),
        categoria: "eletrônico".to_string(),
        marca: "dell".to_string(),
        preco: 3500.0,
    });

    println!("Digite o nome do produto:");
    
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    let entrada = entrada.trim().to_lowercase();

    match sistema.buscar_por_nome(&entrada) {
        Some(produto) => {
            println!("Produto encontrado:");
            println!("Nome: {}", produto.nome);
            println!("Categoria: {}", produto.categoria);
            println!("Marca: {}", produto.marca);
            println!("Preço: R$ {:.2}", produto.preco);
        }
        None => println!("Produto não encontrado."),
    }
}