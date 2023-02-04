CREATE TABLE produtos (
    id_produto INT(7) AUTO_INCREMENT,
    codigo_produto VARCHAR(9) NOT NULL UNIQUE,	#ABC 000 000
    nome_produto VARCHAR(50) NOT NULL UNIQUE,
    qtd_estoque INT(9) NOT NULL,	#000 000 000
    preco DECIMAL(9, 2) NOT NULL,	#1234567.89
        PRIMARY KEY (id_produto)
);