use pyo3::prelude::*;
use ws::{connect, CloseCode, Sender};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymethods]
impl UgcGateway {
    #[new]
    fn new() -> Self {
        UgcGateway()
    }

    fn connect(&self) -> PyResult<()> {
        println!("Connecting...");
        connect("https://ugc.renorari.net", |out| {
            self.on_open(out);

            move |msg| {
                self.recv(msg);
            }
        }).unwrap();
        Ok(())
    }

    fn on_open(&self, out: Sender) -> PyResult<()> {
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
