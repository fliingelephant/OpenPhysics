# OpenPhysics

A curated, machine-readable registry of open physics problems, for agentic reasoning systems.

## Format

Each problem is one `PROBLEM.md` file:

```text
problems/<Field>/<slug>/PROBLEM.md
```

The problem statement is XML in Markdown. The `<claim>` is the source of truth.

## Standard

Each entry should have:

- one precise `<claim>`
- local `<def>` definitions
- source-faithful `<known>` results
- complete `<refs>`
- exact proof/disproof conditions in `<ask>`
- reviewer checks for mathematical clarity

## Layout

```text
problems/   problem entries
refs/       source papers and notes
skills/     agent workflows
```

## Use

Agents should solve only the stated `<claim>` and return one of `proof`,
`disproof`, `gap`, or `progress`.
