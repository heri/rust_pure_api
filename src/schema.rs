table! {
    sessions (id) {
        id -> Varchar,
        player_number -> Varchar,
        total_ticks -> Int4,
        game_name -> Varchar,
        table_name -> Varchar,
        player_points -> Nullable<Int4>,
        begin_at -> Nullable<Timestamp>,
        end_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        player_number -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        tier -> Nullable<Int4>,
        address1 -> Varchar,
        city -> Varchar,
        zip -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        id3 -> Nullable<Varchar>,
        is_banned -> Nullable<Int4>,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        gender -> Nullable<Int4>,
        created -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
