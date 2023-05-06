use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Queryable,
    Identifiable,
    Insertable,
    AsChangeset,
)]
pub struct User {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Me {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
}
