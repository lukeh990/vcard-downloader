// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2023 Luke Harding

pub mod sqlite;

type Result<T> = super::Result<T>;

pub trait Database {
    fn find(&mut self, string: &str) -> Result<()>;
}

pub struct SQLite {
    connection: rusqlite::Connection
}

pub struct _MySQL {
    // TODO: Implement MySQL
}