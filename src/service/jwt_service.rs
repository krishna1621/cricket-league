use actix_web::{post, HttpResponse, web};
use jsonwebtoken::EncodingKey;
use serde::{Serialize,Deserialize};
use sqlx::MySqlPool;
use crate::entity::models::{AccessToken, LoginRequest, User};


#[post("/login")]
async fn login(
    pool: web::Data<MySqlPool>,
    request: web::Json<LoginRequest>,
) -> actix_web::Result<HttpResponse> {
    // Check if the email and password are valid (you may want to hash the password before comparing)
    // Perform the database query to validate the user credentials
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = ?",
        request.email,

    )
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Error querying the database: {:?}", e);
            actix_web::error::ErrorInternalServerError("Internal Server Error")
        })?;

    // Check if the user exists
    let user = user.ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;

    // Generate JWT token (you may use a library like jsonwebtoken)
    let token = generate_jwt_token(&user)?;

    // Return the token in the response
    Ok(HttpResponse::Ok().json(AccessToken { token }))
}

// main.rs

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32, // Adjust the type based on your user identifier type
    // Add other claims...
}


fn generate_jwt_token(user: &User) -> Result<String, actix_web::Error> {
    let claims = Claims {
        sub: user.id,
        // Add other claims...
    };



    let secret = "3541c97a4943c2122391978963cfaf17d65251f4e916d925d61656ee1e0dad22";
    // Convert the secret key to an EncodingKey
    let secret_key = EncodingKey::from_secret(secret.as_bytes());
    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &secret_key)
        .map_err(|e| {
            eprintln!("Error encoding JWT token: {:?}", e);
            actix_web::error::ErrorInternalServerError("Internal Server Error")
        })?;

    Ok(token)
}
