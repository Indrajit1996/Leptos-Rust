use leptos::*;
use leptos_router::*;

#[component]
pub fn PostDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let content = format!("Post #{} content goes here...", id());

    view! { cx,
        <article>
            <h2>{format!("Post {}", id())}</h2>
            <p>{content}</p>
        </article>
    }
}
