//! Grafico a linea del carico massimo per sessione (porta `drawChart`).
use leptos::prelude::*;

use crate::state::{AppState, fmt, hist_series, round_half};

#[component]
pub fn Chart() -> impl IntoView {
    let state = expect_context::<AppState>();

    move || {
        let giorni = state.giorni.get();
        let logs = state.logs.get();
        let current = state.current.get();
        let ex = state.hist_ex.get();
        let series = hist_series(&giorni, &logs, &current, &ex);
        if series.is_empty() {
            return view! { <div class="mt-2"></div> }.into_any();
        }

        let (w, h, pad, pad_l) = (360.0_f64, 150.0_f64, 26.0_f64, 30.0_f64);
        let ys: Vec<f64> = series.iter().map(|p| p.kg).collect();
        let min_y = ys.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_y = ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let span_y = if (max_y - min_y).abs() < f64::EPSILON {
            1.0
        } else {
            max_y - min_y
        };
        let lo = min_y - span_y * 0.25;
        let hi = max_y + span_y * 0.25;
        let n = series.len();
        let x = |i: usize| {
            let frac = if n == 1 {
                0.5
            } else {
                i as f64 / (n - 1) as f64
            };
            pad_l + (w - pad_l - pad) * frac
        };
        let y = |v: f64| pad + (h - pad * 2.0) * (1.0 - (v - lo) / (hi - lo));
        let pts: Vec<(f64, f64)> = series
            .iter()
            .enumerate()
            .map(|(i, p)| (x(i), y(p.kg)))
            .collect();

        let line = pts
            .iter()
            .enumerate()
            .map(|(i, (px, py))| format!("{}{:.1} {:.1}", if i == 0 { "M" } else { "L" }, px, py))
            .collect::<Vec<_>>()
            .join(" ");

        let mut area = format!("M {} {:.1} ", pts[0].0, h - pad);
        area.push_str(
            &pts.iter()
                .map(|(px, py)| format!("L {px:.1} {py:.1}"))
                .collect::<Vec<_>>()
                .join(" "),
        );
        area.push_str(&format!(" L {} {:.1} Z", pts[n - 1].0, h - pad));

        let grid = [hi, (hi + lo) / 2.0, lo]
            .iter()
            .map(|v| {
                let gy = y(*v);
                format!(
                    "<line x1=\"{pad_l}\" y1=\"{gy:.1}\" x2=\"{:.1}\" y2=\"{gy:.1}\" stroke=\"oklch(0.9 0.006 255)\" stroke-width=\"1\"/><text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" font-size=\"9\" fill=\"oklch(0.6 0.01 260)\" font-weight=\"600\">{}</text>",
                    w - pad,
                    pad_l - 6.0,
                    gy + 3.0,
                    fmt(round_half(*v))
                )
            })
            .collect::<String>();

        let dots = series
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let (px, py) = pts[i];
                let r = if p.partial { 5 } else { 4 };
                let fill = if p.partial { "var(--color-base-100)" } else { "var(--color-primary)" };
                format!(
                    "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{r}\" fill=\"{fill}\" stroke=\"var(--color-primary)\" stroke-width=\"2.5\"/><text x=\"{px:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"10\" font-weight=\"800\" fill=\"var(--color-base-content)\">{}</text>",
                    py - 11.0,
                    fmt(p.kg)
                )
            })
            .collect::<String>();

        let xlab = series
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let (px, _) = pts[i];
                format!(
                    "<text x=\"{px:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"9\" fill=\"oklch(0.6 0.01 260)\" font-weight=\"600\">S{}{}</text>",
                    h - 8.0,
                    p.s,
                    if p.partial { "•" } else { "" }
                )
            })
            .collect::<String>();

        let svg = format!(
            "<svg viewBox=\"0 0 {w} {h}\" width=\"100%\" height=\"{h}\" style=\"display:block\">\
             <defs><linearGradient id=\"ag\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">\
             <stop offset=\"0\" stop-color=\"var(--color-primary)\" stop-opacity=\"0.18\"/>\
             <stop offset=\"1\" stop-color=\"var(--color-primary)\" stop-opacity=\"0\"/></linearGradient></defs>\
             {grid}\
             <path d=\"{area}\" fill=\"url(#ag)\"/>\
             <path d=\"{line}\" fill=\"none\" stroke=\"var(--color-primary)\" stroke-width=\"2.5\" stroke-linejoin=\"round\" stroke-linecap=\"round\"/>\
             {dots}{xlab}</svg>"
        );

        view! { <div class="mt-2" inner_html=svg></div> }.into_any()
    }
}
