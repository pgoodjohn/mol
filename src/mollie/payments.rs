use log::debug;
use reqwest::StatusCode;
use serde::{ser, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ListPaymentsResponse {
    pub count: i32,
    #[serde(rename(deserialize = "_embedded"))]
    pub embedded: PaymentResources,
    // #[serde(rename(deserialize = "_links"))]
    // pub links: Option<HashMap<String, super::Link>>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentResources {
    pub payments: Vec<PaymentResource>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentResource {
    pub id: String,
    pub mode: String,
    pub created_at: String,
    pub status: String,
    pub is_cancelable: Option<bool>,
    pub authorized_at: Option<String>,
    pub paid_at: Option<String>,
    pub expires_at: Option<String>,
    pub expired_at: Option<String>,
    pub failed_at: Option<String>,
    pub amount: Amount,
    pub amount_refunded: Option<Amount>,
    pub amount_remaining: Option<Amount>,
    pub amount_captured: Option<Amount>,
    pub amount_chargedback: Option<Amount>,
    pub settlement_amount: Option<Amount>,
    pub description: String,
    pub redirect_url: String,
    pub webhook_url: Option<String>,
    pub locale: Option<String>,
    pub country_code: Option<String>,
    pub method: Option<String>,
    pub restrict_payment_methods_to_country: Option<String>,
    pub profile_id: String,
    pub settlement_id: Option<String>,
    #[serde(rename = "_links")]
    pub links: HashMap<String, super::Link>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentRequest {
    pub amount: Amount,
    pub description: String,
    pub redirect_url: String,
    pub profile_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

impl PaymentsApi for super::ApiClient {
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

pub trait PaymentsApi {
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

    fn list_payments(
        &self,
        limit: Option<i32>,
        from: &Option<String>,
    ) -> Result<ListPaymentsResponse, super::errors::ApiClientError> {
        let mut query_params: std::collections::HashMap<&str, String> = HashMap::new();

        match limit {
            Some(limit) => {
                query_params.insert("limit", String::from(limit.to_string()));
            }
            None => {}
        }

        match from {
            Some(from) => {
                query_params.insert("from", String::from(from));
            }
            None => {}
        }

        let response = self
            .get(String::from("v2/payments"), None, Some(query_params))
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
            .get(
                String::from("v2/payments"),
                Some(String::from(payment_id)),
                None,
            )
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

    fn create_payment(
        &self,
        request: CreatePaymentRequest,
    ) -> Result<PaymentResource, super::errors::ApiClientError> {
        let response = self
            .post(request, String::from("v2/payments"))
            .map_err(super::errors::ApiClientError::CouldNotPerformRequest)?;

        if response.status() == StatusCode::CREATED {
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
