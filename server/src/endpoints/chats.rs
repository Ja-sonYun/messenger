use crate::models::chat::ChatChunk;
use actix_web::dev::HttpServiceFactory;
use actix_web::rt::time::interval;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_lab::{
    sse::{self, Sse},
    util::InfallibleStream,
};
use futures_util::future;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub fn chat_endpoints() -> impl HttpServiceFactory {
    web::scope("/chats")
        .service(event_stream)
        .service(broadcast)
}

pub struct ChatBroadcaster {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    connections: HashMap<String, Vec<mpsc::Sender<sse::Event>>>,
}

impl ChatBroadcaster {
    pub fn create() -> Arc<Self> {
        let this = Arc::new(ChatBroadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        ChatBroadcaster::spawn_ping(Arc::clone(&this));

        this
    }

    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    async fn remove_stale_clients(&self) {
        let connections = self.inner.lock().connections.clone();

        let mut ok_clients = HashMap::new();

        for (channel_id, clients) in connections.iter() {
            for client in clients {
                if client
                    .send(sse::Event::Comment("ping".into()))
                    .await
                    .is_ok()
                {
                    ok_clients
                        .entry(channel_id.clone())
                        .or_insert_with(Vec::new)
                        .push(client.clone());
                }
            }
        }

        for (channel_id, clients) in ok_clients.iter() {
            if clients.len() != connections.get(channel_id).unwrap().len() {
                println!("Removed stale clients for channel {}", channel_id);
            }
        }

        self.inner.lock().connections = ok_clients;
    }

    pub async fn new_client(
        &self,
        channel_id: String,
        user_id: String,
    ) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        let (tx, rx) = mpsc::channel(1000);

        tx.send(sse::Data::new("connected").into()).await.unwrap();

        self.inner
            .lock()
            .connections
            .entry(channel_id.clone())
            .or_insert_with(Vec::new)
            .push(tx);

        self.broadcast(
            channel_id.clone(),
            &ChatChunk {
                session_id: "server".to_string(),
                user_id: user_id.clone(),
                event: crate::models::chat::ChatEvent::NewClient,
            },
        )
        .await;

        Sse::from_infallible_receiver(rx)
    }

    pub async fn broadcast(&self, channel_id: String, chat_chunk: &ChatChunk) {
        let clients = self
            .inner
            .lock()
            .connections
            .get(&channel_id)
            .unwrap_or(&Vec::new())
            .clone();

        let send_futures = clients.iter().map(|client| {
            client.send(sse::Data::new(chat_chunk.to_broadcast_chunk().to_string()).into())
        });

        let _ = future::join_all(send_futures).await;
    }
}

#[get("/{channel_id}/{user_id}")]
async fn event_stream(
    path: web::Path<(String, String)>,
    broadcaster: web::Data<ChatBroadcaster>,
) -> impl Responder {
    let (channel_id, user_id) = path.into_inner();
    broadcaster.new_client(channel_id, user_id).await
}

#[post("/broadcast/{channel_id}")]
async fn broadcast(
    broadcaster: web::Data<ChatBroadcaster>,
    path: web::Path<String>,
    chunk: web::Json<ChatChunk>,
) -> impl Responder {
    let channel_id = path.into_inner();
    broadcaster.broadcast(channel_id, &chunk).await;

    HttpResponse::Ok().body(chunk.session_id.clone())
}
