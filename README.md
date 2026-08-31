# zrrp

A small CLI for Unreal Engine developers. Subcommands operate on the current working directory.

```
Usage: zrrp <command>

Commands:
  clean    Remove Unreal build artifact directories
           (Saved, Intermediate, Binaries, DerivedDataCache, .idea, .vs, .xcworkspace)
```

`clean` recursively deletes those directories from the current directory down. If no
`.uproject` file is found it prompts before doing anything.

## Install

### Windows

Download `zrrp.exe` from the [latest release](https://github.com/sadunkit/zrrp/releases/latest),
create an `Apps` directory under your user, put the exe there, and add that directory to your `PATH`.

### Mac

Build from source (below) and copy `zig-out/bin/zrrp` to `~/bin`, then run it as `zrrp`.

## Build from source

Requires Zig 0.16.0 or newer.

```bash
zig build --release=fast          # binary at zig-out/bin/zrrp
zig build run -- clean            # build and run
zig build test                    # run tests
```

Cross-compile a Windows binary from any host:

```bash
zig build -Dtarget=x86_64-windows -Doptimize=ReleaseFast
```

## Releases

CI (`.github/workflows/release.yml`) runs tests and cross-compiles a ReleaseFast
`x86_64-windows` binary on every push and pull request, uploading it as a build artifact.
Pushing a `v*` tag additionally publishes a GitHub release with `zrrp.exe` attached.

```bash
# bump .version in build.zig.zon to match, then:
git tag v0.1.0
git push origin v0.1.0
```
