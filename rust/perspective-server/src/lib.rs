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

//! This crate contains the server/engine components of the
//! [Perspective](https://perspective.finos.org) data visualization suite. It is
//! meant to be used in conjunction with the other crates of this project,
//! e.g. `perspective-client` to create client connections to a server.
//!
//! The [`perspective`] crate provides a convenient frontend for Rust
//! developers, including both `perspective-client` and `perspective-server` as
//! well as other convenient integration helpers.
//!
//! # Architecture
//!
//! The basic dataflow of a Perspective applications looks something like this:
//!                                                                             
//! ```text
//!                      : Network or sync boundary
//!                      :
//!  Client 1            :   Session 1                      Server
//! ┏━━━━━━━━━━━━━━━━━━┓ :  ┏━━━━━━━━━━━━━━━━━━┓           ┏━━━━━━━━━━━━━━━━━━┓
//! ┃ receive_response ┃<━━━┃ on_send_response ┃<━┳━━━━━━━━┃ on_send_response ┃
//! ┃ on_send_request  ┃━┳━>┃ handle_request   ┃━━━━━┳━━━━>┃ handle_request   ┃
//! ┗━━━━━━━━━━━━━━━━━━┛ ┗━>┃ poll             ┃━━━━━━━━┳━>┃ poll             ┃
//!                      :  ┃ session_id       ┃  ┃  ┃  ┃  ┃ generate_id      ┃
//!                      :  ┗━━━━━━━━━━━━━━━━━━┛  ┃  ┃  ┃  ┃ cleanup_id       ┃
//!                      :                        ┃  ┃  ┃  ┗━━━━━━━━━━━━━━━━━━┛
//!  Client 2            :   Session 2            ┃  ┃  ┃
//! ┏━━━━━━━━━━━━━━━━━━┓ :  ┏━━━━━━━━━━━━━━━━━━┓  ┃  ┃  ┃  
//! ┃ receive_response ┃<━━━┃ on_send_response ┃<━┛  ┃  ┃                         
//! ┃ on_send_request  ┃━┳━>┃ handle_request   ┃━━━━━┛  ┃                                        
//! ┗━━━━━━━━━━━━━━━━━━┛ ┗━>┃ poll             ┃━━━━━━━━┛
//!                      :  ┃ session_id       ┃                                                 
//!                      :  ┗━━━━━━━━━━━━━━━━━━┛
//! ```
//!
//! # Feature Flags
//!
//! The following feature flags are available to enable in your `Cargo.toml`:
//!
//! - `external-cpp` Set this flag to configure this crate's compile process to
//!   look for Perspective C++ source code in the environment rather than
//!   locally, e.g. for when you build this crate in-place in the Perspective
//!   repo source tree.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_lock::RwLock;
use cxx::UniquePtr;

mod ffi;

type SessionCallback<E> = Arc<
    dyn Fn(&[u8]) -> Pin<Box<dyn Future<Output = Result<(), E>> + 'static + Sync + Send>>
        + 'static
        + Sync
        + Send,
>;

/// An instance of a Perspective server. Each [`Server`] instance is separate,
/// and does not share [`perspective_client::Table`] (or other) data with other
/// [`Server`]s.
pub struct Server<E> {
    server: Arc<UniquePtr<ffi::ProtoApiServer>>,
    callbacks: Arc<RwLock<HashMap<u32, SessionCallback<E>>>>,
}

/// This needs a manual implementation because `derive` adds the bounds
/// `E: Clone`, which is unnecessary.
impl<E> Clone for Server<E> {
    fn clone(&self) -> Self {
        Self {
            server: self.server.clone(),
            callbacks: self.callbacks.clone(),
        }
    }
}

impl<E> Default for Server<E> {
    fn default() -> Self {
        let server = Arc::new(ffi::new_proto_server());
        let callbacks = Arc::default();
        Self { server, callbacks }
    }
}

impl<E> Server<E> {
    /// Create a new [`Session`] for this [`Server`], suitable for exactly one
    /// [`Client`] (not necessarily in this process).
    ///
    /// # Arguments
    ///
    /// - `send_response` A function invoked by the [`Server`] when a response
    ///   message needs to be sent to the [`Client`] (via its respective
    ///   [`Session`]). The response itself should be passed to
    ///   [`Client::handle_response`], which may-or-may-not be in the same
    ///   process or host language.
    pub async fn new_session<F, G>(&self, send_response: F) -> Session<E>
    where
        F: Fn(&[u8]) -> G + 'static + Sync + Send,
        G: Future<Output = Result<(), E>> + 'static + Sync + Send,
    {
        let id = ffi::new_session(&self.server);
        let server = self.clone();
        self.callbacks
            .write()
            .await
            .insert(id, Arc::new(move |resp| Box::pin(send_response(resp))));

        Session { id, server }
    }

    async fn handle_request(&self, client_id: u32, val: &[u8]) -> Result<(), E> {
        for response in ffi::handle_request(&self.server, client_id, val).0 {
            if let Some(f) = self.callbacks.read().await.get(&response.client_id) {
                f(&response.resp).await?
            }
        }

        Ok(())
    }

    async fn poll(&self) -> Result<(), E> {
        for response in ffi::poll(&self.server).0 {
            if let Some(f) = self.callbacks.read().await.get(&response.client_id) {
                f(&response.resp).await?
            }
        }

        Ok(())
    }
}

/// The server-side representation of a connection to a [`Client`]. For each
/// [`Client`] that wants to connect to a [`Server`], a dedicated [`Session`]
/// must be created. The [`Session`] handles routing messages emitted by the
/// [`Server`], as well as owning any resources the [`Client`] may request.
pub struct Session<E> {
    id: u32,
    server: Server<E>,
}

/// This needs a manual implementation because `derive` adds the bounds
/// `E: Clone`, which is unnecessary.
impl<E> Clone for Session<E> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            server: self.server.clone(),
        }
    }
}

impl<E> Session<E> {
    /// Handle an incoming request from the [`Client`]. Calling
    /// [`Session::handle_request`] will result in the `send_response` parameter
    /// which was used to construct this [`Session`] to fire one or more times.
    ///
    /// # Arguments
    ///
    /// - `request` An incoming request message, generated from a
    ///   [`Client::new`]'s `send_request` handler (which may-or-may-not be
    ///   local).
    pub async fn handle_request(&self, request: &[u8]) -> Result<(), E> {
        self.server.handle_request(self.id, request).await
    }

    /// Flush any pending messages which may have resulted from previous
    /// [`Session::handle_request`] calls. Calling [`Session::poll`] may result
    /// in the `send_response` parameter which was used to construct this
    /// [`Session`] to fire.
    pub async fn poll(&self) -> Result<(), E> {
        self.server.poll().await
    }
}
