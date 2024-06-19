// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ ██████ ██████ ██████       █      █      █      █      █ █▄  ▀███ █       ┃
// ┃ ▄▄▄▄▄█ █▄▄▄▄▄ ▄▄▄▄▄█  ▀▀▀▀▀█▀▀▀▀▀ █ ▀▀▀▀▀█ ████████▌▐███ ███▄  ▀█ █ ▀▀▀▀▀ ┃
// ┃ █▀▀▀▀▀ █▀▀▀▀▀ █▀██▀▀ ▄▄▄▄▄ █ ▄▄▄▄▄█ ▄▄▄▄▄█ ████████▌▐███ █████▄   █ ▄▄▄▄▄ ┃
// ┃ █      ██████ █  ▀█▄       █ ██████      █      ███▌▐███ ███████▄ █       ┃
// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
// ┃ Copyright (c) 2017, the Perspective Authors.                              ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃ This file is part of the Perspective library, distributed under the terms ┃
// ┃ of the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0). ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::fs::{self, File};
use std::io::Read;
use std::net::SocketAddr;

use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use futures::future::select_all;
use futures::{select, FutureExt};
use perspective::client::{ClientError, TableData, TableInitOptions, UpdateData};
use perspective::server::Server;
use tower_http::trace::TraceLayer;

fn init_tracing() {
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::fmt::layer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::registry;
    registry()
        .with(layer().pretty().with_filter(LevelFilter::INFO))
        .init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let server = Server::default();
    let client = perspective::create_local_client(&server).await?;

    const FILENAME: &str = "../../node_modules/superstore-arrow/superstore.lz4.arrow";
    let mut f = File::open(FILENAME).expect("no file found");
    let metadata = fs::metadata(FILENAME).expect("unable to read metadata");
    let mut feather = Vec::with_capacity(metadata.len() as usize);
    f.read_to_end(&mut feather).expect("buffer overflow");

    let _table = client
        .table(
            TableData::Update(UpdateData::Arrow(feather.into())),
            TableInitOptions {
                name: Some("superstore".to_owned()),
                ..TableInitOptions::default()
            },
        )
        .await?;

    let app = Router::new()
        .route("/ws", get(websocket_handshake))
        .with_state(server)
        .layer(TraceLayer::new_for_http());

    let service = app.into_make_service_with_connect_info::<SocketAddr>();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, service).await?;
    Ok(())
}

async fn websocket_handshake(
    ws: WebSocketUpgrade,
    State(server): State<Server<axum::Error>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    tracing::info!("{addr} Connected.");
    ws.on_upgrade(move |socket| websocket_session(server, socket, addr))
}

async fn websocket_session(server: Server<axum::Error>, mut socket: WebSocket, addr: SocketAddr) {
    let (send, mut recv) = tokio::sync::mpsc::unbounded_channel::<Result<Message, axum::Error>>();

    let session = server
        .new_session(move |resp| {
            let send = send.clone();
            let resp = resp.to_vec();
            async move {
                send.send(Ok(Message::Binary(resp)));
                Ok(())
                //     tracing::error!("{addr} unexpected error {e:?}");
                //     return;
                // }

                // Ok
            }
        })
        .await;

    while let (Some(msg), ..) = select_all([socket.recv().boxed(), recv.recv().boxed()]).await {
        if let Ok(Message::Binary(x)) = msg {
            session.handle_request(&x).await;
            session.poll().await;
        } else {
            tracing::error!("{addr} Unexpected msg {msg:?}");
        }
    }

    tracing::info!("{addr} disconnected");
}
