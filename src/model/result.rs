use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub rsp_desc: String,
    pub rsp_code: String,
}
