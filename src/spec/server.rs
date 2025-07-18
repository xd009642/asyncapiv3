//! Contains types related to the [servers
//! field](https://www.asyncapi.com/docs/concepts/asyncapi-document/structure#servers-field)
use crate::spec::common::{ExternalDocumentation, RefOr, Tag};
use crate::spec::security::SecurityScheme;
use std::collections::HashMap;

pub type Servers = HashMap<String, RefOr<Server>>;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    /// The server host name. It MAY include the port. This field supports Server Variables. Variable substitutions will be made when a variable is named in {braces}.
    pub host: String,
    /// The protocol this server supports for connection.
    pub protocol: String,
    /// The version of the protocol used for connection. For instance: AMQP 0.9.1, HTTP 2.0, Kafka 1.0.0, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<String>,
    /// The path to a resource in the host. This field supports Server Variables. Variable substitutions will be made when a variable is named in {braces}.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathname: Option<String>,
    /// An optional string describing the server. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A human-friendly title for the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A short summary of the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A map between a variable name and its value. The value is used for substitution in the server's host and pathname template.
    #[serde(default)]
    pub variables: HashMap<String, RefOr<Variable>>,
    /// A declaration of which security schemes can be used with this server. The list of values includes alternative security scheme objects that can be used. Only one of the security scheme objects need to be satisfied to authorize a connection or operation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<RefOr<SecurityScheme>>,
    /// Tags Object A list of tags for logical grouping and categorization of servers.
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Additional external documentation for this server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<RefOr<ExternalDocumentation>>,
    /// A map where the keys describe the name of the protocol and the values describe protocol-specific definitions for the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bindings: Option<RefOr<ServerBindings>>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    /// An enumeration of string values to be used if the substitution options are from a limited set.
    #[serde(rename = "enum")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    /// The default value to use for substitution, and to send, if an alternate value is not supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// An optional description for the server variable. CommonMark syntax MAY be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An array of examples of the server variable.
    #[serde(default)]
    pub examples: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerBindings {
    //TODO: implement server-binding object https://www.asyncapi.com/docs/reference/specification/v3.0.0#serverBindingsObject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws: Option<WebSocketServerBinding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nats: Option<NatsServerBinding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpServerBinding>,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WebSocketServerBinding;

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NatsServerBinding;

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HttpServerBinding;
