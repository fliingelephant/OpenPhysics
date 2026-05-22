---
name: review
description: Review OpenPhysics problem entries for mathematical rigor, source faithfulness, XML-in-Markdown validity, and proof/disproof clarity. Use when a subagent or reviewer is asked to audit `PROBLEM.md` files, compare entries against source papers, OQP listings, and other cited sources, or provide findings before accepting an open-problem entry.
---

# Review

## Principle

This is a read-only quality gate. Reviewers do not edit files, broaden scope, or
repair the entry. They report every mathematical or source-faithfulness issue
they find, with enough precision that the main agent can decide the fix.

Mathematical clarity outranks style. A problem entry passes only if an expert
can judge a proposed proof, disproof, gap, or progress claim from the entry
alone.

## Prompt Form

Use XML-delimited prompts for reviewer subagents. This follows the current
Claude Opus 4.7 and GPT-5.5 prompting guidance: explicit scope, consistent
descriptive tags, nested inputs when useful, concise outcome-first instructions,
and no hidden filtering of low-severity findings.

```xml
<review>
  <task>Review an OpenPhysics problem entry for mathematical rigor.</task>
  <files>
    <file>problems/.../PROBLEM.md</file>
  </files>
  <sources>
    <source>DOI, URL, local PDF, or local TeX path</source>
  </sources>
  <scope>Check source faithfulness, definitions, claim strength, known results, proof/disproof logic, XML validity, and grep keys.</scope>
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
3. Check the Markdown heading and XML problem id, `status`, `name`, `keys`,
   `claim`, `def`, `known`, `refs`, `ask`, and `out`.
4. Verify that every symbol in the mathematical statement is locally defined,
   including domains, quantifiers, finiteness, asymptotics, equivalences,
   regularity assumptions, and inverse or normalization conventions.
5. Compare the exact strength of `<claim>` against the source. Flag stronger
   claims, weaker claims that change the problem, missing hypotheses, and
   conflated variants.
6. Check `<known>` as provenance, not motivation. Separate proved facts,
   conjectures, numerical evidence, and nearby stronger or weaker variants.
7. Check `<ask>` for logical polarity. A requested proof must prove the exact
   `<claim>`; a requested disproof must negate that exact claim.
8. Check XML-in-Markdown validity: one root `<problem>`, stable tags, valid
   nesting, no raw `<`, `>`, or `&` in text unless they are tags or entities.
9. Return findings ordered by severity. Include uncertainties instead of
   suppressing them.

## Severity

- `high`: wrong problem, wrong status, source mismatch, overclaim, missing
  hypothesis, false "open" status, invalid definition, or proof/disproof target
  not matching `<claim>`.
- `medium`: ambiguous quantifier, undefined symbol, unverified equivalence,
  hidden convention, variant conflation, or provenance that could mislead a
  solver.
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
    <check name="defs">pass|fail|uncertain</check>
    <check name="ask">pass|fail|uncertain</check>
    <check name="xml">pass|fail|uncertain</check>
  </checks>
</review_result>
```

If there are no findings, say so explicitly and list any residual uncertainty,
such as a source not available to the reviewer.
