use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatContent {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatChunk {
    pub session_id: String,
    pub user_id: String,
    pub content: ChatContent,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBroadcastChunk {
    pub session_id: String,
    pub user_id: String,
    pub content: ChatContent,
}

impl ChatChunk {
    pub fn to_broadcast_chunk(&self) -> ChatBroadcastChunk {
        ChatBroadcastChunk {
            session_id: self.session_id.clone(),
            user_id: self.user_id.clone(),
            content: self.content.clone(),
        }
    }
}

impl ChatBroadcastChunk {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
