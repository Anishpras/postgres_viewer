#[macro_use] extern crate rocket;

use rocket::State;
use rocket::serde::json::Json;
use rocket::fs::{FileServer, relative};
use serde::{Serialize, Deserialize};
use tokio_postgres::{Row, types::Type};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres_rustls::MakeRustlsConnect;

type Pool = bb8::Pool<PostgresConnectionManager<MakeRustlsConnect>>;

#[derive(Serialize, Deserialize)]
struct TableData {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

#[get("/tables")]
async fn get_tables(pool: &State<Pool>) -> Json<Vec<String>> {
    let client = pool.get().await.unwrap();
    let rows = client.query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'", &[]).await.unwrap();
    let tables: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
    Json(tables)
}

#[get("/table/<name>")]
async fn get_table_data(name: String, pool: &State<Pool>) -> Json<TableData> {
    let client = pool.get().await.unwrap();
    let query = format!("SELECT * FROM \"{}\" LIMIT 10", name);
    let rows = client.query(&query, &[]).await.unwrap();
    
    let columns: Vec<String> = if !rows.is_empty() {
        rows[0].columns().iter().map(|col| col.name().to_string()).collect()
    } else {
        vec![]
    };
    let data: Vec<Vec<String>> = rows.iter().map(|row| format_row(row)).collect();
    
    Json(TableData { columns, rows: data })
}

#[post("/edit", data = "<edit_data>")]
async fn edit_table_data(edit_data: Json<serde_json::Value>, pool: &State<Pool>) -> Json<bool> {
    let client = pool.get().await.unwrap();
    let table_name = edit_data["table"].as_str().unwrap();
    let column_name = edit_data["column"].as_str().unwrap();
    let id = edit_data["id"].as_str().unwrap();
    let new_value = edit_data["value"].as_str().unwrap();

    let query = format!(
        "UPDATE \"{}\" SET \"{}\" = $1 WHERE id = $2",
        table_name, column_name
    );
    let result = client.execute(&query, &[&new_value, &id]).await.unwrap();

    Json(result > 0)
}

fn format_row(row: &Row) -> Vec<String> {
    row.columns().iter().enumerate().map(|(i, col)| {
        match col.type_() {
            &Type::BOOL => format!("{:?}", row.get::<_, Option<bool>>(i)),
            &Type::INT2 | &Type::INT4 => format!("{:?}", row.get::<_, Option<i32>>(i)),
            &Type::INT8 => format!("{:?}", row.get::<_, Option<i64>>(i)),
            &Type::FLOAT4 | &Type::FLOAT8 => format!("{:?}", row.get::<_, Option<f64>>(i)),
            &Type::TEXT | &Type::VARCHAR => format!("{:?}", row.get::<_, Option<String>>(i)),
            &Type::TIMESTAMP => format!("{:?}", row.get::<_, Option<chrono::NaiveDateTime>>(i)),
            &Type::TIMESTAMPTZ => format!("{:?}", row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(i)),
            &Type::DATE => format!("{:?}", row.get::<_, Option<chrono::NaiveDate>>(i)),
            &Type::JSON | &Type::JSONB => format!("{:?}", row.get::<_, Option<serde_json::Value>>(i)),
            _ => format!("{:?}", row.get::<_, Option<String>>(i)),
        }
    }).collect()
}

#[launch]
async fn rocket() -> _ {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let tls_connector = {
        let mut roots = rustls::RootCertStore::empty();
        roots.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));
        rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(roots)
            .with_no_client_auth()
    };

    let tls = MakeRustlsConnect::new(tls_connector);
    let manager = PostgresConnectionManager::new_from_stringlike(db_url, tls).unwrap();
    let pool = bb8::Pool::builder().build(manager).await.unwrap();

    rocket::build()
        .mount("/", routes![get_tables, get_table_data, edit_table_data])
        .mount("/", FileServer::from(relative!("static")))
        .manage(pool)
}