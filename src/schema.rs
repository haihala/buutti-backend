table! {
    books (id) {
        id -> Nullable<Integer>,
        title -> Text,
        author -> Text,
        year -> Integer,
        publisher -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}
