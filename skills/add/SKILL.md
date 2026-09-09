---
name: add
description: Add an OpenPhysics problem entry `problems/N/PROBLEM.md` from a source link, DOI, or paper, or derive a construction entry from an existing proof entry. Use when collecting open physics problems, converting source statements into the registry format, or running the review gate before accepting an entry.
---

# Add

## Principle

Treat mathematical rigorousness as the acceptance criterion. Do not record an
open problem until the claim, definitions, quantifiers, variants, provenance,
and disproof condition are clear enough that an expert can judge a proposed
solution from the entry alone.

Use XML-in-Markdown as the canonical prompt form. Keep tags minimal and
semantic. Do not embed long reasoning procedures in each problem; define the
outcome, constraints, evidence, and output contract clearly, then let the model
choose the proof or search strategy.

## Arguments

- `/add <url-or-doi>`: write a new entry from the source.
- `/add construction-variant <N>`: write a new construction entry derived from
  existing entry N. See below.

## Format

The entry format is defined in `docs/DESIGN.md`. Follow it exactly;
`openphysics-cli check` enforces it. The acceptance rules are in
`references/checklist.md`.

## Workflow

1. Read the source links or papers before writing. Prefer primary sources.
2. Check for duplicates. Run `openphysics-cli grep` on each main term and
   alias. If an entry states the same claim, stop and report it. If a nearby
   variant exists, plan a `<related>` link.
3. Extract all nearby formulations: weak, strong, equivalent, conjectural,
   numerical, covariant, non-covariant, finite, asymptotic, or experimental.
4. Choose exactly one canonical `<claim>`. If sources disagree or the intended
   strength is ambiguous, ask the user before editing.
5. Set `<kind>` by the test in `docs/DESIGN.md` and `<field>` from its table.
6. Write `<def>`, `<known>`, `<refs>`, `<ask>`, `<keys>`, and `<related>`
   under the rules in `references/checklist.md`.
7. Take the id from `openphysics-cli next` and create `problems/N/PROBLEM.md`.
8. Run `openphysics-cli check`, then go through `references/checklist.md`
   yourself.
9. Pass the review gate below. Apply only semantic precision fixes, then
   review again if the claim or definitions changed materially.
10. Run `openphysics-cli index`.

## Construction variant

`/add construction-variant <N>` derives a construction entry from a proof
entry.

1. Read entry N with `openphysics-cli show N`. It must have
   `<kind>proof</kind>`.
2. Identify the object that the claim implies or requires: an achiever of a
   bound, a witness of an inequality, or a state, protocol, or Hamiltonian
   whose existence the claim asserts inside its argument.
3. Write the new `<claim>` as an existence statement for that object with its
   properties explicit, so that a positive answer exhibits the object.
4. Set `<kind>construction</kind>`, the same `<field>`, `<related>N</related>`,
   and a `<known>` that states what entry N asserts and its status.
5. Continue from step 6 of the workflow. Do not edit entry N.

## Review gate

Use two independent reviewer agents when the active agent policy and user
authorization permit subagents. Use the same effective model, reasoning,
sandbox, approval, and tool-access settings as the main agent; do not downgrade.
Give each reviewer only the file path, source path or DOI, and review task. Do
not tell them the expected answer or proposed fix. Reviewer agents use
`skills/review/SKILL.md` and check every item in `references/checklist.md`.

If subagents are not permitted, ask for authorization before accepting, or
clearly mark the entry as not yet independently reviewed.

Use this reviewer prompt shape:

```xml
<review>
  <task>Review this OpenPhysics problem entry for mathematical rigor.</task>
  <file>problems/N/PROBLEM.md</file>
  <source>DOI, URL, or arXiv id</source>
  <check>every item in skills/add/references/checklist.md</check>
  <out>Findings first with file/line references, then concise suggested edits. Use LaTeX notation for mathematics.</out>
</review>
```

Do not add the next problem until the current entry passes this gate.
