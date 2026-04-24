
use megastore_busca::produto::*;
use megastore_busca::busca::*;

#[test]
fn teste_busca_nome() {
    let sistema = SistemaBusca::novo(criar_produtos());
    assert!(sistema.buscar_nome("Notebook Dell").is_some());
}

#[test]
fn teste_busca_marca() {
    let sistema = SistemaBusca::novo(criar_produtos());
    assert!(sistema.buscar_marca("Dell").is_some());
}

#[test]
fn teste_busca_categoria() {
    let sistema = SistemaBusca::novo(criar_produtos());
    assert!(sistema.buscar_categoria("Eletrônicos").is_some());
}

#[test]
fn teste_busca_preco() {
    let sistema = SistemaBusca::novo(criar_produtos());
    let r = sistema.buscar_por_preco(100.0, 4000.0);
    assert!(r.len() > 0);
}
