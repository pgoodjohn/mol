use std::collections::HashMap;

use serde::Deserialize;

use super::{address::Address, link::Link};

/// Organization model; More information:
/// - <https://docs.mollie.com/reference/v2/organizations-api/get-organization#response>
#[derive(Debug, Deserialize)]
pub struct Organization {
    /// Unique identifier of the organization
    pub id: String,

    /// Name of the organization
    pub name: Option<String>,

    /// Email address associated with the organization
    pub email: String,

    /// Preferred locale of the merchant
    /// Can be defined on Mollie dashboard
    pub locale: String,

    /// Registration number of the organization at the chamber of commerce
    #[serde(rename(deserialize = "registrationNumber"))]
    pub registration_number: Option<String>,

    /// VAT number of the organization
    #[serde(rename(deserialize = "vatNumber"))]
    pub vat_number: Option<String>,

    /// Address of the organization
    pub address: Address,

    /// VAT regulation if based in the EU
    /// Can be either "shifted" or "dutch"
    #[serde(rename(deserialize = "vatRegulation"))]
    pub vat_regulation: Option<String>,

    #[serde(rename(deserialize = "_links"))]
    pub links: HashMap<String, Link>,
}
