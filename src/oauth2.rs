//This will be extracted out into a seperate reusable crate
pub trait AuthorisationServer {
    fn get_access_token(AuthorizationGrant) -> Result<(AccessToken, Option<RefreshToken>), Error>;
    fn refresh_access_token(RefreshToken) -> Result<AccessToken, Error>;
    fn register_client(ClientRegistration) -> Result<(), String>;
}

pub trait ResourceOwner {
    fn get_authorization_grant(AuthorizationRequest) -> Result<AuthorizationGrant, Error>;
}

pub struct AccessToken(String);
pub struct RefreshToken(String);
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
