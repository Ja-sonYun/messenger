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
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub fn chat_endpoints() -> impl HttpServiceFactory {
    web::scope("/chats")
        .service(event_stream)
        .service(broadcast)
}

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: Vec<mpsc::Sender<sse::Event>>,
}

impl Broadcaster {
    pub fn create() -> Arc<Self> {
        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        Broadcaster::spawn_ping(Arc::clone(&this));

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
        let clients = self.inner.lock().clients.clone();
        println!("Checking {} clients", clients.len());

        let mut ok_clients = Vec::new();

        for client in clients {
            if client
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone());
            }
        }
        println!("Active {} clients", ok_clients.len());

        self.inner.lock().clients = ok_clients;
    }

    pub async fn new_client(&self) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        let (tx, rx) = mpsc::channel(1000);

        tx.send(sse::Data::new("connected").into()).await.unwrap();

        self.inner.lock().clients.push(tx);

        Sse::from_infallible_receiver(rx)
    }

    pub async fn broadcast(&self, chat_chunk: &ChatChunk) {
        let clients = self.inner.lock().clients.clone();

        let send_futures = clients.iter().map(|client| {
            client.send(sse::Data::new(chat_chunk.to_broadcast_chunk().to_string()).into())
        });

        println!("Broadcasting to {} clients", clients.len());
        let _ = future::join_all(send_futures).await;
        println!("Broadcasting done");
    }
}

#[get("/")]
async fn event_stream(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    broadcaster.new_client().await
}

#[post("/broadcast")]
async fn broadcast(
    broadcaster: web::Data<Broadcaster>,
    // path: Path<(String,)>,
    chunk: web::Json<ChatChunk>,
) -> impl Responder {
    broadcaster.broadcast(&chunk).await;
    println!("{:?}", chunk);
    HttpResponse::Ok().body(chunk.session_id.clone())
}
