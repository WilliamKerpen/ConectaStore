# ConectaStore

Sistema de recomendação de produtos baseado em grafos em Rust.

## 1. Visão geral

A MegaStore é uma empresa de comércio eletrônico com um catálogo amplo, incluindo eletrônicos, roupas, livros, alimentos, móveis e itens de decoração. Com o crescimento do volume de produtos, o sistema tradicional passou a sugerir itens genéricos e pouco relevantes, sem explorar as relações entre clientes, compras e categorias.

O projeto ConectaStore foi desenvolvido para demonstrar como grafos e estruturas complementares podem ser usados para criar um mecanismo de recomendação mais inteligente, considerando o comportamento de compra e a similaridade entre clientes e produtos.

A solução foi implementada em Rust com foco em:

- relacionamento entre clientes e produtos por grafos;
- acesso rápido por identificador com HashMap;
- navegação por vizinhos com BFS;
- eliminação de itens duplicados nas recomendações;
- ordenação por relevância;
- testes automatizados;
- benchmark de desempenho.

---

## 2. Contexto do problema

Os marketplaces modernos lidam com grandes catálogos e alta variedade de produtos. Em cenários dessa escala, recomendações baseadas apenas em popularidade ou categoria podem gerar sugestões pouco personalizadas.

A proposta do ConectaStore é mostrar uma alternativa baseada em conexões reais entre clientes e itens comprados. A ideia central é que produtos que aparecem em vizinhanças relevantes do cliente tenham maior chance de serem úteis e interessantes.

---

## 3. Objetivo do projeto

O objetivo principal é demonstrar, de forma prática, como estruturas de dados e algoritmos podem ser aplicados para gerar recomendações relevantes em um catálogo de grande volume.

A solução utiliza:

- clientes e produtos como vértices;
- compras como arestas;
- busca por vizinhos para descobrir itens conectados;
- contagem de clientes compartilhados para medir relevância;
- ordenação dos resultados para priorizar os itens mais prováveis de interesse.

---

## 4. Arquitetura da solução

A arquitetura combina grafos e estruturas auxiliares:

### 4.1 Grafo

- clientes são vértices;
- produtos são vértices;
- compras conectam cliente e produto;
- a navegação entre vizinhos permite descobrir itens relacionados.

### 4.2 Estruturas complementares

- HashMap para pesquisa rápida por identificador;
- HashMap para mapear produtos por cliente;
- HashMap para mapear clientes por produto;
- VecDeque para explorar a busca em largura;
- HashSet para evitar duplicação e visitas repetidas.

Essas estruturas foram escolhidas para equilibrar:

- eficiência de consulta;
- simplicidade de implementação;
- uso de memória;
- escalabilidade.

---

## 5. Estrutura do repositório

```text
ConectaStore/
├── README.md
├── requirements.txt
├── megastore/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── src/
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── tests/
│   │   └── recomendacao_test.rs
│   ├── dados/
│   │   ├── clientes.json
│   │   ├── compras.json
│   │   └── produtos.json
│   ├── gerador/
│   │   └── gerar_dados.py
│   └── requirements.txt
```

---

## 6. Pré-requisitos

Antes de executar o projeto, verifique se o ambiente possui:

- Python 3.x
- Rust e Cargo instalados
- Git

---

## 7. Como clonar e executar localmente

### 7.1 Clonar o projeto

```bash
git clone <url-do-repositorio>
cd ConectaStore
```

### 7.2 Criar ambiente virtual Python

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

Se preferir, também é possível criar o ambiente dentro da pasta do projeto Rust:

```bash
cd megastore
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

### 7.3 Gerar os dados

O gerador em Python cria dados em JSON para produtos, clientes e compras.

```bash
cd megastore/gerador
python3 gerar_dados.py
```

O volume gerado no projeto atual inclui:

- 10.000 produtos;
- 3.000 clientes;
- 30.000 compras.

Esse conjunto simula melhor um catálogo de e-commerce em escala real.

### 7.4 Compilar e executar o Rust

```bash
cd ../
cargo build
cargo run
```

### 7.5 Executar testes

```bash
cargo test
```

---

## 8. Como o gerador funciona

O gerador em Python cria dados em massa com categorias e padrões realistas, incluindo:

- categorias como Eletronicos, Livros, Roupas, Casa e Esportes;
- produtos com nomes, categorias e preços aleatórios;
- clientes com interesses em uma ou mais categorias;
- compras simuladas com base nos interesses do cliente;
- persistência dos dados em arquivos JSON dentro de `megastore/dados`.

### Arquivo principal

- `megastore/gerador/gerar_dados.py`

### Dependências do Python

O projeto usa apenas a biblioteca padrão do Python, então não há dependências externas pesadas.

Arquivos:

- `requirements.txt`
- `megastore/requirements.txt`

---

## 9. Algoritmo de recomendação

O sistema usa busca em largura para explorar o grafo a partir do cliente de origem. Em seguida, calcula uma pontuação para cada produto candidato com base em:

- número de clientes que compartilham o produto;
- categoria em comum com produtos já comprados;
- proximidade no grafo.

A pontuação foi definida como:

- clientes compartilhados × 10;
- afinidade por categoria × 3.

Depois, os resultados são ordenados em ordem decrescente e limitados por quantidade.

---

## 10. Testes automatizados

Os testes de integração estão em:

- `megastore/tests/recomendacao_test.rs`

Eles validam:

- ordenação das recomendações;
- ausência de duplicidade;
- consistência da lógica de pontuação;
- respeitar o limite máximo de itens retornados.

Comando:

```bash
cargo test
```

---

## 11. Benchmark de desempenho

O projeto também mede o tempo de execução da consulta em diferentes volumes de dados.

### Resultados obtidos no ambiente atual

| Volume de dados | Tempo de execução |
| --- | --- |
| 50 compras | 233.521 µs |
| 100 compras | 490.877 µs |
| 500 compras | 3.865123 ms |
| 1000 compras | 19.927189 ms |
| 2000 compras | 53.14176 ms |

Esses resultados mostram que o custo cresce conforme o volume aumenta, mas continua em uma faixa aceitável para uma solução de demonstração em Rust.

---

## 12. Exemplo de execução

```text
RECOMENDAÇÕES RANQUEADAS PARA O CLIENTE 1
====================================
1. Smartphone Modelo 3 | Categoria: Eletronicos | Pontuação: 13 | Clientes compartilhados: 1
2. Notebook Modelo 4 | Categoria: Eletronicos | Pontuação: 13 | Clientes compartilhados: 1
3. Colchonete Modelo 7 | Categoria: Esportes | Pontuação: 13 | Clientes compartilhados: 1
4. Monitor Modelo 9 | Categoria: Eletronicos | Pontuação: 13 | Clientes compartilhados: 1
5. Camiseta Modelo 11 | Categoria: Roupas | Pontuação: 13 | Clientes compartilhados: 1
```

---

## 13. Observações de escalabilidade

A combinação de grafo + HashMap é adequada para aplicações de grande porte porque:

- o grafo representa relações e conexões;
- o HashMap reduz o custo de consulta por identificador;
- a busca por vizinhos evita percorrer o catálogo completo sem necessidade;
- o sistema pode evoluir com pesos, avaliações, perfis de cliente e afinidades por categoria.

---

## 14. Vídeo pitch

Link do vídeo pitch: será adicionado após a publicação do vídeo público do projeto.

---

## 15. Conclusão

O ConectaStore demonstra, de forma prática, como grafos, estruturas complementares e algoritmos de busca podem ser usados para gerar recomendações relevantes em um catálogo grande. A solução foi pensada para mostrar não apenas a funcionalidade, mas também a escolha consciente das estruturas de dados em função de eficiência, escalabilidade e manutenção.

O projeto está funcional, validado por testes e pronto para servir como base de apresentação, documentação e evolução futura.
