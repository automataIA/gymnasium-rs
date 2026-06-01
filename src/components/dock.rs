//! Dock inferiore di navigazione. Nascosto durante Workout e Trenino.
use leptos::prelude::*;

use crate::components::icons;
use crate::state::{AppState, Screen};

#[component]
pub fn Dock() -> impl IntoView {
    let state = expect_context::<AppState>();

    let style = move || {
        if matches!(state.screen.get(), Screen::Workout | Screen::Trenino) {
            "display:none"
        } else {
            ""
        }
    };
    let cls = move |s: Screen| {
        let active = state.screen.get() == s;
        format!("dock-btn{}", if active { " active" } else { "" })
    };

    view! {
        <nav
            id="dock"
            class="sticky bottom-0 z-30 bg-base-100/90 backdrop-blur border-t border-base-200"
            style=style
        >
            <div class="grid grid-cols-4">
                <button class=move || cls(Screen::Home) on:click=move |_| state.go(Screen::Home)>
                    {icons::dock_home()}
                    <span>"Oggi"</span>
                </button>
                <button
                    class=move || cls(Screen::History)
                    on:click=move |_| state.go(Screen::History)
                >
                    {icons::dock_history()}
                    <span>"Storico"</span>
                </button>
                <button
                    class=move || cls(Screen::Calendar)
                    on:click=move |_| state.go(Screen::Calendar)
                >
                    {icons::dock_calendar()}
                    <span>"Calendario"</span>
                </button>
                <button
                    class=move || cls(Screen::Settings)
                    on:click=move |_| state.go(Screen::Settings)
                >
                    {icons::dock_settings()}
                    <span>"Programma"</span>
                </button>
            </div>
        </nav>
    }
}
