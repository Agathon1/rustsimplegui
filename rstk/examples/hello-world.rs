use rstk::*;

fn main() {
    if let Ok(root) = rstk::start_wish() {

        let hello = rstk::make_label(&root);
        hello.text("Hello from Rust/Tk");

        hello.grid().row(0).column(0).layout();

        rstk::mainloop();
    } else {
        println!("Failed to start wish program");
    }
}
