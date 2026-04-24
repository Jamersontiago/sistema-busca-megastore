
#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub marca: String,
    pub preco: f64,
}

pub fn criar_produtos() -> Vec<Produto> {
    vec![
        Produto {
            id: 1,
            nome: "Notebook Dell".to_string(),
            categoria: "Eletrônicos".to_string(),
            marca: "Dell".to_string(),
            preco: 3500.0,
        },
        Produto {
            id: 2,
            nome: "Mouse Gamer".to_string(),
            categoria: "Eletrônicos".to_string(),
            marca: "Logitech".to_string(),
            preco: 150.0,
        },
    ]
}
