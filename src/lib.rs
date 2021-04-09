#[macro_use]
extern crate cpython;

use cpython::{FromPyObject, PyDict, PyResult, Python, ToPyObject};

mod layout;
mod lex;
mod response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Font {
    pub linespace: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseBuffer {
    pub layout: Vec<(f32, f32, String)>,
    pub body: String,
    pub width: f32,
    pub height: f32,
    pub scroll: f32,
    pub font: Font,
    pub measure: f32,
}

impl ToPyObject for ResponseBuffer {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "layout", self.layout.clone()).unwrap();
        dict.set_item(py, "body", self.body.clone()).unwrap();
        dict.set_item(py, "width", self.width).unwrap();
        dict.set_item(py, "height", self.height).unwrap();
        dict.set_item(py, "scroll", self.scroll).unwrap();
        dict.set_item(py, "font", self.font.clone()).unwrap();
        dict.set_item(py, "measure", self.measure).unwrap();

        dict
    }
}

impl ToPyObject for Font {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "linespace", self.linespace).unwrap();

        dict
    }
}

impl<'source> FromPyObject<'source> for Font {
    fn extract(
        _: cpython::Python<'_>,
        _: &'source cpython::PyObject,
    ) -> std::result::Result<Self, cpython::PyErr> {
        todo!()
    }
}

fn load_and_compute_layout(_py: Python, url: &str) -> PyResult<ResponseBuffer> {
    let resp = response::request(&url);
    let body = resp.get_body();
    Ok(ResponseBuffer {
        layout: layout::layout(&lex::lex(&body.body_buffer), 800f32, 600f32, 0f32)
            .iter()
            .map(|(x, y, c)| (*x, *y, c.to_string()))
            .collect::<Vec<(f32, f32, String)>>(),
        body: body.body_buffer.to_string(),
        width: 800f32,
        height: 600f32,
        scroll: 0f32,
        font: Font { linespace: 1f32 },
        measure: 16f32,
    })
}

fn recompute_layout(
    _py: Python,
    body: &str,
    width: f32,
    height: f32,
    scroll: f32,
    font: Font,
    measure: f32,
) -> PyResult<ResponseBuffer> {
    Ok(ResponseBuffer {
        layout: layout::layout(&lex::lex(&body), width, height, scroll)
            .iter()
            .map(|(x, y, c)| (*x, *y, c.to_string()))
            .collect::<Vec<(f32, f32, String)>>(),
        body: body.to_string(),
        width: width,
        height: height,
        scroll: scroll,
        font: font,
        measure: measure,
    })
}

py_module_initializer!(
    layout_engine,
    initlayout_engine,
    PyInit_layout_engine,
    |py, m| {
        m.add(py, "__doc__", "This module is implemented in Rust")?;
        m.add(
            py,
            "load_and_compute_layout",
            py_fn!(py, load_and_compute_layout(val: &str)),
        )?;

        m.add(
            py,
            "recompute_layout",
            py_fn!(
                py,
                recompute_layout(
                    body: &str,
                    width: f32,
                    height: f32,
                    scroll: f32,
                    font: Font,
                    linespace: f32
                )
            ),
        )?;
        Ok(())
    }
);
