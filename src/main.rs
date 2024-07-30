mod schema;

use actix_web::{get, post, middleware, web, App, HttpResponse, HttpServer, Responder, route};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use std::{sync::Arc};
use actix_cors::Cors;
use actix_web::web::Data;
use crate::schema::{create_schema, Schema};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    web::Html::new(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Arc::new(create_schema());

    println!("GraphiQL playground: http://localhost:8080/graphiql");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql_playground)
            .service(graphql)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}