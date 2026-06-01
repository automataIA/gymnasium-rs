//! Programma: personalizza esercizi, blocchi e serie.
use std::sync::atomic::{AtomicU32, Ordering};

use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::{JsCast, JsValue};

use crate::components::exercise_card::edit_state_from;
use crate::components::icons;
use crate::state::{AppState, Block, EditState, Screen};

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

fn new_ex_id() -> String {
    format!("x{}", NEXT_ID.fetch_add(1, Ordering::Relaxed))
}

/// Scarica una stringa come file (anchor + object URL).
fn download_json(json: &str, filename: &str) -> Option<()> {
    let parts = js_sys::Array::of1(&JsValue::from_str(json));
    let blob = web_sys::Blob::new_with_str_sequence(&parts).ok()?;
    let url = web_sys::Url::create_object_url_with_blob(&blob).ok()?;
    let doc = web_sys::window()?.document()?;
    let a = doc
        .create_element("a")
        .ok()?
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .ok()?;
    a.set_href(&url);
    a.set_download(filename);
    a.click();
    let _ = web_sys::Url::revoke_object_url(&url);
    Some(())
}

#[component]
pub fn Settings() -> impl IntoView {
    let state = expect_context::<AppState>();

    // Esporta: scarica un file gymnasium.json.
    let export = move |_: web_sys::MouseEvent| {
        download_json(&state.export_data(), "gymnasium.json");
    };

    // Importa: il bottone apre il file-picker nascosto; il <input> legge il file.
    let file_input: NodeRef<leptos::html::Input> = NodeRef::new();
    let pick_file = move |_: web_sys::MouseEvent| {
        if let Some(input) = file_input.get() {
            input.click();
        }
    };
    let on_file = move |ev: web_sys::Event| {
        let Some(input) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        else {
            return;
        };
        let Some(file) = input.files().and_then(|fs| fs.get(0)) else {
            return;
        };
        input.set_value(""); // consente di reimportare lo stesso file
        let file = gloo_file::File::from(file);
        spawn_local(async move {
            let ok = matches!(
                gloo_file::futures::read_as_text(&file).await,
                Ok(text) if state.import_data(&text)
            );
            state.show_toast(if ok {
                "Dati importati"
            } else {
                "Import fallito: file non valido"
            });
        });
    };

    let reset = move |_: web_sys::MouseEvent| {
        let ok = web_sys::window()
            .and_then(|w| {
                w.confirm_with_message(
                    "Azzerare tutti i dati? Storico e allenamento in corso verranno cancellati e si ripartirà dal Giorno 1. Il programma torna a quello di default.",
                )
                .ok()
            })
            .unwrap_or(false);
        if ok {
            state.reset();
        }
    };

    let days = move || {
        state
            .giorni
            .get()
            .into_iter()
            .map(|g| {
                let gid = g.id;
                let rows = g
                    .es
                    .iter()
                    .map(|e| {
                        let sub = format!(
                            "{} serie · {}",
                            e.serie,
                            if e.tren {
                                "progressione Trenino".to_string()
                            } else {
                                format!("{} reps", e.target.clone().unwrap_or_default())
                            }
                        );
                        let edit = edit_state_from(e);
                        let blk = e.blk;
                        let nome = e.nome.clone();
                        view! {
                            <button
                                class="w-full flex items-center gap-3 px-3.5 py-2.5 text-left active:bg-base-200/50 transition"
                                on:click=move |_| state.edit_modal.set(Some(edit.clone()))
                            >
                                <span class=format!(
                                    "w-1.5 h-8 rounded-full blk-rail blk-{} shrink-0",
                                    blk.lower(),
                                )></span>
                                <div class="flex-1 min-w-0">
                                    <div class="flex items-center gap-2">
                                        <span class=format!(
                                            "blk-badge blk-{} text-[9px] font-bold px-1.5 py-0.5 rounded-full shrink-0",
                                            blk.lower(),
                                        )>{blk.as_str()}</span>
                                        <span class="font-bold text-[13.5px] truncate">{nome}</span>
                                    </div>
                                    <p class="text-[11px] text-base-content/45">{sub}</p>
                                </div>
                                <span class="text-base-content/30 shrink-0">{icons::chev()}</span>
                            </button>
                        }
                    })
                    .collect_view();

                view! {
                    <div class="mb-4 bg-base-100 rounded-2xl border border-base-200 overflow-hidden">
                        <div class="flex items-center gap-2 px-4 py-3 bg-base-200/50 border-b border-base-200">
                            <div class="flex-1 min-w-0">
                                <h3 class="font-extrabold text-[15px] leading-tight">
                                    {format!("Giorno {gid}")}
                                </h3>
                                <p class="text-[11.5px] text-base-content/55 truncate">{g.focus.clone()}</p>
                            </div>
                            <button
                                class="w-8 h-8 grid place-items-center rounded-lg text-base-content/45 bg-base-100 active:scale-90 transition"
                                aria-label="Modifica giorno"
                                on:click=move |_| state.day_modal.set(Some(gid))
                            >
                                {icons::note()}
                            </button>
                        </div>
                        <div class="divide-y divide-base-200">{rows}</div>
                        <button
                            class="w-full px-4 py-2.5 text-[12.5px] font-bold text-primary text-left active:bg-primary/5 transition"
                            on:click=move |_| {
                                state
                                    .edit_modal
                                    .set(
                                        Some(EditState {
                                            id: new_ex_id(),
                                            giorno_id: gid,
                                            nome: String::new(),
                                            blk: Block::B3,
                                            serie: 3,
                                            target: "10-12".into(),
                                            is_new: true,
                                        }),
                                    )
                            }
                        >
                            "+ Aggiungi esercizio"
                        </button>
                    </div>
                }
            })
            .collect_view()
    };

    view! {
        <div class="scrollarea">
            <header class="px-5 pt-[calc(env(safe-area-inset-top)+22px)] pb-3">
                <h1 class="text-[26px] font-extrabold tracking-tight">"Programma"</h1>
                <p class="text-sm text-base-content/55">"Personalizza esercizi, blocchi e serie"</p>
            </header>
            <div class="px-4 pb-6">
                <div class="flex gap-3 items-start bg-primary/8 border border-primary/15 rounded-2xl px-4 py-3 mb-5">
                    <span class="text-primary mt-0.5 shrink-0">{icons::info()}</span>
                    <p class="text-[12.5px] text-base-content/70 leading-snug">
                        "Le modifiche valgono " <b>"dalle prossime sessioni"</b>
                        " (oggi in poi). Le sedute " <b>"già svolte"</b>
                        " restano invariate nello storico."
                    </p>
                </div>
                {days}
                <button
                    class="w-full mt-1 rounded-2xl border border-base-200 bg-base-100 px-4 py-3.5 flex items-center gap-3 active:scale-[0.99] transition"
                    on:click=move |_| state.go(Screen::Trenino)
                >
                    <span class="w-9 h-9 rounded-xl grid place-items-center bg-primary/10 text-primary shrink-0">
                        {icons::train()}
                    </span>
                    <div class="flex-1 text-left min-w-0">
                        <p class="font-bold text-[14px]">"Schema Trenino inVictus"</p>
                        <p class="text-[11.5px] text-base-content/50">
                            "Progressione del Blocco 1 · ciclo di 6 sessioni"
                        </p>
                    </div>
                    <span class="text-base-content/30">{icons::chev()}</span>
                </button>

                <div class="grid grid-cols-2 gap-2 mt-4">
                    <button
                        class="rounded-2xl border border-base-200 bg-base-100 px-4 py-3 font-bold text-[13.5px] active:scale-[0.98] transition"
                        on:click=export
                    >
                        "Esporta JSON"
                    </button>
                    <button
                        class="rounded-2xl border border-base-200 bg-base-100 px-4 py-3 font-bold text-[13.5px] active:scale-[0.98] transition"
                        on:click=pick_file
                    >
                        "Importa JSON"
                    </button>
                </div>
                <input
                    node_ref=file_input
                    type="file"
                    accept="application/json,.json"
                    class="hidden"
                    on:change=on_file
                />

                <button
                    class="w-full mt-2 rounded-2xl border border-error/30 bg-error/5 text-error px-4 py-3.5 flex items-center gap-3 active:scale-[0.99] transition"
                    on:click=reset
                >
                    <span class="w-9 h-9 rounded-xl grid place-items-center bg-error/10 text-error shrink-0">
                        {icons::trash()}
                    </span>
                    <div class="flex-1 text-left min-w-0">
                        <p class="font-bold text-[14px]">"Azzera dati"</p>
                        <p class="text-[11.5px] text-error/70">
                            "Cancella storico e progressi · riparti dal Giorno 1"
                        </p>
                    </div>
                </button>
            </div>
        </div>
    }
}
