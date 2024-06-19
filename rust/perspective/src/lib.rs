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

use std::fmt::Debug;
use std::sync::{Arc, OnceLock};

use perspective_client::*;
use perspective_server::*;
pub use {perspective_client as client, perspective_server as server};

pub async fn create_local_client<E: Debug + 'static>(server: &Server<E>) -> ClientResult<Client> {
    let server = server.clone();
    let client: Arc<OnceLock<Client>> = Arc::default();
    let session: Session<E> = server
        .new_session({
            let client = client.clone();
            move |resp| {
                tracing::error!("{}", client.get().is_some());
                tracing::error!("resp");
                let resp = resp.to_vec();
                let client = client.clone();
                async move {
                    client.get().unwrap().handle_response(&resp).await.unwrap();
                    Ok(())
                }
            }
        })
        .await;

    client.get_or_init(|| {
        Client::new({
            move |_client, req| {
                tracing::error!("req");
                let session = session.clone();
                let req = req.to_vec();
                async move {
                    session.handle_request(&req).await.unwrap();
                    session.poll().await.unwrap();
                    Ok(())
                }
            }
        })
    });

    tracing::error!("{}", client.get().is_some());

    client.get().unwrap().init().await?;
    Ok(client.get().unwrap().clone())
}
