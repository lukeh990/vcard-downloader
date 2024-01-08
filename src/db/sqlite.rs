// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2023 Luke Harding

use rusqlite::{params, Connection};

type Result<T> = super::Result<T>;

pub fn open() -> super::Result<super::SQLite> {
    let conn = Connection::open("./card_data.db3")?;

    Ok(super::SQLite { connection: conn })
}

impl super::Database for super::SQLite {
    fn find(&mut self, string: &str) -> Result<()> {
        println!("{:?}", self.connection);
        Ok(())
    }
}