CREATE TABLE empresas (
    id_empresa INT(7) AUTO_INCREMENT, #0 000 000
    nome_empresa VARCHAR(50) NOT NULL,
    email VARCHAR(65) NOT NULL UNIQUE,
    senha VARCHAR(50) NOT NULL,
        PRIMARY KEY (id_empresa)
);
