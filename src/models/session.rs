use chrono::NaiveDateTime;
use crate::schema::sessions;

#[derive(Serialize, Deserialize)]
pub struct SessionList(pub Vec<Session>);

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Session {
    pub id: String,
    pub playerNumber: String,
    pub total_ticks: i32,
    pub game_name: String,
    pub table_name: String,
    pub player_points: Option<i32>,
    pub begin_at: Option<NaiveDateTime>,
    pub end_at: Option<NaiveDateTime>
}

#[derive(Insertable, AsChangeset)]
#[table_name="sessions"]
pub struct InsertableSession {
    pub playerNumber: String,
    pub total_ticks: i32,
    pub game_name: String,
    pub table_name: String,
    pub player_points: Option<i32>,
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
            .on_conflict(sessions::id)
            .do_update()
            .set(session)
            .execute(&connection)?; 
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionWebhook {
    pub UniqueId: String,
    pub TotalTicks: i32,
    pub GameName: String,
    pub TableName: String,
    pub PlayerPoints: i32,
    pub OpenDate: NaiveDateTime,
    pub ClosedDate: NaiveDateTime
}

impl SessionWebhook {

    pub fn upsert(webhook: &SessionWebhook) -> Result<(), diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        let session = InsertableSession {
            playerNumber: webhook.UniqueId.to_string(),
            total_ticks: webhook.TotalTicks,
            game_name: webhook.GameName.to_string(),
            table_name: webhook.TableName.to_string(),
            player_points: Some(webhook.PlayerPoints),
            begin_at: Some(webhook.OpenDate),
            end_at: Some(webhook.ClosedDate)
        };

        diesel::insert_into(sessions::table)
            .values(&session)
            .on_conflict(sessions::id)
            .do_update()
            .set(&session)
            .execute(&connection)?; 
        Ok(())
    }
}