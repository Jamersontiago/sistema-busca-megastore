
use std::collections::HashMap;
use crate::produto::Produto;

pub struct SistemaBusca {
    pub por_nome: HashMap<String, Vec<Produto>>,
    pub por_categoria: HashMap<String, Vec<Produto>>,
    pub por_marca: HashMap<String, Vec<Produto>>,
}

impl SistemaBusca {
    pub fn novo(produtos: Vec<Produto>) -> Self {
        let mut por_nome = HashMap::new();
        let mut por_categoria = HashMap::new();
        let mut por_marca = HashMap::new();

        for p in produtos {
            por_nome.entry(p.nome.to_lowercase()).or_insert(Vec::new()).push(p.clone());
            por_categoria.entry(p.categoria.to_lowercase()).or_insert(Vec::new()).push(p.clone());
            por_marca.entry(p.marca.to_lowercase()).or_insert(Vec::new()).push(p.clone());
        }

        Self { por_nome, por_categoria, por_marca }
    }

    pub fn buscar_nome(&self, nome: &str) -> Option<&Vec<Produto>> {
        self.por_nome.get(&nome.to_lowercase())
    }

    pub fn buscar_categoria(&self, categoria: &str) -> Option<&Vec<Produto>> {
        self.por_categoria.get(&categoria.to_lowercase())
    }

    pub fn buscar_marca(&self, marca: &str) -> Option<&Vec<Produto>> {
        self.por_marca.get(&marca.to_lowercase())
    }

    pub fn buscar_por_preco(&self, min: f64, max: f64) -> Vec<Produto> {
        let mut resultado = Vec::new();
        for lista in self.por_nome.values() {
            for p in lista {
                if p.preco >= min && p.preco <= max {
                    resultado.push(p.clone());
                }
            }
        }
        resultado
    }
}
