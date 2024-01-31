use actix_web::{web, HttpResponse, post};
use sqlx::MySqlPool;
use tokio::fs::File;
use tokio::io::{self, BufReader, AsyncBufReadExt};
use std::path::Path;


#[post("/process_data")]
pub async fn parsing_data(
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let path1 = Path::new("./src/files/icd10cm_order.txt");
    let path2 = Path::new("./src/files/Alternate-Terms-2023.txt");

    // Open files asynchronously
    let file1 = File::open(&path1).await;
    let file2 = File::open(&path2).await;

    match (file1, file2) {
        (Ok(file1), Ok(file2)) => {
            let reader1 = BufReader::new(file1);
            let reader2 = BufReader::new(file2);

            // Process files in parallel
            let (result1, result2) = tokio::join!(
                process_file_codes(reader1, pool.clone()),
                process_file_alterterm(reader2, pool.clone()),
            );

            // Check results
            if let Err(e) = result1 {
                return HttpResponse::InternalServerError().body(format!("Error processing file 1: {:?}", e));
            }
            if let Err(e) = result2 {
                return HttpResponse::InternalServerError().body(format!("Error processing file 2: {:?}", e));
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