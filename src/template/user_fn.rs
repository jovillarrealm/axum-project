use super::user::PROFILE_TEMPLATE;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use minijinja::render;
use serde::{Deserialize, Serialize};

/// Extracts the user's name from url, mocks some orders related to that user and returns a html response from a jinja template
pub async fn get_profile(Path(profile_name): Path<String>) -> Html<String> {
    let orders_example = vec![
        Item {
            id: 1,
            name: "Article banana".into(),
        },
        Item {
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

///
/// This will be converted into a JSON response
/// with a status code of `201 Created`
///
pub async fn create_user(
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

/// The input to our `create_user` handler.
#[derive(Deserialize)]
pub struct CreateUser {
    /// owned string 
    username: String,
}

/// The output to our `create_user` handler.
#[derive(Serialize)]
pub struct User {
    /// db should use something else
    id: u64,
    /// If unchecked, this is what will be used by most operations which is a shame
    username: String,
}

/// Item data, honestly it could be called Item
#[derive(Debug, Serialize)]
pub struct Item {
    id: i32,
    name: String,
}

/// Profile data
#[derive(Debug, Serialize)]
pub struct Profile {
    full_name: String,
    items: Vec<Item>,
}
