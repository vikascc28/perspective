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

use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, get_service};
use axum::Router;
use perspective::client::{TableData, TableInitOptions, UpdateData};
use perspective::server::{Server, SessionHandler};
use tokio::sync::Mutex;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry;

/// A thread-safe wrapper for a [`WebSocket`], which will be accessed from
/// separate _writing_ (via [`WebSocket::send`]) and _reading_
/// (via [`WebSocket::recv`]) async tasks.
#[derive(Clone)]
struct PerspectiveWSConnection(Arc<Mutex<WebSocket>>);

/// The [`SessionHandler`] implementation provides a method for a [`Session`] to
/// send messages to this [`WebSocket`], which may (or ay not) be solicited
/// (e.g. within the async call stack of [`Session::handle_request`]).
impl SessionHandler for PerspectiveWSConnection {
    async fn send_response<'a>(
        &'a self,
        resp: &'a [u8],
    ) -> Result<(), perspective::server::ServerError> {
        let mut socket = self.0.lock().await;
        socket.send(Message::Binary(resp.to_vec())).await?;
        Ok(())
    }
}

/// This handler is responsible for the beginning-to-end lifecycle of a single
/// WebSocket connection to the Axum server. Messages will come in from the
/// [`WebSocket`] in binary form via [`Message::Binary`], where they'll be
/// routed to [`Session::handle_request`]. The server may generate one or more
/// responses, which it will then send back to the [`WebSocket::send`] method
/// via its [`SessionHandler`] impl.
async fn perspective_websocket_handler(
    ws: WebSocketUpgrade,
    State(server): State<Server>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    tracing::info!("{addr} Connected.");
    ws.on_upgrade(move |socket| async move {
        // A new connection is established, create a new `Session` for it.
        let conn = PerspectiveWSConnection(Arc::new(Mutex::new(socket)));

        // Use the [`SessionHandler`] impl to connect messages from the `Server`
        // to `WebSocket::send`. This needs to be set first because calling
        // [`Session::handle_request`] will cause the `Server` to emit messages.
        let session = server.new_session(conn.clone()).await;

        // Loop until there are no more messages, being careful not to hold the
        // `Mutex` while awaiting method calls on `Session`.
        loop {
            let msg = conn.0.lock().await.recv().await;
            if let Some(Ok(Message::Binary(bytes))) = &msg {
                // Pass the message to `Session::handle_request`, which will
                // invoke `SessionHandler::handle_response` with any
                // generated resposnes.
                session.handle_request(bytes).await.expect("Internal error");

                // [`Session::poll`] flushes any asynchronous messages, such as
                // `View::on_update` notifications. It can be called "later",
                // as long as a call is scheduled whenever the [`Server`] is
                // disturbed by a [`Session::handle_request`] call.
                session.poll().await.expect("Internal error");
            } else {
                break;
            };
        }

        tracing::info!("{addr} Disconnected");
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize application logging via [`tracing`].
    registry()
        .with(layer().compact().with_filter(LevelFilter::INFO))
        .init();

    // Create a `Server` which will be shared by this app server and any
    // browsers which connect remotely.
    let server = Server::default();

    // Create a local `Client` connection to `server` for the app server
    // process to use.
    let client = perspective::create_local_client(&server).await?;

    // Load the "Superstore" Arrow example data.
    let feather = {
        let mut file = File::open("../../node_modules/superstore-arrow/superstore.lz4.arrow")?;
        let mut feather = Vec::with_capacity(file.metadata()?.len() as usize);
        file.read_to_end(&mut feather)?;
        feather
    };

    // Use the local `client` to create a Perspective `Table` named "superstore"
    // from the Superstore bytes data. Only browser clients will be accessing
    // this [`perspective::client::Table`] once it is created, so the client handle
    // returned is not needed.
    let _ = client
        .table(
            TableData::Update(UpdateData::Arrow(feather.into())),
            TableInitOptions {
                name: Some("superstore".to_owned()),
                ..TableInitOptions::default()
            },
        )
        .await?;

    // Create an Axum server which routes Web Socket message bytes to
    // `perspective_websocket_handler` on the path `/ws`, and serves a simple
    // HTML client application at the root.
    let app = Router::new()
        .route("/", get_service(ServeFile::new("src/index.html")))
        .route("/ws", get(perspective_websocket_handler))
        .fallback_service(ServeDir::new("../.."))
        .with_state(server)
        .layer(TraceLayer::new_for_http());

    let service = app.into_make_service_with_connect_info::<SocketAddr>();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, service).await?;
    Ok(())
}
