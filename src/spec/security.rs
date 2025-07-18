//! Represents the AsyncAPI security property as well as the various security schemes supported in
//! the specification.
use std::collections::HashMap;

/// You can describe how your server is secured with the security property where you define
/// which security schemes can be used with the server in context. Each server in the
/// AsyncAPI document can have one or more security schemes declared. A security scheme
/// defines a security requirement that must be satisfied to authorize an operation, such as an
/// API key or a username and password.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_password: Option<UserPasswordSecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<ApiKeySecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509: Option<X509SecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symmetric_encryption: Option<SymmetricEncryptionSecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asymmetric_encryption: Option<AsymmetricEncryptionSecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_api_key: Option<HttpApiKeySecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpSecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<Oauth2SecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id_connect: Option<OpenIdConnectSecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plain: Option<PlainSecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scram_sha256: Option<ScramSha256SecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scram_sha512: Option<ScramSha512SecurityScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gssapi: Option<GssapiSecurityScheme>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPasswordSecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeySecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The location of the API key. Valid values are "user" and "password" for apiKey and "query", "header" or "cookie" for httpApiKey.
    #[serde(rename = "in")]
    pub location: ApiKeyLocation,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct X509SecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymmetricEncryptionSecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsymmetricEncryptionSecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpApiKeySecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the header, query or cookie parameter to be used.
    pub name: String,
    /// The location of the API key. Valid values are "user" and "password" for apiKey and "query", "header" or "cookie" for httpApiKey.
    #[serde(rename = "in")]
    pub location: HttpApiKeyLocation,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpSecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the HTTP Authorization scheme to be used in the [Authorization header as defined in RFC7235](https://tools.ietf.org/html/rfc7235#section-5.1).
    pub scheme: String,
    /// A hint to the client to identify how the bearer token is formatted. Bearer tokens are usually generated by an authorization server, so this information is primarily for documentation purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Oauth2SecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An object containing configuration information for the flow types supported.
    pub flows: OAuthFlows,
    /// List of the needed scope names. An empty array means no scopes are needed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenIdConnectSecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// OpenId Connect URL to discover OAuth2 configuration values. This MUST be in the form of an absolute URL.
    pub open_id_connect_url: String,
    /// List of the needed scope names. An empty array means no scopes are needed.
    #[serde(default)]
    pub scopes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainSecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScramSha256SecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScramSha512SecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GssapiSecurityScheme {
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlows {
    /// Configuration for the OAuth Implicit flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implicit: Option<ImplicitOAuthFlow>,
    /// Configuration for the OAuth Resource Owner Protected Credentials flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<PasswordOAuthFlow>,
    /// Configuration for the OAuth Client Credentials flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<ClientCredentialsOAuthFlow>,
    /// Configuration for the OAuth Authorization Code flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<AuthorizationCodeOAuthFlow>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitOAuthFlow {
    /// The authorization URL to be used for this flow. This MUST be in the form of an absolute URL.
    authorization_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it.
    available_scopes: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordOAuthFlow {
    /// The token URL to be used for this flow. This MUST be in the form of an absolute URL.
    token_url: String,
    /// The URL to be used for obtaining refresh tokens. This MUST be in the form of an absolute URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it.
    available_scopes: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCredentialsOAuthFlow {
    /// The token URL to be used for this flow. This MUST be in the form of an absolute URL.
    token_url: String,
    /// The URL to be used for obtaining refresh tokens. This MUST be in the form of an absolute URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it.
    available_scopes: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationCodeOAuthFlow {
    /// The authorization URL to be used for this flow. This MUST be in the form of an absolute URL.
    authorization_url: String,
    /// The token URL to be used for this flow. This MUST be in the form of an absolute URL.
    token_url: String,
    /// The URL to be used for obtaining refresh tokens. This MUST be in the form of an absolute URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it.
    available_scopes: HashMap<String, String>,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiKeyLocation {
    User,
    Password,
}

/// Represents where the users API key is located.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HttpApiKeyLocation {
    /// Located in the HTTP query string e.g. `?api_key=<KEY>`
    Query,
    /// Located in a HTTP Header for example the `Authorization` header
    Header,
    /// Located in a session cookie
    Cookie,
}
