use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatContent {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ChatEvent {
    ChatContent(ChatContent),
    NewClient,
    LeftClient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatChunk {
    pub session_id: String,
    pub user_id: String,
    pub event: ChatEvent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBroadcastChunk {
    pub session_id: String,
    pub user_id: String,
    pub event: ChatEvent,
}

impl ChatChunk {
    pub fn to_broadcast_chunk(&self) -> ChatBroadcastChunk {
        ChatBroadcastChunk {
            session_id: self.session_id.clone(),
            user_id: self.user_id.clone(),
            event: self.event.clone(),
        }
    }
}

impl ChatBroadcastChunk {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
