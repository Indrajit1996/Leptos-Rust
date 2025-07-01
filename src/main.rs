use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_rust::app::App;
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}