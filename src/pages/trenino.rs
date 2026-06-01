//! Schema Trenino inVictus: progressione del Blocco 1 (ciclo di 6 sessioni).
use leptos::prelude::*;

use crate::components::icons;
use crate::state::{AppState, Screen, TRENINO};

#[component]
pub fn Trenino() -> impl IntoView {
    let state = expect_context::<AppState>();
    let sess_num = state.current.get_untracked().sess_num;
    let cur_cycle = ((sess_num - 1) % 6) as usize;
    let ciclo_no = (sess_num - 1) / 6 + 1;
    let sess_in_cycle = (sess_num - 1) % 6 + 1;

    let rows = TRENINO
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let highlight = i == cur_cycle;
            let row_cls = format!(
                "grid grid-cols-[58px_repeat(4,1fr)_56px] items-center px-1 py-2.5 border-t border-base-200 {}",
                if highlight { "bg-primary/8" } else { "" }
            );
            let sess_cls = format!(
                "text-center font-extrabold text-[14px] {}",
                if highlight { "text-primary" } else { "" }
            );
            let cells = r
                .t
                .iter()
                .map(|t| {
                    view! {
                        <span class="text-center text-[12.5px] font-semibold tnum">{*t}</span>
                    }
                })
                .collect_view();
            let note_cls =
                format!("px-3 pb-2.5 border-t-0 {}", if highlight { "bg-primary/8" } else { "" });
            let note_prefix = highlight.then(|| {
                view! { <b class="text-primary">"Sessione attuale · "</b> }
            });
            view! {
                <div class=row_cls>
                    <span class=sess_cls>{format!("{}{}", if highlight { "▸ " } else { "" }, i + 1)}</span>
                    {cells}
                    <span class="text-center text-[11px] font-bold text-base-content/50 tnum">
                        {r.tot}
                    </span>
                </div>
                <div class=note_cls>
                    <p class="text-[11px] text-base-content/45 leading-snug">{note_prefix} {r.note}</p>
                </div>
            }
        })
        .collect_view();

    view! {
        <header class="px-3 pt-[calc(env(safe-area-inset-top)+12px)] pb-3 flex items-center gap-1">
            <button
                class="w-10 h-10 -ml-1 grid place-items-center rounded-full active:bg-base-200 transition"
                on:click=move |_| state.go(Screen::Settings)
            >
                {icons::back()}
            </button>
            <div class="min-w-0">
                <h1 class="text-[20px] font-extrabold tracking-tight leading-tight">
                    "Trenino inVictus"
                </h1>
                <p class="text-[12px] text-base-content/55">"Schema Blocco 1 · Paolo Evangelista"</p>
            </div>
        </header>
        <div class="scrollarea px-4 pb-6">
            <p class="text-[13px] text-base-content/65 leading-snug bg-base-200/50 rounded-2xl px-4 py-3 mb-4">
                "Si applica al " <b>"1° fondamentale del Blocco 1"</b>
                " di ogni Giorno (Panca, Squat, Lento, Rematore, RDL, Dip…). 4 serie a sessione · "
                <b>"Buffer"</b> ", mai cedimento · recupero 2-3 min · carico ↑ progressivo."
            </p>

            <div class="bg-base-100 rounded-2xl border border-base-200 overflow-hidden">
                <div class="grid grid-cols-[58px_repeat(4,1fr)_56px] text-[11px] font-bold uppercase tracking-wide text-base-content/45 bg-base-200/60 px-1 py-2.5">
                    <span class="text-center">"Sess."</span>
                    <span class="text-center">"S1"</span>
                    <span class="text-center">"S2"</span>
                    <span class="text-center">"S3"</span>
                    <span class="text-center">"S4"</span>
                    <span class="text-center">"Tot."</span>
                </div>
                {rows}
            </div>

            <div class="flex gap-3 items-start mt-4 text-[12.5px] text-base-content/60 bg-base-200/50 rounded-2xl px-4 py-3">
                <span class="text-base-content/40 mt-0.5">{icons::info()}</span>
                <p class="leading-snug">
                    "Le 18 sessioni dei B1 ripetono il ciclo 1→6 " <b>"tre volte"</b>
                    " (3 cicli completi). Sei al " <b>{format!("Ciclo {ciclo_no}")}</b>
                    {format!(", sessione {sess_in_cycle} del ciclo.")}
                </p>
            </div>
        </div>
    }
}
