
import json
import random
from pathlib import Path

# Define uma semente fixa para que os dados sejam reproduzidos
# da mesma maneira sempre que o programa for executado.
random.seed(42)

# Localiza a pasta raiz do projeto a partir deste arquivo.
PASTA_PROJETO = Path(__file__).resolve().parent.parent
PASTA_DADOS = PASTA_PROJETO / "dados"

# Cria a pasta de dados caso ela ainda não exista.
PASTA_DADOS.mkdir(parents=True, exist_ok=True)

# Define a quantidade de registros que serão gerados.
# Os valores foram ampliados para simular um catálogo realista
# de e-commerce com volume maior de produtos, clientes e compras.
QUANTIDADE_PRODUTOS = 10000
QUANTIDADE_CLIENTES = 3000
QUANTIDADE_COMPRAS = 30000

# Define as categorias disponíveis e exemplos de produtos.
CATEGORIAS = {
    "Eletronicos": [
        "Notebook", "Smartphone", "Monitor", "Teclado",
        "Mouse", "Fone de ouvido", "Camera", "Tablet"
    ],
    "Livros": [
        "Livro de Python", "Livro de Rust", "Romance",
        "Livro de Historia", "Livro de Ciencia"
    ],
    "Roupas": [
        "Camiseta", "Calca", "Jaqueta", "Tenis", "Moletom"
    ],
    "Casa": [
        "Lampada", "Cadeira", "Mesa", "Almofada",
        "Estante", "Ventilador"
    ],
    "Esportes": [
        "Bola", "Bicicleta", "Halter", "Colchonete",
        "Garrafa esportiva"
    ]
}

# Cria uma lista com todas as categorias disponíveis.
LISTA_CATEGORIAS = list(CATEGORIAS.keys())


def gerar_produtos():
    """Gera produtos com identificadores, nomes, preços e categorias."""

    produtos = []

    for identificador in range(1, QUANTIDADE_PRODUTOS + 1):
        # Escolhe uma categoria aleatória para o produto.
        categoria = random.choice(LISTA_CATEGORIAS)

        # Escolhe um nome de produto da categoria selecionada.
        nome_base = random.choice(CATEGORIAS[categoria])

        produto = {
            "id": identificador,
            "nome": f"{nome_base} Modelo {identificador}",
            "categoria": categoria,
            "preco": round(random.uniform(10.0, 3000.0), 2)
        }

        produtos.append(produto)

    return produtos


def gerar_clientes():
    """Gera clientes com interesses em uma ou mais categorias."""

    clientes = []

    for identificador in range(1, QUANTIDADE_CLIENTES + 1):
        # Cada cliente recebe entre uma e três categorias de interesse.
        quantidade_interesses = random.randint(1, 3)

        interesses = random.sample(
            LISTA_CATEGORIAS,
            quantidade_interesses
        )

        cliente = {
            "id": identificador,
            "nome": f"Cliente {identificador:04d}",
            "interesses": interesses
        }

        clientes.append(cliente)

    return clientes


def gerar_compras(produtos, clientes):
    """Gera compras priorizando produtos das categorias de interesse."""

    compras = []

    # Organiza os produtos por categoria para facilitar a seleção.
    produtos_por_categoria = {}

    for produto in produtos:
        categoria = produto["categoria"]

        produtos_por_categoria.setdefault(categoria, []).append(
            produto["id"]
        )

    # Cria uma lista de IDs de clientes para selecionar compradores.
    ids_clientes = [cliente["id"] for cliente in clientes]

    # Cria um índice para consultar os interesses de cada cliente.
    interesses_por_cliente = {
        cliente["id"]: cliente["interesses"]
        for cliente in clientes
    }

    for identificador in range(1, QUANTIDADE_COMPRAS + 1):
        # Escolhe o cliente responsável pela compra.
        cliente_id = random.choice(ids_clientes)

        interesses = interesses_por_cliente[cliente_id]

        # Define quantos produtos serão incluídos na compra.
        quantidade_itens = random.randint(1, 5)

        # Separa produtos de categorias de interesse do cliente.
        produtos_interessantes = []

        for categoria in interesses:
            produtos_interessantes.extend(
                produtos_por_categoria[categoria]
            )

        # Em 80% dos casos, seleciona produtos de categorias
        # pelas quais o cliente demonstrou interesse.
        # Nos demais casos, seleciona produtos do catálogo inteiro.
        if random.random() < 0.8:
            candidatos = produtos_interessantes
        else:
            candidatos = [
                produto["id"] for produto in produtos
            ]

        # Evita repetir o mesmo produto dentro de uma compra.
        quantidade_itens = min(quantidade_itens, len(candidatos))
        itens = random.sample(candidatos, quantidade_itens)

        compra = {
            "id": identificador,
            "cliente_id": cliente_id,
            "produtos": itens
        }

        compras.append(compra)

    return compras


def salvar_json(nome_arquivo, dados):
    """Salva os dados em JSON com indentação para facilitar a leitura."""

    caminho = PASTA_DADOS / nome_arquivo

    with caminho.open("w", encoding="utf-8") as arquivo:
        json.dump(
            dados,
            arquivo,
            ensure_ascii=False,
            indent=2
        )

    print(f"Arquivo criado: {caminho}")


def main():
    """Executa a geração e salva os três conjuntos de dados."""

    print("Iniciando a geração dos dados do ConectaStore...")

    produtos = gerar_produtos()
    clientes = gerar_clientes()
    compras = gerar_compras(produtos, clientes)

    salvar_json("produtos.json", produtos)
    salvar_json("clientes.json", clientes)
    salvar_json("compras.json", compras)

    print("\nGeração concluída!")
    print(f"Total de produtos: {len(produtos)}")
    print(f"Total de clientes: {len(clientes)}")
    print(f"Total de compras: {len(compras)}")


if __name__ == "__main__":
    main()