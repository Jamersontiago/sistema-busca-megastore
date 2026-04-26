use megastore_busca::{Sistema, Produto};

#[test]
fn teste_busca_nome() {
    let mut sistema = Sistema::novo();

    sistema.adicionar_produto(Produto::novo("Arroz", "Alimento", "Tio João", 20.0));

    let resultado = sistema.buscar_por_nome("arroz");

    assert!(resultado.is_some());
}

#[test]
fn teste_busca_categoria() {
    let mut sistema = Sistema::novo();

    sistema.adicionar_produto(Produto::novo("Feijão", "Alimento", "Camil", 10.0));

    let resultado = sistema.buscar_por_categoria("alimento");

    assert_eq!(resultado.len(), 1);
}

#[test]
fn teste_busca_marca() {
    let mut sistema = Sistema::novo();

    sistema.adicionar_produto(Produto::novo("Notebook Dell", "Eletrônico", "Dell", 3500.0));

    let resultado = sistema.buscar_por_marca("dell");

    assert_eq!(resultado.len(), 1);
}

#[test]
fn teste_busca_preco() {
    let mut sistema = Sistema::novo();

    sistema.adicionar_produto(Produto::novo("Notebook Dell", "Eletrônico", "Dell", 3500.0));

    let resultado = sistema.buscar_por_preco_min(1000.0);

    assert_eq!(resultado.len(), 1);
}