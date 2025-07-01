use leptos::*;
use leptos_router::*;
// use crate::components::home::Home;
// use crate::components::new_post::NewPost;
// use crate::components::post_detail::PostDetail;
use crate::components::{home::Home, post_detail::PostDetail};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <link rel="stylesheet" href="/css/style.css" />
                <h1>"Leptos CMS"</h1>
                <nav>
                    <A href="/">"Home"</A>
                    " | "
                    <A href="/new">"New Post"</A>
                </nav>
                <Routes>
                    <Route path="/" view=|| view! { <Home /> } />
                    <Route path="/post/:id" view=|| view! { <PostDetail /> } />
                </Routes>
            </main>
        </Router>
    }
}
