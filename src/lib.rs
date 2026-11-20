#![allow(static_mut_refs)]

//! A Hiragana reference chart for Grimoire.
//!
//! The widget is deliberately static: it draws the gojūon, then the dakuten and
//! handakuten rows, and has no controls. Grimoire owns the frame, resizing and
//! closing; this module only uses the generic drawing ABI.

#[link(wasm_import_module = "grimoire")]
extern "C" {
    fn rect(x: i32, y: i32, w: i32, h: i32, style: i32);
    fn text(x: i32, y: i32, w: i32, h: i32, ptr: i32, len: i32, px: i32, center: i32);
    fn status(ptr: i32, len: i32);
}

type Cell = (&'static str, &'static str);

const BLANK: Cell = ("", "");

/// The gojūon, in the usual five-column order. Empty cells keep や and わ rows
/// aligned with the vowels above them; ん sits on its own final row.
const BASIC: [[Cell; 5]; 11] = [
    [("あ", "a"), ("い", "i"), ("う", "u"), ("え", "e"), ("お", "o")],
    [("か", "ka"), ("き", "ki"), ("く", "ku"), ("け", "ke"), ("こ", "ko")],
    [("さ", "sa"), ("し", "shi"), ("す", "su"), ("せ", "se"), ("そ", "so")],
    [("た", "ta"), ("ち", "chi"), ("つ", "tsu"), ("て", "te"), ("と", "to")],
    [("な", "na"), ("に", "ni"), ("ぬ", "nu"), ("ね", "ne"), ("の", "no")],
    [("は", "ha"), ("ひ", "hi"), ("ふ", "fu"), ("へ", "he"), ("ほ", "ho")],
    [("ま", "ma"), ("み", "mi"), ("む", "mu"), ("め", "me"), ("も", "mo")],
    [("や", "ya"), BLANK, ("ゆ", "yu"), BLANK, ("よ", "yo")],
    [("ら", "ra"), ("り", "ri"), ("る", "ru"), ("れ", "re"), ("ろ", "ro")],
    [("わ", "wa"), BLANK, BLANK, BLANK, ("を", "wo")],
    [BLANK, BLANK, ("ん", "n"), BLANK, BLANK],
];

/// Dakuten rows, followed by the single handakuten row.
const VOICED: [[Cell; 5]; 5] = [
    [("が", "ga"), ("ぎ", "gi"), ("ぐ", "gu"), ("げ", "ge"), ("ご", "go")],
    [("ざ", "za"), ("じ", "ji"), ("ず", "zu"), ("ぜ", "ze"), ("ぞ", "zo")],
    [("だ", "da"), ("ぢ", "ji"), ("づ", "zu"), ("で", "de"), ("ど", "do")],
    [("ば", "ba"), ("び", "bi"), ("ぶ", "bu"), ("べ", "be"), ("ぼ", "bo")],
    [("ぱ", "pa"), ("ぴ", "pi"), ("ぷ", "pu"), ("ぺ", "pe"), ("ぽ", "po")],
];

const COLUMNS: [&str; 5] = ["a", "i", "u", "e", "o"];
const STATUS: &str = "hiragana|basic=46|voiced=25";

fn txt(x: i32, y: i32, w: i32, h: i32, s: &str, px: i32, center: bool) {
    unsafe { text(x, y, w, h, s.as_ptr() as i32, s.len() as i32, px, center as i32) };
}

fn frame(x: i32, y: i32, w: i32, h: i32, style: i32) {
    unsafe { rect(x, y, w, h, style) };
}

fn draw_grid(rows: &[[Cell; 5]], x: i32, top: i32, cell_w: i32, cell_h: i32, gap: i32, s: f32) {
    let kana_px = ((cell_h as f32 * 0.49).clamp(10.0, 31.0 * s)) as i32;
    let roman_px = ((cell_h as f32 * 0.21).clamp(7.0, 12.0 * s)) as i32;
    for (row, cells) in rows.iter().enumerate() {
        for (col, (kana, romaji)) in cells.iter().enumerate() {
            if kana.is_empty() {
                continue;
            }
            let cx = x + col as i32 * (cell_w + gap);
            let cy = top + row as i32 * (cell_h + gap);
            frame(cx, cy, cell_w, cell_h, 0);
            txt(cx, cy + (cell_h as f32 * 0.04) as i32, cell_w, (cell_h as f32 * 0.63) as i32, kana, kana_px, true);
            txt(cx, cy + (cell_h as f32 * 0.59) as i32, cell_w, (cell_h as f32 * 0.29) as i32, romaji, roman_px, true);
        }
    }
}

fn draw(w: i32, h: i32, s: f32) {
    unsafe { status(STATUS.as_ptr() as i32, STATUS.len() as i32) };
    if w < 120 || h < 120 {
        return;
    }

    let pad = (14.0 * s) as i32;
    let header_h = (48.0 * s) as i32;
    let panel_title_h = (21.0 * s) as i32;
    let col_head_h = (17.0 * s) as i32;
    let footer_h = (18.0 * s) as i32;
    let row_gap = (3.0 * s) as i32;
    let col_gap = (4.0 * s) as i32;
    let panel_gap = (14.0 * s) as i32;

    let content_top = pad + header_h + (10.0 * s) as i32;
    let content_w = w - pad * 2;
    if content_w <= panel_gap + 20 {
        return;
    }
    let right_w = (content_w - panel_gap) / 2;
    let left_w = content_w - panel_gap - right_w;
    let left_x = pad;
    let right_x = pad + left_w + panel_gap;

    let grid_top = content_top + panel_title_h + col_head_h;
    let rows = BASIC.len() as i32;
    let grid_h = h - grid_top - pad - footer_h - row_gap * (rows - 1);
    let cell_h = (grid_h / rows).max((24.0 * s) as i32);
    let cell_w = (left_w - col_gap * 4) / 5;
    if cell_w <= 0 || cell_h <= 0 {
        return;
    }

    // A light filled header keeps the chart's name legible above the grid.
    frame(pad, pad, content_w, header_h, 1);
    txt(
        pad,
        pad,
        content_w,
        header_h,
        "ひらがな  ·  Hiragana",
        ((header_h as f32 * 0.46).clamp(12.0, 26.0 * s)) as i32,
        true,
    );

    txt(left_x, content_top, left_w, panel_title_h, "Basic · 46", (15.0 * s) as i32, false);
    txt(right_x, content_top, right_w, panel_title_h, "Dakuten · Handakuten", (15.0 * s) as i32, false);

    for (col, label) in COLUMNS.iter().enumerate() {
        let x = left_x + col as i32 * (cell_w + col_gap);
        txt(x, grid_top - col_head_h, cell_w, col_head_h, label, (10.0 * s) as i32, true);
        let x = right_x + col as i32 * (cell_w + col_gap);
        txt(x, grid_top - col_head_h, cell_w, col_head_h, label, (10.0 * s) as i32, true);
    }

    draw_grid(&BASIC, left_x, grid_top, cell_w, cell_h, row_gap, s);

    // The first four rows are dakuten; the last is handakuten and gets its own heading.
    let dakuten_rows = 4usize;
    let voiced_top = grid_top;
    draw_grid(&VOICED[..dakuten_rows], right_x, voiced_top, cell_w, cell_h, row_gap, s);

    let hand_title_y = voiced_top + dakuten_rows as i32 * (cell_h + row_gap) + (8.0 * s) as i32;
    txt(right_x, hand_title_y, right_w, panel_title_h, "Handakuten", (14.0 * s) as i32, false);
    draw_grid(
        &VOICED[dakuten_rows..],
        right_x,
        hand_title_y + panel_title_h,
        cell_w,
        cell_h,
        row_gap,
        s,
    );

    txt(
        right_x,
        hand_title_y + panel_title_h + cell_h + (18.0 * s) as i32,
        right_w,
        (18.0 * s) as i32,
        "゛ voices  ·  ゜ semi-voices",
        (11.0 * s) as i32,
        false,
    );

    txt(
        pad,
        h - pad - footer_h,
        content_w,
        footer_h,
        "46 basic  +  25 dakuten / handakuten",
        (10.0 * s) as i32,
        true,
    );
}

#[no_mangle]
pub extern "C" fn widget_draw(w: i32, h: i32, scale_milli: i32) {
    draw(w, h, scale_milli.max(500) as f32 / 1000.0);
}

#[no_mangle]
pub extern "C" fn widget_tap(_id: i32) {}
