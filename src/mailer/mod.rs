use reqwest::{header::HeaderMap, Client, Error};
use serde_json::json;
use uuid::Uuid;

pub struct BrevoApi {
    api_key: String,
    email: String,
}

impl BrevoApi {
    pub fn new(api_key: String, email: String) -> Self {
        Self { api_key, email }
    }
    pub async fn send_verification_email(
        &self,
        recipient_email_address: String,
        verification_code: Uuid,
    ) -> Result<(), Error> {
        let url = "https://api.sendgrid.com/v3/mail/send";

        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );
        let client = Client::new();

        let email_content = format!(
            r#"
        Click the link below to verify your email:

        
        http://localhost:8080/verify?code={verification_code}
        
        "#,
        );

        let response = client
            .post(url)
            .headers(headers)
            .json(&json!({
                "personalizations": [
                    {
                        "to": [{"email": recipient_email_address}],
                        "subject": "Welcome to OurService"
                    }
                ],
                "from": {
                    "email": self.email
                },
                "content": [
                    {
                        "type": "text/html",
                        "value": email_content
                    }
                ]
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            log::error!("Failed to send email : {:#?}", response.text().await?);
        } else {
            log::info!("Email sent to {} successfully", recipient_email_address);
        }

        Ok(())
    }
}
