use serde::{Deserialize, Serialize};
use crate::utils::database_utils::SqlConnection;
use diesel::{
    insert_into,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods
};

use crate::utils::error_mapper::ElogError;

use super::schema::user_category;
use super::schema::user_category::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserCategory {
    pub id: i16,
    pub user_id: i16,
    pub category: String,
    pub description: String
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[serde(default)]
#[table_name = "user_category"]
pub struct NewUserCategory {
    pub user_id: i16,
    pub category: String,
    pub description: String
}

impl Default for NewUserCategory {
    fn default() -> Self {
        NewUserCategory {
            user_id: 0,
            category: String::from(""),
            description: String::from("")
        }
    }
}

impl UserCategory {

    pub fn insert(
        connection: &SqlConnection,
        new_user_category: NewUserCategory
    ) -> Result<usize, ElogError> {
        insert_into(user_category)
            .values(&new_user_category)
            .execute(connection)
            .map_err(|error| { ElogError::InsertFailure(error.to_string()) })
    }

    pub fn get_list(
        connection: &SqlConnection,
        logged_user_id: i16
    ) -> Result<Vec<UserCategory>, ElogError> {
        user_category
            .filter(user_id.eq(logged_user_id))
            .load::<UserCategory>(connection)
            .map_err(|error| { ElogError::ObjectNotFound(error.to_string()) })
    }
}
