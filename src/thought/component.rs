use crate::thought::{
    dto::{ThoughtDto, TrackDto},
    server::CreateThought,
};
use leptos::{ev::SubmitEvent, html::S, prelude::*};

#[component]
pub fn ThoughtTable(action: ServerAction<CreateThought>) -> impl IntoView {
    let thoughts = Resource::new(
        move || action.version().get(),
        |_| crate::thought::server::get_all_thoughts(),
    );

    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || Suspend::new(async move {
                match thoughts.await {
                    Ok(thoughts) => view! {
                        <table class="flex-4/5 bg-blue-600 ">
                            <thead>
                                <tr>
                                    <th>"Artist"</th>
                                    <th>"Track"</th>
                                    <th>"Context"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {thoughts
                                    .into_iter()
                                    .map(|thought| view! {
                                        <tr>
                                            <td class ="flex-16 bg-indigo-500 overflow-auto">{thought.track.track_artist}</td>
                                            <td class="flex-16 bg-yellow-400 overflow-visible">{thought.track.track_name}</td>
                                            <td class="flex-16 bg-yellow-400 overflow-visible">{thought.context}</td>
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
// ...rest unchanged
/// Form + table sharing one action, so the table reloads after each insert.
#[component]
pub fn Songs() -> impl IntoView {
    let create = ServerAction::<CreateThought>::new();

    view! {
        <CreateSong action=create />
        <ThoughtTable action=create />
    }
}

#[component]
pub fn CreateSong(action: ServerAction<CreateThought>) -> impl IntoView {
    let artist = RwSignal::new(String::new());
    let name = RwSignal::new(String::new());
    let context = RwSignal::new(String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        action.dispatch(CreateThought {
            input: ThoughtDto {
                track: TrackDto {
                    track_artist: artist.get(),
                    track_name: name.get(),
                },
                context: context.get(),
            },
        });
        artist.set(String::new());
        context.set(String::new());
        name.set(String::new());
    };

    view! {
        <form on:submit=on_submit class= "bg-amber-600">
            <input  type="text" placeholder="Artist" required bind:value=artist />
            <input type="text" placeholder="Song" required bind:value=name />
            <input type="text" placeholder="Context" required bind:value=context />
            <button type="submit" disabled=move || action.pending().get()>"Add"</button>
        </form>
        {move || {
            action
                .value()
                .get()
                .and_then(|res| res.err())
                .map(|e| view! { <p>"Error: " {e.to_string()}</p> })
        }}
    }
}
