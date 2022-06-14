use pyo3::prelude::*;
use tungstenite::{connect, Message, Websocket};
use tungstenite::client::Client
use url::Url;

use protocol::UgcGatewayProtocol

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass]
struct UgcGateway {
    open: bool,
    protocol: Arc<Mutex<UgcGatewayProtocol>>
}

#[pymethods]
impl UgcGateway {

    fn connect(&self) -> PyResult<()> {
        let ws = protocol.lock();
        ws.connect()
        Ok(())
    }

    fn recv(&self) -> PyResult<String> {
        Ok(self.socket.read_message().unwrap())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ugc_sdk_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<UgcGateway>()?;
    Ok(())
}
