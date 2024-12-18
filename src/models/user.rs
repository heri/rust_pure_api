use crate::schema::users;
use chrono::{NaiveDateTime, Local};
use diesel::ExpressionMethods;
use std::sync::Arc;
use crate::db_connection::DbPool;

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct NewUser {
    pub firstName: String,
    pub lastName: String,
    pub playerNumber: String,
    pub created: NaiveDateTime
}

impl NewUser {
    pub fn create(&self, pool: Arc<DbPool>) -> Result<User, diesel::result::Error> {
        use diesel::RunQueryDsl;

        let connection = pool.get()?;
        
        let new_user = NewUser {
            firstName: self.firstName.clone(),
            lastName: self.lastName.clone(),
            playerNumber: self.playerNumber.clone(),
            created: Local::now().naive_local()
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&connection)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserList(pub Vec<User>);

impl UserList {
    pub fn list(pool: Arc<DbPool>) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::users::dsl::*;

        let connection = pool.get().expect("Failed to get DB connection");

        let result = users
            .limit(10)
            .load::<User>(&connection)
            .expect("Error loading users");

        UserList(result)
    }
}

use askama::Template;
#[derive(Template)]
#[template(path = "user.html")]
pub struct UsersTemplate {
    pub users: Vec<User>
}

impl UsersTemplate {
    pub fn latest(pool: Arc<DbPool>) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::users::dsl::*;

        let connection = pool.get().expect("Failed to get DB connection");

        let result = users
            .limit(10)
            .order(created.desc())
            .load::<User>(&connection)
            .expect("Error loading users");

        UsersTemplate { users: result }
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Insertable)]
pub struct User {
    pub Id: i32,
    pub playerNumber: String,
    pub firstName: String,
    pub lastName: String,
    pub tier: Option<i32>,
    pub address1: String,
    pub city: String,
    pub zip: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub id3: Option<String>,
    pub isBanned: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub gender: Option<i32>,
    pub created: NaiveDateTime
}

#[derive(Serialize, Deserialize)]
pub struct Webhook {
    pub model: String,
    pub data: User,
}

impl User {
    pub fn find(Id: &i32, pool: Arc<DbPool>) -> Result<User, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let connection = pool.get()?;
        users::table.find(Id).first(&connection)
    }

    pub fn destroy(Id: &i32, pool: Arc<DbPool>) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::users::dsl;

        let connection = pool.get()?;
        diesel::delete(dsl::users.find(Id)).execute(&connection)?;
        Ok(())
    }

    pub fn update(Id: &i32, user: &User, pool: Arc<DbPool>) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::users::dsl;

        let connection = pool.get()?;
        diesel::update(dsl::users.find(Id))
            .set(user)
            .execute(&connection)?;
        Ok(())
    }
    
    pub fn upsert(webhook: &Webhook, pool: Arc<DbPool>) -> Result<(), diesel::result::Error> {
        use diesel::RunQueryDsl;

        let connection = pool.get()?;
        let user = &webhook.data;

        diesel::insert_into(users::table)
            .values(user)
            .on_conflict(users::Id)
            .do_update()
            .set(user)
            .execute(&connection)?;
        Ok(())
    }
}