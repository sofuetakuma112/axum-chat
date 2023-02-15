use sqlx::{Pool, Postgres};

use crate::entities::account::Account;

use std::collections::{HashMap, HashSet};

pub async fn find(pool: Pool<Postgres>, ids: HashSet<i32>) -> HashMap<i32, Account> {
    if ids.is_empty() {
        return HashMap::new();
    }

    let ids_str = ids
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let accounts =
        sqlx::query_as::<_, Account>(&format!("SELECT * FROM accounts WHERE id in ({})", ids_str))
            .fetch_all(&pool)
            .await
            .unwrap();

    accounts
        .into_iter()
        .map(|account| (account.id().unwrap(), account))
        .collect()
}

/// emailを使ってaccountsからユーザーを検索する
pub async fn find_by(pool: Pool<Postgres>, email: &str) -> Option<Account> {
    sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE email = $1")
        .bind(email)
        .fetch_optional(&pool)
        .await
        .unwrap()
}

/// DBにアカウントレコードを新規追加
pub async fn store(pool: Pool<Postgres>, entity: &Account) {
    sqlx::query("INSERT INTO accounts (email, password, display_name) VALUES ($1, $2, $3)")
        .bind(&entity.email)
        .bind(&entity.hashed_password)
        .bind(&entity.display_name)
        .execute(&pool)
        .await
        .unwrap();
}