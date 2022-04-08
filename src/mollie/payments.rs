use serde::Deserialize;
use reqwest::StatusCode;
use log::debug;

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
    pub description: String,
    pub amount: PaymentAmount
}

#[derive(Debug, Deserialize)]
pub struct PaymentAmount {
    pub value: String,
    pub currency: String,
}

#[derive(Debug)]
pub enum ApiError {
    CouldNotPerformRequest(reqwest::Error),
    CouldNotUnderstandResponse(reqwest::Error),
    MollieApiReturnedAnError(super::MollieApiError)
}

pub trait PaymentsApi {
    fn get(
        &self,
        url: String,
        parameter: Option<String>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error>;

    fn list_payments(&self) -> Result<ListPaymentsResponse, ApiError> {
        let response = self.get(String::from("v2/payments"), None).map_err(ApiError::CouldNotPerformRequest)?;

        if response.status() == StatusCode::OK {
            let decoded_response = response
            .json::<ListPaymentsResponse>()
            .unwrap();
                debug!("{:?}", decoded_response);

            return Ok(decoded_response);
        }

        let decoded_error_response = response.json::<super::MollieApiError>().map_err(ApiError::CouldNotUnderstandResponse)?;
        return Err(ApiError::MollieApiReturnedAnError(decoded_error_response));
    }

    fn details(&self, payment_id: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let response = self
            .get(String::from("v2/payments"), Some(payment_id))
            .unwrap();

        Ok(response)
    }
}
