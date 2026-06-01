//! Schermata di allenamento attivo: log serie per serie.
use leptos::prelude::*;

use crate::components::exercise_card::ExerciseCard;
use crate::components::icons;
use crate::state::{AppState, Screen};

#[component]
pub fn Workout() -> impl IntoView {
    let state = expect_context::<AppState>();
    let cur = state.current.get_untracked();
    let giorno_id = cur.giorno_id;
    let sess_num = cur.sess_num;
    let cycle_n = ((sess_num - 1) % 6) + 1;

    let giorni = state.giorni.get_untracked();
    let g = giorni
        .iter()
        .find(|x| x.id == giorno_id)
        .cloned()
        .expect("giorno valido");
    let title = format!("Giorno {} · {}", g.id, g.priorita);
    let sub = format!("Sessione {sess_num} · Trenino S{cycle_n}/6 · {}", g.focus);

    let progress = move || {
        let c = state.current.get();
        let total: usize = c.sets.values().map(|v| v.len()).sum();
        let done: usize = c.sets.values().flatten().filter(|s| s.kg.is_some()).count();
        let pct = (done * 100).checked_div(total).unwrap_or(0);
        (done, total, pct)
    };

    let cards = move || {
        let giorni = state.giorni.get();
        giorni.iter().find(|x| x.id == giorno_id).map(|g| {
            g.es.iter()
                .map(|e| view! { <ExerciseCard ex=e.clone() /> })
                .collect_view()
        })
    };

    view! {
        <header class="px-3 pt-[calc(env(safe-area-inset-top)+12px)] pb-3 bg-base-100/95 backdrop-blur sticky top-0 z-20 border-b border-base-200">
            <div class="flex items-center gap-1">
                <button
                    class="w-10 h-10 -ml-1 grid place-items-center rounded-full active:bg-base-200 transition"
                    on:click=move |_| state.go(Screen::Home)
                >
                    {icons::back()}
                </button>
                <div class="flex-1 min-w-0">
                    <h1 class="text-[17px] font-extrabold leading-tight truncate">{title}</h1>
                    <p class="text-[11.5px] text-base-content/55 truncate">{sub}</p>
                </div>
                <button
                    class="w-10 h-10 grid place-items-center rounded-full text-base-content/50 active:bg-base-200 transition"
                    on:click=move |_| state.info_modal.set(true)
                >
                    {icons::info()}
                </button>
            </div>
            <div class="mt-2.5 flex items-center gap-2.5">
                <div class="flex-1 h-2 rounded-full bg-base-200 overflow-hidden">
                    <div
                        class="h-full rounded-full bg-primary transition-all"
                        style=move || format!("width:{}%", progress().2)
                    ></div>
                </div>
                <span class="text-[11.5px] font-bold text-base-content/55 tnum">
                    {move || {
                        let (done, total, _) = progress();
                        format!("{done}/{total} serie")
                    }}
                </span>
            </div>
        </header>

        <div class="scrollarea px-4 pt-4 pb-28 space-y-3.5">
            {cards}
            <button
                class="w-full h-14 rounded-2xl bg-neutral text-neutral-content font-extrabold text-[15.5px] flex items-center justify-center gap-2 active:scale-[0.98] transition mt-2"
                on:click=move |_| state.finish_workout()
            >
                {icons::check()}
                "Completa allenamento"
            </button>
        </div>
    }
}
