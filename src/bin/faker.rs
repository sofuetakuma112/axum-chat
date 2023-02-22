use std::env;

use fake::locales::{JA_JP, EN};
use fake::{Dummy, Fake, Faker};

use fake::faker::internet::raw::{FreeEmail, Username, Password};
use sha2::{Sha256, Digest};
use sqlx::{QueryBuilder, Postgres};

struct User {
    name: String,
    user_id: String,
    email: String,
    password: String
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let users = (0..100).into_iter().map(|_| {
        let name: String = Username(EN).fake();
        let user_id: String = name.clone();
        let email: String = FreeEmail(EN).fake();
        let password: String = "test".to_string();

        User {
            name,
            user_id,
            email,
            password
        }
    }).collect::<Vec<User>>();

    // 参照: https://docs.rs/sqlx-core/0.5.13/sqlx_core/query_builder/struct.QueryBuilder.html#method.push_values
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO users (name, user_id, email, hashed_password) ",
    );

    query_builder.push_values(users.into_iter(), |mut b, user| {
        b.push_bind(user.name).push_bind(user.user_id).push_bind(user.email)
            .push_bind(to_sha256(&user.password));
    });

    let query = query_builder.build();

    query.execute(&pool).await.unwrap();
}

fn to_sha256(str: &str) -> String {
    let str = str.as_bytes();
    let hashed_str = Sha256::digest(str);
    format!("{:x}", hashed_str)
}