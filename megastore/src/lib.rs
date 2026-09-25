use serde::Deserialize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Deserialize)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

#[derive(Debug, Deserialize)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub interesses: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Compra {
    pub id: u32,
    pub cliente_id: u32,
    pub produtos: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Vertice {
    Cliente(u32),
    Produto(u32),
}

pub struct Grafo {
    pub adjacencias: HashMap<Vertice, Vec<Vertice>>,
}

impl Grafo {
    pub fn novo() -> Self {
        Self {
            adjacencias: HashMap::new(),
        }
    }

    pub fn adicionar_vertice(&mut self, vertice: Vertice) {
        self.adjacencias.entry(vertice).or_default();
    }

    pub fn adicionar_aresta(&mut self, origem: Vertice, destino: Vertice) {
        self.adicionar_vertice(origem.clone());
        self.adicionar_vertice(destino.clone());

        let vizinhos_origem = self
            .adjacencias
            .get_mut(&origem)
            .expect("O vértice de origem deve existir");

        if !vizinhos_origem.contains(&destino) {
            vizinhos_origem.push(destino.clone());
        }

        let vizinhos_destino = self
            .adjacencias
            .get_mut(&destino)
            .expect("O vértice de destino deve existir");

        if !vizinhos_destino.contains(&origem) {
            vizinhos_destino.push(origem);
        }
    }

    pub fn vizinhos(&self, vertice: &Vertice) -> Vec<Vertice> {
        self.adjacencias
            .get(vertice)
            .cloned()
            .unwrap_or_default()
    }

    pub fn quantidade_vertices(&self) -> usize {
        self.adjacencias.len()
    }

    pub fn quantidade_arestas(&self) -> usize {
        self.adjacencias
            .values()
            .map(|vizinhos| vizinhos.len())
            .sum::<usize>() / 2
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recomendacao {
    pub produto_id: u32,
    pub nome: String,
    pub categoria: String,
    pub pontuacao: u32,
    pub clientes_compartilhados: usize,
}

pub fn recomendar_bfs(
    grafo: &Grafo,
    cliente_id: u32,
    profundidade_maxima: usize,
) -> Vec<Vertice> {
    let inicio = Vertice::Cliente(cliente_id);
    let mut fila: VecDeque<(Vertice, usize)> = VecDeque::new();
    let mut visitados: HashMap<Vertice, usize> = HashMap::new();
    let mut recomendacoes = Vec::new();

    fila.push_back((inicio.clone(), 0));
    visitados.insert(inicio, 0);

    while let Some((vertice_atual, distancia)) = fila.pop_front() {
        if distancia >= profundidade_maxima {
            continue;
        }

        let vizinhos = grafo.vizinhos(&vertice_atual);

        for vizinho in vizinhos {
            if visitados.contains_key(&vizinho) {
                continue;
            }

            let nova_distancia = distancia + 1;
            visitados.insert(vizinho.clone(), nova_distancia);

            if let Vertice::Produto(_) = vizinho {
                if nova_distancia == profundidade_maxima {
                    recomendacoes.push(vizinho.clone());
                }
            }

            fila.push_back((vizinho, nova_distancia));
        }
    }

    recomendacoes
}

pub fn construir_produtos_por_cliente(compras: &[Compra]) -> HashMap<u32, Vec<u32>> {
    let mut produtos_por_cliente: HashMap<u32, Vec<u32>> = HashMap::new();

    for compra in compras {
        let produtos = produtos_por_cliente.entry(compra.cliente_id).or_default();
        produtos.extend(compra.produtos.iter().copied());
    }

    for produtos in produtos_por_cliente.values_mut() {
        produtos.sort_unstable();
        produtos.dedup();
    }

    produtos_por_cliente
}

pub fn construir_clientes_por_produto(compras: &[Compra]) -> HashMap<u32, Vec<u32>> {
    let mut clientes_por_produto: HashMap<u32, Vec<u32>> = HashMap::new();

    for compra in compras {
        for produto_id in &compra.produtos {
            clientes_por_produto
                .entry(*produto_id)
                .or_default()
                .push(compra.cliente_id);
        }
    }

    for clientes in clientes_por_produto.values_mut() {
        clientes.sort_unstable();
        clientes.dedup();
    }

    clientes_por_produto
}

pub fn construir_grafo(
    clientes: &[Cliente],
    produtos_por_id: &HashMap<u32, Produto>,
    compras: &[Compra],
) -> Grafo {
    let mut grafo = Grafo::novo();

    for cliente in clientes {
        grafo.adicionar_vertice(Vertice::Cliente(cliente.id));
    }

    for produto_id in produtos_por_id.keys() {
        grafo.adicionar_vertice(Vertice::Produto(*produto_id));
    }

    for compra in compras {
        let cliente = Vertice::Cliente(compra.cliente_id);

        for produto_id in &compra.produtos {
            let produto = Vertice::Produto(*produto_id);

            if produtos_por_id.contains_key(produto_id) {
                grafo.adicionar_aresta(cliente.clone(), produto);
            }
        }
    }

    grafo
}

pub fn gerar_recomendacoes(
    produtos_por_id: &HashMap<u32, Produto>,
    compras: &[Compra],
    cliente_id: u32,
    limite: usize,
) -> Vec<Recomendacao> {
    let produtos_por_cliente = construir_produtos_por_cliente(compras);
    let clientes_por_produto = construir_clientes_por_produto(compras);

    let produtos_alvo = produtos_por_cliente
        .get(&cliente_id)
        .cloned()
        .unwrap_or_default();

    if produtos_alvo.is_empty() {
        return Vec::new();
    }

    let mut fila: VecDeque<u32> = VecDeque::from(produtos_alvo.clone());
    let mut visitados: HashSet<u32> = produtos_alvo.iter().copied().collect();
    let mut candidatos: HashMap<u32, (HashSet<u32>, u32)> = HashMap::new();
    let limite_profundidade = 2usize;

    for _ in 0..limite_profundidade {
        if fila.is_empty() {
            break;
        }

        let tamanho_nivel = fila.len();

        for _ in 0..tamanho_nivel {
            let produto_atual = fila.pop_front().expect("A fila deve conter itens");

            if let Some(clientes_relacionados) = clientes_por_produto.get(&produto_atual) {
                for cliente_relacionado in clientes_relacionados {
                    if *cliente_relacionado == cliente_id {
                        continue;
                    }

                    if let Some(produtos_do_cliente) = produtos_por_cliente.get(cliente_relacionado) {
                        for produto_sugerido in produtos_do_cliente {
                            if visitados.contains(produto_sugerido) {
                                continue;
                            }

                            let entrada = candidatos
                                .entry(*produto_sugerido)
                                .or_insert((HashSet::new(), 0));

                            entrada.0.insert(*cliente_relacionado);

                            if let (Some(produto_original), Some(produto_alvo_sugerido)) = (
                                produtos_por_id.get(&produto_atual),
                                produtos_por_id.get(produto_sugerido),
                            ) {
                                if produto_original.categoria == produto_alvo_sugerido.categoria {
                                    entrada.1 += 1;
                                }
                            }

                            fila.push_back(*produto_sugerido);
                            visitados.insert(*produto_sugerido);
                        }
                    }
                }
            }
        }
    }

    let mut recomendacoes: Vec<Recomendacao> = candidatos
        .into_iter()
        .filter_map(|(produto_id, (clientes_compartilhados, categorias_compartilhadas))| {
            let produto = produtos_por_id.get(&produto_id)?;
            let clientes_compartilhados_total = clientes_compartilhados.len();
            let pontuacao = (clientes_compartilhados_total as u32) * 10 + categorias_compartilhadas * 3;

            Some(Recomendacao {
                produto_id,
                nome: produto.nome.clone(),
                categoria: produto.categoria.clone(),
                pontuacao,
                clientes_compartilhados: clientes_compartilhados_total,
            })
        })
        .collect();

    recomendacoes.sort_by(|esquerda, direita| {
        direita
            .pontuacao
            .cmp(&esquerda.pontuacao)
            .then_with(|| direita.clientes_compartilhados.cmp(&esquerda.clientes_compartilhados))
            .then_with(|| esquerda.produto_id.cmp(&direita.produto_id))
    });

    recomendacoes.truncate(limite);
    recomendacoes
}

pub fn carregar_json<T: for<'de> Deserialize<'de>>(
    caminho: &str,
) -> Result<Vec<T>, Box<dyn Error>> {
    let conteudo = fs::read_to_string(caminho)?;
    let dados: Vec<T> = serde_json::from_str(&conteudo)?;

    Ok(dados)
}

pub fn benchmark_consulta(
    produtos_por_id: &HashMap<u32, Produto>,
    compras: &[Compra],
    cliente_id: u32,
    volumes: &[usize],
) -> Vec<(usize, Duration)> {
    let mut resultados = Vec::new();

    for &volume in volumes {
        let limite = volume.min(compras.len());
        let dados_parciais = &compras[..limite];

        let inicio = Instant::now();
        let _ = gerar_recomendacoes(produtos_por_id, dados_parciais, cliente_id, 5);
        let duracao = inicio.elapsed();

        resultados.push((volume, duracao));
    }

    resultados
}
