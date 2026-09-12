use sqlx::MySqlPool;

pub async fn init_db(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    println!("Conectando ao banco de dados MariaDB...");
    let pool = MySqlPool::connect(database_url).await?;

    println!("Verificando e criando tabelas...");
    
    // Tabela de Produtos
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id VARCHAR(36) PRIMARY KEY,
            sku VARCHAR(50) NOT NULL UNIQUE,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            barcode VARCHAR(100) NOT NULL,
            category VARCHAR(100) NOT NULL,
            unit ENUM('UN', 'CX', 'KG', 'PALLET') NOT NULL DEFAULT 'UN',
            min_stock INT NOT NULL DEFAULT 0
        );
        "#
    )
    .execute(&pool)
    .await?;

    // Insere dados de exemplo se ainda não existirem
    sqlx::query(
        r#"
        INSERT IGNORE INTO products (id, sku, name, description, barcode, category, unit, min_stock) 
        VALUES 
        ('123e4567-e89b-12d3-a456-426614174000', 'PROD-1001', 'Caixa de Papelão P', 'Caixa para armazenamento pequeno', '7891234567890', 'Embalagens', 'CX', 10),
        ('987e6543-e21b-12d3-a456-426614174999', 'PROD-1002', 'Palete PBR', 'Palete de madeira padrão', '7891234567891', 'Logística', 'PALLET', 5);
        "#
    )
    .execute(&pool)
    .await?;

    println!("Banco de dados e tabelas prontos!");

    Ok(pool)
}