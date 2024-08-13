use crate::schema::transactions;
use crate::PhotoError;
use async_graphql::Error;
use async_graphql::{Context, InputObject, Object, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::{FixedOffset, Utc};
use diesel::ExpressionMethods;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Default)]
pub struct PurchasePhoto;

#[derive(InputObject)]
pub struct PurchasePhotoInput {
    // pub user_id: i32,
    // pub photo_id: i32,
    pub amount: i64,
    pub phone_number: String,
}

#[Object]
impl PurchasePhoto {
    pub async fn purchase_photo(
        &self,
        ctx: &Context<'_>,
        input: PurchasePhotoInput,
    ) -> Result<String> {
        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        log::info!(
            "Processing photo purchase request: amount={}, phone_number={}",
            input.amount,
            input.phone_number
        );

        let stk_push_result = send_stk_push(&input.phone_number, (input.amount as i32).into())
            .await
            .map_err(|e| async_graphql::Error::new(format!("M-Pesa payment failed: {:?}", e)))?;

        if !stk_push_result.is_successful {
            log::warn!(
                "M-Pesa payment not successful for phone_number={} and amount={}. Error: {}",
                input.phone_number,
                input.amount,
                stk_push_result.transaction_id // Assuming this contains error details or message
            );
            return Err(async_graphql::Error::new(format!(
                "M-Pesa payment was not successful: {}",
                stk_push_result.transaction_id
            )));
        }

        let purchase_id: i32 = diesel::insert_into(transactions::table)
            .values((
                // transactions::user_id.eq(input.user_id),
                // transactions::photo_id.eq(input.photo_id),
                transactions::amount.eq(input.amount),
                transactions::mpesa_transaction_id.eq(&stk_push_result.transaction_id),
                transactions::created_at.eq(Utc::now().naive_utc()),
            ))
            .returning(transactions::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(|e| {
                log::error!("Failed to insert purchase into database: {:?}", e);
                PhotoError::DatabaseError
            })?;

        log::info!(
            "Photo purchased successfully. Purchase ID: {} ",
            purchase_id,
        );

        Ok(format!(
            "Photo purchased successfully. Purchase ID: {}",
            purchase_id
        ))
    }
}

#[derive(Serialize, Debug)]
pub struct StkPushRequest {
    pub business_short_code: String,
    pub password: String,
    pub timestamp: String,
    pub transaction_type: String,
    pub amount: String,
    pub party_a: String,
    pub party_b: String,
    pub phone_number: String,
    pub call_back_url: String,
    pub account_reference: String,
    pub transaction_desc: String,
}

#[derive(Deserialize, Debug)]
pub struct StkPushResponse {
    pub merchant_request_id: Option<String>,
    pub checkout_request_id: Option<String>,
    pub response_code: Option<String>,
    pub response_description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: String,
}

pub struct StkPushResult {
    pub is_successful: bool,
    pub transaction_id: String,
}

pub async fn send_stk_push(phone_number: &str, amount: f64) -> Result<StkPushResult, Error> {
    let mpesa_env = env::var("MPESA_ENVIRONMENT").unwrap_or_else(|_| "sandbox".to_string());
    log::info!("Using M-Pesa environment: {}", mpesa_env);

    let client = Client::new();

    let consumer_key = env::var("MPESA_CONSUMER_KEY")
        .map_err(|e| Error::new(format!("Env var error: {:?}", e)))?;
    let consumer_secret = env::var("MPESA_CONSUMER_SECRET")
        .map_err(|e| Error::new(format!("Env var error: {:?}", e)))?;
    let auth = general_purpose::STANDARD.encode(format!("{}:{}", consumer_key, consumer_secret));

    let token_resp: TokenResponse = client
        .get("https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials")
        .header("Authorization", format!("Basic {}", auth))
        .send()
        .await
        .map_err(|e| Error::new(format!("Failed to request access token: {:?}", e)))?
        .json()
        .await
        .map_err(|e| Error::new(format!("Failed to parse access token response: {:?}", e)))?;

    log::info!("Generated access token for M-Pesa API: {:?}", token_resp);

    let now = Utc::now();
    let expires_at =
        now + chrono::Duration::seconds(token_resp.expires_in.parse::<u64>().unwrap_or_default().try_into().unwrap());

    // Check if the token is expired
    if now > expires_at {
        return Err(Error::new("Access token has expired."));
    }

    let timestamp = Utc::now()
        .with_timezone(&FixedOffset::east_opt(3 * 3600).unwrap())
        .format("%Y%m%d%H%M%S")
        .to_string();

    let shortcode = "174379";
    let passkey =
        env::var("MPESA_PASSKEY").map_err(|e| Error::new(format!("Env var error: {:?}", e)))?;
    let password =
        general_purpose::STANDARD.encode(format!("{}{}{}", shortcode, passkey, timestamp));

    let cleaned_number = phone_number
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>();
    let formatted_phone = format!("254{}", &cleaned_number[cleaned_number.len() - 9..]);

    log::info!(
        "Formatted phone number for M-Pesa: {} -> {}",
        phone_number,
        formatted_phone
    );

    let amount = amount.round() as i32;

    let stk_request = StkPushRequest {
        business_short_code: shortcode.to_string(),
        password,
        timestamp,
        transaction_type: "CustomerPayBillOnline".to_string(),
        amount: amount.to_string(),
        party_a: formatted_phone.clone(),
        party_b: shortcode.to_string(),
        phone_number: formatted_phone,
        call_back_url: "https://prime-fairly-honeybee.ngrok-free.app".to_string(),
        account_reference: "Test".to_string(),
        transaction_desc: "Test".to_string(),
    };

    log::info!("STK Push Request Payload: {:?}", stk_request);

    let resp = client
        .post("https://sandbox.safaricom.co.ke/mpesa/stkpush/v1/processrequest")
        .header(
            "Authorization",
            format!("Bearer {}", token_resp.access_token),
        )
        .header("Content-Type", "application/json")
        .json(&stk_request)
        .send()
        .await
        .map_err(|e| {
            log::error!("Failed to send STK push request: {:?}", e);
            Error::new(format!("Failed to send STK push request: {:?}", e))
        })?;

    let status = resp.status();
    let body = resp.text().await.map_err(|e| {
        log::error!("Failed to read response body: {:?}", e);
        Error::new(format!("Failed to read response body: {:?}", e))
    })?;

    log::info!("M-Pesa STK Push response status: {}", status);
    log::info!("M-Pesa STK Push raw response: {}", body);
    if body.is_empty() {
        log::warn!("Received empty response from M-Pesa API");
        return Err(Error::new("Received empty response from M-Pesa API"));
    }

    let resp: StkPushResponse = serde_json::from_str(&body)
        .map_err(|e| Error::new(format!("Json decode error: {:?}", e)))?;

    if resp.response_code != Some("0".to_string()) {
        log::warn!(
            "M-Pesa STK Push failed: {} (Merchant Request ID: {:?})",
            resp.response_description
                .unwrap_or_else(|| "Error".to_string()),
            resp.merchant_request_id
        );
        return Ok(StkPushResult {
            is_successful: false,
            transaction_id: resp
                .merchant_request_id
                .unwrap_or_else(|| "Error".to_string()),
        });
    }

    log::info!(
        "M-Pesa STK Push successful: Checkout Request ID: {:?}",
        resp.checkout_request_id
    );

    Ok(StkPushResult {
        is_successful: true,
        transaction_id: resp
            .checkout_request_id
            .unwrap_or_else(|| "Error".to_string()),
    })
}
