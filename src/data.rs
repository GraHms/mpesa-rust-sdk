
pub struct B2CInput {
    transaction_reference: String,
    customer_msisdn: String,
    amount: String,
    third_party_reference: String,
    service_provider_code: String,
}

impl B2CInput{
    pub fn set_transaction_reference(&mut self, transaction_reference: String) {
        self.transaction_reference = transaction_reference;
    }
    pub fn set_customer_msisdn(&mut self, customer_msisdn: String) {
        self.customer_msisdn = customer_msisdn;
    }
    pub fn set_amount(&mut self, amount: String) {
        self.amount = amount;
    }
    pub fn set_third_party_reference(&mut self, third_party_reference: String) {
        self.third_party_reference = third_party_reference;
    }
    pub fn set_service_provider_code(&mut self, service_provider_code: String) {
        self.service_provider_code = service_provider_code;
    }
    pub fn transaction_reference(&self) -> &str {
        &self.transaction_reference
    }
    pub fn customer_msisdn(&self) -> &str {
        &self.customer_msisdn
    }
    pub fn amount(&self) -> &str {
        &self.amount
    }
    pub fn third_party_reference(&self) -> &str {
        &self.third_party_reference
    }
    pub fn service_provider_code(&self) -> &str {
        &self.service_provider_code
    }



}

