//! Riga di una serie: cella reps + cella carico, con diff vs sessione precedente.
use leptos::prelude::*;

use crate::state::{AppState, CurrentSet, PadCtx, PadField, fmt};

#[component]
pub fn SetRow(ex_id: String, k: usize) -> impl IntoView {
    let state = expect_context::<AppState>();

    let ex_memo = ex_id.clone();
    let set = Memo::new(move |_| -> Option<CurrentSet> {
        state
            .current
            .get()
            .sets
            .get(&ex_memo)
            .and_then(|v| v.get(k))
            .cloned()
    });

    let open = move |field: PadField| {
        let Some(s) = set.get_untracked() else { return };
        let cur = match field {
            PadField::Kg => s.kg,
            PadField::Reps => s.reps.map(|r| r as f64),
        };
        let buf = cur.map(fmt).unwrap_or_default();
        state.pad.set(Some(PadCtx {
            ex_id: ex_id.clone(),
            k,
            field,
            buf,
        }));
    };

    let reps_class = move || {
        let filled = set.get().is_some_and(|s| s.reps.is_some());
        format!(
            "cell {} rounded-xl border border-base-300 h-[52px] flex flex-col items-center justify-center gap-0.5",
            if filled { "cell-filled" } else { "cell-empty" }
        )
    };
    let reps_inner = move || {
        let s = set.get();
        let target = s.as_ref().map(|s| s.target.clone()).unwrap_or_default();
        match s.and_then(|s| s.reps) {
            Some(reps) => view! {
                <span class="text-[19px] font-extrabold tnum leading-none">{reps}</span>
                <span class="text-[9.5px] text-base-content/45 mt-0.5">
                    {format!("target {target}")}
                </span>
            }
            .into_any(),
            None => view! {
                <span class="text-[13px] font-bold text-base-content/40">{target}</span>
                <span class="text-[9px] text-base-content/35 mt-0.5">"reps"</span>
            }
            .into_any(),
        }
    };

    let kg_class = move || {
        let filled = set.get().is_some_and(|s| s.kg.is_some());
        format!(
            "cell {} rounded-xl border border-base-300 h-[52px] flex flex-col items-center justify-center gap-0.5 relative",
            if filled { "cell-filled" } else { "cell-empty" }
        )
    };
    let kg_inner = move || {
        let s = set.get();
        let prev = s.as_ref().and_then(|s| s.prev);
        let prev_line = match prev {
            Some(p) => view! {
                <span class="text-[10.5px] text-base-content/40 tnum">
                    {format!("prec. {}", fmt(p))}
                </span>
            }
            .into_any(),
            None => {
                view! { <span class="text-[10.5px] text-base-content/30">"—"</span> }.into_any()
            }
        };
        match s.as_ref().and_then(|s| s.kg) {
            Some(kg) => {
                let diff = prev.map(|p| {
                    if kg > p {
                        view! { <span class="diff-up font-bold">"▲"</span> }.into_any()
                    } else if kg < p {
                        view! { <span class="diff-down font-bold">"▼"</span> }.into_any()
                    } else {
                        view! { <span class="diff-eq font-bold">"="</span> }.into_any()
                    }
                });
                view! {
                    <span class="flex items-baseline gap-1">
                        <span class="text-[20px] font-extrabold tnum leading-none">{fmt(kg)}</span>
                        <span class="text-[11px] text-base-content/45 font-semibold">"kg"</span>
                        {diff}
                    </span>
                    {prev_line}
                }
                .into_any()
            }
            None => view! {
                <span class="text-[13px] font-bold text-primary">"+ kg"</span>
                {prev_line}
            }
            .into_any(),
        }
    };

    view! {
        <div class="grid grid-cols-[34px_1fr_1fr] gap-2 items-center">
            <span class="text-center text-[13px] font-extrabold text-base-content/45">
                {format!("S{}", k + 1)}
            </span>
            <button class=reps_class on:click={
                let open = open.clone();
                move |_| open(PadField::Reps)
            }>{reps_inner}</button>
            <button class=kg_class on:click=move |_| open(PadField::Kg)>
                {kg_inner}
            </button>
        </div>
    }
}
