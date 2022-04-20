use log::debug;
use reqwest::StatusCode;
use serde::{ser, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

#[derive(Serialize, Debug)]
pub struct RefundPaymentRequest {
    pub amount: Amount,
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RefundResource {
    pub id: String,
    amount: Amount,
    status: String,
    created_at: String,
    description: String,
    payment_id: String,
    #[serde(rename = "_links")]
    pub links: HashMap<String, super::Link>,
}

impl RefundsApi for super::ApiClient {
    fn get(
        &self,
        url: String,
        parameter: Option<String>,
        query: Option<HashMap<&str, String>>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.get(url, parameter, query)
    }

    fn post<T: ser::Serialize>(
        &self,
        request: T,
        url: String,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.post(request, url)
    }
}

pub trait RefundsApi {
    fn get(
        &self,
        url: String,
        parameter: Option<String>,
        query: Option<HashMap<&str, String>>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error>;

    fn post<T: ser::Serialize>(
        &self,
        request: T,
        url: String,
    ) -> Result<reqwest::blocking::Response, reqwest::Error>;

    fn refund_payment(
        &self,
        payment_id: String,
        request: RefundPaymentRequest,
    ) -> Result<RefundResource, super::errors::ApiClientError> {
        let response = self
            .post(request, format!("v2/payments/{}/refunds", payment_id))
            .map_err(super::errors::ApiClientError::CouldNotPerformRequest)?;

        if response.status() == StatusCode::CREATED {
            let decoded_response = response
                .json::<RefundResource>()
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
