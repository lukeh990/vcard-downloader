use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::*;
use models::*;

pub fn establish_connection() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(SqliteConnection::establish(&database_url)?)
}

pub fn search_card(find_uuid: String) -> Result<VCard> {
    use schema::vcards::dsl::*;
    
    let connection = &mut establish_connection()?;
    let results: Vec<VCard> = vcards
        .filter(uuid.eq(find_uuid))
        .select(VCard::as_select())
        .load(connection)?;

    let card: VCard = match results.iter().nth(0) {
        Some(card) => card.clone(),
        None => return Err("No Match".into())
    };

    Ok(card)
}