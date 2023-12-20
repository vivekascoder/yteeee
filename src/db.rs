use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Queryable, Selectable, Insertable, Clone, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::tasks)]
pub struct Task {
    pub id: String,
    pub youtube_url: String,
    pub status: String,
    pub result: Option<String>,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
