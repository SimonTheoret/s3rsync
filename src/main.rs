use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::*;
use self::schema::documents::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    println!("Hello, world!");
}

fn show_posts() {
    let connection = &mut establish_connection();
    let results = documents
        .filter(date.gt(0))
        .limit(5)
        .select(Document::as_select())
        .load(connection)
        .expect("Error loading documents");
    println!("Display {} documents", results.len());
    for doc in results {
        println!("{}", doc.title);
        println!("{}", doc.body);
    }
}
