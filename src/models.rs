// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2023-2024 Luke Harding

use diesel::prelude::*;

use crate::schema::analytics;
use crate::schema::vcards;

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::vcards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct VCard {
    pub uuid: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub middlename: String,
    pub organization: String,
    pub w_phone: String,
    pub title: String,
    pub url: String,
    pub workurl: String,
    pub note: String,
    pub nickname: String,
    pub prefix: String,
    pub suffix: String,
    pub role: String,
    pub h_phone: String,
    pub c_phone: String,
    pub p_phone: String,
    pub h_fax: String,
    pub w_fax: String,
    pub h_email: String,
    pub w_email: String,
    pub alias: String,
    pub h_address: String,
    pub w_address: String,
}

#[derive(Insertable)]
#[diesel(table_name = vcards)]
pub struct NewVCard<'a> {
    pub uuid: &'a str,
    pub email: &'a str,
}

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::analytics)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Analytic {
    pub uuid: String,
    pub timestamp: i32,
    pub card_uuid: String,
}

#[derive(Insertable)]
#[diesel(table_name = analytics)]
pub struct NewAnalytic<'a> {
    pub uuid: &'a str,
    pub timestamp: i32,
    pub card_uuid: &'a str,
}
