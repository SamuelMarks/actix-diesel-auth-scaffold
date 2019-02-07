use ring::{
    rand,
    signature::{self, KeyPair},
};

use error::Error;


pub type KeyPair = ring::signature::KeyPair;

pub fn generate_keypair() -> Result<KeyPair, Error> {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;

    signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(pkcs8_bytes.as_ref()))?
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    access_token: String,
    token_type: TokenType,
    expires_in: u32,
    scope: Scope,

}

type Key = String;

impl Token {
    fn new(expiry: u32, scope: Scope, token_type: TokenType, key: KeyPair) {
        Token {
            access_token: "".to_string(),
            token_type: TokenType::Bearer,
            expires_in: 0,
            scope: Scope::Create
        }
    }

    fn verify(&self, key: Key) -> bool {

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
pub struct Credentials {
    pub username: String,
    pub password: String,
}

