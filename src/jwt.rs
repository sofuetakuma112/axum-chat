pub struct JWT {
    pub token: String,
}

impl JWT {
    pub fn new(token: &str) -> JWT {
        JWT {
            token: token.to_string(),
        }
    }

    // pub fn clear() -> JWT {
    //     JWT {
    //         token: "deleted".to_string(),
    //     }
    // }
}

impl JWT {
    pub fn cookie(&self) -> String {
        format!("token={}; Path=/; HttpOnly", self.token)
    }
}
