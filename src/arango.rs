use arangors::{uclient::reqwest::ReqwestClient, Connection, Database};

pub async fn connection() -> Result<Database<ReqwestClient>, String> {
    // ARANGO_URL
    let con = match Connection::establish_jwt("http://127.0.0.1:8529/", "root", "").await {
        Ok(connection) => connection,
        Err(err) => return Err(format!("Error establishing connection: {:?}", err)),
    };
    // COLLECTION NAME
    match con.db("DATABASE_NAME").await {
        Ok(db) => Ok(db),
        Err(err) => Err(format!("Error getting database: {:?}", err)),
    }
}

pub fn print_to(num: i32) {
    for n in 1..num {
        println!("{}", n)
    }
}
