extern crate "pkg-config" as pkg_config;

fn main() {
    pkg_config::find_library("gobject-2.0").unwrap();
}
