use conectastore::{
    benchmark_consulta, carregar_json, construir_grafo, gerar_recomendacoes, recomendar_bfs,
    Cliente, Compra, Produto, Vertice,
};
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("====================================");
    println!("       CONECTASTORE - MEGASTORE");
    println!("====================================\n");

    let produtos: Vec<Produto> = carregar_json("dados/produtos.json")?;
    let clientes: Vec<Cliente> = carregar_json("dados/clientes.json")?;
    let compras: Vec<Compra> = carregar_json("dados/compras.json")?;

    let mut produtos_por_id: HashMap<u32, Produto> = HashMap::new();

    for produto in produtos {
        produtos_por_id.insert(produto.id, produto);
    }

    let grafo = construir_grafo(&clientes, &produtos_por_id, &compras);

    println!("DADOS CARREGADOS");
    println!("Produtos: {}", produtos_por_id.len());
    println!("Clientes: {}", clientes.len());
    println!("Compras: {}", compras.len());

    println!("\nESTRUTURA DO GRAFO");
    println!("Vértices: {}", grafo.quantidade_vertices());
    println!("Arestas: {}", grafo.quantidade_arestas());

    if let Some(cliente) = clientes.first() {
        let vertice_cliente = Vertice::Cliente(cliente.id);
        let vizinhos = grafo.vizinhos(&vertice_cliente);

        println!("\nCONEXÕES DO CLIENTE");
        println!("Cliente: {} (ID {})", cliente.nome, cliente.id);
        println!("Interesses: {:?}", cliente.interesses);

        for vizinho in &vizinhos {
            if let Vertice::Produto(produto_id) = vizinho {
                if let Some(produto) = produtos_por_id.get(produto_id) {
                    println!(
                        "Comprou: {} | Categoria: {} | Preço: {:.2}",
                        produto.nome,
                        produto.categoria,
                        produto.preco
                    );
                }
            }
        }
    }

    let id_consulta = 1;

    println!("\nCONSULTA DE PRODUTO POR ID");

    match produtos_por_id.get(&id_consulta) {
        Some(produto) => {
            println!("Produto encontrado: {}", produto.nome);
            println!("Categoria: {}", produto.categoria);
            println!("Preço: {:.2}", produto.preco);
        }
        None => {
            println!("Produto não encontrado.");
        }
    }

    let cliente_id = 1;
    let conexoes = recomendar_bfs(&grafo, cliente_id, 3);

    println!("\n====================================");
    println!("CONEXÕES EM PROFUNDIDADE PARA O CLIENTE {}", cliente_id);
    println!("====================================");

    if conexoes.is_empty() {
        println!("Nenhuma conexão encontrada.");
    } else {
        for (indice, vertice) in conexoes.iter().enumerate() {
            if let Vertice::Produto(produto_id) = vertice {
                if let Some(produto) = produtos_por_id.get(produto_id) {
                    println!(
                        "{}. {} | Categoria: {} | Preço: {:.2}",
                        indice + 1,
                        produto.nome,
                        produto.categoria,
                        produto.preco
                    );
                }
            }
        }
    }

    let recomendacoes_rankeadas = gerar_recomendacoes(&produtos_por_id, &compras, cliente_id, 5);

    println!("\n====================================");
    println!("RECOMENDAÇÕES RANQUEADAS PARA O CLIENTE {}", cliente_id);
    println!("====================================");

    if recomendacoes_rankeadas.is_empty() {
        println!("Nenhuma recomendação encontrada.");
    } else {
        for (indice, recomendacao) in recomendacoes_rankeadas.iter().enumerate() {
            println!(
                "{}. {} | Categoria: {} | Pontuação: {} | Clientes compartilhados: {}",
                indice + 1,
                recomendacao.nome,
                recomendacao.categoria,
                recomendacao.pontuacao,
                recomendacao.clientes_compartilhados
            );
        }
    }

    let volumes = [50, 100, 500, 1000, compras.len().min(2000)];
    let benchmark = benchmark_consulta(&produtos_por_id, &compras, cliente_id, &volumes);

    println!("\n====================================");
    println!("BENCHMARK DE CONSULTA");
    println!("====================================");

    for (volume, duracao) in benchmark {
        println!("Volume: {} | Tempo: {:?}", volume, duracao);
    }

    println!("\nExecução concluída.");

    Ok(())
}
