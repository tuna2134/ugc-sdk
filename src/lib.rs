use pyo3::prelude::*;
use tungstenite::connect;
use url::Url;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass]
struct UgcGateway {
    open: bool
}

#[pymethods]
impl UgcGateway {
    #[new]
    fn new() -> Self {
        UgcGateway {
            open: false
        }
    }

    fn connect(&self) -> PyResult<()> {
        println!("Connecting...");
        let (mut socket, response) =
            connect(Url::parse("wss://ugc.renorari.net/api/v1/gateway").unwrap()).expect("なんらかの原因で接続できない");
        self.on_open();
        Ok(())
    }

    fn on_open(&self) -> PyResult<()> {
        println!("Connected!");
        Ok(())
    }

    fn recv(&self, msg: &str) -> PyResult<()> {
        println!("Received: {}", msg);
        Ok(())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ugc_sdk_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<UgcGateway>()?;
    Ok(())
}
