mod database {

    pub const POST_STATUS_TYPE_PUBLISH: &'static str = "publish";
    pub const POST_STATUS_TYPE_INHERIT: &'static str = "inherit";
    pub const POST_STATUS_TYPE_PRIVATE: &'static str = "private";
    pub const POST_STATUS_TYPE_TRASH: &'static str = "trash";
    pub const POST_STATUS_TYPE_AUTO_DRAFT: &'static str = "auto-draft";

    pub const POST_TYPE_PAGE: &'static str = "page";
    pub const POST_TYPE_POST: &'static str = "post";
    pub const POST_TYPE_REVISION: &'static str = "revision";

    pub const COMMENT_STATUS_CLOSED: &'static str = "closed";
    pub const COMMENT_STATUS_OPEN: &'static str = "open";

    pub const PING_STATUS_CLOSED: &'static str = "closed";
    pub const PING_STATUS_OPEN: &'static str = "open";
}