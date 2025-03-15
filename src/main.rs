use aws_config::load_from_env;
use aws_sdk_s3 as s3;
use derive_more::{Display, Error};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::error::Error;
use tokio;

pub mod models;
pub mod schema;

use self::models::*;
use self::schema::documents::dsl::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn establish_db_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn show_documents() {
    let connection = &mut establish_db_connection();
    let results = documents
        .filter(date.gt(0))
        .limit(5)
        .select(Document::as_select())
        .load(connection)
        .expect("Error loading documents");
    println!("Display {} documents", results.len());
    for doc in results {
        println!("{}", doc.title);
        println!("{}", doc.path);
    }
}

#[derive(Display, Default, Debug)]
enum StorageService {
    #[default]
    Aws,
}
struct Client {
    service: StorageService,
}

struct ListFilesResponse<FM, M> {
    // NOTE: These 3 fields should all be of the same length
    files: Vec<String>,
    files_metadata: Vec<FM>,
    response_metadata: Vec<M>,
}

struct ListFilesResponseRefIter<'a, FM, M>(&'a ListFilesResponse<FM, M>, usize);

impl<'a, FM, M> Iterator for ListFilesResponseRefIter<'a, FM, M> {
    type Item = (&'a str, &'a FM, &'a M);
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.1;
        if idx < self.0.files.len() {
            self.1 += 1;
            Some((
                self.0.files[idx].as_str(),
                &self.0.files_metadata[idx],
                &self.0.response_metadata[idx],
            ))
        } else {
            return None;
        }
    }
}
trait StorageClient {
    async fn list_files(&self) -> Vec<String>;
}
