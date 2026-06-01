//! Modello dati, dati seed, funzioni pure e stato reattivo dell'app.
//! Port 1:1 della logica di `mock/gym.app.js`.

use std::collections::HashMap;

use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "gymnasium-state";

/* ============================ Date civili ============================ */
/// Data come numero di giorni dall'epoch (algoritmo di H. Hinnant).
/// Evita la dipendenza da `chrono` (costo size contro `opt-level=z`).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Date(pub i64);

const MESI: [&str; 12] = [
    "gen", "feb", "mar", "apr", "mag", "giu", "lug", "ago", "set", "ott", "nov", "dic",
];
const GG: [&str; 7] = ["dom", "lun", "mar", "mer", "gio", "ven", "sab"];

impl Date {
    pub const fn from_civil(y: i64, m: i64, d: i64) -> Date {
        let y = if m <= 2 { y - 1 } else { y };
        let era = (if y >= 0 { y } else { y - 399 }) / 400;
        let yoe = y - era * 400;
        let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        Date(era * 146097 + doe - 719468)
    }

    pub fn to_civil(self) -> (i64, i64, i64) {
        let z = self.0 + 719468;
        let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        (if m <= 2 { y + 1 } else { y }, m, d)
    }

    fn weekday(self) -> usize {
        let z = self.0;
        (if z >= -4 {
            (z + 4) % 7
        } else {
            (z + 5) % 7 + 6
        }) as usize
    }

    pub fn add_days(self, n: i64) -> Date {
        Date(self.0 + n)
    }
}

/// Data odierna reale (calendario locale del browser).
pub fn today() -> Date {
    let d = js_sys::Date::new_0();
    Date::from_civil(
        d.get_full_year() as i64,
        d.get_month() as i64 + 1,
        d.get_date() as i64,
    )
}

/// Data fissa di riferimento, usata solo nei test.
#[cfg(test)]
pub const TODAY: Date = Date::from_civil(2026, 6, 1);

/// "lun 1 giu"
pub fn d_short(d: Date) -> String {
    let (_, m, day) = d.to_civil();
    format!("{} {} {}", GG[d.weekday()], day, MESI[(m - 1) as usize])
}

/// "oggi" / "ieri" / "N giorni fa"
pub fn rel_days(d: Date) -> String {
    let diff = today().0 - d.0;
    match diff {
        i if i <= 0 => "oggi".into(),
        1 => "ieri".into(),
        n => format!("{n} giorni fa"),
    }
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/* ============================ Tipi modello ============================ */
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Block {
    B1,
    B2,
    B3,
}

impl Block {
    pub fn as_str(self) -> &'static str {
        match self {
            Block::B1 => "B1",
            Block::B2 => "B2",
            Block::B3 => "B3",
        }
    }
    pub fn lower(self) -> &'static str {
        match self {
            Block::B1 => "b1",
            Block::B2 => "b2",
            Block::B3 => "b3",
        }
    }
    pub fn tag(self) -> &'static str {
        match self {
            Block::B1 => "Fondamentale",
            Block::B2 => "Volume",
            Block::B3 => "Finisher",
        }
    }
    pub fn desc(self) -> &'static str {
        match self {
            Block::B1 => {
                "Trenino inVictus · 4 serie · Buffer (mai a cedimento) · rec. 2-3 min · Tensione meccanica."
            }
            Block::B2 => "3 serie a carico fisso · RIR 0-1 · 1ª con margine, 3ª quasi cedimento.",
            Block::B3 => "3 serie · cedimento · prolunga il TUT (time under tension).",
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub blk: Block,
    pub grp: String,
    pub nome: String,
    pub serie: u8,
    pub tren: bool,
    pub target: Option<String>,
    pub base: f64,
    pub inc: f64,
    pub giorno: u8,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Giorno {
    pub id: u8,
    pub focus: String,
    pub priorita: String,
    pub es: Vec<Exercise>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    pub kg: Vec<f64>,
    pub reps: Vec<u32>,
    pub name: String,
    pub date: Date,
}

/// Una seduta completata (per calendario e attività recente).
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionRecord {
    pub giorno_id: u8,
    pub date: Date,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentSet {
    pub kg: Option<f64>,
    pub reps: Option<u32>,
    pub target: String,
    pub prev: Option<f64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Current {
    pub giorno_id: u8,
    pub sess_num: u32,
    pub sets: HashMap<String, Vec<CurrentSet>>,
    pub notes: HashMap<String, String>,
}

pub struct TreninoRow {
    pub t: [&'static str; 4],
    pub tot: &'static str,
    pub note: &'static str,
}

pub const TRENINO: [TreninoRow; 6] = [
    TreninoRow {
        t: ["8-10", "8-10", "10-12", "10-12"],
        tot: "36-44",
        note: "Apertura ciclo: trova carico allenante.",
    },
    TreninoRow {
        t: ["6-8", "8-10", "8-10", "10-12"],
        tot: "32-40",
        note: "+2,5 / 5 kg vs Sess. 1.",
    },
    TreninoRow {
        t: ["6-8", "6-8", "8-10", "8-10"],
        tot: "28-36",
        note: "Consolida.",
    },
    TreninoRow {
        t: ["4-6", "6-8", "8-10", "8-10"],
        tot: "26-34",
        note: "+ kg sulla serie pesante.",
    },
    TreninoRow {
        t: ["4-6", "6-8", "6-8", "8-10"],
        tot: "24-32",
        note: "Picco intensità.",
    },
    TreninoRow {
        t: ["6-8", "6-8", "6-8", "6-8"],
        tot: "24-32",
        note: "Chiusura: carico ↑ vs Sess. 1.",
    },
];

/* ============================ Funzioni pure ============================ */
/// Arrotondamento al mezzo (come `Math.round(v*2)/2`).
pub fn round_half(v: f64) -> f64 {
    (v * 2.0).round() / 2.0
}

/// Formatta un numero: intero senza decimali, altrimenti 1 cifra.
pub fn fmt(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{}", v as i64)
    } else {
        format!("{v:.1}")
    }
}

/// Top del range reps (es. "8-10" -> 10).
pub fn range_top(range: &str) -> u32 {
    range
        .split('-')
        .nth(1)
        .or_else(|| range.split('-').next())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

pub fn target_for(e: &Exercise, sess_num: u32, set_idx: usize) -> String {
    if e.tren {
        TRENINO[((sess_num - 1) % 6) as usize].t[set_idx].to_string()
    } else {
        e.target.clone().unwrap_or_default()
    }
}

pub fn prev_kg(logs: &HashMap<String, Vec<LogEntry>>, ex_id: &str, set_idx: usize) -> Option<f64> {
    logs.get(ex_id)
        .and_then(|a| a.last())
        .and_then(|e| e.kg.get(set_idx))
        .copied()
}

pub fn find_exercise<'a>(giorni: &'a [Giorno], id: &str) -> Option<&'a Exercise> {
    giorni.iter().flat_map(|g| &g.es).find(|e| e.id == id)
}

#[derive(Clone, PartialEq)]
pub struct HistPoint {
    pub s: u32,
    pub kg: f64,
    pub sets: usize,
    pub date: Date,
    pub name: String,
    pub partial: bool,
    pub diff: Option<f64>,
}

/// Serie storica del carico massimo per sessione (porta `histSeries`).
pub fn hist_series(
    giorni: &[Giorno],
    logs: &HashMap<String, Vec<LogEntry>>,
    current: &Current,
    ex_id: &str,
) -> Vec<HistPoint> {
    let Some(e) = find_exercise(giorni, ex_id) else {
        return Vec::new();
    };
    let empty = Vec::new();
    let arr = logs.get(ex_id).unwrap_or(&empty);

    let mut out: Vec<HistPoint> = Vec::new();
    for (s, rec) in arr.iter().enumerate() {
        if rec.kg.is_empty() {
            continue;
        }
        let kg = rec.kg.iter().cloned().fold(f64::MIN, f64::max);
        out.push(HistPoint {
            s: (s + 1) as u32,
            kg,
            sets: rec.kg.len(),
            date: rec.date,
            name: rec.name.clone(),
            partial: false,
            diff: None,
        });
    }
    if let Some(sets) = current.sets.get(ex_id) {
        let cs: Vec<f64> = sets.iter().filter_map(|x| x.kg).collect();
        if !cs.is_empty() {
            out.push(HistPoint {
                s: (out.len() + 1) as u32,
                kg: cs.iter().cloned().fold(f64::MIN, f64::max),
                sets: cs.len(),
                date: today(),
                name: e.nome.clone(),
                partial: true,
                diff: None,
            });
        }
    }
    for i in 0..out.len() {
        out[i].diff = (i > 0).then(|| round_half(out[i].kg - out[i - 1].kg));
    }
    out
}

/* ============================ Dati seed ============================ */
fn seed_giorni() -> Vec<Giorno> {
    // (blk, grp, nome, serie, tren, target, base, inc)
    type Row = (
        Block,
        &'static str,
        &'static str,
        u8,
        bool,
        Option<&'static str>,
        f64,
        f64,
    );
    fn day(id: u8, focus: &str, prio: &str, rows: &[(&str, Row)]) -> Giorno {
        Giorno {
            id,
            focus: focus.into(),
            priorita: prio.into(),
            es: rows
                .iter()
                .map(
                    |(eid, (blk, grp, nome, serie, tren, target, base, inc))| Exercise {
                        id: (*eid).into(),
                        blk: *blk,
                        grp: (*grp).into(),
                        nome: (*nome).into(),
                        serie: *serie,
                        tren: *tren,
                        target: target.map(Into::into),
                        base: *base,
                        inc: *inc,
                        giorno: id,
                    },
                )
                .collect(),
        }
    }
    use Block::*;
    vec![
        day(
            1,
            "Petto + Spalle + Bicipiti",
            "Petto",
            &[
                (
                    "g1a",
                    (
                        B1,
                        "Petto · Spinta orizzontale",
                        "Panca Piana (Bilanciere)",
                        4,
                        true,
                        None,
                        50.0,
                        2.5,
                    ),
                ),
                (
                    "g1b",
                    (
                        B1,
                        "Spalle · Spinta verticale",
                        "Lento Manubri / Military Press",
                        4,
                        true,
                        None,
                        18.0,
                        1.0,
                    ),
                ),
                (
                    "g1c",
                    (
                        B2,
                        "Petto · Spinta inclinata",
                        "Spinte Manubri panca inclinata",
                        3,
                        false,
                        Some("8-10"),
                        22.0,
                        1.0,
                    ),
                ),
                (
                    "g1d",
                    (
                        B2,
                        "Spalle · Alzata laterale",
                        "Alzate Laterali (Manubri)",
                        3,
                        false,
                        Some("8-10"),
                        10.0,
                        1.0,
                    ),
                ),
                (
                    "g1e",
                    (
                        B3,
                        "Petto · Apertura",
                        "Croci ai Cavi",
                        3,
                        false,
                        Some("12-15"),
                        12.0,
                        1.0,
                    ),
                ),
                (
                    "g1f",
                    (
                        B3,
                        "Bicipiti · Flessione",
                        "Curl Bilanciere EZ",
                        3,
                        false,
                        Some("10-12"),
                        25.0,
                        2.5,
                    ),
                ),
            ],
        ),
        day(
            2,
            "Gambe + Dorso + Tricipiti",
            "Gambe",
            &[
                (
                    "g2a",
                    (B1, "Gambe · Accosciata", "Squat", 4, true, None, 70.0, 2.5),
                ),
                (
                    "g2b",
                    (
                        B1,
                        "Dorso · Tirata orizzontale",
                        "Rematore Manubrio / Pulley",
                        4,
                        true,
                        None,
                        30.0,
                        2.0,
                    ),
                ),
                (
                    "g2c",
                    (
                        B2,
                        "Gambe · Accosciata singola",
                        "Affondi Manubri o Pressa",
                        3,
                        false,
                        Some("8-10"),
                        24.0,
                        2.0,
                    ),
                ),
                (
                    "g2d",
                    (
                        B2,
                        "Dorso · Tirata verticale",
                        "Lat Machine (presa media-larga)",
                        3,
                        false,
                        Some("8-10"),
                        50.0,
                        2.5,
                    ),
                ),
                (
                    "g2e",
                    (
                        B3,
                        "Femorali",
                        "Leg Curl",
                        3,
                        false,
                        Some("12-15"),
                        40.0,
                        2.5,
                    ),
                ),
                (
                    "g2f",
                    (
                        B3,
                        "Tricipiti",
                        "Push-down cavo alto",
                        3,
                        false,
                        Some("10-12"),
                        27.0,
                        2.0,
                    ),
                ),
            ],
        ),
        day(
            3,
            "Spalle + Petto + Bicipiti",
            "Spalle",
            &[
                (
                    "g3a",
                    (
                        B1,
                        "Spalle · Spinta verticale",
                        "Lento Avanti seduto (Manubri)",
                        4,
                        true,
                        None,
                        20.0,
                        1.0,
                    ),
                ),
                (
                    "g3b",
                    (
                        B1,
                        "Petto/Tricipiti · Spinta",
                        "Dip alle Parallele (Zavorrate)",
                        4,
                        true,
                        None,
                        8.0,
                        2.5,
                    ),
                ),
                (
                    "g3c",
                    (
                        B2,
                        "Petto · Spinta inclinata",
                        "Spinte Panca Inclinata",
                        3,
                        false,
                        Some("8-10"),
                        24.0,
                        1.0,
                    ),
                ),
                (
                    "g3d",
                    (
                        B2,
                        "Spalle · Laterale (allung.)",
                        "Alzate Laterali al Cavo",
                        3,
                        false,
                        Some("12-15"),
                        8.0,
                        0.5,
                    ),
                ),
                (
                    "g3e",
                    (
                        B3,
                        "Petto · Apertura inclinata",
                        "Croci su Panca Inclinata",
                        3,
                        false,
                        Some("12-15"),
                        12.0,
                        1.0,
                    ),
                ),
                (
                    "g3f",
                    (
                        B3,
                        "Spalle · Posteriore",
                        "Croci Inverse (Cavi/Macchina)",
                        3,
                        false,
                        Some("12-15"),
                        11.0,
                        1.0,
                    ),
                ),
                (
                    "g3g",
                    (
                        B3,
                        "Bicipiti · Pre-allungamento",
                        "Curl panca inclinata",
                        3,
                        false,
                        Some("10-12"),
                        12.0,
                        1.0,
                    ),
                ),
            ],
        ),
        day(
            4,
            "Dorso + Gambe + Tricipiti",
            "Dorso",
            &[
                (
                    "g4a",
                    (
                        B1,
                        "Dorso · Tirata orizzontale",
                        "Rematore Bilanciere / T-Bar",
                        4,
                        true,
                        None,
                        50.0,
                        2.5,
                    ),
                ),
                (
                    "g4b",
                    (
                        B1,
                        "Gambe · Estensione anca",
                        "Stacco Rumeno (RDL)",
                        4,
                        true,
                        None,
                        60.0,
                        2.5,
                    ),
                ),
                (
                    "g4c",
                    (
                        B2,
                        "Dorso · Tirata vert. stretta",
                        "Trazioni / Lat (Triangolo)",
                        3,
                        false,
                        Some("8-10"),
                        55.0,
                        2.5,
                    ),
                ),
                (
                    "g4d",
                    (
                        B3,
                        "Femorali",
                        "Leg Curl",
                        3,
                        false,
                        Some("12-15"),
                        40.0,
                        2.5,
                    ),
                ),
                (
                    "g4e",
                    (
                        B3,
                        "Dorso · Estensione",
                        "Pullover al cavo alto",
                        3,
                        false,
                        Some("12-15"),
                        25.0,
                        1.0,
                    ),
                ),
                (
                    "g4f",
                    (
                        B3,
                        "Tricipiti · Pre-allungamento",
                        "French Press (panca inclinata)",
                        3,
                        false,
                        Some("10-12"),
                        18.0,
                        1.0,
                    ),
                ),
            ],
        ),
    ]
}

/// Storico vuoto: una voce-lista per ogni esercizio, nessuna seduta registrata.
fn empty_logs(giorni: &[Giorno]) -> HashMap<String, Vec<LogEntry>> {
    giorni
        .iter()
        .flat_map(|g| &g.es)
        .map(|e| (e.id.clone(), Vec::new()))
        .collect()
}

fn blank_current(
    giorni: &[Giorno],
    logs: &HashMap<String, Vec<LogEntry>>,
    giorno_id: u8,
    sess_num: u32,
) -> Current {
    let g = giorni
        .iter()
        .find(|x| x.id == giorno_id)
        .expect("giorno seed valido");
    let mut sets = HashMap::new();
    for e in &g.es {
        let v: Vec<CurrentSet> = (0..e.serie as usize)
            .map(|k| CurrentSet {
                kg: None,
                reps: None,
                target: target_for(e, sess_num, k),
                prev: prev_kg(logs, &e.id, k),
            })
            .collect();
        sets.insert(e.id.clone(), v);
    }
    Current {
        giorno_id,
        sess_num,
        sets,
        notes: HashMap::new(),
    }
}

#[derive(Serialize, Deserialize)]
struct Persist {
    giorni: Vec<Giorno>,
    logs: HashMap<String, Vec<LogEntry>>,
    current: Current,
    sessions: Vec<SessionRecord>,
}

/* ============================ UI state ============================ */
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Workout,
    History,
    Calendar,
    Settings,
    Trenino,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PadField {
    Kg,
    Reps,
}

#[derive(Clone, PartialEq)]
pub struct PadCtx {
    pub ex_id: String,
    pub k: usize,
    pub field: PadField,
    pub buf: String,
}

#[derive(Clone, PartialEq)]
pub struct EditState {
    pub id: String,
    pub giorno_id: u8,
    pub nome: String,
    pub blk: Block,
    pub serie: u8,
    pub target: String,
    pub is_new: bool,
}

/* ============================ AppState ============================ */
#[derive(Clone, Copy)]
pub struct AppState {
    pub giorni: RwSignal<Vec<Giorno>>,
    pub logs: RwSignal<HashMap<String, Vec<LogEntry>>>,
    pub current: RwSignal<Current>,
    pub screen: RwSignal<Screen>,
    pub hist_ex: RwSignal<String>,
    pub pad: RwSignal<Option<PadCtx>>,
    pub note_modal: RwSignal<Option<String>>,
    pub info_modal: RwSignal<bool>,
    pub edit_modal: RwSignal<Option<EditState>>,
    pub day_modal: RwSignal<Option<u8>>,
    pub toast: RwSignal<Option<String>>,
    pub sessions: RwSignal<Vec<SessionRecord>>,
}

/// Stato pulito di partenza: programma seed, storico vuoto, Giorno 1 sessione 1.
fn fresh_state() -> (
    Vec<Giorno>,
    HashMap<String, Vec<LogEntry>>,
    Current,
    Vec<SessionRecord>,
) {
    let giorni = seed_giorni();
    let logs = empty_logs(&giorni);
    let current = blank_current(&giorni, &logs, 1, 1);
    (giorni, logs, current, Vec::new())
}

impl AppState {
    pub fn new() -> Self {
        let (giorni, logs, current, sessions) = match LocalStorage::get::<Persist>(STORAGE_KEY) {
            Ok(p) => (p.giorni, p.logs, p.current, p.sessions),
            Err(_) => fresh_state(),
        };
        Self {
            giorni: RwSignal::new(giorni),
            logs: RwSignal::new(logs),
            current: RwSignal::new(current),
            screen: RwSignal::new(Screen::Home),
            hist_ex: RwSignal::new("g1a".into()),
            pad: RwSignal::new(None),
            note_modal: RwSignal::new(None),
            info_modal: RwSignal::new(false),
            edit_modal: RwSignal::new(None),
            day_modal: RwSignal::new(None),
            toast: RwSignal::new(None),
            sessions: RwSignal::new(sessions),
        }
    }

    fn persist(&self) -> Persist {
        Persist {
            giorni: self.giorni.get_untracked(),
            logs: self.logs.get_untracked(),
            current: self.current.get_untracked(),
            sessions: self.sessions.get_untracked(),
        }
    }

    /// Salva la parte persistente in localStorage.
    pub fn save(&self) {
        let _ = LocalStorage::set(STORAGE_KEY, self.persist());
    }

    /// Esporta tutti i dati come stringa JSON.
    pub fn export_data(&self) -> String {
        serde_json::to_string(&self.persist()).unwrap_or_default()
    }

    /// Importa i dati da una stringa JSON. Ritorna `false` se non è valida.
    pub fn import_data(&self, json: &str) -> bool {
        let Ok(p) = serde_json::from_str::<Persist>(json) else {
            return false;
        };
        self.giorni.set(p.giorni);
        self.logs.set(p.logs);
        self.current.set(p.current);
        self.sessions.set(p.sessions);
        self.hist_ex.set("g1a".into());
        self.go(Screen::Home);
        true
    }

    /// Riporta tutto allo stato pulito (programma seed, storico vuoto, Giorno 1).
    pub fn reset(&self) {
        let (giorni, logs, current, sessions) = fresh_state();
        self.giorni.set(giorni);
        self.logs.set(logs);
        self.current.set(current);
        self.sessions.set(sessions);
        self.hist_ex.set("g1a".into());
        self.go(Screen::Home);
        self.show_toast("Dati azzerati · si riparte dal Giorno 1");
    }

    pub fn go(&self, screen: Screen) {
        self.screen.set(screen);
    }

    pub fn show_toast(&self, msg: impl Into<String>) {
        self.toast.set(Some(msg.into()));
    }

    /// Ricostruisce `current.sets[ex_id]` mantenendo kg/reps inseriti e
    /// ricalcolando target/prev (porta `rebuildCurrentSets`).
    fn rebuild_current_sets(&self, ex_id: &str) {
        let giorni = self.giorni.get_untracked();
        let Some(e) = find_exercise(&giorni, ex_id).cloned() else {
            return;
        };
        let cur = self.current.get_untracked();
        if e.giorno != cur.giorno_id {
            return;
        }
        let logs = self.logs.get_untracked();
        let old = cur.sets.get(ex_id).cloned().unwrap_or_default();
        let arr: Vec<CurrentSet> = (0..e.serie as usize)
            .map(|k| {
                let o = old.get(k);
                CurrentSet {
                    kg: o.and_then(|x| x.kg),
                    reps: o.and_then(|x| x.reps),
                    target: target_for(&e, cur.sess_num, k),
                    prev: prev_kg(&logs, ex_id, k),
                }
            })
            .collect();
        self.current.update(|c| {
            c.sets.insert(ex_id.to_string(), arr);
        });
    }

    /// Salva o crea un esercizio (porta `saveExercise`).
    pub fn save_exercise(&self, s: EditState) {
        let nome = {
            let t = s.nome.trim();
            if t.is_empty() {
                "Esercizio".to_string()
            } else {
                t.to_string()
            }
        };
        if s.is_new {
            let is_b1 = s.blk == Block::B1;
            let obj = Exercise {
                id: s.id.clone(),
                blk: s.blk,
                grp: "Personalizzato".into(),
                nome,
                serie: if is_b1 { 4 } else { s.serie },
                tren: is_b1,
                target: (!is_b1).then(|| {
                    let t = s.target.trim();
                    if t.is_empty() {
                        "8-10".into()
                    } else {
                        t.to_string()
                    }
                }),
                base: 10.0,
                inc: 1.0,
                giorno: s.giorno_id,
            };
            self.giorni.update(|gs| {
                if let Some(g) = gs.iter_mut().find(|g| g.id == s.giorno_id) {
                    g.es.push(obj.clone());
                }
            });
            self.logs.update(|l| {
                l.insert(s.id.clone(), Vec::new());
            });
            self.rebuild_current_sets(&s.id);
        } else {
            let is_b1 = s.blk == Block::B1;
            self.giorni.update(|gs| {
                if let Some(e) = gs.iter_mut().flat_map(|g| &mut g.es).find(|e| e.id == s.id) {
                    e.nome = nome;
                    e.blk = s.blk;
                    if is_b1 {
                        e.tren = true;
                        e.target = None;
                        e.serie = 4;
                    } else {
                        e.tren = false;
                        e.target = Some({
                            let t = s.target.trim();
                            if t.is_empty() {
                                "8-10".into()
                            } else {
                                t.to_string()
                            }
                        });
                        e.serie = s.serie;
                    }
                }
            });
            self.rebuild_current_sets(&s.id);
        }
    }

    /// Elimina un esercizio (porta `deleteExercise`).
    pub fn delete_exercise(&self, id: &str) {
        self.giorni.update(|gs| {
            for g in gs.iter_mut() {
                g.es.retain(|e| e.id != id);
            }
        });
        self.logs.update(|l| {
            l.remove(id);
        });
        self.current.update(|c| {
            c.sets.remove(id);
        });
        if self.hist_ex.get_untracked() == id {
            let fallback = self
                .giorni
                .get_untracked()
                .get(1)
                .and_then(|g| g.es.first())
                .map(|e| e.id.clone())
                .unwrap_or_default();
            self.hist_ex.set(fallback);
        }
    }

    pub fn edit_day(&self, giorno_id: u8, focus: String, priorita: String) {
        self.giorni.update(|gs| {
            if let Some(g) = gs.iter_mut().find(|g| g.id == giorno_id) {
                let f = focus.trim();
                let p = priorita.trim();
                if !f.is_empty() {
                    g.focus = f.to_string();
                }
                if !p.is_empty() {
                    g.priorita = p.to_string();
                }
            }
        });
    }

    /// Salva la seduta nello storico (con data odierna) e avanza al giorno
    /// successivo della rotazione, calcolando la nuova sessione del Trenino.
    pub fn finish_workout(&self) {
        let date = today();
        let cur = self.current.get_untracked();
        let giorni = self.giorni.get_untracked();
        let Some(g) = giorni.iter().find(|x| x.id == cur.giorno_id).cloned() else {
            return;
        };

        // Registra, per ogni esercizio, le serie effettivamente compilate.
        self.logs.update(|logs| {
            for e in &g.es {
                let Some(sets) = cur.sets.get(&e.id) else {
                    continue;
                };
                let kg: Vec<f64> = sets.iter().filter_map(|s| s.kg).collect();
                if kg.is_empty() {
                    continue;
                }
                let reps: Vec<u32> = sets
                    .iter()
                    .filter(|s| s.kg.is_some())
                    .map(|s| s.reps.unwrap_or(0))
                    .collect();
                logs.entry(e.id.clone()).or_default().push(LogEntry {
                    kg,
                    reps,
                    name: e.nome.clone(),
                    date,
                });
            }
        });

        self.sessions.update(|s| {
            s.push(SessionRecord {
                giorno_id: cur.giorno_id,
                date,
            });
        });

        // Avanza: prossimo giorno nella rotazione 1→2→3→4→1, sessione = quante
        // volte quel giorno è stato svolto + 1 (per il ciclo Trenino).
        let next_g = (cur.giorno_id % 4) + 1;
        let next_sess = self
            .sessions
            .get_untracked()
            .iter()
            .filter(|r| r.giorno_id == next_g)
            .count() as u32
            + 1;
        let logs = self.logs.get_untracked();
        self.current
            .set(blank_current(&giorni, &logs, next_g, next_sess));

        self.show_toast(format!(
            "Giorno {} salvato · {}",
            cur.giorno_id,
            capitalize(&d_short(date))
        ));
        self.go(Screen::Home);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_roundtrip() {
        assert_eq!(TODAY.to_civil(), (2026, 6, 1));
        // 2026-06-01 è un lunedì -> weekday 1 (GG[1] = "lun")
        assert_eq!(TODAY.weekday(), 1);
        assert_eq!(d_short(TODAY), "lun 1 giu");
    }

    #[test]
    fn round_and_fmt() {
        assert_eq!(round_half(52.4), 52.5);
        assert_eq!(round_half(52.6), 52.5);
        assert_eq!(fmt(55.0), "55");
        assert_eq!(fmt(57.5), "57.5");
    }

    #[test]
    fn fresh_starts_day_one_empty() {
        let (giorni, logs, current, sessions) = fresh_state();
        assert_eq!(current.giorno_id, 1);
        assert_eq!(current.sess_num, 1);
        assert!(sessions.is_empty());
        assert!(logs.values().all(|v| v.is_empty()));
        // Trenino sessione 1 -> primo range della tabella
        assert_eq!(current.sets["g1a"][0].target, "8-10");
        assert!(find_exercise(&giorni, "g1a").is_some());
    }

    #[test]
    fn target_trenino_vs_fixed() {
        let g = seed_giorni();
        let panca = find_exercise(&g, "g1a").unwrap(); // B1 tren
        assert_eq!(target_for(panca, 1, 0), "8-10");
        assert_eq!(target_for(panca, 2, 0), "6-8");
        let croci = find_exercise(&g, "g1e").unwrap(); // B3 fixed
        assert_eq!(target_for(croci, 4, 1), "12-15");
        assert_eq!(range_top(&target_for(croci, 1, 0)), 15);
    }

    #[test]
    fn hist_uses_log_dates_and_diff() {
        let g = seed_giorni();
        let mut logs = empty_logs(&g);
        logs.insert(
            "g1a".into(),
            vec![
                LogEntry {
                    kg: vec![50.0, 50.0],
                    reps: vec![10, 10],
                    name: "Panca".into(),
                    date: Date::from_civil(2026, 5, 1),
                },
                LogEntry {
                    kg: vec![52.5, 52.5],
                    reps: vec![8, 10],
                    name: "Panca".into(),
                    date: Date::from_civil(2026, 5, 8),
                },
            ],
        );
        let current = blank_current(&g, &logs, 1, 1); // nessuna cella compilata -> nessun parziale
        let series = hist_series(&g, &logs, &current, "g1a");
        assert_eq!(series.len(), 2);
        assert_eq!(series[0].date, Date::from_civil(2026, 5, 1));
        assert_eq!(series[0].diff, None);
        assert_eq!(series[1].diff, Some(2.5));
    }
}
