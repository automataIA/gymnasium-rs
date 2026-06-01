//! Icone SVG inline (portate da `ICON` del mock). Stroke = currentColor.
use leptos::prelude::*;

pub fn chev() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2">
            <path d="m9 6 6 6-6 6" />
        </svg>
    }
}

pub fn back() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2">
            <path d="m15 6-6 6 6 6" />
        </svg>
    }
}

pub fn note() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="1.8">
            <path d="M12 20h9" />
            <path d="M16.5 3.5a2.1 2.1 0 0 1 3 3L7 19l-4 1 1-4Z" />
        </svg>
    }
}

pub fn info() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="1.8">
            <circle cx="12" cy="12" r="9" />
            <path d="M12 11v5M12 8h.01" />
        </svg>
    }
}

pub fn check() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.4">
            <path d="m5 13 4 4L19 7" />
        </svg>
    }
}

pub fn train() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="1.8">
            <path d="M5 18V7a3 3 0 0 1 3-3h8a3 3 0 0 1 3 3v11" />
            <path d="M5 14h14M8.5 18l-1.5 2M15.5 18l1.5 2" />
        </svg>
    }
}

pub fn trash() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="1.8">
            <path d="M4 7h16M9 7V5a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2M6 7l1 13a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1l1-13" />
        </svg>
    }
}

pub fn backspace() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="1.8">
            <path d="M9 6h11a1 1 0 0 1 1 1v10a1 1 0 0 1-1 1H9L3 12Z" />
            <path d="m13 9 4 6m0-6-4 6" />
        </svg>
    }
}

// --- icone del dock ---
pub fn dock_home() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="1.8">
            <path d="M3 11.5 12 4l9 7.5" />
            <path d="M5 10v9a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-9" />
        </svg>
    }
}

pub fn dock_history() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="1.8">
            <path d="M4 19V5" />
            <path d="M4 17l5-5 4 3 7-8" />
        </svg>
    }
}

pub fn dock_calendar() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="1.8">
            <rect x="3.5" y="5" width="17" height="16" rx="2.5" />
            <path d="M3.5 9.5h17M8 3v4M16 3v4" />
        </svg>
    }
}

pub fn dock_settings() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="1.8">
            <path d="M4 7h9M17 7h3M4 17h3M11 17h9" />
            <circle cx="15" cy="7" r="2.3" />
            <circle cx="9" cy="17" r="2.3" />
        </svg>
    }
}
