use std::env;

/// .envのDATABASE_URLに対応する値を返す
pub fn database_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}
