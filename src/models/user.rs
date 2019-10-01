use crate::schema::users;

#[derive(Serialize, Deserialize)]
pub struct UserList(pub Vec<User>);

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub playerNumber: i32,
    pub firstName: String,
    pub lastName: String,
    pub tier: Option<i32>,
    pub address1: String,
    pub city: String,
    pub zip: Option<String>,
    pub country: String,
    pub state: Option<String>,
    pub email: Option<String>,
    pub id3: Option<String>,
    pub isBanned: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub gender: Option<String>
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct NewUser {
    pub lastName: Option<String>,
    pub address1: Option<String>,
    pub id3: Option<String>
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
}

impl NewUser {
    pub fn create(&self) -> Result<User, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();
        diesel::insert_into(users::table)
            .values(self)
            .get_result(&connection)
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

    pub fn update(id: &i32, new_user: &NewUser) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::users::dsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        diesel::update(dsl::users.find(id))
            .set(new_user)
            .execute(&connection)?;
        Ok(())
    }
}