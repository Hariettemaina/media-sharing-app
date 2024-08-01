use crate::schema::transactions;
use crate::PhotoError;
use async_graphql::Error;
use async_graphql::{Context, InputObject, Object, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
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
    pub user_id: i32,
    pub photo_id: i32,
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

        // Log input data
        log::info!(
            "Processing photo purchase request: user_id={}, photo_id={}, amount={}, phone_number={}",
            input.user_id,
            input.photo_id,
            input.amount,
            input.phone_number
        );

        // Send M-Pesa STK push
        let stk_push_result = send_stk_push(&input.phone_number, input.amount as f64)
            .await
            .map_err(|e| async_graphql::Error::new(format!("M-Pesa payment failed: {:?}", e)))?;

        if !stk_push_result.is_successful {
            log::warn!(
                "M-Pesa payment not successful for phone_number={} and amount={}",
                input.phone_number,
                input.amount
            );
            return Err(async_graphql::Error::new(
                "M-Pesa payment was not successful",
            ));
        }

        // Record the purchase in the database
        let purchase_id: i32 = diesel::insert_into(transactions::table)
            .values((
                transactions::user_id.eq(input.user_id),
                transactions::photo_id.eq(input.photo_id),
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
            "Photo purchased successfully. Purchase ID: {} for user_id={}",
            purchase_id,
            input.user_id
        );

        Ok(format!(
            "Photo purchased successfully. Purchase ID: {}",
            purchase_id
        ))
    }
}

#[derive(Serialize, Debug)]
struct StkPushRequest {
    business_short_code: i32,
    password: String,
    timestamp: String,
    transaction_type: String,
    amount: String,
    party_a: String,
    party_b: i32,
    phone_number: String,
    call_back_url: String,
    account_reference: String,
    transaction_desc: String,
}

#[derive(Deserialize)]
struct StkPushResponse {
    checkout_request_id: String,
    response_code: String,
    response_description: String,
    merchant_request_id: String,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
}

struct StkPushResult {
    is_successful: bool,
    transaction_id: String,
}

async fn send_stk_push(phone_number: &str, amount: f64) -> Result<StkPushResult, Error> {
    let mpesa_env = env::var("MPESA_ENVIRONMENT").unwrap_or_else(|_| "sandbox".to_string());
    let base_url = if mpesa_env == "live" {
        "https://api.safaricom.co.ke"
    } else if mpesa_env == "sandbox" {
        "https://sandbox.safaricom.co.ke"
    } else {
        return Err(Error::new("Invalid MPESA_ENVIRONMENT"));
    };

    log::info!("Using M-Pesa environment: {}", mpesa_env);

    let client = Client::new();

    // Generate authorization token
    let consumer_key = env::var("MPESA_CONSUMER_KEY")
        .map_err(|e| Error::new(format!("Env var error: {:?}", e)))?;
    let consumer_secret = env::var("MPESA_CONSUMER_SECRET")
        .map_err(|e| Error::new(format!("Env var error: {:?}", e)))?;
    let auth = general_purpose::STANDARD.encode(format!("{}:{}", consumer_key, consumer_secret));

    let token_resp: TokenResponse = client
        .get(&format!(
            "{}/oauth/v1/generate?grant_type=client_credentials",
            base_url
        ))
        .header("Authorization", format!("Basic {}", auth))
        .send()
        .await
        .map_err(|e| Error::new(format!("Request error: {:?}", e)))?
        .json()
        .await
        .map_err(|e| Error::new(format!("Json error: {:?}", e)))?;

    log::info!("Generated access token for M-Pesa API");
    println!("Token Response: {:?}", token_resp);

    // Prepare STK push request
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let shortcode = 174379;
    // let shortcode =
    //     env::var("MPESA_SHORTCODE").map_err(|e| Error::new(format!("Env var error: {:?}", e)))?;
    let passkey =
        env::var("MPESA_PASSKEY").map_err(|e| Error::new(format!("Env var error: {:?}", e)))?;
    let password =
        general_purpose::STANDARD.encode(format!("{}{}{}", shortcode, passkey, timestamp));

    // Remove non-digits from the phone number and format it to ensure it starts with "254"
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

    // Prepare STK push request
    let stk_request = StkPushRequest {
        business_short_code: shortcode.clone(),
        password,
        timestamp,
        transaction_type: "CustomerPayBillOnline".to_string(),
        amount: amount.to_string(),
        party_a: formatted_phone.clone(),
        party_b: shortcode,
        phone_number: formatted_phone,
        call_back_url: "https://mydomain.com/path".to_string(),
        account_reference: phone_number.to_string(),
        transaction_desc: "Photo purchase".to_string(),
    };

    log::info!("STK Push Request Payload: {:?}", stk_request);
    log::info!("Using Business ShortCode: {}", shortcode);

    // Send STK push request
    let resp = client
        .post(&format!("{}/mpesa/stkpush/v1/processrequest", base_url))
        .header(
            "Authorization",
            format!("Bearer {}", token_resp.access_token),
        )
        .json(&stk_request)
        .send()
        .await
        .map_err(|e| Error::new(format!("Request error: {:?}", e)))?;

    // Log the raw response body
    let body = resp
        .text()
        .await
        .map_err(|e| Error::new(format!("Error reading response body: {:?}", e)))?;
    log::info!("M-Pesa STK Push response body: {}", body);
    

    let resp: StkPushResponse = serde_json::from_str(&body)
        .map_err(|e| Error::new(format!("Json decode error: {:?}", e)))?;

    if resp.response_code != "0" {
        log::warn!(
            "M-Pesa STK Push failed: {} (Merchant Request ID: {})",
            resp.response_description,
            resp.merchant_request_id
        );
        return Ok(StkPushResult {
            is_successful: false,
            transaction_id: resp.checkout_request_id,
        });
    }

    log::info!(
        "M-Pesa STK Push successful: Checkout Request ID: {}",
        resp.checkout_request_id
    );

    Ok(StkPushResult {
        is_successful: true,
        transaction_id: resp.checkout_request_id,
    })
}
