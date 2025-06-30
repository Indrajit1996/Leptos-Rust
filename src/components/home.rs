use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let posts = vec![
        (1, "First Post"),
        (2, "Second Post"),
    ];

    view! { cx,
        <ul>
            {posts.into_iter().map(|(id, title)| view! { cx,
                <li>
                    <A href=format!("/post/{}", id)>{title}</A>
                </li>
            }).collect_view(cx)}
        </ul>
    }
}
