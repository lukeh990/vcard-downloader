use crate::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use models::*;
use std::env;

pub fn establish_connection() -> Result<SqliteConnection> {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or("file:card_data.db3".to_owned());
    Ok(SqliteConnection::establish(&database_url)?)
}

pub fn search_card(find_uuid: String) -> Result<VCard> {
    use schema::vcards::dsl::*;

    let connection = &mut establish_connection()?;
    let results: Vec<VCard> = vcards
        .filter(uuid.eq(find_uuid))
        .select(VCard::as_select())
        .load(connection)?;

    let card = match results.first() {
        Some(card) => card.clone(),
        None => return Err("No Match Found".into()),
    };

    Ok(card)
}

pub fn alias_search_card(find_alias: String) -> Result<VCard> {
    use schema::vcards::dsl::*;

    let connection = &mut establish_connection()?;
    let alias_results: Vec<VCard> = vcards
        .filter(alias.eq(find_alias))
        .select(VCard::as_select())
        .load(connection)?;

    let card = match alias_results.first() {
        Some(card) => card.clone(),
        None => return Err("No Match Found".into()),
    };

    Ok(card)
}
