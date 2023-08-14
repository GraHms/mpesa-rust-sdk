// src/lib.rs

mod err;
mod token;

extern crate reqwest;
extern crate rsa;
extern crate base64;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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

    pub async fn b2c_payment(&self) -> Result<(), err::MPesaError> {
        let client = reqwest::Client::new();
        let request_builder = client.post(&format!("{}{}", self.base_url, self.path));

        let request_builder = self.headers.iter().fold(request_builder, |acc, (k, v)| acc.header(k, v));

        let res = request_builder.json(&self.parameters).send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(err::MPesaError::from_code(res.status().as_u16(), &res.text().await.unwrap_or_default()))
        }
    }
}