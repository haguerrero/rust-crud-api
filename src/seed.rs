use rand::RngExt;
use sqlx::MySqlPool;
use uuid::Uuid;

pub async fn seed_db(pool: &MySqlPool) {
    println!("🌱 Starting seed process...");

    let mut rng = rand::rng();

    let mut tx = pool.begin().await.expect("Failed to start transaction");

    for i in 0..10_000 {
        // UUID v4 → BINARY(16)
        let uuid = Uuid::new_v4();
        let id = uuid.as_bytes(); // &[u8; 16]

        let random_number: u32 = rng.random();
        let email = format!("user{}@test.com", random_number);

        // ⚠ solo para testing
        let password_hash = format!("hash{}", i);

        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(&id[..])
        .bind(&email)
        .bind(&password_hash)
        .execute(&mut *tx)
        .await
        .expect("Failed to insert user");
    }

    tx.commit().await.expect("Failed to commit transaction");

    println!("✅ Seed completed!");
}
