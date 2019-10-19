use chrono::NaiveDateTime;
use crate::schema::sessions;

#[derive(Serialize, Deserialize)]
pub struct SessionList(pub Vec<Session>);

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Session {
    pub Id: String,
    pub playerNumber: String,
    pub totalTicks: i32,
    pub gameName: String,
    pub tableName: String,
    pub playerPoints: Option<i32>,
    pub begin_at: Option<NaiveDateTime>,
    pub end_at: Option<NaiveDateTime>
}

impl SessionList {
    pub fn list() -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::sessions::dsl::*;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        let result = 
            sessions
                .limit(10)
                .load::<Session>(&connection)
                .expect("Error loading sessions");

        SessionList(result)
    }

    pub fn for_user(_player_number: &String) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::sessions::dsl::*;
        use crate::db_connection::establish_connection;
        use diesel::ExpressionMethods;

        let connection = establish_connection();

        let result = 
            sessions
                .filter(playerNumber.eq(_player_number))
                .limit(10)
                .load::<Session>(&connection)
                .expect("Error loading sessions for user");

        SessionList(result)
    }
}

impl Session {

    pub fn upsert(session: &Session) -> Result<(), diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        diesel::insert_into(sessions::table)
            .values(session)
            .on_conflict(sessions::Id)
            .do_update()
            .set(session)
            .execute(&connection)?; 
        Ok(())
    }
}