table! {
    base_news (id) {
        id -> Int4,
        tag -> Text,
        title -> Text,
        n_image_urls -> Nullable<Int4>,
        image_urls -> Array<Text>,
        n_likes -> Nullable<Int4>,
        n_comments -> Nullable<Int4>,
        src_website -> Nullable<Text>,
        craw_time -> Timestamp,
        create_time -> Timestamp,
    }
}
