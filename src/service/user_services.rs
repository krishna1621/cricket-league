use actix_web::{delete, get, HttpResponse, post, put, web};
use bcrypt::hash;
use sqlx::MySqlPool;
use crate::entity::models::User;

#[post("/add_user")]
async fn add_user(user: web::Json<User>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let username = &user.username;
    let email = &user.email;
    let password = &user.password;
    let confirm_password = &user.confirm_password;

    let hashed_password = hash(password, 12).expect("Failed to hash password");

    let result = sqlx::query(
        r#"
        INSERT INTO users (username, email, password, confirm_password) VALUES (?, ?, ?, ?)
    "#,
    )
        .bind(username)
        .bind(email)
        .bind(hashed_password)
        .bind(confirm_password)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => {
            eprintln!("Failed to insert user: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Failed to insert user: {:?}", err))
        }
    }
}


#[get("/get_user/{username}")]
async fn get_user(username: web::Path<String>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let username = username.into_inner();

    // Execute a SELECT query to retrieve user details by username
    let result = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE username = ?
    "#,
    )
        .bind(username)
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            eprintln!("Failed to retrieve user: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Failed to retrieve user: {:?}", err))
        }
    }
}


#[put("/update_user/{username}")]
async fn update_user(
    username: web::Path<String>,
    new_user: web::Json<User>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let username_param = username.into_inner();
    let new_username = &new_user.username;
    let new_email = &new_user.email;
    let new_password = &new_user.password;
    let new_confirm_password = &new_user.confirm_password;

    // You may want to add validation logic here if needed.

    // Hash the new password if it's provided.
    let hashed_password = if !new_password.is_empty() {
        Some(hash(new_password, 12).expect("Failed to hash password"))
    } else {
        None
    };

    let result = sqlx::query(
        r#"
        UPDATE users
        SET username = ?, email = ?, password = ?, confirm_password = ?
        WHERE username = ?
    "#,
    )
        .bind(new_username)
        .bind(new_email)
        .bind(hashed_password.as_deref())
        .bind(new_confirm_password)
        .bind(&username_param)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            eprintln!("Failed to update user: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Failed to update user: {:?}", err))
        }
    }
}

#[delete("/delete_user/{username}")]
async fn delete_user(username: web::Path<String>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let username_param = username.into_inner();

    // Execute a DELETE query to remove the user by username
    let result = sqlx::query(
        r#"
        DELETE FROM users WHERE username = ?
    "#,
    )
        .bind(&username_param)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            eprintln!("Failed to delete user: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Failed to delete user: {:?}", err))
        }
    }
}