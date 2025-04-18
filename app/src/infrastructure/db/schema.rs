use diesel::table;

table! {
    users (id) {
        id -> Int8,
        external_id -> Uuid,
        username -> Varchar,
        password_hash -> Varchar,  // Alterado para varchar para comportar hashes
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}
