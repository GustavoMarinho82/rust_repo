CREATE TABLE empresas_produtos (
    empresa_id INT(7) NOT NULL,
    produto_id INT(7) NOT NULL,
        FOREIGN KEY (empresa_id) REFERENCES empresas (id_empresa),
        FOREIGN KEY (produto_id) REFERENCES produtos (id_produto)
);
