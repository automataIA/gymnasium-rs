use leptos::prelude::*;

mod components;
mod pages;
mod state;

use components::dock::Dock;
use components::keypad::Keypad;
use components::modals::Modals;
use components::toast::ToastView;
use pages::calendar::Calendar;
use pages::history::History;
use pages::home::Home;
use pages::settings::Settings;
use pages::trenino::Trenino;
use pages::workout::Workout;
use state::{AppState, Screen};

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

    // Persiste l'intero stato in localStorage ad ogni modifica.
    Effect::new(move |_| {
        state.giorni.track();
        state.logs.track();
        state.current.track();
        state.save();
    });

    let screen = move || match state.screen.get() {
        Screen::Home => view! { <Home /> }.into_any(),
        Screen::Workout => view! { <Workout /> }.into_any(),
        Screen::History => view! { <History /> }.into_any(),
        Screen::Calendar => view! { <Calendar /> }.into_any(),
        Screen::Settings => view! { <Settings /> }.into_any(),
        Screen::Trenino => view! { <Trenino /> }.into_any(),
    };

    view! {
        <div id="app">
            <main id="screens" class="contents">
                <section class="screen">{screen}</section>
            </main>
            <Dock />
            <Keypad />
            <Modals />
            <ToastView />
        </div>
    }
}
