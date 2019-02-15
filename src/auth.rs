use ring::{
    rand,
    signature::{self, KeyPair},
};

use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, Header, Validation, TokenData};

pub fn generate_oauth_secret() -> String {
    "secret".to_string()
}

pub fn generate_token(claims: Claims, key: String) -> String {
    encode(&Header::default(), &claims, key.as_ref()).unwrap()
}

pub fn is_token_valid(token: String, key: String) -> TokenData<Claims> {
    let validation = Validation::default();

    let token_data = decode::<Claims>(&token, key.as_ref(), &validation);
    token_data.unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: TokenType,
    pub expires_in: u32,
}

type Key = String;

impl Token {
    fn new(expiry: u32, scope: Scope, token_type: TokenType, key: KeyPair) -> Token {
        Token {
            access_token: "".to_string(),
            token_type: TokenType::Bearer,
            expires_in: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    Bearer
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Scope {
    Create
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCredentials {
    pub client_id: String,
    pub client_secret: String,
}
