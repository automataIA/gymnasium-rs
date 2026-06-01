//! Tastierino numerico bottom-sheet per inserire reps/carico.
use leptos::prelude::*;

use crate::components::icons;
use crate::components::ui::blk_badge;
use crate::state::{AppState, PadField, find_exercise, fmt, range_top, round_half};

#[component]
pub fn Keypad() -> impl IntoView {
    let state = expect_context::<AppState>();

    let open = move || state.pad.get().is_some();
    let scrim_class = move || if open() { "show" } else { "" };
    let sheet_class = move || {
        format!(
            "bg-base-100 rounded-t-3xl shadow-2xl border-t border-base-200 pb-[max(16px,env(safe-area-inset-bottom))] {}",
            if open() { "show" } else { "" }
        )
    };

    let close = move |_| state.pad.set(None);

    let pad_digit = move |d: &'static str| {
        state.pad.update(|opt| {
            if let Some(c) = opt {
                if d == "." && c.buf.contains('.') {
                    return;
                }
                if c.buf.len() >= 6 {
                    return;
                }
                if c.field == PadField::Reps && d == "." {
                    return;
                }
                c.buf.push_str(d);
            }
        });
    };
    let pad_back = move |_| {
        state.pad.update(|opt| {
            if let Some(c) = opt {
                c.buf.pop();
            }
        });
    };
    let pad_quick = move |q: f64| {
        state.pad.update(|opt| {
            if let Some(c) = opt {
                let mut v = c.buf.parse::<f64>().unwrap_or(0.0) + q;
                if v < 0.0 {
                    v = 0.0;
                }
                c.buf = fmt(round_half(v));
            }
        });
    };
    let confirm = move |_| {
        let Some(ctx) = state.pad.get_untracked() else {
            return;
        };
        let v: Option<f64> = if ctx.buf.is_empty() {
            None
        } else {
            ctx.buf.parse().ok()
        };
        state.current.update(|c| {
            if let Some(set) = c.sets.get_mut(&ctx.ex_id).and_then(|v| v.get_mut(ctx.k)) {
                match ctx.field {
                    PadField::Kg => {
                        set.kg = v;
                        if v.is_some() && set.reps.is_none() {
                            let top = range_top(&set.target);
                            if top > 0 {
                                set.reps = Some(top);
                            }
                        }
                    }
                    PadField::Reps => set.reps = v.map(|x| x as u32),
                }
            }
        });
        state.pad.set(None);
    };

    let content = move || {
        let ctx = state.pad.get()?;
        let giorni = state.giorni.get();
        let e = find_exercise(&giorni, &ctx.ex_id)?;
        let blk = e.blk;
        let nome = e.nome.clone();
        let is_kg = ctx.field == PadField::Kg;
        let set = state
            .current
            .get()
            .sets
            .get(&ctx.ex_id)
            .and_then(|v| v.get(ctx.k))
            .cloned();
        let prev = set.as_ref().and_then(|s| s.prev);
        let target = set.as_ref().map(|s| s.target.clone()).unwrap_or_default();
        let val = if ctx.buf.is_empty() {
            "0".to_string()
        } else {
            ctx.buf.clone()
        };
        let quick: [f64; 3] = if is_kg {
            [-2.5, 2.5, 5.0]
        } else {
            [-1.0, 1.0, 2.0]
        };

        let right = match (is_kg, prev) {
            (true, Some(p)) => view! {
                <p class="text-base-content/45">"prec."</p>
                <p class="font-extrabold tnum text-[15px] whitespace-nowrap">
                    {format!("{} kg", fmt(p))}
                </p>
            }
            .into_any(),
            _ => view! {
                <p class="text-base-content/45">"target"</p>
                <p class="font-extrabold tnum text-[15px] whitespace-nowrap">{target}</p>
            }
            .into_any(),
        };

        let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"]
            .into_iter()
            .map(|n| {
                view! {
                    <button class="key" on:click=move |_| pad_digit(n)>
                        {n}
                    </button>
                }
            })
            .collect_view();

        let dot_style = if is_kg { "" } else { "visibility:hidden" };

        let quick_btns = quick
            .into_iter()
            .map(|q| {
                let label =
                    if q > 0.0 { format!("+{}", fmt(q)) } else { fmt(q) };
                view! {
                    <button
                        class="flex-1 h-10 rounded-xl bg-primary/10 text-primary font-bold text-[14px] active:scale-95 transition"
                        on:click=move |_| pad_quick(q)
                    >
                        {label}
                    </button>
                }
            })
            .collect_view();

        Some(view! {
            <div class="px-5 pt-4">
                <div class="w-10 h-1.5 rounded-full bg-base-300 mx-auto mb-3"></div>
                <div class="flex items-center justify-between">
                    <div class="min-w-0">
                        <div class="flex items-center gap-2">
                            {blk_badge(blk)}
                            <span class="text-[11px] text-base-content/45 font-semibold">
                                {format!("Serie {}", ctx.k + 1)}
                            </span>
                        </div>
                        <h3 class="text-[15px] font-extrabold mt-1 truncate">{nome}</h3>
                    </div>
                    <button
                        class="text-[13px] font-bold text-base-content/50 px-3 py-1.5 rounded-full bg-base-200 active:scale-95 transition"
                        on:click=close
                    >
                        "Chiudi"
                    </button>
                </div>

                <div class="mt-3 rounded-2xl bg-base-200/60 px-4 py-3 flex items-end justify-between">
                    <div>
                        <p class="text-[11px] font-semibold text-base-content/45 uppercase tracking-wide">
                            {if is_kg { "Carico" } else { "Reps eseguite" }}
                        </p>
                        <p class="text-[40px] font-extrabold tnum leading-none mt-1">
                            {val}
                            <span class="text-[18px] text-base-content/45 font-bold ml-1">
                                {if is_kg { "kg" } else { "reps" }}
                            </span>
                        </p>
                    </div>
                    <div class="text-right text-[11.5px] leading-tight shrink-0 ml-3">{right}</div>
                </div>

                <div class="flex gap-2 mt-3">{quick_btns}</div>
            </div>

            <div class="grid grid-cols-3 gap-2.5 px-5 mt-3">
                {digits}
                <button class="key text-[20px]" style=dot_style on:click=move |_| pad_digit(".")>
                    "."
                </button>
                <button class="key" on:click=move |_| pad_digit("0")>
                    "0"
                </button>
                <button class="key" on:click=pad_back>
                    {icons::backspace()}
                </button>
            </div>

            <div class="px-5 mt-3.5">
                <button
                    class="w-full h-14 rounded-2xl bg-primary text-primary-content font-extrabold text-[16px] flex items-center justify-center gap-2 active:scale-[0.98] transition"
                    on:click=confirm
                >
                    {icons::check()}
                    "Conferma"
                </button>
            </div>
        })
    };

    view! {
        <div id="sheet-scrim" class=scrim_class on:click=close></div>
        <section id="keypad" class=sheet_class>
            {content}
        </section>
    }
}
