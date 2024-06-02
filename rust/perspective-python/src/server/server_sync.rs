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

use std::sync::Arc;

use perspective_client::clone;
use perspective_server::{Server, Session};
use pollster::FutureExt;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyFunction};

#[pyclass]
#[derive(Clone)]
pub struct PySyncSession {
    session: Arc<Session<PyErr>>,
}

#[pyclass]
#[derive(Clone, Default)]
pub struct PySyncServer {
    pub server: Server<PyErr>,
}

#[pymethods]
impl PySyncServer {
    #[new]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_session(&self, _py: Python, response_cb: Py<PyFunction>) -> PySyncSession {
        let session = self
            .server
            .new_session(move |response| {
                clone!(response_cb);
                Python::with_gil(move |py| {
                    response_cb.call1(py, (PyBytes::new_bound(py, response),))
                })?;
                Ok(())
            })
            .block_on();

        PySyncSession {
            session: Arc::new(session),
        }
    }
}

#[allow(non_local_definitions)]
#[pymethods]
impl PySyncSession {
    pub fn handle_request(&self, _py: Python<'_>, data: Vec<u8>) -> PyResult<()> {
        // TODO: Make this return a boolean for "should_poll" to determine whether we
        // immediately schedule a poll after this request.
        self.session.handle_request(&data).block_on()
    }

    pub fn poll(&self, _py: Python<'_>) -> PyResult<()> {
        self.session.poll().block_on()
    }
}
