table! {
    users (Id) {
        Id -> Int4,
        playerNumber -> Int4,
        firstName -> VarChar,
        lastName -> VarChar,
        tier -> Nullable<Int4>,
        address1 -> VarChar,
        city -> VarChar,
        zip -> Nullable<VarChar>,
        country -> VarChar,
        state -> Nullable<VarChar>,
        email -> Nullable<VarChar>,
        id3 -> Nullable<VarChar>,
        isBanned -> Nullable<Int4>,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        gender -> Nullable<VarChar>
    }
}
