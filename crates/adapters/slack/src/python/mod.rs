use nautilus_core::python::to_pyvalue_err;
use pyo3::{prelude::*, types::PyType};

use crate::{
    client::SlackClient,
    execution::SlackExecutionClient,
}

#[pymethods]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
impl SlackClient {
    #[new]
    #[pyo3(signature = (
        channel_id=None,
        api_key=None,
    ))]
    fn py_new(
        channel_id: Option<String>,
        api_key: Option<String>,
    ) -> PyResult<Self> {
        let channel_id = channel_id.ok_or_else(|| to_pyvalue_err("channel_id is required"))?;
        let api_key = api_key.ok_or_else(|| to_pyvalue_err("api_key is required"))?;
        Ok(Self::new(channel_id, api_key))
    }

    #[pyo3(name = "channel_id")]
    #[must_use]
    pub fn set_channel_id(&mut self, channel_id: String) {
        self.set_channel_id(channel_id);
    }
}

#[pymodule]
pub fn architect(_: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SlackClient>()?;

    Ok(())
}
