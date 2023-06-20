use lemmy_api_common::{comment::{CommentResponse, CreateComment}, lemmy_db_schema::newtypes::{PostId, CommentId}};

use crate::util;


pub fn create_comment(
    post_id: i32,
    content: String,
    parent_id: Option<i32>,
) -> Result<CommentResponse, reqwest::Error> {
    let params = CreateComment {
        post_id: PostId(post_id),
        content,
        parent_id: parent_id.map(CommentId),
        auth: util::get_auth_token().unwrap(),
        ..Default::default()
    };
    super::post("/comment", &params)
}