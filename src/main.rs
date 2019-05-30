#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

fn main() {
    let path = std::env::args().nth(1).expect("Missing configration file path");
    let app = app::bootstrap::Bootstrap::new(path);
    app.launch();
}