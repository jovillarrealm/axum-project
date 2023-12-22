//! Run with
//!
//! ```not_rust
//! cargo run -p example-readme
//! ```

use std::error::Error;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use minijinja::render;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // Hard coded BS
    // Load the MongoDB connection string from an environment variable:
    let client_uri = "mongodb+srv://medbuser:YppbFDwNGXoI6x6J@medecluster.izhjskf.mongodb.net/?retryWrites=true&w=majority";

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;
    dbg!(client);
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        //.route("/usertemp", get(template))
        .route("/profile/:profile_name", get(get_profile))
        .route("/hello", get(hello))
        //.route("/getusers", get(get_users))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}

/// Extracts the user's name from url, mocks some orders related to that user and returns a html response from a jinja template
async fn get_profile(Path(profile_name): Path<String>) -> Html<String> {
    let orders_example = vec![
        Items {
            id: 1,
            name: "Article banana".into(),
        },
        Items {
            id: 2,
            name: "Article apple".into(),
        },
    ];
    let profile_example = Profile {
        full_name: profile_name,
        items: orders_example,
    };
    let r = render!(PROFILE_TEMPLATE, profile => profile_example );
    Html(r)
}

/// basic handler that responds with a Html
/// This should build docs now?
async fn root() -> Html<String> {
    let mut x = String::from("helo");
    x = x.replace('l', "ll");
    Html(x)
}

/// basic handler that responds with a static string
async fn hello() -> &'static str {
    "Hello, World!"
}

///
/// This will be converted into a JSON response
/// with a status code of `201 Created`
///
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

/*
async get_users(client) -> impl IntoResponse {
    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }
    
    
}
*/

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

/// Item data, honestly it could be called Item
#[derive(Debug, Serialize)]
struct Items {
    id: i32,
    name: String,
}

/// Profile data
#[derive(Debug, Serialize)]
struct Profile {
    full_name: String,
    items: Vec<Items>,
}

/// Template for a simple html viewport for some mocked data :3
const PROFILE_TEMPLATE: &'static str = r#"
<!doctype html>

<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">

<title>A Basic HTML5 Template</title>
<meta name="description" content="A basic HTML5 Template for new projects.">
<meta name="author" content="Woile">
</head>

<body>
<h1>Profile of {{ profile.full_name|title }}</h1>
<p>This is a template example to show some functionality</p>
<h2>Items</h3>
<ul>
{% for item in profile.items %}
<li>{{ item.name }} ({{ item.id }})</li>
{% endfor %}
<ul>
</body>
</html>
"#;

/// Template for a simple html viewport for some mocked data :3
const USERS_TEMPLATE: &'static str = r#"
<!doctype html>

<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">

<title>A Basic HTML5 Template</title>
<meta name="description" content="A basic HTML5 Template for new projects.">
<meta name="author" content="Woile">
</head>

<body>
<h1>Profile of {{ profile.full_name|title }}</h1>
<p>This is a template example to show some functionality</p>
<h2>Items</h3>
<ul>
{% for item in profile.items %}
<li>{{ item.name }} ({{ item.id }})</li>
{% endfor %}
<ul>
</body>
</html>
"#;
#[tokio::test]
async fn each() {
    let x = Html("Hello, World!").0;
    assert_eq!(root().await.0, x);
}
