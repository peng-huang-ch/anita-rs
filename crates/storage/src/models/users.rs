use async_trait::async_trait;
use chrono;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{schema::users, utils::hash::verify_password, DatabaseError};

/// User details.
#[derive(Queryable, Selectable, AsChangeset, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Auth {
    pub id: i32,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl Auth {
    pub fn verify_password(&self, password: &str) -> bool {
        verify_password(password, self.password.as_str())
    }
}

/// New user details.
#[derive(Insertable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[async_trait]
pub trait UserTrait {
    /// Get a auth by email.
    async fn get_auth_by_email(&self, email: &str) -> Result<Option<Auth>, DatabaseError>;

    /// get a user by id.
    async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, DatabaseError>;
}
