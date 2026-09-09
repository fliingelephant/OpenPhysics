# Agent instructions

This repository is a registry of open physics problems. One entry is one
file, `problems/N/PROBLEM.md`. The format is specified in `docs/DESIGN.md`.
Read it before you write or edit an entry.

## Solving

- Solve only the `<claim>`. Do not reinterpret it.
- Return exactly one of the values listed in `<out>`, under the conditions
  in `<ask>`.
- Use `<def>` as the only source of definitions.

## Editing

- Use the `add` skill to create an entry and the `review` skill to check one.
- Take the id from `openphysics-cli next`. Never change or reuse an id.
- Never delete an entry. Change `<status>` instead.
- Keep the tag order in `docs/DESIGN.md`. Cite by DOI or arXiv id only.
- Run `openphysics-cli check` and `openphysics-cli index` before you commit.

## Tool

```sh
cargo build --release --manifest-path tools/openphysics-cli/Cargo.toml
tools/openphysics-cli/target/release/openphysics-cli --help
```
