use crate::schema::users;
use chrono::{NaiveDateTime, Local};
use diesel::ExpressionMethods;

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct NewUser {
    pub firstName: String,
    pub lastName: String,
    pub playerNumber: String,
    pub created: NaiveDateTime
}

impl NewUser {

    pub fn create(&self) -> Result<User, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();
        
        let new_user = NewUser {
            // first_name: "test_string" would not work . That would pass a reference but this expects insead a std::string::String
            firstName: self.lastName.to_string(), 
            lastName: self.lastName.to_string(), 
            playerNumber: self.playerNumber.to_string(), 
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
    pub fn list() -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::users::dsl::*;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        let result = 
            users
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

    pub fn latest() -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::users::dsl::*;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        let result = 
            users
                .limit(10)
                .order(created.desc())
                .load::<User>(&connection)
                .expect("Error loading users");

        return UsersTemplate{ users: result };
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
    pub fn find(Id: &i32) -> Result<User, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        users::table.find(Id).first(&connection)
    }

    pub fn destroy(Id: &i32) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::users::dsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        diesel::delete(dsl::users.find(Id)).execute(&connection)?;
        Ok(())
    }

    pub fn update(Id: &i32, user: &User) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::users::dsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        diesel::update(dsl::users.find(Id))
            .set(user)
            .execute(&connection)?;
        Ok(())
    }
    
    pub fn upsert(webhook: &Webhook) -> Result<(), diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

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