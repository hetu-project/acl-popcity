use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    //pub req_id: String,
    pub code: u16,
    pub result: T,
}
