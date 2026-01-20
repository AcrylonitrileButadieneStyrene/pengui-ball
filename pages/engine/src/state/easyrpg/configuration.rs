#[derive(serde::Serialize)]
pub struct Configuration {
    pub game: String,
    #[serde(rename = "wsUrl")]
    pub websocket_url: String,
}
