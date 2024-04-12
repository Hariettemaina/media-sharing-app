use async_graphql::{Context, InputObject, Object, Result};
use chrono::Utc;
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;
use validator::Validate;

use crate::{mailer::BrevoApi, models::EmailAddress};

#[derive(Default)]
pub struct Verify;

#[derive(InputObject, Validate)]
pub struct VerifyEmail {
    #[validate(length(min = 32))]
    code: String,
}

#[Object]
impl Verify {
    pub async fn verify_email(&self, ctx: &Context<'_>, input: VerifyEmail) -> Result<bool> {
        input.validate()?;

        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        let verification_code_uuid = Uuid::parse_str(&input.code).map_err(|_| "Invalid Uuid")?;

        let brevo = ctx.data::<BrevoApi>().map_err(|e| {
            log::error!("Failed to get BrevoApi: {:?}", e);
            e
        })?;

        let email_addr: EmailAddress = crate::schema::email_address::dsl::email_address
            .filter(crate::schema::email_address::dsl::verification_code.eq(verification_code_uuid))
            .first(&mut conn)
            .await
            .map_err(|_| "Email address not found")?;

        if Utc::now().naive_utc() > email_addr.verification_code_expires_at {
            brevo
                .send_verification_email(email_addr.email, email_addr.verification_code)
                .await?;
            return Ok(false);
        }

        diesel::update(crate::schema::email_address::dsl::email_address.filter(
            crate::schema::email_address::dsl::verification_code.eq(verification_code_uuid),
        ))
        .set(crate::schema::email_address::dsl::verified_at.eq(Some(Utc::now().naive_utc())))
        .execute(&mut conn)
        .await
        .map_err(|_| "Failed to update email address")?;

        Ok(true)
    }
}
