# Problems

Current fields:

```text
QuantumComputation/
QuantumInformation/
QuantumThermodynamics/
```

Fields are broad physics-area buckets, not a verbatim copy of source-site
taxonomies. OQP's finer categories such as Entanglement theory, Quantum
communication, Quantum foundations, Quantum cryptography, and Quantum
thermodynamics remain provenance metadata inside each entry.

Use `QuantumInformation/` for entanglement, Bell/nonlocality, communication,
cryptography, tomography, metrology, and Gibbs-state structure and
thermalization of many-body systems (quantum Markov properties, Gibbs
samplers, clustering of correlations, thermal area laws).
Use `QuantumComputation/` for circuits, error models, proof systems,
complexity-of-quantum-tasks, and code-like AME/QECC problems, including
Hamiltonian-complexity thermal problems such as thermal NLTS.
Use `QuantumThermodynamics/` for resource-theoretic thermodynamics (thermal
operations, catalysis), thermal machines, batteries and clocks, Landauer and
cooling limits, and relaxation criteria.

Problem ids are keyed to their source collection and are never reused.
`QT046`-style ids come from the IQOQI OQP list. `QTD001`–`QTD039` come from
the 2026 quantum-thermodynamics survey
(`Quantum-Thermodynamics-Open-Problems-Survey.md`) and are numbered by survey
entry; a number stays reserved even if its lead is rejected. Existing entries
stay where they are: `QT046` predates the `QuantumThermodynamics/` field and
remains in `QuantumInformation/`.

`QI002` is intentionally omitted as a separate entry. In Horodecki, Rudnicki,
and Zyczkowski's 2022 source list, Problem 2 is the \(d=6\) mutually unbiased
bases problem; this is covered here by `QC013`, the broader OQP Problem 13
asking for the maximal number of mutually unbiased bases in every finite
dimension, with \(d=6\) as the highlighted special case.
