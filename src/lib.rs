#[macro_use]
extern crate cpython;

use cpython::{PyDict, PyResult, Python, ToPyObject};

mod layout;
mod lex;
mod response;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ResponseBuffer {
    pub layout: Vec<(i32, i32, String)>,
    pub body: String,
    pub width: i32,
    pub height: i32,
    pub scroll: i32,
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

        dict
    }
}

fn load_and_compute_layout(_py: Python, url: &str) -> PyResult<ResponseBuffer> {
    let resp = response::request(&url);
    let body = resp.get_body();
    Ok(ResponseBuffer {
        layout: layout::layout(&lex::lex(&body.body_buffer), 800, 600, 0)
            .iter()
            .map(|(x, y, c)| (*x, *y, c.to_string()))
            .collect::<Vec<(i32, i32, String)>>(),
        body: body.body_buffer.to_string(),
        width: 800,
        height: 600,
        scroll: 0,
    })
}

fn recompute_layout(
    _py: Python,
    body: &str,
    width: i32,
    height: i32,
    scroll: i32,
) -> PyResult<ResponseBuffer> {
    Ok(ResponseBuffer {
        layout: layout::layout(&lex::lex(&body), width, height, scroll)
            .iter()
            .map(|(x, y, c)| (*x, *y, c.to_string()))
            .collect::<Vec<(i32, i32, String)>>(),
        body: body.to_string(),
        width: width,
        height: height,
        scroll: scroll,
    })
}

py_module_initializer!(libmyrustlib, initlibmyrustlib, PyInit_myrustlib, |py, m| {
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
            recompute_layout(body: &str, width: i32, height: i32, scroll: i32)
        ),
    )?;
    Ok(())
});
