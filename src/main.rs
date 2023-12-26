//! Run with
//!
//! ```not_rust
//! cargo run -p example-readme
//! ```


use std::error::Error;

use axum::{
    response::Html,
    routing::{get, post},
    Router,
};


pub mod template;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize tracing
    tracing_subscriber::fmt::init();
    println!("Requiere un secret.txt en cwd");
    
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        //.route("/usertemp", get(template))
        .route("/profile/:profile_name", get(template::user_fn::get_profile))
        .route("/hello", get(hello))
        //.route("/getusers", get(get_users))
        // `POST /users` goes to `create_user`
        .route("/c_user", post(template::user_fn::create_user))
        .route("/users", get(template::db::get_mongo_users));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}



/// basic handler that responds with a Html
/// This should build docs now.
async fn root() -> Html<String> {
    let mut x = String::from("helo");
    x = x.replace('l', "ll");
    Html(x)
}

/// basic handler that responds with a static string
async fn hello() -> &'static str {
    "Hello, World!"
}




#[tokio::test]
async fn each() {
    let x = Html("Hello, World!").0;
    assert_eq!(root().await.0, x);
}
