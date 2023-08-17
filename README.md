## MPesa SDK (Rust Library)

A Rust library to interact with the MPesa API, simplifying B2C payment integrations and more.

### Features

- Configuration management for API and public keys.
- Easy B2C Payment initiation.
- Customizable headers and parameters for requests.
- Built-in encryption and token generation for request authentication.
- Comprehensive error handling with the `MPesaError` enum.

### Getting Started

#### Prerequisites

- Rust compiler and Cargo.
- An MPesa Developer account to obtain API and public keys.

#### Installation

Include this in your `Cargo.toml`:

```toml
[dependencies]
mpesa_sdk = "0.1.0"
```

Then, build your project with:

```bash
$ cargo build
```

#### Basic Usage

```rust
use mpesa_sdk::{MPesaClient, data::B2CInput};

let client = MPesaClient::new("YOUR_API_KEY", "YOUR_PUBLIC_KEY");
let mut payment_request = B2CInput::new();

payment_request.set_transaction_reference("TRANSACTION_REF");
payment_request.set_customer_msisdn("258848255237");
payment_request.set_amount("1000");
payment_request.set_third_party_reference("THIRD_PARTY_REF");
payment_request.set_service_provider_code("171717");

let response = client.b2c_payment(&payment_request);

match response {
    Ok(resp) => println!("{:?}", resp),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### Error Handling

The SDK uses the `MPesaError` enum for comprehensive error feedback. When making requests, results can be pattern-matched against these errors:

```rust
match result {
    Ok(data) => {
        // Process success scenario
    },
    Err(MPesaError::NetworkError(_)) => {
        // Handle network errors
    },
    Err(MPesaError::SerializationError(_)) => {
        // Handle serialization errors
    },
    // ... other specific errors
    Err(_) => {
        // Handle general or unknown errors
    }
}
```

For a full list of error variants, refer to the `MPesaError` documentation.

### Contributing

We welcome contributions! Whether it's bug fixes, feature additions, or documentation improvements, your input is valuable. For significant changes, please open an issue for discussion before making a pull request.

### License

This SDK is under the [MIT License](https://choosealicense.com/licenses/mit/).

---

This README aims to provide clear, concise instructions to quickly get users started. Over time, consider enhancing the documentation by including more in-depth tutorials, best practices, or use-case examples to further guide users.