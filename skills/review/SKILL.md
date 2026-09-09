---
name: review
description: Review OpenPhysics problem entries `problems/N/PROBLEM.md` for mathematical rigor, source faithfulness, XML-in-Markdown validity, duplicates, and proof/disproof clarity. Use when a subagent or reviewer is asked to audit `PROBLEM.md` files, compare entries against source papers, OQP listings, and other cited sources, or provide findings before accepting an open-problem entry.
---

# Review

## Principle

This is a read-only quality gate. Reviewers do not edit files, broaden scope, or
repair the entry. They report every mathematical or source-faithfulness issue
they find, with enough precision that the main agent can decide the fix.

Mathematical clarity outranks style. A problem entry passes only if an expert
can judge a proposed proof, disproof, gap, or progress claim from the entry
alone.

The rules to check are in `skills/add/references/checklist.md`. The file
format is in `docs/DESIGN.md`.

## Prompt Form

Use XML-delimited prompts for reviewer subagents: explicit scope, consistent
descriptive tags, nested inputs when useful, concise outcome-first
instructions, and no hidden filtering of low-severity findings.

```xml
<review>
  <task>Review an OpenPhysics problem entry for mathematical rigor.</task>
  <files>
    <file>problems/N/PROBLEM.md</file>
  </files>
  <sources>
    <source>DOI, URL, or arXiv id</source>
  </sources>
  <scope>Check source faithfulness, definitions, claim strength, kind and field, known results, proof/disproof logic, duplicates, XML validity, and grep keys.</scope>
  <out>Return findings first with file/line references. Use LaTeX notation for mathematics.</out>
</review>
```

Keep the prompt literal. Pass the file path, source path or DOI, and review
scope. Do not tell the reviewer the expected answer or proposed fix.

## Workflow

1. Read the assigned `PROBLEM.md`.
2. Read the cited primary source. Prefer the published DOI or paper over
   secondary pages. If an arXiv/source file differs from the published source,
   flag the difference.
3. Run `openphysics-cli check`. Check the Markdown heading, the problem id,
   and every tag in the order given in `docs/DESIGN.md`.
4. Check for duplicates. Run `openphysics-cli grep` on the entry's main keys.
   An existing entry with the same claim is `high`; an unlinked nearby variant
   is `medium`.
5. Verify that every symbol in the mathematical statement is locally defined,
   including domains, quantifiers, finiteness, asymptotics, equivalences,
   regularity assumptions, and inverse or normalization conventions.
6. Compare the exact strength of `<claim>` against the source. Flag stronger
   claims, weaker claims that change the problem, missing hypotheses, and
   conflated variants.
7. Check `<kind>` against the test in `docs/DESIGN.md` and `<field>` against
   its table.
8. Check `<known>` as provenance, not motivation. Separate proved facts,
   conjectures, numerical evidence, and nearby stronger or weaker variants.
9. Check `<ask>` for logical polarity. A requested proof must prove the exact
   `<claim>`; a requested disproof must negate that exact claim.
10. Check XML-in-Markdown validity: one root `<problem>`, stable tags, valid
    nesting, no raw `<`, `>`, or `&` in text unless they are tags or entities.
11. Go through every remaining item in `skills/add/references/checklist.md`.
12. Return findings ordered by severity. Include uncertainties instead of
    suppressing them.

## Severity

- `high`: wrong problem, wrong status, source mismatch, overclaim, missing
  hypothesis, false "open" status, invalid definition, duplicate of an existing
  entry, or proof/disproof target not matching `<claim>`.
- `medium`: ambiguous quantifier, undefined symbol, unverified equivalence,
  hidden convention, variant conflation, wrong kind or field, unlinked nearby
  variant, or provenance that could mislead a solver.
- `low`: citation cleanup, grep-key clarity, XML polish, naming drift, or
  wording that is clear but less precise than the source.

## Output

Use this shape:

```xml
<review_result>
  <findings>
    <finding severity="high|medium|low">
      <file>path:line</file>
      <issue>Precise problem.</issue>
      <source>What the source says or why the entry lacks support.</source>
      <fix>Minimal semantic correction.</fix>
    </finding>
  </findings>
  <checks>
    <check name="source">pass|fail|uncertain</check>
    <check name="claim">pass|fail|uncertain</check>
    <check name="kind">pass|fail|uncertain</check>
    <check name="defs">pass|fail|uncertain</check>
    <check name="ask">pass|fail|uncertain</check>
    <check name="duplicate">pass|fail|uncertain</check>
    <check name="xml">pass|fail|uncertain</check>
  </checks>
</review_result>
```

If there are no findings, say so explicitly and list any residual uncertainty,
such as a source not available to the reviewer.
