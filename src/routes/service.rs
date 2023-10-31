use crate::bcrypt::verify_password;
use crate::model;
use crate::{arango, model::response::ReturnResponse};
use arangors::AqlQuery;
use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn login(Json(t): Json<model::user::Users>) -> Result<Json<Value>, StatusCode> {
    let conn = arango::connection().await.unwrap();
    let password = t.password.unwrap();
    let username = t.username.unwrap();
    let aql = AqlQuery::builder()
        .query(
            "FOR u IN @@col
             FILTER u.username == @username
             RETURN u",
        )
        .bind_var("@col", "user")
        .bind_var("username", username)
        .batch_size(1)
        .count(true)
        .build();

    let resp: Result<Vec<model::user::Users>, String> = match conn.aql_query(aql).await {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error executing AQL query: {:?}", err)),
    };

    match resp {
        Ok(data) => {
            if let Some(user) = data.first() {
                if let Some(stored_password_hash) = &user.password {
                    if verify_password(password, stored_password_hash).unwrap() {
                        let error_response: ReturnResponse<model::user::Users> =
                            model::response::ReturnResponse {
                                message: "Authorized".to_owned(),
                                status: true,
                                data: Some(data),
                            };

                        return Ok(Json(json!(error_response)));
                    }
                    return Err(StatusCode::BAD_REQUEST);
                }
                return Err(StatusCode::BAD_REQUEST);
            }

            Err(StatusCode::BAD_REQUEST)
        }
        Err(_err) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn refresh() {}

pub async fn force_refresh() {}
