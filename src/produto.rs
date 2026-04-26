#[derive(Debug, Clone)]
pub struct Produto {
    pub nome: String,
    pub categoria: String,
    pub marca: String,
    pub preco: f32,
}

impl Produto {
    pub fn novo(nome: &str, categoria: &str, marca: &str, preco: f32) -> Self {
        Produto {
            nome: nome.to_lowercase(),
            categoria: categoria.to_lowercase(),
            marca: marca.to_lowercase(),
            preco,
        }
    }
}