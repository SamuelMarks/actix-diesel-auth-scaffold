use actix_web::Error;

//This will be extracted out into a seperate reusable crate
pub trait AuthorisationServer {
    fn get_access_token(auth_grant: AuthorizationGrant) -> Result<(AccessToken, Option<RefreshToken>), Error>;
    fn refresh_access_token(refresh_token: RefreshToken) -> Result<AccessToken, Error>;
    fn register_client(client_reg: ClientRegistration) -> Result<(), String>;
}

pub trait ResourceOwner {
    fn get_authorization_grant(auth_req: AuthorizationRequest) -> Result<AuthorizationGrant, Error>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken(pub String);
pub struct RefreshToken(pub String);

pub struct Authorization {
    application_name: String,
    grant_types: Vec<String>,

}

pub struct AuthorizationGrant;

pub struct AuthorizationRequest {
    response_type: String,
    client_id: String,
    redirect_uri: String,
    scope: String,
    state: String,
}

pub struct ClientRegistration {
    application_name: String,
    redirect_urls: Vec<String>,
    grant_types: Vec<String>,
}
