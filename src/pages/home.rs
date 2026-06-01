//! Dashboard: prossimo allenamento, statistiche, blocchi di oggi, attività recente.
use leptos::prelude::*;

use crate::components::icons;
use crate::components::ui::stat;
use crate::state::{AppState, Block, Screen, capitalize, d_short, rel_days, today};

#[component]
pub fn Home() -> impl IntoView {
    let state = expect_context::<AppState>();
    let giorni = state.giorni.get_untracked();
    let current = state.current.get_untracked();
    let sessions = state.sessions.get_untracked();
    let g = giorni
        .iter()
        .find(|x| x.id == current.giorno_id)
        .cloned()
        .expect("giorno valido");

    let cycle_n = ((current.sess_num - 1) % 6) + 1;
    let ciclo_no = (current.sess_num - 1) / 6 + 1;
    let total = sessions.len();
    let week_done = sessions
        .iter()
        .filter(|r| (0..7).contains(&(today().0 - r.date.0)))
        .count();

    let count = |b: Block| g.es.iter().filter(|e| e.blk == b).count();

    let blocchi = [Block::B1, Block::B2, Block::B3]
        .into_iter()
        .map(|b| {
            view! {
                <div class="flex items-center gap-3 px-4 py-3">
                    <span class=format!("w-1.5 h-9 rounded-full blk-rail blk-{}", b.lower())></span>
                    <div class="flex-1 min-w-0">
                        <div class="flex items-center gap-2">
                            <span class="font-bold text-[14px]">{b.as_str()}</span>
                            <span class="text-[12px] text-base-content/55">{b.tag()}</span>
                        </div>
                        <p class="text-[11.5px] text-base-content/50 truncate">{b.desc()}</p>
                    </div>
                    <span class="text-[13px] font-bold text-base-content/40 tnum">
                        {format!("{}×", count(b))}
                    </span>
                </div>
            }
        })
        .collect_view();

    let recent = if sessions.is_empty() {
        view! {
            <p class="text-[12.5px] text-base-content/45 bg-base-200/50 rounded-xl px-4 py-3">
                "Nessuna seduta ancora. Completa il primo allenamento per popolare lo storico."
            </p>
        }
        .into_any()
    } else {
        sessions
            .iter()
            .rev()
            .take(4)
            .map(|rec| {
                let gid = rec.giorno_id;
                let focus = giorni
                    .iter()
                    .find(|x| x.id == gid)
                    .map(|x| x.focus.clone())
                    .unwrap_or_default();
                let date = capitalize(&d_short(rec.date));
                view! {
                    <div class="flex items-center gap-3 bg-base-200/50 rounded-xl px-3.5 py-2.5">
                        <span
                            class="w-9 h-9 rounded-xl grid place-items-center font-extrabold text-[13px] text-success"
                            style="background:color-mix(in oklab, var(--color-success), white 86%)"
                        >
                            {icons::check()}
                        </span>
                        <div class="flex-1 min-w-0">
                            <p class="text-[13.5px] font-bold leading-tight">
                                {format!("Giorno {gid} ")}
                                <span class="font-medium text-base-content/55">
                                    {format!("· {focus}")}
                                </span>
                            </p>
                            <p class="text-[11.5px] text-base-content/45">{date}</p>
                        </div>
                    </div>
                }
            })
            .collect_view()
            .into_any()
    };

    let last_line = match sessions.last() {
        Some(rec) => view! {
            <p class="text-[12px] text-base-content/50 mt-2.5 px-1">
                "Ultima seduta: "
                <span class="font-semibold text-base-content/70">
                    {format!("Giorno {}", rec.giorno_id)}
                </span> {format!(" · {}.", rel_days(rec.date))}
            </p>
        }
        .into_any(),
        None => view! {
            <p class="text-[12px] text-base-content/50 mt-2.5 px-1">
                "Prima seduta in programma · la data verrà registrata quando completi la scheda."
            </p>
        }
        .into_any(),
    };

    let g_id = g.id;
    let g_focus = g.focus.clone();
    let g_prio = g.priorita.clone();
    let n_es = g.es.len();
    let sess_num = current.sess_num;

    view! {
        <div class="scrollarea">
            <header class="px-5 pt-[calc(env(safe-area-inset-top)+22px)] pb-2">
                <p class="text-sm text-base-content/55 font-medium">
                    {capitalize(&d_short(today()))}
                </p>
                <h1 class="text-[26px] font-extrabold tracking-tight mt-0.5">
                    "Pronto ad allenarti?"
                </h1>
            </header>

            <section class="px-4 mt-2">
                <div
                    class="rounded-[1.4rem] p-5 text-primary-content relative overflow-hidden"
                    style="background:linear-gradient(135deg, oklch(0.58 0.17 255), oklch(0.66 0.15 245));"
                >
                    <div
                        class="absolute -right-8 -top-10 w-40 h-40 rounded-full"
                        style="background:oklch(1 0 0 / 0.08)"
                    ></div>
                    <div class="flex items-center gap-2 text-[11px] font-semibold uppercase tracking-wider opacity-90">
                        <span class="w-2 h-2 rounded-full bg-current"></span>
                        "Prossimo allenamento"
                    </div>
                    <h2 class="text-[27px] font-extrabold leading-tight mt-2">
                        {format!("Giorno {g_id}")}
                    </h2>
                    <p class="text-[15px] font-medium opacity-95">{g_focus}</p>
                    <p class="text-[12.5px] opacity-80 mt-0.5">{format!("Priorità: {g_prio}")}</p>

                    <div class="flex flex-wrap gap-2 mt-4">
                        <span
                            class="text-[11px] font-semibold px-2.5 py-1 rounded-full"
                            style="background:oklch(1 0 0 / 0.16)"
                        >{format!("Sessione {sess_num} · Ciclo {ciclo_no}")}</span>
                        <span
                            class="text-[11px] font-semibold px-2.5 py-1 rounded-full"
                            style="background:oklch(1 0 0 / 0.16)"
                        >{format!("Trenino S{cycle_n}/6")}</span>
                        <span
                            class="text-[11px] font-semibold px-2.5 py-1 rounded-full"
                            style="background:oklch(1 0 0 / 0.16)"
                        >{format!("{n_es} esercizi")}</span>
                    </div>

                    <button
                        class="mt-5 w-full h-14 rounded-2xl bg-base-100 text-primary font-extrabold text-[16px] flex items-center justify-center gap-2 active:scale-[0.98] transition"
                        on:click=move |_| state.go(Screen::Workout)
                    >
                        "Inizia allenamento"
                        {icons::chev()}
                    </button>
                </div>
                {last_line}
            </section>

            <section class="px-4 mt-5 grid grid-cols-3 gap-2.5">
                {stat("Questa sett.", format!("{week_done}/3"), "sedute")}
                {stat("Sedute totali", total.to_string(), "completate")}
                {stat("Ciclo Trenino", format!("{ciclo_no}/3"), "in corso")}
            </section>

            <section class="px-4 mt-6">
                <div class="flex items-center justify-between mb-2 px-1">
                    <h3 class="text-[15px] font-bold">"Blocchi di oggi"</h3>
                    <button
                        class="text-base-content/45 active:scale-90 transition"
                        on:click=move |_| state.info_modal.set(true)
                    >
                        {icons::info()}
                    </button>
                </div>
                <div class="bg-base-200/60 rounded-2xl divide-y divide-base-300/70">{blocchi}</div>
            </section>

            <section class="px-4 mt-6 pb-6">
                <h3 class="text-[15px] font-bold mb-2 px-1">"Attività recente"</h3>
                <div class="space-y-2">{recent}</div>
            </section>
        </div>
    }
}
