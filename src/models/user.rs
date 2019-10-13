use crate::schema::users;
use chrono::{NaiveDateTime, Local};
use diesel::ExpressionMethods;

#[derive(Serialize, Deserialize)]
pub struct UserList(pub Vec<User>);

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub player_number: String,
    pub created: NaiveDateTime
}


impl NewUser {

    pub fn create(&self) -> Result<User, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();
        
        let new_user = NewUser {
            // first_name: "test_string" would not work . That would pass a reference but this expects insead a std::string::String
            first_name: self.last_name.to_string(), 
            last_name: self.last_name.to_string(), 
            player_number: self.player_number.to_string(), 
            created: Local::now().naive_local()
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&connection)
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Insertable)]
pub struct User {
    pub id: i32,
    pub player_number: String,
    pub first_name: String,
    pub last_name: String,
    pub tier: Option<i32>,
    pub address1: String,
    pub city: String,
    pub zip: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub id3: Option<String>,
    pub is_banned: Option<i32>,
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
    
    // TO-Do : this should output HTML
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

        UserList(result)
    }
}

impl User {
    pub fn find(id: &i32) -> Result<User, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        users::table.find(id).first(&connection)
    }

    pub fn destroy(id: &i32) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::users::dsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        diesel::delete(dsl::users.find(id)).execute(&connection)?;
        Ok(())
    }

    pub fn update(id: &i32, user: &User) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::users::dsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        diesel::update(dsl::users.find(id))
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
            .on_conflict(users::id)
            .do_update()
            .set(user)
            .execute(&connection)?; 
        Ok(())
    }
}