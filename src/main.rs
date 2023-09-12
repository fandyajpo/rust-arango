use arangors::AqlQuery;
use axum::{
    extract::Path,
    response::Json,
    routing::{delete, get, patch, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
mod arango;

#[derive(Serialize, Deserialize)]
struct Product {
    name: String,
    price: i64,
}

#[derive(Serialize)]
struct ReturnResponse {
    message: String,
    status: bool,
    data: Option<Vec<Users>>,
}

#[derive(Serialize, Debug, Deserialize)]
struct Users {
    username: Option<String>,
    password: Option<String>,
    _key: Option<String>,
}

#[tokio::main]
async fn main() {
    arango::print_to(10);
    let app = Router::new()
        .route("/user", get(get_user))
        .route("/user", post(create_user))
        .route("/user/:_key", patch(update_username))
        .route("/user/:_key", delete(delete_user));
    let addr = SocketAddr::from(([127, 0, 0, 1], 9037));

    println!("Listening to port {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn get_user() -> Result<Json<Value>, Json<Value>> {
    let conn = arango::connection().await.unwrap();
    let resp: Result<Vec<Users>, String> = match conn.aql_str("FOR u IN user RETURN u").await {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error executing AQL query: {:?}", err)),
    };

    match resp {
        Ok(data) => {
            let success_response = ReturnResponse {
                message: "List of user".to_string(),
                status: true,
                data: Some(data),
            };
            Ok(Json(json!(success_response)))
        }
        Err(err) => {
            let error_response = ReturnResponse {
                message: err,
                status: false,
                data: None,
            };
            Err(Json(json!(error_response)))
        }
    }
}

async fn update_username(
    Path(t): Path<Users>,
    Json(u): Json<Users>,
) -> Result<Json<Value>, Json<Value>> {
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
    let resp: Result<Vec<Users>, String> = match conn.aql_query(aql).await {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error executing AQL query: {:?}", err)),
    };
    match resp {
        Ok(data) => {
            let success_response = ReturnResponse {
                message: "Success update".to_string(),
                status: true,
                data: Some(data),
            };
            Ok(Json(json!(success_response)))
        }
        Err(err) => {
            let error_response = ReturnResponse {
                message: err,
                status: false,
                data: None,
            };
            Err(Json(json!(error_response)))
        }
    }
}

async fn create_user(Json(t): Json<Users>) -> Result<Json<Value>, Json<Value>> {
    let conn = arango::connection().await.unwrap();
    let aql = AqlQuery::builder()
        .query("INSERT { username: @username, password: @password, created: DATE_NOW()} IN @@col RETURN NEW")
        .bind_var("username", t.username)
        .bind_var("password", t.password)
        .bind_var("@col", "user")
        .batch_size(1)
        .count(true)
        .build();

    let resp: Result<Vec<Users>, String> = match conn.aql_query(aql).await {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error executing AQL query: {:?}", err)),
    };

    match resp {
        Ok(data) => {
            let success_response = ReturnResponse {
                message: "Success create".to_string(),
                status: true,
                data: Some(data),
            };
            Ok(Json(json!(success_response)))
        }
        Err(err) => {
            let error_response = ReturnResponse {
                message: err,
                status: false,
                data: None,
            };
            Err(Json(json!(error_response)))
        }
    }
}

async fn delete_user(Path(t): Path<Users>) -> Result<Json<Value>, Json<Value>> {
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

    let resp: Result<Vec<Users>, String> = match conn.aql_query(aql).await {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error executing AQL query: {:?}", err)),
    };

    match resp {
        Ok(_data) => {
            let success_response = ReturnResponse {
                message: "Success remove".to_string(),
                status: true,
                data: None,
            };
            Ok(Json(json!(success_response)))
        }
        Err(err) => {
            let error_response = ReturnResponse {
                message: err,
                status: false,
                data: None,
            };
            Err(Json(json!(error_response)))
        }
    }
}
