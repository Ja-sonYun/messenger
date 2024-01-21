use database::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Selectable)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub realname: String,
    pub nickname: Option<String>,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub realname: &'a str,
    pub email: &'a str,
}
