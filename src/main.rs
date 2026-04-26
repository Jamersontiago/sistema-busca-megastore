use megastore_busca::produto::*;
use megastore_busca::busca::*;
use std::io;

fn main() {
    let produtos = criar_produtos();
    let sistema = SistemaBusca::novo(produtos);

    println!("=== SISTEMA DE BUSCA MEGASTORE ===");
    println!("Digite o nome do produto:");

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");

    let entrada = entrada.trim();

    println!("\nResultado da busca:");

    if let Some(r) = sistema.buscar_nome(entrada) {
        println!("{:?}", r);
    } else {
        println!("Produto não encontrado.");
    }
}