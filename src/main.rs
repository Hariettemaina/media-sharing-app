use actix_cors::Cors;
use actix_session::Session;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::SameSite;
use actix_web::HttpRequest;
use actix_web::{cookie::Key, guard, http, web, web::Data, App, HttpResponse, HttpServer, Result};

use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};
use graphql_schema::{Mutation, Query, Subscription};
use photos::graphql_schema::images::subscriptions::new_image::MediaUpdate;
use photos::graphql_schema::users::mutation::login::Shared;
use photos::mailer::BrevoApi;
use photos::models::User;
use photos::password::PassWordHasher;
use photos::services::image_processor::ImageProcessor;
use photos::{graphql_schema, InternalError, RequestContext};
use send_wrapper::SendWrapper;
use std::sync::Arc;

use tokio::sync::{broadcast, Mutex};
pub type ApplicationSchema = Schema<Query, Mutation, Subscription>;

// async fn index(schema: web::Data<ApplicationSchema>, req: GraphQLRequest) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }

async fn index(
    schema: web::Data<ApplicationSchema>,
    req: GraphQLRequest,
    session: Session,
) -> GraphQLResponse {
    let user_id = Shared::new(SendWrapper::new(session.clone()));
    let request = req.into_inner();
    let request_context = RequestContext { session: user_id };
    // Execute the schema asynchronously and await the result
    let result = schema.execute(request.data(request_context)).await;

    // Convert the result into a GraphQLResponse
    GraphQLResponse::from(result)
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/")
                .finish(),
        ))
}

async fn index_ws(
    schema: web::Data<ApplicationSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    println!("Websocket...");
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

#[actix_web::main]
async fn main() -> Result<(), InternalError> {
    dotenvy::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let api_key = dotenvy::var("BREVO_API_KEY").expect("BREVO_API_KEY must be set.");
    let email = dotenvy::var("BREVO_EMAIL").expect("BREVO_EMAIL must be set.");
    let ngrok_url = dotenvy::var("NGROK_URL").expect("NGROK_URL must be set.");

    let brevo_api = BrevoApi::new(api_key, email);

    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    let (user_tx, _) = broadcast::channel::<User>(100);
    let secret_key = Key::generate();
    let (media_update_tx, _) = broadcast::channel::<MediaUpdate>(100);
    let user_tx = Arc::new(Mutex::new(user_tx));
    let media_update_tx = Arc::new(Mutex::new(media_update_tx));

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build()?;
    let image_processor = Arc::new(ImageProcessor::new(pool.clone()));
    let password_hasher = PassWordHasher::new();
    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(pool)
    .data(brevo_api)
    .data(password_hasher)
    .data(image_processor)
    .data(user_tx.clone())
    .data(media_update_tx.clone())
    .finish();

    println!("starting HTTP server at http://localhost:8080");

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin(&ngrok_url)
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_same_site(SameSite::None)
                    .cookie_name("token".into())
                    .cookie_secure(true)
                    .build(),
            )
            .wrap(cors)
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(index_graphiql),
            )
            .service(actix_files::Files::new("/uploads", "./uploads").show_files_listing())
            .service(actix_files::Files::new("/", "./templates").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
