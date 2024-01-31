use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub(crate) struct User{
    pub(crate) id: i32,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) confirm_password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LoginRequest{
    pub(crate)  email: String,
    pub(crate) password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AccessToken {
    pub(crate) token: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub(crate) struct GroundDetails {
    pub(crate) ground_id: String,
    pub(crate) ground_name: String,
    pub(crate) ground_address: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub(crate) struct Team {
    pub(crate) team_id: String,
    pub(crate) team_name: String,
    pub(crate) is_internal: bool,
    pub(crate) team_captain: String,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub(crate) struct Booking {
    pub(crate) booking_id: i32,
    pub(crate) availability: bool,
    pub(crate) from_time: String,
    pub(crate) to_time: String,
    pub(crate) status: String,
    pub(crate) match_id: i32,
    pub(crate) ground_id: i32,
    pub(crate) home_team_id: i32,
    pub(crate) guest_team_id: i32,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub(crate) struct TeamMember {
    pub player_id: i32,
    pub player_name: String,
    pub role: String,
    pub is_captain: bool,
    pub team_id: i32,
}