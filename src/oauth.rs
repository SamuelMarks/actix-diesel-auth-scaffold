#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    access_token: String,
    token_type: TokenType,
    expires_in: u32,
    scope: Scope,
}

type Key = String;

impl Token {
    fn new(expiry: u32, scope: Scope, token_type: TokenType, key: Key) {
        Token {
            access_token: "".to_string(),
            token_type: TokenType::Bearer,
            expires_in: 0,
            scope: Scope::Create,
        }
    }

    fn verify(&self, key: Key) -> bool {}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    Bearer,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Scope {
    Create,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
