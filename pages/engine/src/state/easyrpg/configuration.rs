use std::sync::Arc;

#[derive(serde::Serialize)]
pub struct Configuration {
    pub game: Arc<str>,
    #[serde(rename = "wsUrl")]
    pub websocket_url: String,
}
