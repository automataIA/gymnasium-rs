//! Piccoli helper UI condivisi.
use leptos::prelude::*;

use crate::state::Block;

/// Badge del blocco "B1 · Fondamentale".
pub fn blk_badge(blk: Block) -> impl IntoView {
    let class = format!(
        "blk-badge blk-{} text-[10px] font-bold tracking-wide px-2 py-0.5 rounded-full whitespace-nowrap shrink-0",
        blk.lower()
    );
    view! { <span class=class>{format!("{} · {}", blk.as_str(), blk.tag())}</span> }
}

/// Card statistica (label / valore / sottotitolo).
pub fn stat(label: &str, val: String, sub: &str) -> impl IntoView {
    let label = label.to_string();
    let sub = sub.to_string();
    view! {
        <div class="bg-base-200/60 rounded-2xl px-3 py-3 text-center">
            <p class="text-[10.5px] font-semibold text-base-content/50 uppercase tracking-wide">
                {label}
            </p>
            <p class="text-[22px] font-extrabold tnum mt-0.5 leading-none">{val}</p>
            <p class="text-[10.5px] text-base-content/40 mt-0.5">{sub}</p>
        </div>
    }
}
