//! Storico: andamento del carico per esercizio.
use leptos::prelude::*;

use crate::components::chart::Chart;
use crate::components::ui::{blk_badge, stat};
use crate::state::{AppState, capitalize, d_short, find_exercise, fmt, hist_series, round_half};

#[component]
pub fn History() -> impl IntoView {
    let state = expect_context::<AppState>();

    let options = move || {
        let sel = state.hist_ex.get();
        state
            .giorni
            .get()
            .iter()
            .flat_map(|g| {
                let gid = g.id;
                g.es.iter()
                    .map(move |e| (gid, e.id.clone(), e.nome.clone()))
            })
            .map(|(gid, id, nome)| {
                let selected = id == sel;
                view! {
                    <option value=id selected=selected>
                        {format!("Giorno {gid} · {nome}")}
                    </option>
                }
            })
            .collect_view()
    };

    let stats = move || {
        let giorni = state.giorni.get();
        let logs = state.logs.get();
        let current = state.current.get();
        let series = hist_series(&giorni, &logs, &current, &state.hist_ex.get());
        if series.is_empty() {
            return view! { <div></div> }.into_any();
        }
        let vals: Vec<f64> = series.iter().map(|p| p.kg).collect();
        let best = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let last_v = *vals.last().unwrap();
        let first_v = vals[0];
        let delta = round_half(last_v - first_v);
        let delta_s = format!("{}{}", if delta >= 0.0 { "+" } else { "" }, fmt(delta));
        view! {
            <div class="grid grid-cols-3 gap-2.5 mb-4">
                {stat("Migliore", fmt(best), "kg")} {stat("Attuale", fmt(last_v), "kg")}
                {stat("Progresso", delta_s, "kg")}
            </div>
        }
        .into_any()
    };

    let blk_chip = move || {
        let giorni = state.giorni.get();
        find_exercise(&giorni, &state.hist_ex.get()).map(|e| blk_badge(e.blk))
    };

    let dettaglio = move || {
        let giorni = state.giorni.get();
        let logs = state.logs.get();
        let current = state.current.get();
        let ex_id = state.hist_ex.get();
        let cur_name = find_exercise(&giorni, &ex_id)
            .map(|e| e.nome.clone())
            .unwrap_or_default();
        let mut series = hist_series(&giorni, &logs, &current, &ex_id);
        series.reverse();
        series
            .into_iter()
            .map(|p| {
                let renamed = (!p.name.is_empty() && p.name != cur_name).then(|| {
                    view! {
                        <p class="text-[10.5px] text-warning font-semibold">
                            {format!("svolto come: {}", p.name)}
                        </p>
                    }
                });
                let diff = match p.diff {
                    Some(d) if d > 0.0 => view! {
                        <span class="text-[13px] font-bold diff-up">{format!("▲ +{}", fmt(d))}</span>
                    }
                    .into_any(),
                    Some(d) if d < 0.0 => view! {
                        <span class="text-[13px] font-bold diff-down">{format!("▼ {}", fmt(d))}</span>
                    }
                    .into_any(),
                    Some(_) => view! {
                        <span class="text-[13px] font-bold diff-eq">"="</span>
                    }
                    .into_any(),
                    None => view! {
                        <span class="text-[12px] text-base-content/35">"inizio"</span>
                    }
                    .into_any(),
                };
                view! {
                    <div class="flex items-center gap-3 bg-base-200/50 rounded-xl px-4 py-3">
                        <div class="w-11 text-center">
                            <p class="text-[10px] font-bold text-base-content/45 uppercase">"Sess."</p>
                            <p class="text-[17px] font-extrabold tnum leading-none">{p.s}</p>
                        </div>
                        <div class="flex-1">
                            <p class="text-[15px] font-extrabold tnum">
                                {format!("{} kg ", fmt(p.kg))}
                                <span class="text-[12px] font-medium text-base-content/45">
                                    "· max serie"
                                </span>
                            </p>
                            <p class="text-[11.5px] text-base-content/45">
                                {format!("{} serie · {}", p.sets, capitalize(&d_short(p.date)))}
                            </p>
                            {renamed}
                        </div>
                        {diff}
                    </div>
                }
            })
            .collect_view()
    };

    view! {
        <header class="px-5 pt-[calc(env(safe-area-inset-top)+22px)] pb-3">
            <h1 class="text-[26px] font-extrabold tracking-tight">"Storico"</h1>
            <p class="text-sm text-base-content/55">"Andamento del carico per esercizio"</p>
        </header>

        <div class="scrollarea px-4 pb-6">
            <select
                class="select w-full rounded-2xl bg-base-200/60 border-base-300 font-bold text-[14.5px] h-13 mb-4"
                on:change=move |ev| state.hist_ex.set(event_target_value(&ev))
            >
                {options}
            </select>

            {stats}

            <div class="bg-base-100 rounded-2xl border border-base-200 p-4">
                <div class="flex items-center justify-between mb-1">
                    <h3 class="font-bold text-[14px]">"Carico massimo per sessione"</h3>
                    {blk_chip}
                </div>
                <Chart />
            </div>

            <h3 class="font-bold text-[14px] mt-5 mb-2 px-1">"Dettaglio sessioni"</h3>
            <div class="space-y-2">{dettaglio}</div>
        </div>
    }
}
