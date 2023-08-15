// src/lib.rs

mod err;
mod token;
mod data;

extern crate reqwest;
extern crate rsa;
extern crate base64;
extern crate serde;

use std::collections::HashMap;
use serde::Deserialize;
use serde_json::json;

const BASE_URL: &str = "https://api.sandbox.vm.co.mz";


pub struct MPesaClient {
    configuration : token::Configuration,
    api_key: String,
    public_key: String,
    base_url: String,
    path: String,
    headers: HashMap<String, String>,
    parameters: HashMap<String, String>,
}

impl MPesaClient {
    pub fn new(api_key: String, public_key: String) -> Self {
        let mut client = MPesaClient {
            configuration: token::Configuration::new(),
            api_key,
            public_key,
            base_url: BASE_URL.to_string(),
            path: "/ipg/v1x/b2cPayment/".to_string(),
            headers: HashMap::new(),
            parameters: HashMap::new(),
        };
        client.configuration.set_api_key(client.api_key.clone()).unwrap();
        client.configuration.set_public_key(client.public_key.clone()).unwrap();
        client.headers.insert("User-Agent".to_string(), client.configuration.get_user_agent().clone());
        client.headers.insert("Content-Type".to_string(), "application/json".to_string());
        client.headers.insert("Authorization".to_string(), format!("Bearer {}", client.configuration.get_token().unwrap()));
        client
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.into(), value.into());
    }

    pub fn add_parameter(&mut self, key: &str, value: &str) {
        self.parameters.insert(key.into(), value.into());
    }

    pub async fn b2c_payment(&self, payment_request: &data::B2CInput) -> Result<MPesaPaymentResponse, err::MPesaError> {
        let client = reqwest::Client::new();
        let body_data = json!({
            "input_TransactionReference": payment_request.transaction_reference(),
            "input_CustomerMSISDN": payment_request.customer_msisdn(),
            "input_Amount": payment_request.amount(),
            "input_ThirdPartyReference": payment_request.third_party_reference(),
            "input_ServiceProviderCode": payment_request.service_provider_code(),
        });

        let body = serde_json::to_string(&body_data)?;

        let request_builder = client.post(&format!("{}{}", self.base_url, self.path));
        let request_builder = self.headers.iter().fold(request_builder, |acc, (k, v)| acc.header(k, v));

        let res = request_builder.send().await?;

        if res.status().is_success() {
            let payment_response: MPesaPaymentResponse = res.json().await?;

            // Check if the response code indicates a successful transaction
            if payment_response.output_ResponseCode == "INS-0" {
                Ok(payment_response)
            } else {
                Err(err::MPesaError::from_code(res.status().as_u16(), &payment_response.output_ResponseDesc))
            }
        } else {
            let error_text: String = res.text().await.unwrap_or_default();
            Err(err::MPesaError::from_code(res.status().as_u16(), &error_text))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MPesaPaymentResponse {
    output_ConversationID: String,
    output_ResponseCode: String,
    output_ResponseDesc: String,
    output_ThirdPartyReference: String,
    output_TransactionID: String,
}
