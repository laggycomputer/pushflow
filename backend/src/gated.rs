use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct SessionUser {
    pub user_id: String,
    pub avatar: Option<String>,
}
