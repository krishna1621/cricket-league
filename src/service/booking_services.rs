use actix_web::{post, web, HttpResponse};
use crate::entity::models::Booking;
use sqlx::MySqlPool;
#[post("/add_booking_details")]
async fn add_booking_details(
    booking: web::Json<Booking>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let booking_id = &booking.booking_id;
    let availability = &booking.availability;
    let from_time = &booking.from_time;
    let to_time = &booking.to_time;
    let status = &booking.status;
    let match_id = &booking.match_id;
    let ground_id = &booking.ground_id;
    let home_team_id = &booking.home_team_id;
    let guest_team_id = &booking.guest_team_id;

    let result = sqlx::query(
        r#"
        INSERT INTO bookings_details (booking_id, availability, from_time, to_time, status, match_id, ground_id, home_team_id, guest_team_id)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#,
    )
        .bind(booking_id)
        .bind(availability)
        .bind(from_time)
        .bind(to_time)
        .bind(status)
        .bind(match_id)
        .bind(ground_id)
        .bind(home_team_id)
        .bind(guest_team_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().body("Booking created successfully"),
        Err(err) => {
            eprintln!("Failed to insert booking_details details: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Failed to insert booking_details details: {:?}", err))
        }
    }
}