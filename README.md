# OpenPhysics

A registry of open physics problems, written for AI agents.

Each problem is one file, `problems/N/PROBLEM.md`, with an XML body inside
Markdown. The `<claim>` is the statement to prove or disprove. The format
is specified in [docs/DESIGN.md](docs/DESIGN.md). The index of all entries
is [problems/README.md](problems/README.md).

## Use

An agent takes one entry, solves only its `<claim>`, and returns one of
`proof`, `disproof`, `gap`, or `progress`, as `<ask>` and `<out>` define.

## Tool

`openphysics-cli` lists, shows, searches, and validates entries.

```sh
cargo build --release --manifest-path tools/openphysics-cli/Cargo.toml
tools/openphysics-cli/target/release/openphysics-cli list --field QTD
tools/openphysics-cli/target/release/openphysics-cli show 50 claim
tools/openphysics-cli/target/release/openphysics-cli check
```

## Contribute

Use the `add` skill to write an entry and the `review` skill to check it.
Ids are integers, minted at merge, never reused. Run `check` before a pull
request.

## License

MIT.
