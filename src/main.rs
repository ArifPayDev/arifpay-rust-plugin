extern crate reqwest;
extern crate serde_json;
use std::collections::HashMap;
use serde_json::Value;
use uuid::Uuid;
use tokio;

pub struct ArifPay {
    api_key: String,
    expire_date: String,
}

impl ArifPay {
    pub fn new(api_key: String, expire_date: String) -> ArifPay {
        ArifPay {
            api_key,
            expire_date,
        }
    }

    pub async fn make_payment(&self, mut payment_info: HashMap<&str, Value>) -> Result<String, Box<dyn std::error::Error>> {
        let required_fields = vec!["cancelUrl", "successUrl", "errorUrl", "notifyUrl", "paymentMethods", "items", "beneficiaries"];
        let mut missing_fields = Vec::new();
        for field in &required_fields {
            if !payment_info.contains_key(field) {
                missing_fields.push(format!("{} is a required field please enter this field", field));
            }
        }
        if !missing_fields.is_empty() {
            return Ok(serde_json::to_string(&missing_fields)?);
        }
        payment_info.insert("nonce", serde_json::json!(Uuid::new_v4().to_string()));
        payment_info.insert("expireDate", serde_json::json!(self.expire_date.clone()));
        println!("{:?}", payment_info);

        let client = reqwest::Client::new();
        let res = client.post("https://gateway.arifpay.org/api/sandbox/checkout/session")
        .header("Content-Type", "application/json")
        .header("x-arifpay-key", &self.api_key)
        .json(&payment_info)
        .send()
        .await?;



        if res.status().is_success() {
            let text = res.text().await?;
            Ok(text)
        } else {
            let mut error = HashMap::new();
            error.insert("status", res.status().to_string());
            error.insert("message", res.text().await?);
            Ok(serde_json::to_string(&error)?)
        }
    }
}
fn main() {
    let mut payment_info: HashMap<&str, Value> = HashMap::new();
    
    payment_info.insert("cancelUrl", serde_json::json!("https://example.com"));
    payment_info.insert("phone", serde_json::json!("251944294981"));
    payment_info.insert("email", serde_json::json!("natnael@arifpay.net"));
    payment_info.insert("nonce", serde_json::json!("6rdgajm1p1c"));
    payment_info.insert("errorUrl", serde_json::json!("http://error.com"));
    payment_info.insert("notifyUrl", serde_json::json!("https://gateway.arifpay.net/test/callback"));
    payment_info.insert("successUrl", serde_json::json!("http://example.com"));
    payment_info.insert("paymentMethods", serde_json::json!(["TELEBIRR"]));
    payment_info.insert("lang", serde_json::json!("EN"));
    
    payment_info.insert("items", serde_json::json!([
        {
            "name": "ሙዝ",
            "quantity": 1,
            "price": 1,
            "description": "Fresh Corner preimuim Banana.",
            "image": "https://4.imimg.com/data4/KK/KK/GLADMIN-/product-8789_bananas_golden-500x500.jpg"
        },
        {
            "name": "ሙዝ",
            "quantity": 1,
            "price": 1,
            "description": "Fresh Corner preimuim Banana.",
            "image": "https://4.imimg.com/data4/KK/KK/GLADMIN-/product-8789_bananas_golden-500x500.jpg"
        }
    ]));
    
    payment_info.insert("beneficiaries", serde_json::json!([
        {
            "accountNumber": "01320811436100",
            "bank": "AWINETAA",
            "amount": "2.0"
        }
    ]));
    

    let arif_pay = ArifPay::new("G8FbER8zZ9uco5tLuVnNKycJwXzvJTyo".to_string(), "2025-02-01T03:45:27".to_string());
    let future = async {
        match arif_pay.make_payment(payment_info).await {
            Ok(response) => println!("{}", response),
            Err(e) => println!("Error: {}", e),
        }
    };
     tokio::runtime::Runtime::new().unwrap().block_on(future);
}
