use std::collections::HashMap;
use crate::Produto;

pub struct Sistema {
    pub produtos: Vec<Produto>,
    pub indice_nome: HashMap<String, usize>,
    pub indice_categoria: HashMap<String, Vec<usize>>,
    pub indice_marca: HashMap<String, Vec<usize>>,
}

impl Sistema {
    pub fn novo() -> Self {
        Sistema {
            produtos: Vec::new(),
            indice_nome: HashMap::new(),
            indice_categoria: HashMap::new(),
            indice_marca: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        let index = self.produtos.len();

        self.indice_nome.insert(produto.nome.clone(), index);

        self.indice_categoria
            .entry(produto.categoria.clone())
            .or_insert(Vec::new())
            .push(index);

        self.indice_marca
            .entry(produto.marca.clone())
            .or_insert(Vec::new())
            .push(index);

        self.produtos.push(produto);
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Option<&Produto> {
        let nome = nome.to_lowercase();
        self.indice_nome
            .get(&nome)
            .map(|&i| &self.produtos[i])
    }

    pub fn buscar_por_categoria(&self, categoria: &str) -> Vec<&Produto> {
        let categoria = categoria.to_lowercase();
        match self.indice_categoria.get(&categoria) {
            Some(indices) => indices.iter().map(|&i| &self.produtos[i]).collect(),
            None => Vec::new(),
        }
    }

    pub fn buscar_por_marca(&self, marca: &str) -> Vec<&Produto> {
        let marca = marca.to_lowercase();
        match self.indice_marca.get(&marca) {
            Some(indices) => indices.iter().map(|&i| &self.produtos[i]).collect(),
            None => Vec::new(),
        }
    }

    pub fn buscar_por_preco_min(&self, preco_min: f32) -> Vec<&Produto> {
        self.produtos
            .iter()
            .filter(|p| p.preco >= preco_min)
            .collect()
    }
}