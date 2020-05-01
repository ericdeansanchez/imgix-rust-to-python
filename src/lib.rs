use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs;
use std::fmt;
use std::path::PathBuf;

#[pyclass(module = "imgix")]
#[derive(Debug)]
struct Url {
    domain: String,
    path: String,
    scheme: String,
}

#[pymethods]
impl Url {
    #[new]
    fn new(domain: String) -> Self {
        Url {
            domain, 
            path: String::new(),
            scheme: String::new(),
        }
    }

    fn path(&self, path: String) -> Self {
        Url {
            domain: self.domain.clone(),
            path,
            scheme: self.scheme.clone(),
        }
    }

    fn scheme(&self, scheme: String) -> Self {
        Url {
            domain: self.domain.clone(),
            path: self.path.clone(),
            scheme,
        }
    }

    fn get_path(&self) -> String {
        self.path.clone()
    }

    fn __repr__(&self) {
        println!("{:#?}", self);
    }
}

#[pymodule]
fn imgix(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Url>()?;

    Ok(())
}
