// use nwg::NativeUi;
use std::env;

// #[macro_use]
// extern crate native_windows_gui as nwg;

mod gui;
mod iced_gui;
mod load;

use load::load;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let url = if args.len() == 2 {
        &args[1]
    } else {
        "http://example.org/index.html"
    };

    // nwg::init().expect("Failed to init Native Windows GUI");

    // let app = gui::ExternCanvas::build_ui(Default::default()).expect("Failed to build UI");

    // // Make sure to render everything at least once before showing the window to remove weird artifacts.
    // app.canvas.create_context();
    // app.canvas.render();

    // // Here we use the `with_callback` version of dispatch_thread_events
    // // Internally the callback will be executed almost as fast as `loop { callback() }`
    // nwg::dispatch_thread_events_with_callback(move || {
    //     app.canvas.render();
    // });

    load(url);

    use iced::{
        Sandbox, Settings,
    };

    iced_gui::Example::run(Settings {
        antialiasing: false,
        ..Settings::default()
    }).ok();
}
