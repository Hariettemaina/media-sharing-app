use async_graphql::{Context, InputObject, Object, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use diesel::ExpressionMethods;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::TryFutureExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::schema::transactions;
use crate::PhotoError;
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

        // Send M-Pesa STK push
        let stk_push_result = send_stk_push(&input.phone_number, input.amount as f64)
            .await
            .map_err(|e| async_graphql::Error::new(format!("M-Pesa payment failed: {}", e)))?;

        if !stk_push_result.is_successful {
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
            .map_err(|e| {
                log::error!("Failed to insert purchase into database: {}", e);
                PhotoError::DatabaseError
            })
            .await?;

        Ok(format!(
            "Photo purchased successfully. Purchase ID: {}",
            purchase_id
        ))
    }
}

#[derive(Serialize)]
struct StkPushRequest {
    business_short_code: String,
    password: String,
    timestamp: String,
    transaction_type: String,
    amount: String,
    party_a: String,
    party_b: String,
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

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

struct StkPushResult {
    is_successful: bool,
    transaction_id: String,
}

async fn send_stk_push(
    phone_number: &str,
    amount: f64,
) -> Result<StkPushResult, Box<dyn std::error::Error>> {
    let mpesa_env = env::var("MPESA_ENVIRONMENT").unwrap_or_else(|_| "sandbox".to_string());
    let base_url = if mpesa_env == "live" {
        "https://api.safaricom.co.ke"
    } else {
        "https://sandbox.safaricom.co.ke"
    };

    let client = Client::new();

    // Generate authorization token
    let consumer_key = env::var("MPESA_CONSUMER_KEY")?;
    let consumer_secret = env::var("MPESA_CONSUMER_SECRET")?;
    let auth = general_purpose::STANDARD.encode(format!("{}:{}", consumer_key, consumer_secret));

    let token_resp: TokenResponse = client
        .get(&format!(
            "{}/oauth/v1/generate?grant_type=client_credentials",
            base_url
        ))
        .header("Authorization", format!("Basic {}", auth))
        .send()
        .await?
        .json()
        .await?;

    // Prepare STK push request
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let shortcode = env::var("MPESA_SHORTCODE")?;
    let passkey = env::var("MPESA_PASSKEY")?;
    let password =
        general_purpose::STANDARD.encode(format!("{}{}{}", shortcode, passkey, timestamp));

    // Remove non-digits from the phone number and format it to ensure it starts with "254"
    let cleaned_number = phone_number.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let formatted_phone = format!("254{}", &cleaned_number[cleaned_number.len() - 9..]);

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

    // Send STK push request
    let resp: StkPushResponse = client
        .post(&format!("{}/mpesa/stkpush/v1/processrequest", base_url))
        .header(
            "Authorization",
            format!("Bearer {}", token_resp.access_token),
        )
        .json(&stk_request)
        .send()
        .await?
        .json()
        .await?;

    Ok(StkPushResult {
        is_successful: resp.response_code == "0",
        transaction_id: resp.checkout_request_id,
    })
}
