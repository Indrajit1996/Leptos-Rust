use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // let count = RwSignal::new(0);
    // let on_click = move |_| *count.write() += 1;

    let data_points: Vec<(usize, String)> = (1..=100)
        .map(|i| (i, format!("Data Point #{}", i)))
        .collect();

    view! {
        <div style="
            display: flex;
            flex-direction: column;
            align-items: center;
            min-height: 100vh;
            padding: 20px;
            font-family: Arial, sans-serif;
        ">
            <h1 style="margin-bottom: 20px;">"Leptos Fullstack Rust Application"</h1>
            
            // <button 
            //     on:click=on_click
            //     style="
            //         padding: 10px 20px;
            //         font-size: 16px;
            //         background-color: #007acc;
            //         color: white;
            //         border: none;
            //         border-radius: 5px;
            //         cursor: pointer;
            //         margin-bottom: 20px;
            //     "
            // >
            //     "Click Me: " {count}
            // </button>

            <div style="
                width: 80%;
                max-width: 600px;
                height: 400px;
                border: 2px solid #ccc;
                border-radius: 10px;
                overflow-y: auto;
                padding: 15px;
                background-color: #f9f9f9;
                box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            ">
                <h3 style="margin-top: 0; text-align: center; color: #333;">
                    "Data Points (100 items)"
                </h3>
                
                <div style="display: flex; flex-direction: column; gap: 8px;">
                    {data_points.into_iter().map(|(index, text)| {
                        view! {
                            <div style="
                                padding: 10px;
                                background-color: white;
                                border-radius: 5px;
                                border-left: 4px solid #007acc;
                                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
                            ">
                                <span style="font-weight: bold; color: #007acc;">
                                    {index}". "
                                </span>
                                {text}
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}