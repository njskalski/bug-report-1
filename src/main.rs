extern crate cursive;
use cursive::*;

extern crate stderrlog;
#[macro_use]
extern crate log;

fn main() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(5)
        .init()
        .unwrap();

    let mut siv = Cursive::new();

    siv.add_global_callback(cursive::event::Event::CtrlChar('p'), |_| { debug!("ctrl-p pressed"); });
    siv.add_global_callback(cursive::event::Event::CtrlChar('P'), |_| { debug!("shift-ctrl-p pressed"); });
    siv.run();
}
