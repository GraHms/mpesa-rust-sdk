// src/lib.rs

mod err;
mod token;

extern crate reqwest;
extern crate rsa;
extern crate base64;

use std::collections::HashMap;

const BASE_URL: &str = "https://api.sandbox.vm.co.mz";
#[derive(Debug, Deserialize)]
pub struct MPesaPaymentResponse {
    output_ConversationID: String,
    output_ResponseCode: String,
    output_ResponseDesc: String,
    output_ThirdPartyReference: String,
    output_TransactionID: String,
}

#[derive(Debug, Serialize)]
pub struct B2CInput {
    input_TransactionReference: String,
    input_CustomerMSISDN: String,
    input_Amount: String,
    input_ThirdPartyReference: String,
    input_ServiceProviderCode: String,
}

impl B2CInput{
    pub fn set_transaction_reference(&mut self, transaction_reference: String) {
        self.input_TransactionReference = transaction_reference;
    }
    pub fn set_customer_msisdn(&mut self, customer_msisdn: String) {
        self.input_CustomerMSISDN = customer_msisdn;
    }
    pub fn set_amount(&mut self, amount: String) {
        self.input_Amount = amount;
    }
    pub fn set_third_party_reference(&mut self, third_party_reference: String) {
        self.input_ThirdPartyReference = third_party_reference;
    }
    pub fn set_service_provider_code(&mut self, service_provider_code: String) {
        self.input_ServiceProviderCode = service_provider_code;
    }


}


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

    pub async fn b2c_payment(&self) -> Result<MPesaPaymentResponse, err::MPesaError> {
        let client = reqwest::Client::new();
        let body = serde_json::to_string(&self.parameters)?;

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
