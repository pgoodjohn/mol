use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Address {
    pub country: String,
    #[serde(rename(deserialize = "streetAndNumber"))]
    pub street_and_number: Option<String>,
    #[serde(rename(deserialize = "postalCode"))]
    pub postal_code: Option<String>,
    pub city: Option<String>,
}
