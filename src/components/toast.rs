//! Toast transitorio (auto-dismiss dopo ~2.4s).
use std::time::Duration;

use leptos::prelude::*;

use crate::state::AppState;

#[component]
pub fn ToastView() -> impl IntoView {
    let state = expect_context::<AppState>();

    Effect::new(move |_| {
        if state.toast.get().is_some() {
            set_timeout(move || state.toast.set(None), Duration::from_millis(2400));
        }
    });

    move || {
        state.toast.get().map(|msg| {
            view! {
                <div
                    class="fixed left-1/2 -translate-x-1/2 bottom-24 z-[60] px-4 py-2.5 rounded-full bg-neutral text-neutral-content text-[13px] font-semibold shadow-lg"
                    style="max-width:90%"
                >
                    {msg}
                </div>
            }
        })
    }
}
