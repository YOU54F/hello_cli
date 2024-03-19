use pact_models::http_utils::HttpAuth;

#[derive(Clone)]
pub struct BrokerDetails {
    pub(crate) auth: Option<HttpAuth>,
    pub(crate) url: String,
}
#[derive(Clone)]
pub enum OutputType {
    Json,
    Table,
    Text,
    Pretty,
}
