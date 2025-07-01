// use leptos::*;

// #[component]
// pub fn NewPost() -> impl IntoView {
//     let (title, set_title) = create_signal(String::new());
//     let (content, set_content) = create_signal(String::new());

//     let submit = move |_| {
//         log::info!("Submitted: {}\n{}", title.get(), content.get());
//         set_title.set(String::new());
//         set_content.set(String::new());
//     };

//     view! { cx,
//         <form on:submit=move |ev| { ev.prevent_default(); submit(ev); }>
//             <input placeholder="Title" bind:value=title />
//             <textarea placeholder="Content" bind:value=content></textarea>
//             <button type="submit">"Save"</button>
//         </form>
//     }
// }
