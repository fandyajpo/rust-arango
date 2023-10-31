use arangors::AqlQuery;
use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    middleware::{self},
    response::Json,
    routing::{delete, get, patch, post},
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
mod arango;
mod bcrypt;
mod middlewares;
mod model;
mod routes;

#[tokio::main]
async fn main() {
    // let cors: CorsLayer = CorsLayer::new()
    //     .allow_methods([Method::GET])
    //     .allow_origin(Any);

    let app = Router::new()
        .route("/", get(ok))
        .route("/user", get(get_user))
        .route("/user", post(create_user))
        .route("/user/:_key", patch(update_username))
        .route("/user/:_key", delete(delete_user))
        .route("/header", get(header_info))
        .layer(middleware::from_fn(middlewares::middleware))
        .route("/login", post(routes::service::login));

    let addr = SocketAddr::from(([127, 0, 0, 1], 9037));

    println!("Listening to port {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn get_user() -> Result<Json<Value>, StatusCode> {
    let conn = arango::connection().await.unwrap();
    let aql = AqlQuery::builder()
        .query("FOR u IN @@col RETURN u")
        .bind_var("@col", "user")
        .batch_size(1)
        .count(true)
        .build();

    let resp: Result<Vec<model::user::Users>, String> = match conn.aql_query(aql).await {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error executing AQL query: {:?}", err)),
    };

    match resp {
        Ok(data) => {
            let success_response = model::response::ReturnResponse {
                message: "List of user".to_owned(),
                status: true,
                data: Some(data),
            };
            Ok(Json(json!(success_response)))
        }
        Err(_err) => Err(StatusCode::BAD_GATEWAY),
    }
}

async fn update_username(
    Path(t): Path<model::user::Users>,
    Json(u): Json<model::user::Users>,
) -> Result<Json<Value>, StatusCode> {
    let key = t._key.unwrap();
    let username = u.username.unwrap();
    let conn = arango::connection().await.unwrap();

    let aql = AqlQuery::builder()
        .query(
            "FOR u IN @@col
         FILTER u._key == @key
            UPDATE u WITH {
            'username': @username
            } IN @@col
         RETURN NEW",
        )
        .bind_var("@col", "user")
        .bind_var("username", username)
        .bind_var("key", key)
        .batch_size(1)
        .count(true)
        .build();
    let resp: Result<Vec<model::user::Users>, String> = match conn.aql_query(aql).await {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error executing AQL query: {:?}", err)),
    };
    match resp {
        Ok(data) => {
            let success_response = model::response::ReturnResponse {
                message: "Success update".to_string(),
                status: true,
                data: Some(data),
            };
            Ok(Json(json!(success_response)))
        }
        Err(_err) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn create_user(Json(t): Json<model::user::Users>) -> Result<Json<Value>, StatusCode> {
    let conn = arango::connection().await.unwrap();

    let mut hashed = t.password.unwrap();
    hashed = bcrypt::hash_password(hashed).unwrap();

    let aql = AqlQuery::builder()
        .query("INSERT { username: @username, password: @password, created: DATE_NOW()} IN @@col RETURN NEW")
        .bind_var("username", t.username)
        .bind_var("password", hashed)
        .bind_var("@col", "user")
        .batch_size(1)
        .count(true)
        .build();

    let resp: Result<Vec<model::user::Users>, String> = match conn.aql_query(aql).await {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error executing AQL query: {:?}", err)),
    };

    match resp {
        Ok(data) => {
            let success_response = model::response::ReturnResponse {
                message: "Success create".to_string(),
                status: true,
                data: Some(data),
            };
            Ok(Json(json!(success_response)))
        }
        Err(_err) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn delete_user(Path(t): Path<model::user::Users>) -> Result<Json<Value>, StatusCode> {
    let key = t._key.unwrap();
    let conn = arango::connection().await.unwrap();
    let aql = AqlQuery::builder()
        .query(
            "LET u = DOCUMENT(@@col, @key)
             FILTER u != null
             REMOVE { _key: u._key } IN @@col",
        )
        .bind_var("key", key)
        .bind_var("@col", "user")
        .batch_size(1)
        .count(true)
        .build();

    let resp: Result<Vec<model::user::Users>, String> = match conn.aql_query(aql).await {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error executing AQL query: {:?}", err)),
    };

    match resp {
        Ok(data) => {
            let success_response = model::response::ReturnResponse {
                message: "Success remove".to_string(),
                status: true,
                data: Some(data),
            };
            Ok(Json(json!(success_response)))
        }
        Err(_err) => Err(StatusCode::BAD_REQUEST),
    }
}

// async fn header_info(TypedHeader(agent_header): TypedHeader<UserAgent>) -> String {
//     agent_header.to_string()
// }

async fn header_info(header: HeaderMap) -> String {
    let key = header.get("Connection").unwrap();
    let value = key.to_str().unwrap().to_owned();
    value
}

async fn ok() -> String {
    "ok".to_string()
}
