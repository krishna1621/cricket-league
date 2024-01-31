use crate::entity::models::Team;
use actix_web::{delete, get, post, put, web, HttpResponse};
use sqlx::MySqlPool;

#[post("/add_team_details")]
async fn add_team_details(team: web::Json<Team>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let team_id = &team.team_id;
    let team_name = &team.team_name;
    let is_internal = &team.is_internal;
    let team_caption = &team.team_captain;

    let result = sqlx::query(
        r#"
        INSERT INTO team_details (team_id,team_name,is_internal,team_captain) VALUES (?, ?, ?,?)
    "#,
    )
        .bind(team_id)
        .bind(team_name)
        .bind(is_internal)
        .bind(team_caption)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().body("created successfully"),
        Err(err) => {
            eprintln!("Failed to insert team_details: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Failed to insert team_details: {:?}", err))
        }
    }
}
#[get("/get_team_details/{team_name}")]
async fn get_team_details(path: web::Path<String>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let team_name = path.into_inner();

    // Execute SQL query to fetch team details by team_id
    let result = sqlx::query_as::<_, Team>(
        r#"
        SELECT * FROM team_details WHERE team_name = ?
    "#,
    )
        .bind(&team_name)
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(team)) => HttpResponse::Ok().json(team),
        Ok(None) => HttpResponse::NotFound().body("Team not found"),
        Err(err) => {
            eprintln!("Failed to fetch team details: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Failed to fetch team details: {:?}", err))
        }
    }
}
#[get("/get_all_teams")]
async fn get_all_teams(pool: web::Data<MySqlPool>) -> HttpResponse {

    let result = sqlx::query_as::<_, Team>(
        r#"
        SELECT * FROM team_details
    "#,
    )
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(teams) => HttpResponse::Ok().json(teams),
        Err(err) => {
            eprintln!("Failed to fetch all team details: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Failed to fetch all team details: {:?}", err))
        }
    }
}
#[put("/update_team_details/{team_id}")]
async fn update_team_details(
    path: web::Path<String>,
    updated_team: web::Json<Team>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let team_id = path.into_inner();


    let result = sqlx::query(
        r#"
        UPDATE team_details
        SET team_name = ?, is_internal = ?, team_captain = ?
        WHERE team_id = ?
    "#,
    )
        .bind(&updated_team.team_name)
        .bind(&updated_team.is_internal)
        .bind(&updated_team.team_captain)
        .bind(&team_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Team details updated successfully"),
        Err(err) => {
            eprintln!("Failed to update team details: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Failed to update team details: {:?}", err))
        }
    }
}

#[delete("/delete_team/{team_name}")]
async fn delete_team(path: web::Path<String>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let team_name = path.into_inner();


    let result = sqlx::query(
        r#"
        DELETE FROM team_details WHERE team_name = ?
    "#,
    )
        .bind(&team_name)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Team deleted successfully"),
        Err(err) => {
            eprintln!("Failed to delete team: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Failed to delete team: {:?}", err))
        }
    }
}