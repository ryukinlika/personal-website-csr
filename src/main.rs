mod app;
mod comp;
mod contents;

use app::*;
use leptos::mount;

pub fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(App);
}
