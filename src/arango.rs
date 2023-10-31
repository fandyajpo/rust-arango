use arangors::{uclient::reqwest::ReqwestClient, Connection, Database};

pub async fn connection() -> Result<Database<ReqwestClient>, String> {
    // ARANGO_URL
    let con = match Connection::establish_jwt("http://127.0.0.1:8529/", "root", "").await {
        Ok(connection) => connection,
        Err(err) => return Err(format!("Error establishing connection: {:?}", err)),
    };
    // COLLECTION NAME
    match con.db("fandy").await {
        Ok(db) => Ok(db),
        Err(err) => Err(format!("Error getting database: {:?}", err)),
    }
}

// pub async fn findUsername(username: String) -> Result<Json<Value>, Json<Value>> {
//     let conn = connection().await.unwrap();
//     let aql = AqlQuery::builder()
//         .query(
//             "LET u = DOCUMENT(@@col, @key)
//              FILTER u != null
//              REMOVE { _key: u._key } IN @@col",
//         )
//         .bind_var("key", key)
//         .bind_var("@col", "user")
//         .batch_size(1)
//         .count(true)
//         .build();

//     let resp: Result<Vec<Users>, String> = match conn.aql_query(aql).await {
//         Ok(data) => Ok(data),
//         Err(err) => Err(format!("Error executing AQL query: {:?}", err)),
//     };

//     match resp {
//         Ok(_data) => {
//             let success_response = ReturnResponse {
//                 message: "Success remove".to_string(),
//                 status: true,
//                 data: None,
//             };
//             Ok(Json(json!(success_response)))
//         }
//         Err(err) => {
//             let error_response = ReturnResponse {
//                 message: err,
//                 status: false,
//                 data: None,
//             };
//             Err(Json(json!(error_response)))
//         }
//     }
// }
