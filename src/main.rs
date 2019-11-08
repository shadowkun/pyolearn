extern crate cpython;

use cpython::{PyDict, PyResult, Python};

fn main() {
    let gil = Python::acquire_gil();
    hello(gil.python()).unwrap();
}

fn hello(py: Python) -> PyResult<()> {
    let sys = py.import("sys")?;
    let version: String = sys.get(py, "version")?.extract(py)?;

    let locals = PyDict::new(py);
    locals.set_item(py, "os", py.import("os")?)?;
    let user: String = py
        .eval(
            "os.getenv('USER') or os.getenv('USERNAME')",
            None,
            Some(&locals),
        )?
        .extract(py)?;

    println!("Hello {}, I'm Python {}", user, version);
    Ok(())
}
