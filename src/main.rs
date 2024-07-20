use postgres::{Client, Error, Row};
use postgres::types::Type;
use postgres_openssl::MakeTlsConnector;
use openssl::ssl::{SslConnector, SslMethod};
use std::io::{self, Write};
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <database_url>", args[0]);
        std::process::exit(1);
    }
    
    let db_url = &args[1];

    // Set up SSL connector
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(openssl::ssl::SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());

    let mut client = Client::connect(db_url, connector)?;

    loop {
        print_menu();
        let choice = get_user_input("Enter your choice: ");

        match choice.as_str() {
            "1" => view_tables(&mut client)?,
            "2" => view_table_data(&mut client)?,
            "3" => edit_table_data(&mut client)?,
            "4" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }

    Ok(())
}

fn print_menu() {
    println!("\nPostgreSQL Viewer and Editor");
    println!("1. View Tables");
    println!("2. View Table Data");
    println!("3. Edit Table Data");
    println!("4. Exit");
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn view_tables(client: &mut Client) -> Result<(), Error> {
    let rows = client.query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'", &[])?;

    println!("\nTables in the database:");
    for row in rows {
        let table_name: &str = row.get(0);
        println!("- {}", table_name);
    }

    Ok(())
}

fn view_table_data(client: &mut Client) -> Result<(), Error> {
    let table_name = get_user_input("Enter table name: ");
    
    // First, check if the table exists (case-insensitive)
    let table_exists_query = "
        SELECT table_name 
        FROM information_schema.tables 
        WHERE table_schema = 'public' 
        AND lower(table_name) = lower($1)
    ";
    let rows = client.query(table_exists_query, &[&table_name])?;
    
    if rows.is_empty() {
        println!("Error: Table '{}' does not exist.", table_name);
        return Ok(());
    }

    // Get the correct case for the table name
    let correct_table_name: String = rows[0].get(0);

    let query = format!("SELECT * FROM \"{}\" LIMIT 10", correct_table_name);
    let rows = client.query(&query, &[])?;

    if rows.is_empty() {
        println!("The table '{}' is empty.", correct_table_name);
        return Ok(());
    }

    println!("\nData in table {}:", correct_table_name);
    for row in rows {
        print_row(&row);
    }

    Ok(())
}

fn print_row(row: &Row) {
    let mut values = Vec::new();
    for (i, column) in row.columns().iter().enumerate() {
        let value = match column.type_() {
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
        };
        values.push(format!("{}: {}", column.name(), value));
    }
    println!("{}", values.join(", "));
}


fn edit_table_data(client: &mut Client) -> Result<(), Error> {
    let table_name = get_user_input("Enter table name: ");
    let column_name = get_user_input("Enter column name: ");
    let id = get_user_input("Enter ID of the row to edit: ");
    let new_value = get_user_input("Enter new value: ");

    let query = format!(
        "UPDATE \"{}\" SET \"{}\" = $1 WHERE id = $2",
        table_name, column_name
    );
    let result = client.execute(&query, &[&new_value, &id])?;

    if result == 0 {
        println!("No rows were updated. Please check your input.");
    } else {
        println!("Data updated successfully!");
    }

    Ok(())
}