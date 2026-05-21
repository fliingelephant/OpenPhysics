---
name: add
description: Add rigorous OpenPhysics problem entries from source links or papers. Use when creating or revising `problems/<field>/<slug>/PROBLEM.md` entries such as `problems/QuantumInformation/<slug>/PROBLEM.md`, collecting open physics problems, converting source statements into XML-in-Markdown prompts, checking mathematical clarity, or running the required rigor review before accepting an entry.
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

## File

Create one file per problem:

```text
problems/<field>/<slug>/PROBLEM.md
```

For quantum information entries, use:

```text
problems/QuantumInformation/<slug>/PROBLEM.md
```

Do not put the problem id in the folder name. Put it in the Markdown heading and
in the XML:

````markdown
# QI001 Name

```xml
<problem id="QI001">
  <status>open</status>
  <name>Name</name>
  <keys>lowercase grep terms and common aliases</keys>

  <claim>
    ...
  </claim>

  <def>
    ...
  </def>

  <known>
    ...
  </known>

  <refs>
    <ref>...</ref>
  </refs>

  <ask>
    ...
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
````

Use LaTeX notation for mathematics. Inside XML text, avoid raw `<`, `>`, and
`&`; use LaTeX commands such as `\lt`, `\gt`, or XML entities. Keep the tag set
stable: `status`, `name`, `keys`, `claim`, `def`, `known`, `refs`, `ask`,
`out`.

## Workflow

1. Read the source links or papers before writing. Prefer primary sources.
2. Extract all nearby formulations: weak, strong, equivalent, conjectural,
   numerical, covariant, non-covariant, finite, asymptotic, or experimental.
3. Choose exactly one canonical `<claim>`. If sources disagree or the intended
   strength is ambiguous, ask the user before editing.
4. Write `<def>` locally. Define every symbol and distinguish common shorthand
   from the precise mathematical object.
5. Write `<known>` as provenance, not motivation. Separate proved facts from
   conjectures and stronger or weaker variants.
6. Write `<ask>` so proof and disproof are logically correct for the chosen
   claim. Do not use boilerplate like "give a counterexample" unless a single
   counterexample really disproves the claim.
7. Add grep-friendly `<keys>` with common names, aliases, abbreviations, and
   adjacent terminology. Do not include a keyword that falsely restricts the
   claim.
8. Check literal scope. Avoid words such as "any", "all", "standard", "known",
   "counterexample", "construct", and "solve" unless their mathematical scope is
   pinned down by the claim or definitions.
9. Check source naming. Do not write vague source phrases such as "the paper",
   "the published paper", "the PRX paper", or "the published PRX Quantum
   paper".
10. Run two independent read-only rigor reviews before accepting the entry when
   the active agent policy and user authorization permit subagents. Otherwise,
   ask for authorization before accepting, or clearly mark the entry as not yet
   independently reviewed. Apply only semantic precision fixes, then review
   again if the claim or definitions changed materially.

## Source Rules

Use full citations in `<refs>`. A `<ref>` should identify the authors, title,
journal or venue, volume, page or article number, year, DOI or stable URL, and
the exact problem, theorem, section, or page used.

When referring to a source in `<known>`, `<claim>`, or `<ask>`, name the source
explicitly. Prefer this form:

```xml
Chen et al., "Five open problems in quantum information theory", PRX Quantum 3,
010101 (2022), Problem 3, ask whether ...
```

Do not use source pronouns or narrative shortcuts such as "this work", "the
source", "the paper", "the authors", or "the published PRX paper" unless the
sentence also contains the full citation anchor needed to identify the source
without reading nearby text.

## Claim Rules

The `<claim>` is the source of truth.

- State one mathematical assertion, not a topic.
- Include all quantifiers and domains.
- Make finite, infinite, asymptotic, exact, approximate, numerical, and
  experimental meanings explicit.
- Keep motivation and history out of `<claim>`.
- Prefer a weaker precise claim over a stronger vague one.

For an existence claim over infinitely many dimensions, disproof usually means
showing that the set of admissible dimensions is finite. A failure in one
dimension only disproves an all-dimensions claim.

## Definition Rules

Define objects at the level where proof attempts operate. Common names are not
enough.

For example, for SIC-POVMs distinguish:

\[
|\psi_j\rangle,\qquad
\Pi_j=|\psi_j\rangle\langle\psi_j|,\qquad
E_j=\frac{1}{N}\Pi_j .
\]

If a property such as "informationally complete" is used, state the equivalent
linear-algebra condition when possible.

## Review Gate

Use two independent reviewer agents when the active agent policy and user
authorization permit subagents. Use the same effective model, reasoning,
sandbox, approval, and tool-access settings as the main agent; do not downgrade.
Give each reviewer only the file path, source path or DOI, and review task. Do
not tell them the expected answer or proposed fix. Reviewer agents should use
`skills/review/SKILL.md`.

Ask reviewers to check:

- exact strength of `<claim>` against sources
- local definitions and symbol hygiene
- stronger/weaker nearby variants
- proof and disproof logic in `<ask>`
- overclaims in `<known>`
- XML well-formedness and Markdown clarity
- grep-key quality
- full citations and explicit source naming
- literal-scope hazards such as "any", "standard", "known", and
  "counterexample"; vague source phrases such as "the paper" and "the
  published paper"

Use this reviewer prompt shape:

```xml
<review>
  <task>Review this OpenPhysics problem entry for mathematical rigor.</task>
  <file>problems/.../PROBLEM.md</file>
  <source>DOI, URL, local PDF, or local TeX path</source>
  <check>claim strength, definitions, variants, disproof logic, provenance, full citations, explicit source naming, XML parsing, grep keys, literal-scope hazards</check>
  <out>Findings first with file/line references, then concise suggested edits. Use LaTeX notation for mathematics.</out>
</review>
```

Do not add the next problem until the current entry passes this gate.
