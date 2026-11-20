# gri-hira

A Hiragana chart widget for [Grimoire](https://github.com/nebuff/grimoire-scribe).

## Install

In Grimoire:

1. Open **Settings → Packages → Widgets**.
2. Tap **Install from repository...**.
3. Enter `nebuff/gri-hira`.
4. Read the trust warning and continue.

Then open **Widgets** on the editor toolbar and choose **Hiragana Chart**.

## What it shows

The chart is a static reference sheet with two panels:

- **Basic · 46** — the full gojūon in five vowel columns, including the gaps in the
  や and わ rows and ん on its own final row.
- **Dakuten · Handakuten** — が/ざ/だ/ば rows with dakuten, and the ぱ row with
  handakuten, followed by a short ゛/゜ reminder.

Every cell shows the kana above its romanization. The widget has no controls: it
is drawn once when opened and does not use settings or saved state.

The Japanese glyphs come from Grimoire's built-in kana fallback, so the package
carries no font data.

## Build

The artifact is a WebAssembly module built with the generic Grimoire widget ABI.
From this repository:

```sh
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/gri_hira.wasm hiragana.bin
```

Then update `artifact.sha256` and `artifact.size` in `package.yaml`, for example
with Grimoire's `tools/widget-pack.sh`:

```sh
/path/to/grimoire-scribe/tools/widget-pack.sh . hiragana.bin
```

Bump `version` before publishing a changed build.

## Files

- `package.yaml` — the manifest Grimoire installs.
- `hiragana.bin` — the built WebAssembly artifact named by the manifest.
- `src/lib.rs` — the chart renderer; it imports only `rect`, `text` and `status`.
- `Cargo.toml`, `Cargo.lock` — the Rust build.
- `LICENSE` — MIT.
