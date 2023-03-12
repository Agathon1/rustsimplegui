use rstk::*;

fn setup(root: &impl rstk::TkWidget) {
    let hello = rstk::make_label(root);
    hello.text("Hello from Rust/Tk");

    hello.grid().layout();
}

fn main() {
    let root = rstk::start_wish().unwrap();

    setup(&root);

    rstk::mainloop();
}

