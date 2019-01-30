table! {
    user_tbl (email) {
        title -> Nullable<Text>,
        email -> Text,
        password -> Nullable<Text>,
        roles -> Nullable<Text>,
        createdAt -> Nullable<Timestamptz>,
        updatedAt -> Nullable<Timestamptz>,
    }
}
