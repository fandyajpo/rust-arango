use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn middleware<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // let response = Response::builder()
    //     .status(StatusCode::UNAUTHORIZED)
    //     .body(Body::from("Unauthorized"))
    //     .unwrap();
    // Err(StatusCode::UNAUTHORIZED)
    // print!("{}", "Runn");
    let response = next.run(request).await;
    Ok(response)
}

// pub async fn middleware<B>(
//     request: Request<B>,
//     next: Next<B>,
// ) -> Result<Response<Body>, StatusCode> {
//     let response = Response::builder()
//         .status(StatusCode::UNAUTHORIZED)
//         .body(Body::from("Unauthorized"))
//         .unwrap();
//     response;
// }
