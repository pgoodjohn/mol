use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Partner {
    /// Type of partner. Can be null if authenticated organization is not an enrolled partner.
    #[serde(rename(deserialize = "partnerType"))]
    pub partner_type: Option<String>,

    /// True if receiving comissions
    #[serde(rename(deserialize = "isCommissionPartner"))]
    pub is_commission_partner: Option<bool>,

    /// array containing the user agent tokens if partner is of type `useragent`.
    /// or if the partner has had user agent tokens in the past.
    #[serde(rename(deserialize = "userAgentTokens"))]
    pub user_agent_tokens: Option<Vec<UserAgentToken>>,

    /// Date and time at which the partner contract was signed. [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename(deserialize = "partnerContractSignedAt"))]
    pub partner_contract_signed_at: Option<NaiveDateTime>,

    /// True if contract update is available
    #[serde(rename(deserialize = "partnerContractUpdateAvailable"))]
    pub partner_contract_update_available: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UserAgentToken {
    /// Unique user agent token
    pub token: String,

    /// Date and time from which the token is active. [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    pub starts_at: NaiveDateTime,

    /// Date and time at which the token expires. [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    /// Can be null if date is not set.
    pub ends_at: Option<NaiveDateTime>,
}
