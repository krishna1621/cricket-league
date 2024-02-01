use actix_web::{App, HttpServer, web};
use sqlx::MySqlPool;
mod entity;
mod service;
use service::user_services::add_user;
use service::user_services::get_user;
use service::user_services::delete_user;
use service::user_services::update_user;
use service::booking_services::add_booking_details;
use service::team_services::add_team_details;
use service::team_members_services::add_team_members;
use service::team_services::delete_team;
use service::team_services::get_all_teams;
use service::team_services::update_team_details;
use service::jwt_service::login;
use service::data_parsing::parsing_data;
use service::data_parsing::parsing_alterterm_code;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");

        let pool = MySqlPool::connect(&database_url)
            .await
            .expect("Failed to create MySQL pool");

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(add_user)
                .service(get_user)
                .service(update_user)
                .service(delete_user)
                .service(login)
                .service(add_team_details)
                .service(get_all_teams)
                .service(update_team_details)
                .service(delete_team)
                .service(add_booking_details)
                .service(add_team_members)
                .service(parsing_data)
                .service(parsing_alterterm_code)
               // .wrap(middleware::Logger::default())
              //  .route("/login", web::post().to(login))
            //  .service(web::scope("/add_team_members").guard(jwt_middleware())
            //  .service(add_team_members))
        })
            .bind("0.0.0.0:8081")?
            .run()
            .await
            .map_err(|e| {
                eprintln!("Failed to start the server: {:?}", e);
                e
            })
    }

