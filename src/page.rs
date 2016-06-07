use std::path::Path;

use mount::Mount;
use staticfile::Static;
use iron::Iron;

pub fn serve() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("html/")));

    Iron::new(mount).http("0.0.0.0:3000").unwrap();
}