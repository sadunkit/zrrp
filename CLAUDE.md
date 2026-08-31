# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`zrrp` is a small Zig CLI for Unreal Engine developers. Its subcommands operate on the current working directory. Currently implemented: `clean`, which recursively removes Unreal build artifact directories (`Saved`, `Intermediate`, `Binaries`, `DerivedDataCache`, `.idea`, `.vs`, and `*.xcworkspace`), prompting first if no `.uproject` file is found. The project is a rewrite of an earlier Rust version (see git history); the remaining Rust subcommands (`clean-ddc`, `nuke`, `pua`, `p4-info`) have not been ported yet.

## Commands

```bash
zig build                    # binary at zig-out/bin/zrrp
zig build run -- <command>   # e.g. zig build run -- clean
zig build test               # runs test blocks from all modules (wired via src/root.zig)
zig build --release=fast     # release build
zig build -Dtarget=x86_64-windows -Doptimize=ReleaseFast   # cross-compile zrrp.exe
```

Requires Zig 0.16.0+ (see `minimum_zig_version` in `build.zig.zon`).

Users install by copying `zig-out/bin/zrrp` onto their PATH (`~/bin` on Mac, an `Apps` dir on Windows).

## Architecture

- `src/main.zig` — executable entry point; parses argv by hand and dispatches to the `zrrp` module. Add new subcommands here.
- `src/root.zig` — module root; re-exports `unreal` and `utils` and references them for tests.
- `src/unreal.zig` — the `clean` command. Runs two threads: one deletes only `Intermediate` directories, the other handles the rest; each skips the other's targets so their subtrees never overlap.
- `src/utils.zig` — recursive directory removal (`removeUnwantedDirectories`), the y/n prompt, and a mutex-guarded `Printer` that serializes output from the cleaning threads. Target-list entries starting with `.` also match as name suffixes (so `.xcworkspace` matches `MyProject (Mac).xcworkspace`).

## CI and releases

`.github/workflows/release.yml` runs on pushes to `main`, PRs, `v*` tags, and manual dispatch:

- **test** — `zig build test` on `ubuntu-latest`.
- **build-windows** — cross-compiles `-Dtarget=x86_64-windows -Doptimize=ReleaseFast` on
  `ubuntu-latest` (Zig needs no toolchain to target Windows) and uploads `zig-out/bin/zrrp.exe`
  as an artifact. The build also emits a `zrrp.pdb`, which is not uploaded.
- **release** — `v*` tags only; downloads the artifact and runs `gh release create` with
  `--generate-notes`, attaching `zrrp.exe`.

Zig is pinned to `0.16.0` in the workflow via `mlugg/setup-zig`. Keep that in sync with
`minimum_zig_version` in `build.zig.zon` when bumping.

Cutting a release: bump `.version` in `build.zig.zon`, commit, then push a matching `v*` tag.
Nothing verifies that the tag and `.version` agree, so update both.

## Caveats

- `clean` recursively deletes from the current directory — be careful when testing it; use a scratch directory.
