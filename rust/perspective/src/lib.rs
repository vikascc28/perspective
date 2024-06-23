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

use std::sync::{Arc, OnceLock};

use perspective_client::*;
use perspective_server::*;
pub use {perspective_client as client, perspective_server as server};

#[derive(Clone, Default)]
struct LocalConnection {
    client: Arc<OnceLock<Client>>,
    session: Arc<OnceLock<Session>>,
    server: Server,
}

impl SessionHandler for LocalConnection {
    async fn send_response<'a>(&'a self, msg: &'a [u8]) -> Result<(), ServerError> {
        self.get_client().handle_response(msg).await?;
        Ok(())
    }
}

impl ClientHandler for LocalConnection {
    async fn send_request<'a>(
        &'a self,
        msg: &'a [u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let session = self.get_session().await;
        session.handle_request(msg).await?;
        session.poll().await?;
        Ok(())
    }
}

impl LocalConnection {
    fn get_client(&self) -> &Client {
        self.client.get_or_init(|| Client::new(self.clone()))
    }

    async fn get_session(&self) -> &Session {
        if self.session.get().is_none() {
            let session = self.server.new_session(self.clone()).await;
            self.session.get_or_init(|| session)
        } else {
            self.session.get().unwrap()
        }
    }
}

pub async fn create_local_client(server: &Server) -> ClientResult<Client> {
    let connection = LocalConnection {
        server: server.clone(),
        client: Arc::default(),
        session: Arc::default(), // ..LocalConnection::default()
    };

    Ok(connection.get_client().clone())
}
