use chrono::NaiveDateTime;
use crate::schema::sessions;

#[derive(Serialize, Deserialize)]
pub struct SessionList(pub Vec<Session>);

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct Session {
    pub id: String,
    pub player_number: String,
    pub total_ticks: i32,
    pub game_name: String,
    pub table_name: String,
    pub player_points: Option<i32>,
    pub begin_at: Option<NaiveDateTime>,
    pub end_at: Option<NaiveDateTime>
}

