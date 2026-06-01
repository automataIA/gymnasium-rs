//! Card di un esercizio nella schermata Workout.
use leptos::prelude::*;

use crate::components::icons;
use crate::components::set_row::SetRow;
use crate::components::ui::blk_badge;
use crate::state::{AppState, EditState, Exercise, TRENINO};

/// Costruisce lo stato di edit a partire da un esercizio esistente.
pub fn edit_state_from(e: &Exercise) -> EditState {
    EditState {
        id: e.id.clone(),
        giorno_id: e.giorno,
        nome: e.nome.clone(),
        blk: e.blk,
        serie: e.serie,
        target: e.target.clone().unwrap_or_else(|| "8-10".into()),
        is_new: false,
    }
}

#[component]
pub fn ExerciseCard(ex: Exercise) -> impl IntoView {
    let state = expect_context::<AppState>();
    let id = ex.id.clone();
    let blk = ex.blk;
    let grp = ex.grp.clone();
    let nome = ex.nome.clone();
    let serie = ex.serie;
    let tren = ex.tren;

    let id_note = id.clone();
    let has_note = move || {
        state
            .current
            .get()
            .notes
            .get(&id_note)
            .is_some_and(|n| !n.trim().is_empty())
    };

    let note_text = {
        let id = id.clone();
        move || {
            state
                .current
                .get()
                .notes
                .get(&id)
                .cloned()
                .filter(|n| !n.trim().is_empty())
        }
    };

    let tren_line = tren.then(|| {
        let note = TRENINO[((state.current.get_untracked().sess_num - 1) % 6) as usize].note;
        view! {
            <p
                class="text-[11px] mt-1.5 font-medium"
                style="color:color-mix(in oklab, var(--color-primary), black 8%)"
            >
                {format!("Trenino · {note}")}
            </p>
        }
    });

    let open_edit = {
        let ex = ex.clone();
        move |_| state.edit_modal.set(Some(edit_state_from(&ex)))
    };
    let open_note = {
        let id = id.clone();
        move |_| state.note_modal.set(Some(id.clone()))
    };

    let note_btn_class = move || {
        format!(
            "shrink-0 w-9 h-9 grid place-items-center rounded-xl {} active:scale-90 transition",
            if has_note() {
                "text-primary bg-primary/10"
            } else {
                "text-base-content/40 bg-base-200/70"
            }
        )
    };

    let rows: Vec<_> = (0..serie as usize)
        .map(|k| view! { <SetRow ex_id=id.clone() k=k /> })
        .collect();

    let note_block = move || {
        note_text().map(|n| {
            view! {
                <div class="mt-3 flex gap-2 items-start text-[12px] text-base-content/60 bg-base-200/50 rounded-xl px-3 py-2">
                    <span class="text-primary mt-px shrink-0">{icons::note()}</span>
                    <span class="leading-snug">{n}</span>
                </div>
            }
        })
    };

    let card_class = format!(
        "blk-{} bg-base-100 rounded-2xl border border-base-200 shadow-sm overflow-hidden",
        blk.lower()
    );

    view! {
        <article class=card_class>
            <div class="flex items-stretch">
                <span class="w-1.5 blk-rail shrink-0"></span>
                <div class="flex-1 min-w-0 p-4">
                    <div class="flex items-start gap-2">
                        <div class="flex-1 min-w-0">
                            <div class="flex items-center gap-2 min-w-0">
                                {blk_badge(blk)}
                                <span class="text-[11px] text-base-content/45 font-medium truncate">
                                    {grp}
                                </span>
                            </div>
                            <h3
                                title="Doppio tap per modificare o sostituire"
                                class="text-[16.5px] font-extrabold mt-1.5 leading-tight cursor-pointer select-none"
                                on:dblclick=open_edit
                            >
                                {nome}
                            </h3>
                            {tren_line}
                        </div>
                        <button
                            class=note_btn_class
                            on:click=open_note
                            aria-label="Note"
                        >
                            {icons::note()}
                        </button>
                    </div>

                    <div class="mt-3 space-y-1.5">
                        <div class="grid grid-cols-[34px_1fr_1fr] gap-2 px-1 text-[10px] font-bold uppercase tracking-wide text-base-content/40">
                            <span>"Serie"</span>
                            <span class="text-center">"Reps"</span>
                            <span class="text-center">"Carico"</span>
                        </div>
                        {rows}
                    </div>
                    {note_block}
                </div>
            </div>
        </article>
    }
}
