use leptos::prelude::*;

#[component]
pub fn ThoughtTable() -> impl IntoView {
    let thoughts = Resource::new(|| (), |_| crate::thought::server::get_all_thoughts());

    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || Suspend::new(async move {
                match thoughts.await {
                    Ok(thoughts) => view! {
                        <table>
                            <thead>
                                <tr>
                                    <th>"Id"</th>
                                    <th>"Created"</th>
                                    <th>"Tracks"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {thoughts
                                    .into_iter()
                                    .map(|thought| view! {
                                        <tr>
                                            <td>{thought.id}</td>
                                            <td>{thought.created_at.to_string()}</td>
                                            <td>
                                                <ul>
                                                    {thought
                                                        .tracks
                                                        .into_iter()
                                                        .map(|track| view! { <li>{track.name}</li> })
                                                        .collect_view()}
                                                </ul>
                                            </td>
                                        </tr>
                                    })
                                    .collect_view()}
                            </tbody>
                        </table>
                    }
                    .into_any(),
                    Err(e) => view! { <p>"Error: " {e.to_string()}</p> }.into_any(),
                }
            })}
        </Suspense>
    }
}

