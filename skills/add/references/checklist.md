# Checklist

Every item applies to the writer before the review gate and to each reviewer.
The file format itself is defined in `docs/DESIGN.md`; `openphysics-cli check`
enforces it.

## Duplicates

- Run `openphysics-cli grep` on each main term and alias. No existing entry
  states the same claim.
- A nearby variant that exists as an entry is listed in `<related>`, and the
  difference is stated in `<known>`.

## Claim

- One mathematical assertion, not a topic. No motivation or history.
- Every quantifier and domain is explicit: finite, infinite, asymptotic, exact,
  approximate, numerical, or experimental.
- The strength matches the source: not stronger, not weaker in a way that
  changes the problem, no missing hypothesis, no conflated variants. Prefer a
  weaker precise claim over a stronger vague one.
- An existence claim over infinitely many dimensions is disproved only by
  showing that the set of admissible dimensions is finite. A failure in one
  dimension does not disprove it.

## Kind and field

- `<kind>` follows the test in `docs/DESIGN.md`: a slot to fill is
  `construction`; a closed proposition is `proof`.
- `<field>` is the primary field from the table in `docs/DESIGN.md`.

## Definitions

- Every symbol in `<claim>` is defined in `<def>`, at the level where proof
  attempts operate: domains, quantifiers, finiteness, asymptotics,
  equivalences, regularity assumptions, inverse and normalization conventions.
- Common names are not enough. For SIC-POVMs distinguish
  \(|\psi_j\rangle\), \(\Pi_j=|\psi_j\rangle\langle\psi_j|\), and
  \(E_j=\Pi_j/N\).
- A named property such as "informationally complete" comes with its
  equivalent algebraic condition when one exists.

## Known

- Provenance, not motivation. Proved facts, conjectures, numerical evidence,
  and stronger or weaker variants are separated, and each names its source.
- No overclaim. No false `open` status.

## Refs and source naming

- Each `<ref>` gives authors, title, venue, volume, page or article number,
  year, DOI or arXiv id, and the exact problem, theorem, section, or page used.
  No PDFs in the repository.
- A source named in `<claim>`, `<known>`, or `<ask>` carries a full citation
  anchor, for example: Chen et al., "Five open problems in quantum information
  theory", PRX Quantum 3, 010101 (2022), Problem 3. Never "the paper", "the
  source", "the authors", "this work", or "the seed article".
- Prefer the published version. If the arXiv version differs, say so.

## Ask

- The proof condition proves exactly `<claim>`. The disproof condition negates
  exactly `<claim>`.
- No boilerplate such as "give a counterexample" unless a single counterexample
  really disproves the claim.

## Keys

- Common names, aliases, abbreviations, and adjacent terminology, lowercase.
  No ids. No key that falsely restricts the claim.

## Wording

- "any", "all", "standard", "known", "counterexample", "construct", and
  "solve" appear only where the claim or definitions pin down their scope.

## Markdown and XML

- `openphysics-cli check` passes: heading, id, tag order, enum values, one
  root, text-only tags, no raw `<`, `>`, or `&`.
- Mathematics is LaTeX.
