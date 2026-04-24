use megastore_busca::produto::*;
use megastore_busca::busca::*;


fn main() {
    let produtos = criar_produtos();
    let sistema = SistemaBusca::novo(produtos);

    println!("Busca por nome:");
    if let Some(r) = sistema.buscar_nome("Notebook Dell") {
        println!("{:?}", r);
    }

    println!("Busca por marca:");
    if let Some(r) = sistema.buscar_marca("Dell") {
        println!("{:?}", r);
    }

    println!("Busca por preço:");
    let r = sistema.buscar_por_preco(100.0, 4000.0);
    println!("{:?}", r);
}
