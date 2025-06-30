use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use my_app::app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <App/> });
}
