use log::debug;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListPaymentsResponse {
    pub count: i32,
    #[serde(rename(deserialize = "_embedded"))]
    pub embedded: PaymentResources,
}

#[derive(Debug, Deserialize)]
pub struct PaymentResources {
    pub payments: Vec<PaymentResource>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentResource {
    pub id: String,
    pub mode: String,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: String,
    pub status: String,
    #[serde(rename(deserialize = "isCancelable"))]
    pub is_cancelable: Option<bool>,
    #[serde(rename(deserialize = "authorizedAt"))]
    pub authorized_at: Option<String>,
    #[serde(rename(deserialize = "paidAt"))]
    pub paid_at: Option<String>,
    #[serde(rename(deserialize = "expiresAt"))]
    pub expires_at: Option<String>,
    #[serde(rename(deserialize = "expiredAt"))]
    pub expired_at: Option<String>,
    #[serde(rename(deserialize = "failedAt"))]
    pub failed_at: Option<String>,
    pub amount: Amount,
    #[serde(rename(deserialize = "amountRefunded"))]
    pub amount_refunded: Option<Amount>,
    #[serde(rename(deserialize = "amountRemaining"))]
    pub amount_remaining: Option<Amount>,
    #[serde(rename(deserialize = "amountCaptured"))]
    pub amount_captured: Option<Amount>,
    #[serde(rename(deserialize = "amountChargedBack"))]
    pub amount_chargedback: Option<Amount>,
    #[serde(rename(deserialize = "settlementAmount"))]
    pub settlement_amount: Option<Amount>,
    pub description: String,
    #[serde(rename(deserialize = "redirectUrl"))]
    pub redirect_url: String,
    #[serde(rename(deserialize = "webhookUrl"))]
    pub webhook_url: Option<String>,
    pub locale: Option<String>,
    #[serde(rename(deserialize = "countryCode"))]
    pub country_code: Option<String>,
    pub method: String,
    #[serde(rename(deserialize = "restrictPaymentMethodsToCountry"))]
    pub restrict_payment_methods_to_country: Option<String>,
    #[serde(rename(deserialize = "profileId"))]
    pub profile_id: String,
    #[serde(rename(deserialize = "settlementId"))]
    pub settlement_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

impl PaymentsApi for super::ApiClient {
    fn get(
        &self,
        url: String,
        parameter: Option<String>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.get(url, parameter)
    }
}

pub trait PaymentsApi {
    fn get(
        &self,
        url: String,
        parameter: Option<String>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error>;

    fn list_payments(&self) -> Result<ListPaymentsResponse, super::errors::ApiClientError> {
        let response = self
            .get(String::from("v2/payments"), None)
            .map_err(super::errors::ApiClientError::CouldNotPerformRequest)?;

        if response.status() == StatusCode::OK {
            let decoded_response = response
                .json::<ListPaymentsResponse>()
                .map_err(super::errors::ApiClientError::CouldNotUnderstandResponse)?;
            debug!("{:?}", decoded_response);

            return Ok(decoded_response);
        }

        let decoded_error_response = response
            .json::<super::MollieApiError>()
            .map_err(super::errors::ApiClientError::CouldNotUnderstandResponse)?;
        return Err(super::errors::ApiClientError::MollieApiReturnedAnError(
            decoded_error_response,
        ));
    }

    fn get_payment_details(
        &self,
        payment_id: &String,
    ) -> Result<PaymentResource, super::errors::ApiClientError> {
        let response = self
            .get(String::from("v2/payments"), Some(String::from(payment_id)))
            .map_err(super::errors::ApiClientError::CouldNotPerformRequest)?;

        if response.status() == StatusCode::OK {
            let decoded_response = response
                .json::<PaymentResource>()
                .map_err(super::errors::ApiClientError::CouldNotUnderstandResponse)?;
            debug!("{:?}", decoded_response);

            return Ok(decoded_response);
        }

        let decoded_error_response = response
            .json::<super::MollieApiError>()
            .map_err(super::errors::ApiClientError::CouldNotUnderstandResponse)?;
        return Err(super::errors::ApiClientError::MollieApiReturnedAnError(
            decoded_error_response,
        ));
    }
}
