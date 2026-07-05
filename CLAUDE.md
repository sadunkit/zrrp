# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`zrrp` is a small Rust CLI for Unreal Engine developers. Its subcommands operate on the current working directory: `clean` (removes Unreal build artifact folders like Saved/Intermediate/Binaries/DerivedDataCache), `clean-ddc`, `nuke -f <ext>` (recursively deletes files by extension), `pua [folder]` (counts `UPROPERTY(Config)` occurrences in `.h` files), and `p4-info` (shells out to `p4`).

## Commands

```bash
cargo build --release        # release binary at target/release/zrrp
cargo run -- <subcommand>    # e.g. cargo run -- clean
cargo test                   # note: see test caveat below
```

Users install by copying `target/release/zrrp` onto their PATH (`~/bin` on Mac, an `Apps` dir on Windows).

The `zigport/` directory is an in-progress Zig rewrite (currently mostly `zig init` scaffolding, untracked in git). It builds separately: `cd zigport && zig build` (run with `zig build run`, test with `zig build test`).

## Architecture

- `src/main.rs` — dispatches clap subcommand matches to the modules below.
- `src/cli/mod.rs` — all clap `Command`/arg definitions live here (`create_cli()`); add new subcommands in this file and handle them in `main.rs`.
- `src/unreal/` — Unreal-specific logic: `clean.rs` (directory cleanup, prompts before proceeding if no `.uproject` file is found) and `source_code.rs` (walks `.h` files with `walkdir`, returns a `FileStats` map).
- `src/utils/mod.rs` — generic recursive filesystem removal helpers and the y/n prompt, shared by the unreal module and `nuke`.
- `src/perforce/mod.rs` — wraps the external `p4` binary via `std::process::Command`.
- `src/app_logger.rs` — minimal `log` crate logger installed in `main`; code uses `log::error!`/`debug!` for diagnostics and plain `println!` for user-facing output.

## Caveats

- `src/utils/utils_tests.rs` is not declared as a module anywhere (`mod utils_tests;` is missing), so it is never compiled and `cargo test` runs nothing. Wire it in if you add or rely on tests.
- Destructive commands (`clean`, `nuke`) recursively delete from the current directory — be careful when testing them; use a scratch directory.
