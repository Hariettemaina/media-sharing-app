use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, guard, http, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};

//use handlebars::{DirectorySourceOptions, Handlebars};
use photos::graphql_schema::{Mutation, Query};

use photos::password::PassWordHasher;
use photos::InternalError;

pub type ApplicationSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<ApplicationSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// async fn login_handler(schema: web::Data<ApplicationSchema>, req: web::Json<LoginInput>) -> impl Responder {
//     let mutation = r#"
//         mutation Login($input: LoginInput!) {
//             login(input: $input) {
//                 message
//             }
//         }
//     "#;

//     let variables = json!({
//         "input": {
//             "user_email": req.user_email,
//             "password": req.password,
//         }
//     });

//     //let request = GraphQLRequest::new(mutation.to_string(), variables);
//     let response: GraphQLResponse = schema.execute(mutation).await.into();

//     match response.data {
//         Some(data) => {
//             if let Some(message) = data.get("login").and_then(|login| login.get("message")) {
//                 HttpResponse::Ok().body(message.to_string())
//             } else {
//                 HttpResponse::InternalServerError().body("Failed to authenticate")
//             }
//         }
//         None => HttpResponse::InternalServerError().body("Failed to authenticate"),
//     }
// }

// async fn index_signup() -> Result<NamedFile> {
//     let path: PathBuf = "./templates/base.html".parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }

// async fn index_verify() -> Result<NamedFile> {
//     let path: PathBuf = "./templates/verify_email.html".parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }

// async fn index_login() -> Result<NamedFile> {
//     let path: PathBuf = "./templates/login.html".parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> Result<(), InternalError> {
    // let mut handlebars = Handlebars::new();
    // handlebars
    //     .register_templates_directory(
    //         "./templates",
    //         DirectorySourceOptions {
    //             tpl_extension: ".html".to_owned(),
    //             hidden: false,
    //             temporary: false,
    //         },
    //     )
    //     .unwrap();
    // let handlebars_ref = web::Data::new(handlebars);

    dotenvy::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = dotenvy::var("DATABASE_URL").unwrap();

    let secret_key = Key::generate();

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build()?;
    let password_hasher = PassWordHasher::new();
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(pool)
        .data(password_hasher)
        .finish();

    log::info!("starting HTTP server at http://localhost:8080");

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(cors)
            .app_data(Data::new(schema.clone()))
            //.app_data(handlebars_ref.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/graphiql")
                    .guard(guard::Get())
                    .to(index_graphiql),
            )
            .service(actix_files::Files::new("/", "./templates").show_files_listing())
        //.service(web::resource("/login").route(web::post(). to(login_handler)))
        //.route("/signup", web::get().to(index_signup))
        // .route("/verify", web::get().to(index_verify))
        // .route("/login", web::get().to(index_login))
    })
    .bind(("127.0.0.1", 8080))?
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
