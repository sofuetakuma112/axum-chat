pub struct JWT {
    pub token: String,
}

impl JWT {
    pub fn new(token: &str) -> JWT {
        JWT {
            token: token.to_string(),
        }
    }

    pub fn cookie(&self) -> String {
        // TODO: max-ageを反映する
        format!("token={}; HttpOnly", self.token)
    }

    pub fn clear_cookie() -> String {
        "token=; HttpOnly; Max-Age=0".to_string()
    }
}
