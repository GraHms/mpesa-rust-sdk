use rsa::{RSAPublicKey, PaddingScheme, PublicKey, RsaPublicKey};
use base64::encode;
use std::error::Error;

pub struct Configuration {
    user_agent: String,
    api_key: Option<String>,
    public_key: Option<String>,
    access_token: Option<String>,
    auth: Option<String>,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            user_agent: "PaymentsDS/Mpesa".to_string(),
            api_key: None,
            public_key: None,
            access_token: None,
            auth: None,
        }
    }

    pub fn set_api_key(&mut self, api_key: String) -> Result<(), Box<dyn Error>> {
        self.api_key = Some(api_key);
        self.generate_access_token()
    }

    pub fn set_public_key(&mut self, public_key: String) -> Result<(), Box<dyn Error>> {
        self.public_key = Some(public_key);
        self.generate_access_token()
    }

    pub fn get_token(&self) -> Option<String> {
        self.auth.clone()
    }

    fn generate_access_token(&mut self) -> Result<(), Box<dyn Error>> {
        if let (Some(api_key), Some(public_key)) = (&self.api_key, &self.public_key) {
            let formatted_public_key = self.format_public_key(public_key);
            let encrypted_api_key = self.encrypt_with_public_key(&formatted_public_key, api_key)?;
            self.auth = Some(encode(&encrypted_api_key));
        } else if let Some(access_token) = &self.access_token {
            self.auth = Some(access_token.clone());
        }
        Ok(())
    }

    fn format_public_key(&self, public_key: &str) -> String {
        format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", public_key)
    }

    fn encrypt_with_public_key(&self, public_key: &str, api_key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let rsa_pk = RsaPublicKey::from_pem(public_key.as_bytes())?;
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        let encrypted_data = rsa_pk.encrypt(padding, api_key.as_bytes())?;
        Ok(encrypted_data)
    }
}


