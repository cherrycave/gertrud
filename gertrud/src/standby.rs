use super::messages::WebSocketMessage;
use dashmap::DashMap;
use tokio::sync::oneshot::Sender;

pub struct Standby {
    bystanders: DashMap<String, Sender<WebSocketMessage>>,
}

impl Standby {
    pub fn new() -> Self {
        Standby {
            bystanders: DashMap::new(),
        }
    }

    pub fn wait_for_response(
        &self,
        message_id: String,
    ) -> tokio::sync::oneshot::Receiver<WebSocketMessage> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.bystanders.insert(message_id, tx);
        rx
    }

    pub fn process_message(&self, message: WebSocketMessage) {
        let message_id = message.message_id.clone();
        if let Some((_, sender)) = self.bystanders.remove(&message_id) {
            sender.send(message).unwrap();
        }
    }
}

impl Default for Standby {
    fn default() -> Self {
        Self::new()
    }
}
