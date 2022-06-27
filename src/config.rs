#[derive(serde::Deserialize, std::fmt::Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfig {
    pub steam_api_key: String,
}
