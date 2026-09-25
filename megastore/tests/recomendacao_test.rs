use conectastore::{gerar_recomendacoes, Compra, Produto};
use std::collections::HashMap;

#[test]
fn deve_ordenar_recomendacoes_por_pontuacao() {
    let compras = vec![
        Compra {
            id: 1,
            cliente_id: 10,
            produtos: vec![101, 102],
        },
        Compra {
            id: 2,
            cliente_id: 11,
            produtos: vec![101, 203],
        },
        Compra {
            id: 3,
            cliente_id: 12,
            produtos: vec![203, 204],
        },
        Compra {
            id: 4,
            cliente_id: 13,
            produtos: vec![204],
        },
    ];

    let produtos = vec![
        Produto {
            id: 101,
            nome: "A".to_string(),
            categoria: "Eletronicos".to_string(),
            preco: 10.0,
        },
        Produto {
            id: 102,
            nome: "B".to_string(),
            categoria: "Livros".to_string(),
            preco: 20.0,
        },
        Produto {
            id: 203,
            nome: "C".to_string(),
            categoria: "Eletronicos".to_string(),
            preco: 30.0,
        },
        Produto {
            id: 204,
            nome: "D".to_string(),
            categoria: "Esportes".to_string(),
            preco: 40.0,
        },
    ];

    let produtos_por_id: HashMap<u32, Produto> = produtos.into_iter().map(|p| (p.id, p)).collect();
    let recomendacoes = gerar_recomendacoes(&produtos_por_id, &compras, 10, 5);

    assert!(!recomendacoes.is_empty());
    assert_eq!(recomendacoes[0].produto_id, 203);
    assert!(recomendacoes[0].pontuacao >= recomendacoes[1].pontuacao);
    assert_eq!(recomendacoes.len(), 2);
}

#[test]
fn deve_evitar_recomendacoes_duplicadas() {
    let compras = vec![
        Compra {
            id: 1,
            cliente_id: 10,
            produtos: vec![101, 102],
        },
        Compra {
            id: 2,
            cliente_id: 11,
            produtos: vec![101, 203],
        },
        Compra {
            id: 3,
            cliente_id: 12,
            produtos: vec![101, 203],
        },
    ];

    let produtos = vec![
        Produto {
            id: 101,
            nome: "A".to_string(),
            categoria: "Eletronicos".to_string(),
            preco: 100.0,
        },
        Produto {
            id: 102,
            nome: "B".to_string(),
            categoria: "Livros".to_string(),
            preco: 200.0,
        },
        Produto {
            id: 203,
            nome: "C".to_string(),
            categoria: "Eletronicos".to_string(),
            preco: 300.0,
        },
    ];

    let produtos_por_id: HashMap<u32, Produto> = produtos.into_iter().map(|p| (p.id, p)).collect();
    let recomendacoes = gerar_recomendacoes(&produtos_por_id, &compras, 10, 10);
    let ids: Vec<u32> = recomendacoes.iter().map(|r| r.produto_id).collect();

    assert_eq!(ids.iter().filter(|&&id| id == 203).count(), 1);
    assert_eq!(ids.len(), 1);
}

#[test]
fn deve_respeitar_limite_maximo_de_recomendacoes() {
    let compras = vec![
        Compra {
            id: 1,
            cliente_id: 10,
            produtos: vec![101],
        },
        Compra {
            id: 2,
            cliente_id: 11,
            produtos: vec![101, 201],
        },
        Compra {
            id: 3,
            cliente_id: 12,
            produtos: vec![101, 202],
        },
        Compra {
            id: 4,
            cliente_id: 13,
            produtos: vec![101, 203],
        },
        Compra {
            id: 5,
            cliente_id: 14,
            produtos: vec![101, 204],
        },
    ];

    let produtos = vec![
        Produto {
            id: 101,
            nome: "A".to_string(),
            categoria: "Eletronicos".to_string(),
            preco: 100.0,
        },
        Produto {
            id: 201,
            nome: "B".to_string(),
            categoria: "Eletronicos".to_string(),
            preco: 150.0,
        },
        Produto {
            id: 202,
            nome: "C".to_string(),
            categoria: "Eletronicos".to_string(),
            preco: 180.0,
        },
        Produto {
            id: 203,
            nome: "D".to_string(),
            categoria: "Eletronicos".to_string(),
            preco: 220.0,
        },
        Produto {
            id: 204,
            nome: "E".to_string(),
            categoria: "Eletronicos".to_string(),
            preco: 250.0,
        },
    ];

    let produtos_por_id: HashMap<u32, Produto> = produtos.into_iter().map(|p| (p.id, p)).collect();
    let recomendacoes = gerar_recomendacoes(&produtos_por_id, &compras, 10, usize::MAX);

    assert!(recomendacoes.len() <= 4);
    assert!(recomendacoes.len() > 0);
    assert!(recomendacoes.windows(2).all(|janela| janela[0].pontuacao >= janela[1].pontuacao));
}
