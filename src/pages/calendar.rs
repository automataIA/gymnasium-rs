//! Calendario a scorrimento (rotazione A·B·A·B): svolti, prossimo, previsti.
use leptos::prelude::*;

use crate::components::icons;
use crate::state::{AppState, Date, Screen, capitalize, d_short, rel_days, today};

#[derive(Clone, Copy, PartialEq)]
enum RowState {
    Done,
    Next,
    Future,
}

#[component]
pub fn Calendar() -> impl IntoView {
    let state = expect_context::<AppState>();
    let giorni = state.giorni.get_untracked();
    let current = state.current.get_untracked();
    let sessions = state.sessions.get_untracked();
    let oggi = today();

    let mut rows: Vec<(u8, Date, RowState)> = sessions
        .iter()
        .map(|r| (r.giorno_id, r.date, RowState::Done))
        .collect();
    rows.push((current.giorno_id, oggi, RowState::Next));
    let mut fg = current.giorno_id;
    for i in 1..=3 {
        fg = (fg % 4) + 1;
        rows.push((
            fg,
            oggi.add_days((i as f64 * 2.34) as i64),
            RowState::Future,
        ));
    }
    let start = rows.len().saturating_sub(9);
    let view_rows: Vec<_> = rows[start..].iter().rev().cloned().collect();

    let items = view_rows
        .into_iter()
        .map(|(gid, date, st)| {
            let focus = giorni
                .iter()
                .find(|x| x.id == gid)
                .map(|x| x.focus.clone())
                .unwrap_or_default();
            let is_next = st == RowState::Next;
            let is_fut = st == RowState::Future;

            let dot = match st {
                RowState::Next => view! {
                    <span class="w-6 h-6 rounded-full bg-primary ring-4 ring-primary/20 grid place-items-center text-primary-content">
                        <span class="w-2 h-2 rounded-full bg-current"></span>
                    </span>
                }
                .into_any(),
                RowState::Future => view! {
                    <span class="w-5 h-5 rounded-full border-2 border-dashed border-base-300 bg-base-100"></span>
                }
                .into_any(),
                RowState::Done => view! {
                    <span class="w-5 h-5 rounded-full bg-success/15 grid place-items-center text-success">
                        {icons::check()}
                    </span>
                }
                .into_any(),
            };

            let label = if is_next {
                "Prossimo · oggi".to_string()
            } else if is_fut {
                "Previsto".to_string()
            } else {
                capitalize(&rel_days(date))
            };
            let label_cls = if is_next {
                "text-[11px] font-semibold uppercase tracking-wide whitespace-nowrap text-primary"
            } else {
                "text-[11px] font-semibold uppercase tracking-wide whitespace-nowrap text-base-content/45"
            };
            let date_label = if is_fut {
                format!("~ {}", d_short(date))
            } else {
                capitalize(&d_short(date))
            };
            let card_cls = if is_next {
                "flex-1 min-w-0 rounded-2xl border px-4 py-3 border-primary/40 bg-primary/5"
            } else if is_fut {
                "flex-1 min-w-0 rounded-2xl border px-4 py-3 border-base-200 bg-base-100 opacity-65"
            } else {
                "flex-1 min-w-0 rounded-2xl border px-4 py-3 border-base-200 bg-base-100"
            };
            let open_btn = is_next.then(|| {
                view! {
                    <button
                        class="text-[12px] font-extrabold text-primary flex items-center gap-1 shrink-0"
                        on:click=move |_| state.go(Screen::Workout)
                    >
                        "Apri"
                        {icons::chev()}
                    </button>
                }
            });

            view! {
                <div class="flex gap-3 mb-3 relative">
                    <div class="shrink-0 w-6 flex justify-center pt-2.5">{dot}</div>
                    <div class=card_cls>
                        <div class="flex items-center justify-between gap-2">
                            <p class=label_cls>{label}</p>
                            <p class="text-[11px] font-medium text-base-content/45 whitespace-nowrap shrink-0">
                                {date_label}
                            </p>
                        </div>
                        <div class="flex items-center justify-between gap-2 mt-1">
                            <h3 class="text-[16px] font-extrabold whitespace-nowrap">
                                {format!("Giorno {gid}")}
                            </h3>
                            {open_btn}
                        </div>
                        <p class="text-[12.5px] text-base-content/55 truncate">{focus}</p>
                    </div>
                </div>
            }
        })
        .collect_view();

    view! {
        <header class="px-5 pt-[calc(env(safe-area-inset-top)+22px)] pb-3">
            <h1 class="text-[26px] font-extrabold tracking-tight">"Calendario"</h1>
            <p class="text-sm text-base-content/55">"Rotazione A·B·A·B a scorrimento"</p>
        </header>
        <div class="scrollarea px-4 pb-6">
            <div class="flex gap-3 items-start bg-primary/8 border border-primary/15 rounded-2xl px-4 py-3 mb-5">
                <span class="text-primary mt-0.5 shrink-0">{icons::info()}</span>
                <p class="text-[12.5px] text-base-content/70 leading-snug">
                    "Nessuna data fissa: il calendario " <b>"scorre"</b>
                    ". La data viene registrata quando compili la scheda, e l'app propone sempre la "
                    <b>"scheda successiva"</b> " a quella già fatta."
                </p>
            </div>

            <div class="relative">
                <span class="absolute left-[11px] top-3 bottom-6 w-0.5 timeline-line"></span>
                {items}
            </div>
        </div>
    }
}
