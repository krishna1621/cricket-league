use actix_web::{web, HttpResponse, post};
use sqlx::MySqlPool;
use tokio::fs::File;
use tokio::io::{self, BufReader, AsyncBufReadExt};
use std::path::Path;


#[post("/process_codes")]
pub async fn parsing_data(
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let path = Path::new("./src/files/icd10cm_order.txt");

    // Open files asynchronously
    let file = File::open(&path).await;

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);

            // Process files in parallel
            let result = tokio::join!(
                process_file_codes(reader, pool.clone()),
            );

            // Check results
            if let (Err(e),) = result {
                return HttpResponse::InternalServerError().body(format!("Error processing codes: {:?}", e));
            }

            HttpResponse::Ok().finish()
        }
        _ => HttpResponse::InternalServerError().body("Failed to open files"),
    }
}

async fn process_file_codes(
    mut reader: BufReader<File>,
    pool: web::Data<MySqlPool>,
) -> io::Result<()> {
    let mut line = String::new();
    while reader.read_line(&mut line).await? != 0 {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let code = parts[0].to_string();
            let description = parts[1].to_string();
            let long_description = parts[2..].join(" ");

              sqlx::query(
                  r#"
                INSERT INTO code_details (code, description, longdescription) VALUES (?, ?, ?)
                 "#
              )
                  .bind(&code)
                  .bind(&description)
                  .bind(&long_description)
                  .execute(pool.get_ref())
                  .await.expect(" mysql error msg");

        }
        line.clear();
    }
    Ok(())
}

#[post("/process_alterterm")]
pub async fn parsing_alterterm_code(
    pool: web::Data<MySqlPool>,
) -> HttpResponse {

    let path = Path::new("./src/files/Alternate-Terms-2023.txt");

    // Open files asynchronously
    let file = File::open(&path).await;

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);

            // Process files in parallel
            let result = tokio::join!(
                process_file_alterterm(reader, pool.clone()),
            );

            // Check results
            if let (Err(e),) = result { // Note the comma after `Err(e)`
                return HttpResponse::InternalServerError().body(format!("Error processing alterterm: {:?}", e));
            }

            HttpResponse::Ok().finish()
        }
        _ => HttpResponse::InternalServerError().body("Failed to open files"),
    }
}
async fn process_file_alterterm(
    mut reader: BufReader<File>,
    pool: web::Data<MySqlPool>,
) -> io::Result<()> {
    let mut line = String::new();
    while reader.read_line(&mut line).await? != 0 {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let code = parts[0].to_string();
            let alterdescription = parts[1].to_string();
            let long_description = parts[2..].join(" ");

            sqlx::query(
                r#"
                INSERT INTO Alterterm_details (code, alterdescription, longdescription) VALUES (?, ?, ?)
                 "#
            )
                .bind(&code)
                .bind(&alterdescription)
                .bind(&long_description)
                .execute(pool.get_ref())
                .await.expect(" mysql error msg");

        }
        line.clear();
    }
    Ok(())
}