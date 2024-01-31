use actix_web::{post, web, HttpResponse};
use crate::entity::models::TeamMember;
use sqlx::MySqlPool;
#[post("/add_team_members")]
async fn add_team_members(
    team_members: web::Json<Vec<TeamMember>>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    for team_member in team_members.iter() {
        let result = sqlx::query(
            r#"
            INSERT INTO team_members (player_id, player_name, role, is_captain, team_id)
            VALUES (?, ?, ?, ?, ?)
        "#,
        )
            .bind(&team_member.player_id)
            .bind(&team_member.player_name)
            .bind(&team_member.role)
            .bind(&team_member.is_captain)
            .bind(&team_member.team_id)
            .execute(pool.get_ref())
            .await;

        if let Err(err) = result {
            eprintln!("Failed to insert team member: {:?}", err);
            return HttpResponse::InternalServerError()
                .body(format!("Failed to insert team member: {:?}", err));
        }
    }

    HttpResponse::Created().body("Team members added successfully")
}
