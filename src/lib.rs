use pyo3::prelude::*;
use ws::{connect};

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
        connect("wss://ugc.renorari.net", |out| {
            self.on_open();

            move |msg| {
                self.recv(msg);
            }
        });
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
