mod database {
    use chrono::NaiveDateTime;
    use database::schema::lv_links;
    use database::schema::lv_link_groups;
    use database::schema::wp_commentmeta;
    use database::schema::wp_comments;
    use database::schema::wp_links;
    use database::schema::wp_options;
    use database::schema::wp_postmeta;
    use database::schema::wp_posts;
    use database::schema::wp_termmeta;
    use database::schema::wp_terms;
    use database::schema::wp_term_relationships;
    use database::schema::wp_term_taxonomy;
    use database::schema::wp_usermeta;
    use database::schema::wp_users;

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "lv_links"]
    pub struct LvLinks {
        pub id: i32,
        pub created_at: NaiveDateTime,
        pub created_by: String,
        pub updated_by: String,
        pub updated_at: NaiveDateTime,
        pub is_deleted: bool,
        pub deleted_at: NaiveDateTime,
        pub url: String,
        pub title: String,
        pub image: String,
        pub description: String,
        pub owner: String,
        pub rating: i32,
        pub link_group_id: i32,
        pub visible: bool,
        pub highlight: bool,
        pub sort: i32,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "lv_link_groups"]
    pub struct LvLinkGroups {
        pub id: i32,
        pub created_at: NaiveDateTime,
        pub created_by: String,
        pub updated_by: String,
        pub updated_at: NaiveDateTime,
        pub is_deleted: bool,
        pub deleted_at: NaiveDateTime,
        pub title: String,
        pub description: String,
        pub sort: i32,
    }


    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_commentmeta"]
    pub struct WpCommentMeta {
        pub meta_id: i64,
        pub comment_id: i64,
        pub meta_key: String,
        pub meta_value: String,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_comments"]
    pub struct WpComments {
        #[column_name = "comment_ID"]
        pub comment_id: i64,
        #[column_name = "comment_post_ID"]
        pub comment_post_id: i64,
        pub comment_author: String,
        pub comment_author_email: String,
        pub comment_author_url: String,
        #[column_name = "comment_author_IP"]
        pub comment_author_ip: String,
        pub comment_date: NaiveDateTime,
        pub comment_date_gmt: NaiveDateTime,
        pub comment_content: String,
        pub comment_karma: i32,
        pub comment_approved: String,
        pub comment_agent: String,
        pub comment_type: String,
        pub comment_parent: i64,
        pub user_id: i64,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_links"]
    pub struct WpLinks {
        pub link_id: i64,
        pub link_url: String,
        pub link_name: String,
        pub link_image: String,
        pub link_target: String,
        pub link_description: String,
        pub link_visible: String,
        pub link_owner: i64,
        pub link_rating: i32,
        pub link_updated: NaiveDateTime,
        pub link_rel: String,
        pub link_notes: String,
        pub link_rss: String,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_options"]
    pub struct WpOptions {
        pub option_id: i64,
        pub option_name: String,
        pub option_value: String,
        pub autoload: String,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_postmeta"]
    pub struct WpPostMeta {
        pub meta_id: i64,
        pub post_id: i64,
        pub meta_key: String,
        pub meta_value: String,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_posts"]
    pub struct WpPosts {
        #[column_name = "ID"]
        pub id: i64,
        pub post_author: i64,
        pub post_date: NaiveDateTime,
        pub post_date_gmt: NaiveDateTime,
        pub post_content: String,
        pub post_title: String,
        pub post_excerpt: String,
        pub post_status: String,
        pub comment_status: String,
        pub ping_status: String,
        pub post_password: String,
        pub post_name: String,
        pub to_ping: String,
        pub pinged: String,
        pub post_modified: NaiveDateTime,
        pub post_modified_gmt: NaiveDateTime,
        pub post_content_filtered: String,
        pub post_parent: i64,
        pub guid: String,
        pub menu_order: i32,
        pub post_type: String,
        pub post_mime_type: String,
        pub comment_count: i64,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_termmeta"]
    pub struct WpTermMeta {
        pub meta_id: i64,
        pub term_id: i64,
        pub meta_key: String,
        pub meta_value: String,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_terms"]
    pub struct WpTerms {
        pub term_id: i64,
        pub name: String,
        pub slug: String,
        pub term_group: i64,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_term_relationships"]
    pub struct WpTermRelationships {
        pub object_id: i64,
        pub term_taxonomy_id: i64,
        pub term_order: i32,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_term_taxonomy"]
    pub struct WpTermTaxonomy {
        pub term_taxonomy_id: i64,
        pub term_id: i64,
        pub taxonomy: String,
        pub description: String,
        pub parent: i64,
        pub count: i64,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_usermeta"]
    pub struct WpUserMeta {
        pub umeta_id: i64,
        pub user_id: i64,
        pub meta_key: String,
        pub meta_value: String,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "wp_users"]
    pub struct WpUsers {
        #[column_name = "ID"]
        pub id: i64,
        pub user_login: String,
        pub user_pass: String,
        pub user_nicename: String,
        pub user_email: String,
        pub user_url: String,
        pub user_registered: NaiveDateTime,
        pub user_activation_key: String,
        pub user_status: i32,
        pub display_name: String,
    }
}