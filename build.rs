extern crate "pkg-config" as pkg_config;

use pkg_config::Config;

fn main() {
    Config::new().atleast_version("2.36").find("gobject-2.0").unwrap();
}
