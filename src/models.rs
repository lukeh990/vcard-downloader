use diesel::prelude::*;

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
    pub w_address: String
}

use crate::schema::vcards;

#[derive(Insertable)]
#[diesel(table_name = vcards)]
pub struct NewVCard<'a> {
    pub uuid: &'a str,
    pub email: &'a str,
}