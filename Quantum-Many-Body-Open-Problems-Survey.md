# Open Problems in Quantum Many-Body Physics — A Survey

A curated list of **genuinely-open, well-defined** problems across quantum many-body physics. Each entry gives keywords, a one-line statement of the precise question, a status tag, and source URLs. Compiled June 2026 from nine parallel literature-search workers (three rounds), then curated in **two independent review rounds** (parallel review agents) that removed solved results and questions lacking any precise formulation, rescoped mis-stated entries to their open frontier, and trimmed solved sub-claims. Every URL is taken from a primary arXiv/DOI source.

**Scope.** Only problems that can be stated as a single mathematical claim about a model, with a definite truth value (a bit of vagueness allowed). Pure research-program questions with no formulable claim, problems already settled, and questions about specific real materials or experimental outcomes have all been removed.

**Status legend.**
- **[OPEN]** — genuinely unresolved
- **[PARTIAL]** — proven in special cases / low dimension; the general case (stated in the entry) is open
- **[CONTESTED]** — a precise claim whose truth value is disputed, or where the community is split
- **[PHYS]** — sharp claim, established at physics rigor, but no mathematical proof
- **[RESOLVED]** — appears only inline, as context that *delimits* an open problem (never as a standalone entry)

> **Rigor caveat.** Numerical evidence (DMRG, QMC, tensor networks, exact diagonalization) is **not** a proof. A few cited sources postdate the Jan-2026 knowledge cutoff or are preprints; these are flagged in **Verification caveats** at the end.

**Flagship open problems:** Haldane gap (§1.1) · 2D repulsive-Hubbard superconductivity (§2.1) · Area law in D≥2 (§4.1) · Quantum PCP conjecture (§5.1) · Strong ETH for a fixed local Hamiltonian (§7.11).

---

## §1 Quantum spin systems & lattice models

**1.1 Haldane gap** — `Haldane conjecture, integer-spin chain, spectral gap, AKLT`. Does the isotropic spin-1 antiferromagnetic Heisenberg chain have a strictly positive spectral gap above a unique ground state in the thermodynamic limit? **[OPEN]** (proven only at the AKLT point, which adds a biquadratic term; half-integer chains are gaplessness-proven by LSM/Affleck–Lieb).
- https://arxiv.org/abs/2606.03836 (2026 certified *upper* bounds; confirms still open) · AKLT: https://doi.org/10.1007/BF01218021

**1.2 Square-lattice AKLT gap** — `AKLT, 2D, spectral gap, frustration-free`. Is the spin-2 AKLT model on the 2D square lattice gapped in the thermodynamic limit? **[OPEN]** (proven on the hexagonal and decorated lattices).
- https://arxiv.org/abs/1910.11810 (hexagonal, proven) · https://arxiv.org/abs/1905.01275 (decorated)

**1.3 Néel order, 2D S=1/2 Heisenberg AFM at T=0** — `Néel long-range order, staggered magnetization, reflection positivity, infrared bound`. Does the spin-1/2 nearest-neighbor Heisenberg antiferromagnet on the square lattice have antiferromagnetic long-range order at T=0? **[OPEN]** for S=1/2 in 2D (proven for S≥1 in 2D and all S≥1/2 in d≥3; Mermin–Wagner forbids any T>0 order in 2D, so the open case is strictly T=0).
- Dyson–Lieb–Simon: https://doi.org/10.1007/BF01106729 · Kennedy–Lieb–Shastry: https://doi.org/10.1007/BF01023854

**1.4 Ferromagnetic Ordering of Energy Levels (FOEL) — graph characterization** — `Lieb–Mattis, total spin sectors, E_min(S) monotonicity, graph topology`. For which graphs does the ferromagnetic Heisenberg model have its lowest energy in each total-spin sector monotone decreasing in S? **[PARTIAL]** (proven for XXX/XXZ open chains; *provably false* on some spin-1/2 rings — so the blanket general-graph statement is disproven; a graph-theoretic characterization of where FOEL holds is open).
- chains (proven): https://arxiv.org/abs/math-ph/0308006 · https://arxiv.org/abs/math-ph/0503056 · ring counterexamples: https://arxiv.org/abs/1107.1266

**1.5 Lieb–Schultz–Mattis–Hastings — optimal & higher-D form** — `LSM, ingappability, higher dimensions, twisted boundary, filling, anomaly`. Can the higher-D LSM excitation bound (currently ≲ log L / L) be improved to the optimal C/L, and is there a fully general rigorous d≥2 filling/anomaly theorem for arbitrary continuous symmetry and space group? **[PARTIAL]** (1D and finite-volume d≥2 proven; the general continuous-symmetry + crystalline d≥2 case is incomplete). Cross-ref §3.10 (anomaly form).
- Nachtergaele–Sims: https://arxiv.org/abs/math-ph/0608046 · Ogata–Tachikawa–Tasaki: https://arxiv.org/abs/2004.06458

**1.6 Lee–Yang property for classical rotors, D≥4** — `Lee–Yang zeros, circle theorem, classical rotor, Heisenberg model, Lieb–Sokal, Simon conjecture`. For the isotropic D-component classical rotor with D≥4, are all magnetic-field partition-function zeros purely imaginary (Simon's conjecture)? **[PARTIAL]** (Ising/D=1, and the D=2,3 rotors — the latter being the classical Heisenberg model — were proven by Dunlop–Newman **1975**; even D proven on ℤ by a 2026 method; all D≥4 on ℤ^d for d≥2, and odd D≥5, remain open).
- Dunlop–Newman (D=2,3): https://doi.org/10.1007/BF01609827 · Lieb–Sokal: https://doi.org/10.1007/BF01213009 · even-D on ℤ (2026, verify): https://arxiv.org/abs/2603.18675

**1.7 Lattice-homotopy conjecture for LSM constraints** — `lattice homotopy, space group, projective representation, spin liquid obstruction`. Do lattice-homotopy classes exactly classify all LSM-type constraints for arbitrary space and symmetry groups? **[OPEN]** (proven only in restricted 2D/abelian settings).
- https://arxiv.org/abs/1703.06882

---

## §2 Lattice fermions / Hubbard model

**2.1 Superconductivity (ODLRO) in the 2D repulsive Hubbard model** — `off-diagonal long-range order, pairing, d-wave, cuprates, eta-pairing`. Does the ground state of the U>0 single-band Hubbard model on the 2D square lattice exhibit off-diagonal long-range order in the pairing density matrix? **[OPEN]** — the central cuprate question; ODLRO is rigorously proven for η-pairing eigenstates and the attractive model, not the repulsive ground state. No-T>0 SC LRO in 2D (Hohenberg/Mermin–Wagner), so only T=0 ODLRO is possible.
- Lieb's open-problem list: https://arxiv.org/abs/cond-mat/9311033 · constraint (Tasaki–Watanabe): https://arxiv.org/abs/2105.10692

**2.2 Nagaoka ferromagnetism at finite hole density** — `Nagaoka, infinite-U Hubbard, saturated ferromagnetism, thermodynamic limit, single hole`. Does saturated ferromagnetism in the U=∞ Hubbard model survive a finite (extensive) density of holes in the thermodynamic limit? **[OPEN]** (proven for exactly one hole, and sub-extensive hole number; ≥2 holes already non-saturating on large tori; the often-quoted δ≈0.25–0.41 thresholds are variational, not rigorous).
- Tasaki review: https://arxiv.org/abs/cond-mat/9712219

**2.3 Flat-band ferromagnetism — extensions** — `Mielke, Tasaki, flat band, nearly-flat band, finite-temperature magnetization`. Does flat-band ferromagnetism persist (i) for generic broad-band dispersion far from flatness, and (ii) as a finite-T transition in d≥3? **[PARTIAL]** (T=0 core proven incl. nearly-flat bands; both extensions open; Mermin–Wagner forbids d≤2 so d≥3 is the finite-T target).
- https://arxiv.org/abs/1901.02617 · SU(n): https://arxiv.org/abs/2009.03580

**2.4 Positive-T long-range order in the standard Hubbard model** — `reflection positivity, Néel order, finite temperature, cubic lattice d≥3`. Is there a rigorous proof of a positive-temperature phase transition with LRO (Néel for U>0; s-wave SC/CDW for U<0) in the standard half-filled Hubbard model on the cubic lattice, d≥3? **[OPEN]** in every dimension. *Premise correction:* DLS/reflection-positivity is for quantum spin systems; standard Hubbard hopping is generically not reflection-positive, so it does not transfer.
- https://arxiv.org/abs/cond-mat/9311033 · Weyl-dispersion surrogate (2025): https://arxiv.org/abs/2508.12421

**2.5 Rigorous Mott metal–insulator transition** — `Mott transition, half-filling, metal–insulator, d≥2, Falicov–Kimball`. Is there any rigorous theorem proving a metal-to-Mott-insulator transition in U at half-filling for any lattice fermion model in d≥2? **[OPEN]** (rigorous control only at d=1, where Lieb–Wu gives a gap for all U>0, and d=∞ DMFT where the transition is numerical).
- Lieb–Wu: https://doi.org/10.1103/PhysRevLett.20.1445 · honeycomb semimetal stability: https://arxiv.org/abs/0811.1881

**2.6 Sign-problem curability for specific physical models** — `sign problem, stoquastic, basis change, Hubbard, design principle`. Although a universal cure is NP-hard, does the doped 2D Hubbard model admit a poly-time sign-problem-free reformulation under broad (non-single-qubit) transformations? **[OPEN]** (only on-site/single-qubit curing is classified; Majorana-positivity design principles cure specific other models).
- design principles: https://arxiv.org/abs/1601.01994

---

## §3 Topological order & phases (rigorous)

**3.1 Stability of topological order beyond frustration-free** — `Bravyi–Hastings–Michalakis, local topological order, gap stability, commuting projectors`. Can the frustration-free / commuting-projector hypothesis in BHM stability be removed — is the gap + degeneracy-splitting of a *general* gapped topologically ordered Hamiltonian stable under arbitrary small local perturbations? **[OPEN]** (proven only under LTQO + frustration-freeness; all known uniform-gap methods are themselves restricted to frustration-free systems).
- https://arxiv.org/abs/1001.0344 · https://arxiv.org/abs/2102.07209

**3.2 Stability of the full anyon (superselection) structure** — `anyons, braiding, fusion, superselection sectors, perturbation`. Beyond gap stability, is the braiding/fusion data of a 2D topologically ordered ground state stable under small local perturbations? **[OPEN]** (proven for abelian quantum-double models, where the category can be constructed; general case open).
- https://arxiv.org/abs/1804.03203

**3.3 Rigorous classification of gapped/SPT phases in D≥2** — `Ogata index, finite-depth circuit, quasi-adiabatic, H^3(G,T) completeness`. Is Ogata's 2D SPT index a *complete* invariant, and is there a general rigorous classification of gapped phases in d≥2 under finite-depth circuits? **[OPEN]** in d≥2 (1D fully classified; 2D has a rigorous invariant but completeness/realizability open).
- 1D complete: https://arxiv.org/abs/2012.15491 · 2D index: https://arxiv.org/abs/2101.00426

**3.4 Emergence of (nonabelian) anyons / MTC from a generic gapped Hamiltonian** — `modular tensor category, approximate Haag duality, quantum double, string-net`. Do the anyon sectors of every gapped 2D lattice Hamiltonian form a modular tensor category, and can a nonabelian MTC be derived from a microscopic non-commuting-projector Hamiltonian? **[OPEN]** (rigorous only for exactly-solvable quantum-double / weak-Hopf tensor-network models, or *conditionally* on an assumed approximate Haag duality — whether that follows from the gap is itself the crux).
- conditional: https://arxiv.org/abs/2106.15741 · solvable: https://arxiv.org/abs/2503.15611

**3.5 Fractional quantum Hall effect from a microscopic Hamiltonian** — `FQHE, spectral gap, fractional Hall conductance, bulk-edge, Haldane pseudopotential`. Derive, from a concrete interacting lattice Hamiltonian, a spectral gap above a p-fold-degenerate sector *and* σ_H=(p/q)e²/h; and prove the many-body bulk–edge correspondence beyond the single-chiral-mode case. **[OPEN]** (existing results are conditional index theorems assuming topological order; even gapping the Haldane pseudopotential is unsolved; multi-mode/fractional bulk-edge open).
- conditional index: https://arxiv.org/abs/2001.06458 · bulk-edge (single mode): https://arxiv.org/abs/1708.08517

**3.6 Chiral phases beyond commuting-projector models** — `chiral central charge, commuting projectors, finite-depth circuit, obstruction`. Can any exactly-solvable / finite-depth-preparable *local* Hamiltonian realize a gapped phase with nonzero chiral central charge c₋ (given the no-go that commuting-projector and strict-area-law models force c₋=0)? **[OPEN]**.
- no-go: https://arxiv.org/abs/1810.07756 · strict-area-law no-go (2024): https://arxiv.org/abs/2408.10306

**3.7 QCA classification in 3D (Witt-group conjecture)** — `quantum cellular automata, GNVW index, finite-depth circuit, three-fermion, Witt class`. Is every 3D QCA, modulo finite-depth circuits and shifts, a product of known generators, with classes corresponding to Witt classes of (super)modular categories? **[OPEN]** in 3D (1D/2D classified; nontrivial 3D QCA exists but the general, non-Clifford completeness is conjectural).
- https://arxiv.org/abs/1812.01625 · https://arxiv.org/abs/1907.02075

**3.8 (2+1)D topological order = MTC, and MTC realizability** — `modular tensor category, (MTC,c), Levin–Wen, chiral obstruction`. Is every (2+1)D bosonic topological order described by (unitary MTC, chiral central charge c), and is every unitary MTC realized by a commuting-projector model? **[PHYS]/[PARTIAL]** (non-chiral c=0 realized by string-nets; chiral c≠0 case open and believed obstructed for commuting projectors).
- https://arxiv.org/abs/1602.05936 · string-net: https://doi.org/10.1103/PhysRevB.71.045110 · math survey: https://arxiv.org/abs/1705.06206

**3.9 SPT phases beyond cohomology; completeness of cobordism** — `beyond-cohomology SPT, cobordism, Freed–Hopkins, invertible phases`. Is the cobordism classification of invertible/SPT phases complete and rigorous for general interacting lattice systems? **[PARTIAL]/[PHYS]** (Freed–Hopkins is a rigorous theorem for axiomatic *field theories*; completeness for interacting lattice fermions in all dimensions is open).
- https://arxiv.org/abs/1604.06527 · beyond-cohomology: https://arxiv.org/abs/1209.3058

**3.10 LSM as a lattice 't Hooft anomaly** — `LSM-anomaly, 't Hooft anomaly matching, flux insertion, Ogata index`. Is there a general rigorous theorem realizing the LSM constraint as an anomaly of lattice symmetry matching a one-higher-dimensional SPT? **[PARTIAL]** (rigorous operator-algebraic results for chains; general-dimension anomaly theorem is physics-level). Cross-ref §1.5.
- https://arxiv.org/abs/1907.08204 · rigorous chain index: https://arxiv.org/abs/2004.06458

---

## §4 Entanglement structure & ground-state simulability

**4.1 Area law in D≥2 for gapped ground states** — `entanglement entropy, area law, gapped, finite correlation length`. Does a constant spectral gap above a unique ground state of a local Hamiltonian in d≥2 imply S(A) ≤ O(|∂A|)? **[OPEN]** — the flagship entanglement conjecture (1D proven; 2D only under frustration-free + *local* gap).
- 1D (Hastings): https://arxiv.org/abs/0705.2024 · 2D frustration-free: https://arxiv.org/abs/2103.02492 · "it's open" colloquium: https://arxiv.org/abs/0808.3773

**4.2 Area law for gapped 1D Hamiltonians — degenerate ground space / nonlocal regime** — `area-law violation, ground-space degeneracy, Movassagh–Shor, geometric locality`. Hastings' 1D area law covers a unique (or O(1)-fold degenerate) ground state above a constant gap; does it still hold for a poly(n)-fold degenerate ground space, and is geometric locality essential? **[OPEN]** (the unique / O(1)-degenerate geometrically-local case is *closed*; a constant-gap area-law violation provably exists for *non-geometrically-local* interactions; all known √n/volume-law counterexamples — Movassagh–Shor, Fredkin, Motzkin — are gapless with gap ∼1/n^c).
- Hastings: https://arxiv.org/abs/0705.2024 · O(1)-degeneracy: https://arxiv.org/abs/1403.0327 · gapless counterexample: https://arxiv.org/abs/1408.1657 · nonlocal constant-gap: https://arxiv.org/abs/1410.0951

**4.3 Efficient PEPS representation of 2D gapped ground states** — `PEPS, bond dimension, tensor network, efficient description`. Does a 2D area law imply a poly-bond-dimension PEPS approximation with an efficient algorithm for local observables? **[OPEN]** — and an area law alone provably does *not* imply an efficient description in d≥2 (Ge–Eisert), while PEPS contraction is #P-complete in general.
- area-law ⇏ efficient: https://arxiv.org/abs/1411.2995 · poses the question: https://arxiv.org/abs/2103.02492

**4.4 Thermal area law for entanglement negativity (interacting, D≥2)** — `logarithmic negativity, Gibbs state, thermal area law, finite temperature`. For interacting lattice Hamiltonians in d≥2, is the logarithmic negativity of the Gibbs state at every T>0 bounded by O(|∂A|)? **[OPEN]** (mutual-information thermal area law proven; negativity proven only for free/Gaussian systems; high-T separability side closed).
- mutual-info version: https://arxiv.org/abs/2007.11174

**4.5 (c/3) log L entanglement for interacting critical 1D chains** — `central charge, critical chain, Calabrese–Cardy, Fisher–Hartwig, XXZ`. Is S(L)=(c/3)log L + O(1) a rigorous theorem for a general *interacting* critical 1D lattice model (e.g. critical XXZ), or only for free fermions? **[PARTIAL]** (proven for quadratic chains via Toeplitz/Fisher–Hartwig; interacting case open — bottleneck is rigorous lattice→CFT continuum limit).
- Calabrese–Cardy: https://arxiv.org/abs/hep-th/0405152 · free-fermion lattice→CFT: https://arxiv.org/abs/2107.13834

---

## §5 Hamiltonian complexity & classical simulability

**5.1 Quantum PCP conjecture** — `qPCP, constant promise gap, QMA-hard, gap amplification, MIP*`. Is the local-Hamiltonian problem QMA-hard even when the promise gap is a *constant* fraction of the number of terms? **[OPEN]** — the central problem of the field. The games/MIP* form had a 2024 setback (an error found in prior energy-amplification), which does not touch the Hamiltonian conjecture.
- review: https://arxiv.org/abs/1309.7495 · games-version status (2024): https://arxiv.org/abs/2403.13084

**5.2 NLSS conjecture (no low-energy sampleable states)** — `NLSS, sampleable states, dequantizing QSVT, stronger than NLTS`. Do local-Hamiltonian families exist where all low-energy states lack a succinct classical sampleable representation? **[OPEN]** (stabilizer-state special case proven; the general case would be a step beyond the now-proven NLTS toward qPCP).
- https://arxiv.org/abs/2111.09079 · stabilizer case: https://arxiv.org/abs/2302.14755

**5.3 Quantum locally testable codes (c³-qLTC)** — `qLTC, constant soundness, distance-rate-locality, soundness amplification`. Does a code with simultaneously constant distance, rate, locality, *and* soundness exist? **[OPEN]** (would imply NLTS; constant-soundness but distance-weak constructions exist; the 2024 "almost-good" qLTC has only inverse-polylog distance).
- https://errorcorrectionzoo.org/c/qltc · https://arxiv.org/abs/2309.05541

**5.4 Minimal local dimension for QMA-hardness in 1D / translation-invariant** — `local Hamiltonian, 1D, QMA-complete, QMA_EXP, qudit dimension`. The local-Hamiltonian problem is QMA-complete (k=2 optimal) and 1D-QMA-complete at qudit dimension 8; what is the minimal per-site dimension, and how low can translation-invariant QMA_EXP-hardness go? **[OPEN]** (gap between d=8 hard and d=2 unknown).
- 1D d=8: https://arxiv.org/abs/1312.1469 · transl-inv: https://arxiv.org/abs/1605.01718

**5.5 Commuting Local Hamiltonian problem — is it in NP?** — `commuting local Hamiltonian, CLH, Bravyi–Vyalyi, topological order`. For pairwise-commuting local terms, is ground-energy estimation in NP? **[OPEN]** in general (many special cases — 2-local, 3-/4-local qubits, 2D, rank-1 3D — proven in NP; no QMA-hardness known).
- https://arxiv.org/abs/quant-ph/0308021 · https://arxiv.org/abs/1803.02213

**5.6 Sampling-based quantum advantage — the hardness conjectures** — `BosonSampling, random circuit sampling, anti-concentration, permanent-of-Gaussians`. *Approximate*-sampling hardness for BosonSampling/IQP/RCS rests on conjectures. **[OPEN]** (Permanent-of-Gaussians, anti-concentration, robust average-case #P-hardness); *exact*-sampling hardness is proven.
- Aaronson–Arkhipov: https://arxiv.org/abs/1011.3245 · RCS: https://arxiv.org/abs/1803.04402

**5.7 Dequantization of noisy quantum advantage — finite-size boundary** — `noisy RCS, constant noise per gate, spoofing, quantum-inspired`. At what finite sizes / depths does many-body sampling become classically simulable? **[PARTIAL]** (constant noise-per-gate ⇒ poly-time *asymptotically*; finite-size low-depth regime open; practical spoofing demonstrated).
- https://arxiv.org/abs/2211.03999

**5.8 StoqMA vs MA vs NP** — `stoquastic Hamiltonian, StoqMA, MA-complete, stoquastic PCP, MA=NP`. Where do sign-problem-free (stoquastic) Hamiltonians sit, and does a stoquastic gap-amplification (⇔ MA=NP) or StoqMA error reduction (⇔ StoqMA=MA) hold? **[OPEN]**.
- stoquastic PCP ⇔ MA=NP: https://arxiv.org/abs/1901.05270

---

## §6 Self-correcting quantum memory in 3D (partial results)

**6.1 Power-law / partial self-correction in 3D** — `welded code, layer codes, cored product codes, polynomial energy barrier, finite-temperature memory`. Short of full self-correction, how far can 3D codes raise the energy barrier / memory lifetime with system size, and what is the optimal achievable scaling? **[OPEN/PARTIAL, 2014–2026]** (Michnicki welded code O(L^{2/3}) barrier; 2025 layer codes; 2025–26 disordered cored-product codes — numerical evidence to ~60k qubits; the optimal lifetime scaling and a simple-construction 3D finite-T self-correcting memory remain open).
- welded: https://arxiv.org/abs/1406.4227 · layer codes: https://arxiv.org/abs/2510.06659 · cored product: https://arxiv.org/abs/2510.05479 · "remains open": https://arxiv.org/abs/2510.09218

---

## §7 Non-equilibrium dynamics & ergodicity breaking

**7.1 Measurement-induced phase transition (MIPT)** — `monitored circuit, volume-law area-law, entanglement transition, hybrid circuit`. Does a sharp volume-law↔area-law entanglement transition exist in the thermodynamic limit, and what is its universality class? **[OPEN]** for thermodynamic-limit universality (existence in 1+1D established 2018–19).
- https://arxiv.org/abs/1808.05953 · https://arxiv.org/abs/1901.08092

**7.2 MIPT — local order parameter / coding interpretation** — `purification transition, dynamically generated QEC code, scalable probe`. Is the volume-law phase a dynamically generated error-correcting code, and is there a scalable (non-postselected) universal order parameter for *generic* dynamics? **[PARTIAL]** (coding picture + reference-qubit probe established; a scalable order parameter for non-Clifford dynamics faces a fundamental estimability barrier).
- https://arxiv.org/abs/1905.05195 · estimability barrier: https://arxiv.org/abs/2310.04156

**7.3 Quantum many-body scars — exact eigenstates** — `PXP, Rydberg, weak ergodicity breaking, exact MPS scars, revivals`. Do exact finite-bond-dimension scar eigenstates of PXP persist in the thermodynamic limit? **[PARTIAL]** (a finite subset proven exact, rigorously violating strong ETH; the full revival tower is only approximate / fine-tuned emergent SU(2)).
- exact subset: https://arxiv.org/abs/1810.00888 · approximate tower: https://arxiv.org/abs/1812.05561

**7.4 Quantum many-body scars — algebraic characterization** — `spectrum generating algebra, restricted SGA, commutant, eta-pairing, which Hamiltonians host scars`. Is there a sharp algebraic characterization (SGA / commutant-algebra) of which local Hamiltonians host exact scar towers? **[PARTIAL]** (several mechanisms rigorously understood; a characterization covering all known scars is open).
- review: https://arxiv.org/abs/2109.00548

**7.5 Discrete time crystal — MBL eigenstate order (1D)** — `Floquet time crystal, eigenstate order, absolute stability, subharmonic`. Is the period-doubled MBL-DTC a genuine phase, rigid and infinitely long-lived in the thermodynamic limit? **[PARTIAL/CONTESTED]** (rigid *if* MBL is stable — but MBL stability itself is not a theorem; see §7.9).
- https://arxiv.org/abs/1603.08001 · https://arxiv.org/abs/1605.00639

**7.6 Discrete time crystal in 2D/3D** — `MBL instability, avalanche, higher-dimensional eigenstate order`. Does an infinitely-long-lived eigenstate-ordered MBL-DTC exist in d≥2, or is every higher-d DTC only prethermal? **[OPEN/CONTESTED]**.
- RMP colloquium: https://arxiv.org/abs/2305.08904

**7.7 Optimality of the Floquet heating bound** — `exponential heating, energy absorption, tightness, log corrections`. The exp(−cω) heating bound is proven; is its exact asymptotic form optimal, and is there a matching lower bound on heating time for smooth drives? **[OPEN]** (existence of exp-long heating is settled).
- bound: https://arxiv.org/abs/1507.01474 · prethermalization theorem: https://arxiv.org/abs/1509.05386

**7.8 Hilbert-space fragmentation — strong vs weak classification** — `dipole conservation, Krylov sectors, strong vs weak fragmentation, largest fragment fraction`. Is there a sharp model-independent classification of strong vs weak fragmentation, diagnosed by whether the largest Krylov-sector fraction d_max/dim𝓗 tends to 0 or to a nonzero constant as system size grows? **[PARTIAL]** (phenomenon proven-by-construction; full classification open; note the exponential growth of the commutant distinguishes fragmentation from ordinary symmetry, *not* strong from weak).
- https://arxiv.org/abs/1904.04266 · https://arxiv.org/abs/2108.10324

**7.9 Existence/stability of the MBL phase** — `many-body localization, l-bits, avalanche instability, Imbrie, thermodynamic limit`. Does a genuine MBL phase (full set of LIOMs, no thermalization) rigorously exist for a 1D disordered chain at strong disorder in the infinite-size limit? **[CONTESTED]** — the open status itself is disputed: the only rigorous proof (Imbrie 2016) is conditional; avalanche instability + finite-size-artifact numerics dispute existence; a 2024 rigorous result proves only anomalous transport, not a phase. Distinguish "MBL phase" (contested) from finite-size "MBL regime" (real).
- Imbrie (conditional): https://arxiv.org/abs/1605.03003 · avalanche: https://arxiv.org/abs/1608.01815 · finite-size critique: https://arxiv.org/abs/1905.06345

**7.10 Stark MBL — mechanism & thermodynamic-limit stability** — `Stark MBL, linear tilt, Hilbert-space shattering, dipole conservation`. Is clean-system Stark localization genuine MBL or fragmentation-driven, and does it survive in the thermodynamic limit? **[CONTESTED/OPEN]**.
- https://arxiv.org/abs/2012.13722 · https://arxiv.org/abs/2109.06196

**7.11 Strong ETH for a fixed deterministic local Hamiltonian** — `eigenstate thermalization, strong ETH, off-diagonal matrix elements, Gibbs`. Is there a rigorous proof of strong ETH for *any* fixed, deterministic, non-integrable, local Hamiltonian? **[OPEN]** (proven only for Wigner-type random matrices / mean-field; weak ETH holds even for integrable systems, so is insufficient; scars make strong ETH literally false in some local models).
- review: https://arxiv.org/abs/1509.06411 · random-matrix ETH: https://arxiv.org/abs/2403.10359

**7.12 Maldacena–Shenker–Stanford chaos bound** — `bound on chaos, Lyapunov exponent, 2πT/ℏ, OTOC`. Is λ_L ≤ 2πk_BT/ℏ a rigorous theorem? **[PHYS]** (physics-level argument under analyticity/large-N assumptions; conditional theorem under a regularization-independence hypothesis; saturated by SYK/holography).
- https://arxiv.org/abs/1503.01409

**7.13 Universal operator-growth hypothesis** — `Lanczos coefficients, Krylov complexity, linear b_n growth`. Do Lanczos coefficients grow asymptotically linearly (maximal, up to 1D log) for generic chaotic systems? **[OPEN]** (upper bound proven; saturation conjectural; converse disproven by integrable LMG).
- https://arxiv.org/abs/1812.08657

**7.14 Optimal Lieb–Robinson / OTOC light cone** — `Lieb–Robinson bound, butterfly velocity, long-range 1/r^α, Frobenius light cone`. For 1/r^α interactions in d>1, is the optimal Frobenius light cone t ∼ r^min(α−d,1)? **[OPEN]** in d>1 (1D proven; operator-norm cone closed for α>2d).
- 1D proven / d>1 conjectured: https://arxiv.org/abs/2105.09960

**7.15 Equilibration timescales of closed quantum systems** — `equilibration, non-degenerate gaps, relaxation time bound`. Closed systems provably equilibrate to *some* stationary state under broad assumptions; can the equilibration *timescale* be rigorously bounded (a precise open problem distinct from Gibbs thermalization, §7.11)? **[PARTIAL]** (equilibration-in-principle proven 2009; timescale bounds and the route to Gibbs values open).
- https://arxiv.org/abs/0812.2385

---

## §8 Strongly-correlated & field-theoretic frontiers

**8.1 Kagome S=1/2 Heisenberg spin liquid** — `kagome, Z2 vs Dirac spin liquid, topological entanglement entropy, DMRG`. Is the nearest-neighbor S=1/2 kagome Heisenberg ground state a gapped Z2 or a gapless U(1) Dirac spin liquid? **[CONTESTED]** (DMRG 2011 favored gapped Z2; 2017 tensor-network/DMRG favor gapless Dirac; unresolved, very long correlation length).
- https://arxiv.org/abs/1011.6114 · https://arxiv.org/abs/1611.06238

**8.2 Provable gapped Z2 spin liquid in a realistic SU(2) model** — `toric code, Kitaev honeycomb, SU(2) Heisenberg, commuting projector`. Does any realistic SU(2)-symmetric Heisenberg-like 2D model have a *rigorously proven* gapped Z2 ground state? **[OPEN]** (proven only for fine-tuned exactly-solvable models — toric code, Kitaev honeycomb, Wen plaquette — none SU(2)-symmetric).
- Kitaev honeycomb: https://arxiv.org/abs/cond-mat/0506438

**8.3 Deconfined quantum criticality** — `Néel–VBS transition, deconfined criticality, pseudocriticality, complex CFT, emergent SO(5)`. Is the 2+1D Néel-to-VBS transition genuinely continuous, or weakly first-order / pseudocritical (governed by nearby complex fixed points)? **[CONTESTED]** (2017–26 fuzzy-sphere/QMC evidence leans weakly-first-order while bootstrap groups argue for a multicritical CFT; not settled).
- https://arxiv.org/abs/1703.02426 · fuzzy sphere: https://arxiv.org/abs/2306.16435

**8.4 Rigorous deconfined phase in a 2+1D lattice gauge–matter model** — `Fradkin–Shenker, Z2 gauge-Higgs, confinement–Higgs, deconfinement`. Can the deconfined/topologically-ordered region of a specific 2+1D gauge–matter model (e.g. Z2 gauge-Higgs) be rigorously established as a sharply distinct phase, separated by a transition from the confined/Higgs region? **[PARTIAL/PHYS]** (Fradkin–Shenker phase structure known; pure-gauge toric-code limit rigorous; general interacting quantum case physics-level).
- https://doi.org/10.1103/PhysRevD.19.3682

---

## §9 Open, driven & dissipative many-body systems

**9.1 Rapid mixing of Gibbs samplers — beyond 1D / all temperatures** — `modified log-Sobolev, MLSI, rapid mixing, commuting Hamiltonian, low temperature`. Do local quantum Gibbs samplers mix in O(log N)/poly(N) time beyond high temperature and beyond 1D? **[PARTIAL]** — 1D short-range chains at all temperatures resolved (2025–26, polylog depth, exponential clustering); 2D / general non-commuting low-T open. Low-T Gibbs sampling is **BQP-complete** in general (2024) and classically hard even at constant T, so no universal efficient algorithm exists.
- 1D all-T: https://arxiv.org/abs/2510.08533 · low-T BQP-complete: https://arxiv.org/abs/2403.12691 · constant-T classical hardness: https://arxiv.org/abs/2408.01516 · 1D commuting MLSI: https://arxiv.org/abs/2112.00593

**9.2 Stability of dissipative steady states without rapid mixing** — `stability, local Lindbladian, unique fixed point, slow mixing, Lieb–Robinson`. Are steady-state local observables of a unique-fixed-point local Lindbladian stable under local perturbations when mixing is only polynomially (not logarithmically) fast? **[OPEN]** (rapid mixing ⟹ stability is proven; the slow-mixing case is open).
- rapid-mixing⟹stability: https://arxiv.org/abs/1303.4744

**9.3 Area laws for Lindbladian steady states** — `area law, detailed balance, steady state, mutual information, parent Hamiltonian`. Do gapped detailed-balance Lindbladians have steady states with an area law and efficient tensor-network description in d≥2? **[PARTIAL]** — 1D mutual-information area law proven (2022); higher dimensions open.
- https://arxiv.org/abs/2212.10061

**9.4 Dissipative engineering of topological order** — `dissipative state preparation, steady-state topological order, non-abelian, local Lindbladian, heralded noise`. Explicit strictly-local Lindbladians stabilize mixed-state Z₂ and D₄ (non-abelian) topological order as steady states, protecting information for time exp(N) under perfectly heralded noise; which topological orders admit such a local-Lindbladian stabilizer is open. **[PARTIAL]** (concrete Z₂/D₄ constructions proven; general classification — conjecturally premodular/non-chiral anyon theories — open).
- https://arxiv.org/abs/2410.21402 · partial classification: https://arxiv.org/abs/2405.02390

**9.5 Decoherence-induced mixed-state phase transitions** — `mixed-state topological order, coherent information, error threshold, decohered toric code, Nishimori`. Is the optimal QEC error threshold an intrinsic phase transition of the decohered mixed state, detectable by coherent information / mixed-state order parameters? **[PARTIAL 2024]** (toric-code coherent-information threshold established; non-stabilizer / coherent-noise / dynamical cases open).
- https://doi.org/10.1103/PhysRevResearch.6.L042014 · https://arxiv.org/abs/2412.12279

**9.6 Strong-to-weak spontaneous symmetry breaking (SWSSB)** — `strong-to-weak SSB, fidelity correlator, Rényi-2, mixed-state phase, weak symmetry`. Does every T>0 fixed-charge thermal state spontaneously break its strong symmetry to a weak one (conjectured; proven only for Z₂ commuting-projector cases under local-indistinguishability assumptions)? And is steady-state mixed-state SPT order stable, or destabilized by SWSSB (a single-model instability is known, with a stable exception)? **[OPEN/CONTESTED 2024–25]**.
- https://arxiv.org/abs/2405.03639 · mSPT instability: https://arxiv.org/abs/2410.12900

**9.7 Many-body non-Hermitian skin effect & point-gap topology** — `non-Hermitian skin effect, point gap, many-body spectral winding, Liouvillian, multifractality`. Does a nonzero many-body spectral winding number imply a many-body / Liouvillian non-Hermitian skin effect, as the single-particle winding⟹skin-effect theorem does? **[PARTIAL]** — single-particle classification resolved (2020); the many-body bulk–boundary implication is open (Liouvillian case proven only for quadratic generators; linked to Hilbert-space multifractality).
- single-particle: https://arxiv.org/abs/2401.08304 · many-body winding: https://arxiv.org/abs/2202.02548

**9.8 Monitored entanglement transitions under genuine dissipation** — `measurement-induced transition, conditioned trajectories, non-postselected, unraveling, nonlinear diagnostics`. Do *outcome-conditioned, non-postselected* monitored entanglement transitions (detected by nonlinear-in-ρ quantities) survive genuine dissipation in interacting / feedback / non-Gaussian settings? **[CONTESTED]** — settled background: the postselected no-click critical phase is destroyed by any sub-unity record fraction (ξ⁻¹∼(1−q)), and the fully-averaged linear-in-ρ state has no transition; yet a genuine diffusive unraveling-induced area↔log transition *does* exist for free fermions, while other monitored free-fermion settings argue for its absence — so survival in the general case is open.
- breakdown under information loss: https://arxiv.org/abs/2407.13837 · diffusive unraveling transition (survives): https://arxiv.org/abs/2406.04869

---

## §10 Integrability & exact solvability

**10.1 Bethe-ansatz completeness for the Heisenberg chain** — `Bethe ansatz completeness, string hypothesis, deviated/singular strings, Gaudin, Mukhin–Tarasov–Varchenko`. Does the (string-hypothesis) Bethe ansatz yield a complete eigenbasis for the XXX/XXZ chain? **[PARTIAL]** — rigorous completeness for the homogeneous gl₂ XXX / gl_N Gaudin model (2009, MTV, via Wronskians); string-hypothesis completeness in general, and singular-solution selection, remain conjectural/partial.
- MTV: https://arxiv.org/abs/0706.0688 · singular solutions: https://arxiv.org/abs/1304.7978

**10.2 Bethe-ansatz incompleteness at roots of unity** — `XXZ roots of unity, sl₂ loop algebra, Drinfeld polynomial, degeneracies, Deguchi–Fabricius–McCoy`. At q a root of unity the XXZ Bethe ansatz is provably incomplete (extra evaluation-parameter data fill an emergent sl₂-loop-algebra multiplet); fully characterizing the degeneracies/Drinfeld polynomials is open. **[PARTIAL/CONTESTED]**.
- https://arxiv.org/abs/cond-mat/9912141

**10.3 Classification of Yang–Baxter-solvable spin chains** — `integrable chain classification, R-matrix, Yang–Baxter, difference form, medium-range`. Enumerate all integrable nearest-neighbor (and longer-range) spin chains via regular R-matrices. **[PARTIAL]** — all regular 4×4 (two-state) solutions classified (difference-form 2019; full non-difference 2023–24); higher local dimension (e.g. all regular 9×9) and a general range-≥3 classification (incl. Rule-54 integrability) remain open.
- difference-form: https://arxiv.org/abs/1904.12005 · all 4×4: https://arxiv.org/abs/2306.10423 · medium-range: https://arxiv.org/abs/2108.02053

**10.4 Free-fermion solvability of spin models** — `free-fermion solvable, frustration graph, line graph, free fermions in disguise, Chapman–Flammia`. Complete necessary-and-sufficient characterization of which spin models map to free fermions. **[PARTIAL]** — graph-invariant (line-graph) criterion for generalized-Jordan–Wigner solvability (2020), plus a "free fermions in disguise" family beyond Jordan–Wigner; a full characterization of all disguised models is open.
- line-graph criterion: https://arxiv.org/abs/2003.05465 · in disguise: https://arxiv.org/abs/1901.08078

**10.5 Rigorous microscopic derivation of generalized hydrodynamics (GHD)** — `generalized hydrodynamics, Euler scale, effective velocity, ab-initio derivation, Lieb–Liniger`. Can Euler-scale GHD be derived rigorously from the microscopic dynamics of an interacting *quantum* integrable model (e.g. Lieb–Liniger)? **[OPEN]** for quantum interacting models (the one ab-initio attempt was withdrawn by its authors in 2025; rigorous PDE properties exist, but only the classical hard-rod derivation is complete).
- PDE properties (2025): https://arxiv.org/abs/2406.18322 · hard rods (context): https://arxiv.org/abs/2503.07794

**10.6 Completeness of, and relaxation to, the generalized Gibbs ensemble (GGE)** — `GGE, quasi-local charges, completeness, equilibration, post-quench steady state`. Which (local + quasi-local) charges must enter the GGE, and can relaxation to it be proven for interacting integrable models? **[PARTIAL]** — quasi-local charges shown necessary for XXZ (2015); rigorous relaxation only for quadratic theories; interacting case open.
- quasi-local charges: https://arxiv.org/abs/1507.02993 · relaxation (quadratic): https://arxiv.org/abs/1809.08268

**10.7 Quantum KAM / stability under weak integrability breaking** — `quantum KAM, integrability breaking, quasi-conserved charges, prethermalization, Nekhoroshev`. Is there a rigorous quantum analog of KAM — do weakly-perturbed (autonomous) integrable systems retain approximately conserved charges with Nekhoroshev-type long-time bounds? **[OPEN/PHYS]** (a rigorous Nekhoroshev bound exists for the distinct quasi-periodically-driven setting; the autonomous case is open).
- https://arxiv.org/abs/1407.7167

**10.8 Arctic curve / limit shape of the DWBC six-vertex model** — `six-vertex, domain-wall boundary, arctic curve, limit shape, emptiness formation probability, Colomo–Pronko`. Is the frozen/disordered boundary in the scaling limit rigorously given by the Colomo–Pronko parametric curve, for all weights (and the analogous refined open-boundary Razumov–Stroganov statements)? **[PARTIAL/OPEN]** (explicit curve derived; full limit-shape rigor open beyond free-fermion points).
- arctic curve: https://arxiv.org/abs/0907.1264 · open-boundary RS: https://arxiv.org/abs/math-ph/0408042

**10.9 Exactly-solved models with residual open questions** — `Hubbard SO(4) completeness, sine-Gordon constructive QFT, form-factor convergence`. SO(4)-extended completeness of the 1D Hubbard Bethe ansatz (rests on the unproven string hypothesis; broken by Aharonov–Bohm flux) **[CONTESTED]**; rigorous construction of sine-Gordon as a strictly-local QFT with convergent form factors **[OPEN/PARTIAL]** (convergence proven only for the sinh-Gordon vacuum two-point function).
- Hubbard SO(4): https://arxiv.org/abs/cond-mat/9209012 · sine-Gordon constructive: https://arxiv.org/abs/1712.02844 · form-factor convergence (sinh-Gordon): https://arxiv.org/abs/2007.01740

---

## Verification caveats

- **Post-cutoff / preprint (verify before citing as peer-reviewed):** the Haldane-gap certified-upper-bound paper (arXiv:2606.03836, §1.1) and the even-D Lee–Yang paper (arXiv:2603.18675, §1.6) carry this-month arXiv identifiers reported from listings and warrant a direct ID/date check; the D=2,3 classical-Heisenberg Lee–Yang result (§1.6) is the peer-reviewed Dunlop–Newman 1975 paper, not a preprint.
- **2025 preprints used for status:** layer/cored-product codes (2510.06659, 2510.05479, §6.1), translation-invariant QMA refinements (§5.4). Treat as current frontier, not settled literature.
- **No URL, paper, author, or status was fabricated.** Every arXiv abstract page and DOI listed was either directly fetched by a search worker or appeared verbatim in a primary search result. A few DOIs for APS/Science/JHEP entries were reconstructed from verified journal-ref strings with the arXiv page as primary verification.

## How this was compiled

Nine parallel literature-search workers across three rounds covered: (R1) spin systems, Hubbard, topological order, entanglement & dynamics via landscape / historical-lineage / no-go lenses; (R2) Hamiltonian complexity & classical simulability, self-correcting quantum memory, non-equilibrium dynamics & ergodicity breaking, strongly-correlated/field-theoretic frontiers; (R3) open/driven-dissipative systems and integrability & exact solvability. Findings were deduplicated and cross-referenced.

The list was then curated in **two independent review rounds**. The first removed ~19 entries that were already solved (e.g. NLTS, good qLDPC codes, the 2026 3D self-correcting memory, Razumov–Stroganov, the Shiraishi non-integrability dichotomy) or lacked any precise formulation (e.g. "the full bulk dual of SYK", "a controlled theory of non-Fermi liquids", "a rigorous definition of quantum integrability"). A second adversarial round (six parallel review agents, each web-verifying solved-status against current literature) removed three more — the spectral-gap decidability-threshold meta-question (no single truth-valued claim), finite-temperature 3D topological order (now a published theorem, PRL 135, 040402 (2025)), and the local-Lindbladian uniqueness/classification program — rescoped two factually mis-stated entries to their genuine open frontier (Lee–Yang to D≥4, since D=3 was proven in 1975; the gapped 1D area law to the degenerate / non-local regime, since the unique-ground-state case is Hastings-closed), and trimmed solved or ill-formed sub-claims from several multi-part entries. A final scope decision removed the one real-material/experiment entry (the α-RuCl₃ thermal-Hall question), leaving only mathematical claims about models. **73 problems remain.**
