table! {
    users (Id) {
        Id -> Int4,
        playerNumber -> Varchar,
        firstName -> Varchar,
        lastName -> Varchar,
        tier -> Nullable<Int4>,
        address1 -> Varchar,
        city -> Varchar,
        zip -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        id3 -> Nullable<Varchar>,
        isBanned -> Nullable<Int4>,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        gender -> Nullable<Int4>,
    }
}