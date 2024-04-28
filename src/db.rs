// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2023-2024 Luke Harding

use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use uuid::Uuid;

use models::*;

use crate::*;
use crate::schema::analytics;

use self::models::{Analytic, NewAnalytic};

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

    let new_uuid = Uuid::new_v4().to_string();
    let current_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(current_time) => current_time,
        Err(_) => return Err("Time went backwards!".into()),
    };

    let new_analytic = NewAnalytic {
        uuid: &new_uuid,
        timestamp: current_time.as_secs() as i32,
        card_uuid: &card.uuid,
    };

    let _ = diesel::insert_into(analytics::table)
        .values(&new_analytic)
        .returning(Analytic::as_returning())
        .get_result(connection);

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

    let new_uuid = Uuid::new_v4().to_string();
    let current_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(current_time) => current_time,
        Err(_) => return Err("Time went backwards!".into()),
    };

    let new_analytic = NewAnalytic {
        uuid: &new_uuid,
        timestamp: current_time.as_secs() as i32,
        card_uuid: &card.uuid,
    };

    let _ = diesel::insert_into(analytics::table)
        .values(&new_analytic)
        .returning(Analytic::as_returning())
        .get_result(connection);

    Ok(card)
}
