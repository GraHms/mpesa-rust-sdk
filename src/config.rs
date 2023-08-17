use base64::encode;
use rsa::{RsaPublicKey, PaddingScheme, PublicKey};
use rsa::pkcs8::FromPublicKey;

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

    pub fn get_user_agent(&self) -> String {
        self.user_agent.clone()
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key)
    }

    pub fn set_public_key(&mut self, public_key: String) {
        self.public_key = Some(public_key)
    }

    pub fn get_token(&mut self) -> Option<String> {
        self.generate_access_token();
        self.auth.clone()
    }


    pub fn generate_access_token(&mut self) {

        if let (Some(api_key), Some(public_key)) = (&self.api_key, &self.public_key) {
            let formatted_public_key = self.format_public_key(public_key);
            if let Ok(rsa_public_key) = RsaPublicKey::from_public_key_der(formatted_public_key.as_bytes()) {
                let mut rng = rand::thread_rng();
                if let Ok(encrypted_api_key) = rsa_public_key.encrypt(&mut rng, PaddingScheme::PKCS1v15Encrypt, api_key.as_bytes()) {
                    self.auth = Some(encode(&encrypted_api_key));
                    return;
                }
            }
        }

        if self.access_token.is_some() {
            self.auth = self.access_token.clone();
        }
    }

    fn format_public_key(&self, public_key: &str) -> String {
        format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", public_key)
    }
}
