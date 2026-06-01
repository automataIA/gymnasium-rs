//! Modali: nota esercizio, legenda metodo, edit esercizio, edit giorno.
use leptos::prelude::*;

use crate::components::icons;
use crate::components::ui::blk_badge;
use crate::state::{AppState, Block, EditState, find_exercise};

#[component]
pub fn Modals() -> impl IntoView {
    let state = expect_context::<AppState>();

    let note = move || {
        state
            .note_modal
            .get()
            .map(|ex_id| view! { <NoteModalInner ex_id=ex_id /> })
    };
    let info = move || state.info_modal.get().then(InfoModal);
    let edit = move || {
        state
            .edit_modal
            .get()
            .map(|init| view! { <EditModalInner init=init /> })
    };
    let day = move || {
        state
            .day_modal
            .get()
            .map(|gid| view! { <DayModalInner giorno_id=gid /> })
    };

    view! {
        {note}
        {info}
        {edit}
        {day}
    }
}

fn modal_box(children: AnyView, close: impl Fn() + Copy + 'static) -> impl IntoView {
    view! {
        <div class="modal modal-bottom sm:modal-middle modal-open">
            <div class="modal-box rounded-t-3xl sm:rounded-3xl max-w-[430px] mx-auto">{children}</div>
            <div class="modal-backdrop" on:click=move |_| close()></div>
        </div>
    }
}

#[component]
fn NoteModalInner(ex_id: String) -> impl IntoView {
    let state = expect_context::<AppState>();
    let giorni = state.giorni.get_untracked();
    let Some(e) = find_exercise(&giorni, &ex_id) else {
        return view! { <div></div> }.into_any();
    };
    let blk = e.blk;
    let nome = e.nome.clone();
    let grp = e.grp.clone();
    let init = state
        .current
        .get_untracked()
        .notes
        .get(&ex_id)
        .cloned()
        .unwrap_or_default();
    let text = RwSignal::new(init.clone());

    let close = move || state.note_modal.set(None);
    let save = {
        let ex_id = ex_id.clone();
        move |_| {
            let v = text.get_untracked();
            state.current.update(|c| {
                c.notes.insert(ex_id.clone(), v);
            });
            close();
        }
    };

    let body = view! {
        <div class="flex items-center gap-2 mb-1">{blk_badge(blk)}</div>
        <h3 class="font-extrabold text-[17px] leading-tight">{nome}</h3>
        <p class="text-[12px] text-base-content/50 mb-3">{format!("Note · {grp}")}</p>
        <textarea
            rows="4"
            placeholder="Sensazioni, tecnica, regolazioni macchina, dolori…"
            class="w-full rounded-xl border border-base-300 bg-base-200/50 p-3 text-[14px] leading-snug focus:border-primary"
            prop:value=init
            on:input=move |ev| text.set(event_target_value(&ev))
        ></textarea>
        <div class="flex gap-2 mt-3">
            <button
                class="flex-1 h-12 rounded-xl bg-base-200 font-bold text-[14px] active:scale-95 transition"
                on:click=move |_| close()
            >
                "Annulla"
            </button>
            <button
                class="flex-1 h-12 rounded-xl bg-primary text-primary-content font-extrabold text-[14px] active:scale-95 transition"
                on:click=save
            >
                "Salva nota"
            </button>
        </div>
    }
    .into_any();

    modal_box(body, close).into_any()
}

fn info_row(blk: Block) -> impl IntoView {
    view! {
        <div class="flex gap-3 py-2.5">
            <span class=format!("w-1.5 rounded-full blk-rail blk-{} shrink-0", blk.lower())></span>
            <div>
                <p class="font-extrabold text-[14px]">{format!("{} · {}", blk.as_str(), blk.tag())}</p>
                <p class="text-[12.5px] text-base-content/60 leading-snug">{blk.desc()}</p>
            </div>
        </div>
    }
}

#[component]
fn InfoModal() -> impl IntoView {
    let state = expect_context::<AppState>();
    let close = move || state.info_modal.set(false);
    let body = view! {
        <h3 class="font-extrabold text-[18px]">"Legenda del metodo"</h3>
        <p class="text-[12px] text-base-content/50 mb-1">"Project inVictus · Trenino"</p>
        <div class="divide-y divide-base-200">
            {info_row(Block::B1)} {info_row(Block::B2)} {info_row(Block::B3)}
        </div>
        <div class="mt-3 space-y-2 text-[12.5px] text-base-content/70">
            <p>
                <b>"Buffer"</b>
                " — chiudi la serie sentendo di poter fare ancora 1-2 reps pulite."
            </p>
            <p>
                <b>"RIR"</b>
                " — reps in riserva. RIR 0 = cedimento, RIR 1 = una in canna."
            </p>
            <p>
                <b>"Reps target (grigio)"</b>
                " — precompilate dal Trenino: sovrascrivi con le reps fatte."
            </p>
            <p>
                <b>"Verde / Rosso"</b>
                " — il carico è ▲ o ▼ rispetto alla stessa serie della sessione precedente."
            </p>
        </div>
        <button
            class="mt-4 w-full h-12 rounded-xl bg-neutral text-neutral-content font-extrabold text-[14px]"
            on:click=move |_| close()
        >
            "Ho capito"
        </button>
    }
    .into_any();
    modal_box(body, close)
}

#[component]
fn EditModalInner(init: EditState) -> impl IntoView {
    let state = expect_context::<AppState>();
    let id = init.id.clone();
    let giorno_id = init.giorno_id;
    let is_new = init.is_new;

    let nome = RwSignal::new(init.nome.clone());
    let target = RwSignal::new(init.target.clone());
    let blk = RwSignal::new(init.blk);
    let serie = RwSignal::new(init.serie);

    let close = move || state.edit_modal.set(None);
    let set_blk = move |b: Block| {
        blk.set(b);
        if b == Block::B1 {
            serie.set(4);
        }
    };
    let edit_serie = move |d: i32| {
        let max = if blk.get_untracked() == Block::B1 {
            4
        } else {
            5
        };
        let nv = (serie.get_untracked() as i32 + d).clamp(1, max);
        serie.set(nv as u8);
    };

    let save = {
        let id = id.clone();
        move |_| {
            state.save_exercise(EditState {
                id: id.clone(),
                giorno_id,
                nome: nome.get_untracked(),
                blk: blk.get_untracked(),
                serie: serie.get_untracked(),
                target: target.get_untracked(),
                is_new,
            });
            close();
        }
    };
    let delete = {
        let id = id.clone();
        move |_| {
            state.delete_exercise(&id);
            close();
        }
    };

    let is_b1 = move || blk.get() == Block::B1;

    let blk_buttons = [Block::B1, Block::B2, Block::B3]
        .into_iter()
        .map(|b| {
            let cls = move || {
                if blk.get() == b {
                    format!(
                        "h-12 rounded-xl border blk-badge blk-{} font-extrabold",
                        b.lower()
                    )
                } else {
                    "h-12 rounded-xl border border-base-300 text-base-content/55 font-bold".into()
                }
            };
            view! {
                <button class=cls on:click=move |_| set_blk(b)>
                    <span class="block text-[13px] leading-none">{b.as_str()}</span>
                    <span class="block text-[9px] font-semibold opacity-80 mt-0.5">{b.tag()}</span>
                </button>
            }
        })
        .collect_view();

    let target_area = move || {
        if is_b1() {
            view! {
                <div class="mt-1 h-11 rounded-xl bg-base-200/60 grid place-items-center text-[11.5px] font-semibold text-base-content/50 px-2 text-center leading-tight">
                    "da schema Trenino"
                </div>
            }
            .into_any()
        } else {
            view! {
                <input
                    placeholder="8-10"
                    class="w-full rounded-xl border border-base-300 bg-base-200/50 px-3 h-11 text-[15px] font-semibold text-center focus:border-primary mt-1"
                    prop:value=init.target.clone()
                    on:input=move |ev| target.set(event_target_value(&ev))
                />
            }
            .into_any()
        }
    };

    let delete_btn = (!is_new).then(|| {
        view! {
            <button
                class="h-12 px-4 rounded-xl bg-error/10 text-error font-bold text-[13px] flex items-center gap-1.5 active:scale-95 transition"
                on:click=delete
            >
                {icons::trash()}
            </button>
        }
    });

    let title = if is_new {
        "Nuovo esercizio"
    } else {
        "Modifica esercizio"
    };
    let save_label = if is_new { "Aggiungi" } else { "Salva" };

    let body = view! {
        <h3 class="font-extrabold text-[17px]">{title}</h3>
        <p class="text-[12px] text-base-content/50 mb-3">
            "Vale dalla prossima sessione · lo storico resta invariato"
        </p>

        <label class="text-[11px] font-bold uppercase tracking-wide text-base-content/45">
            "Esercizio"
        </label>
        <input
            placeholder="Es. Pressa 45°"
            class="w-full rounded-xl border border-base-300 bg-base-200/50 px-3 h-12 text-[15px] font-semibold focus:border-primary mt-1 mb-3"
            prop:value=init.nome.clone()
            on:input=move |ev| nome.set(event_target_value(&ev))
        />

        <label class="text-[11px] font-bold uppercase tracking-wide text-base-content/45">
            "Blocco"
        </label>
        <div class="grid grid-cols-3 gap-2 mt-1 mb-3">{blk_buttons}</div>

        <div class="flex gap-3">
            <div class="flex-1">
                <label class="text-[11px] font-bold uppercase tracking-wide text-base-content/45">
                    "Serie"
                </label>
                <div class="flex items-center gap-2 mt-1">
                    <button
                        prop:disabled=is_b1
                        class="w-11 h-11 rounded-xl bg-base-200 font-extrabold text-[20px] disabled:opacity-30 active:scale-95 transition"
                        on:click=move |_| edit_serie(-1)
                    >
                        "−"
                    </button>
                    <span class="flex-1 text-center text-[18px] font-extrabold tnum">
                        {move || serie.get()}
                    </span>
                    <button
                        prop:disabled=is_b1
                        class="w-11 h-11 rounded-xl bg-base-200 font-extrabold text-[20px] disabled:opacity-30 active:scale-95 transition"
                        on:click=move |_| edit_serie(1)
                    >
                        "+"
                    </button>
                </div>
            </div>
            <div class="flex-1">
                <label class="text-[11px] font-bold uppercase tracking-wide text-base-content/45">
                    "Reps target"
                </label>
                {target_area}
            </div>
        </div>
        {move || {
            is_b1()
                .then(|| {
                    view! {
                        <p class="text-[11px] text-base-content/45 mt-2">
                            "Il Blocco 1 usa 4 serie con la progressione del Trenino."
                        </p>
                    }
                })
        }}

        <div class="flex gap-2 mt-5">
            {delete_btn}
            <button
                class="flex-1 h-12 rounded-xl bg-base-200 font-bold text-[14px] active:scale-95 transition"
                on:click=move |_| close()
            >
                "Annulla"
            </button>
            <button
                class="flex-1 h-12 rounded-xl bg-primary text-primary-content font-extrabold text-[14px] active:scale-95 transition"
                on:click=save
            >
                {save_label}
            </button>
        </div>
    }
    .into_any();

    modal_box(body, close)
}

#[component]
fn DayModalInner(giorno_id: u8) -> impl IntoView {
    let state = expect_context::<AppState>();
    let g = state
        .giorni
        .get_untracked()
        .into_iter()
        .find(|g| g.id == giorno_id);
    let Some(g) = g else {
        return view! { <div></div> }.into_any();
    };

    let focus = RwSignal::new(g.focus.clone());
    let prio = RwSignal::new(g.priorita.clone());
    let close = move || state.day_modal.set(None);
    let save = move |_| {
        state.edit_day(giorno_id, focus.get_untracked(), prio.get_untracked());
        close();
    };

    let body = view! {
        <h3 class="font-extrabold text-[17px]">{format!("Giorno {giorno_id}")}</h3>
        <p class="text-[12px] text-base-content/50 mb-3">"Titolo e priorità della seduta"</p>
        <label class="text-[11px] font-bold uppercase tracking-wide text-base-content/45">
            "Focus"
        </label>
        <input
            class="w-full rounded-xl border border-base-300 bg-base-200/50 px-3 h-12 text-[15px] font-semibold focus:border-primary mt-1 mb-3"
            prop:value=g.focus.clone()
            on:input=move |ev| focus.set(event_target_value(&ev))
        />
        <label class="text-[11px] font-bold uppercase tracking-wide text-base-content/45">
            "Priorità"
        </label>
        <input
            class="w-full rounded-xl border border-base-300 bg-base-200/50 px-3 h-12 text-[15px] font-semibold focus:border-primary mt-1"
            prop:value=g.priorita.clone()
            on:input=move |ev| prio.set(event_target_value(&ev))
        />
        <div class="flex gap-2 mt-5">
            <button
                class="flex-1 h-12 rounded-xl bg-base-200 font-bold text-[14px] active:scale-95 transition"
                on:click=move |_| close()
            >
                "Annulla"
            </button>
            <button
                class="flex-1 h-12 rounded-xl bg-primary text-primary-content font-extrabold text-[14px] active:scale-95 transition"
                on:click=save
            >
                "Salva"
            </button>
        </div>
    }
    .into_any();

    modal_box(body, close).into_any()
}
