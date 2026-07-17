# Open Problems in Quantum Thermodynamics — A Survey

**Survey date:** 2026-07-16 · **Scope:** quantum only (three classical kinetic-theory leads removed at maintainer's direction) · **39 leads across 5 groups**

**Method.** 3 lanes (curated collections · recent literature 2023–26 · rigorous thermalization) × 2 independent surveyors each (Claude Opus 4.8 and GPT-5.6-Sol at max reasoning effort), no subagents, web verification against literature through 2026-07-16. The thermalization lane deliberately excludes the 73 problems already catalogued in `Quantum-Many-Body-Open-Problems-Survey.md` (branch `qmb`). Inclusion filter: theorem-shaped claim in an established formalism; proof or explicit construction carries the main credit (modest formulation scaffolding allowed); light numerics only; citable posed-as-open provenance; still-open verified as of the survey date.

> **Every item is a lead, not a fact.** Statements, provenance, and open-status are the surveyors' claims. Each pick gets re-verified from primary sources via `skills/add/SKILL.md` and gated by two blind reviews (`skills/review/SKILL.md`) before entering `problems/`. Update the **Status** line of an entry when its state changes.

**Status legend:** `LEAD` (surveyed, unverified) · `RECORDED <id>` (accepted entry in `problems/`) · `REJECTED — <reason>` (failed verification or review) · `DROPPED — <reason>` (maintainer decision).

---

## §A Thermal operations & catalysis (1–14)

*Thermal operation: anything doable by coupling the system to a heat bath while conserving total energy. Catalyst: an auxiliary that must come back intact. Coherence: superposition between energy levels.*

**1. Coherent states under thermal operations** — `coherent-thermal-operation-convertibility`
Prove the complete rule deciding when one state can be turned into another by a thermal operation, once both carry coherence (solved for states without coherence, 2013; solved for qubits). *Why:* the founding question of single-shot quantum thermodynamics.
Source claims: Ng–Woods (Springer 2018) · Lostaglio (Rep. Prog. Phys. 82, 114001, 2019) — both models — open-confidence: **high**.
Status: `RECORDED QTD001`

**2. Correlated catalysis: one free energy to rule them all** — `correlated-catalytic-thermal-operations-coherence`
Prove or disprove: with a catalyst allowed to keep correlations, a single free energy (plus a coherence-mode compatibility condition) decides every conversion under thermal operations — Shiraishi's explicit conjecture, "left as an open problem" in PRL 2025. *Why:* would be the second law of fully quantum thermodynamics.
Source claims: Shiraishi (PRL 134, 160402, 2025) · Shiraishi–Takagi (arXiv:2510.05642) — both models — open-confidence: **high**.
Status: `REJECTED — Already solved by the issue's own second cited source`

**3. Exact catalysis, catalyst returned uncorrelated** — `strict-catalytic-thermal-operation-convertibility`
Characterize conversions of coherent states when the catalyst must return exactly and uncorrelated — the strictest catalysis notion, listed open in the 2023/2024 reviews. *Why:* separates what correlations buy (problem 2) from what catalysis alone buys.
Source claims: Datta–Kondra–Miller–Streltsov (Rep. Prog. Phys. 86, 116002, 2023) · Lipka-Bartosik–Wilming–Ng (RMP 96, 025005, 2024) — gpt — open-confidence: **high**.
Status: `RECORDED QTD003`

**4. "All states are universal catalysts" — the broken proof** — `universal-thermal-catalysts-proof-gap`
Prove or refute the 2021 PRX claim that many copies of any resourceful state catalyze any free-energy-allowed transition: the RMP 2024 review reports its key lemma is invalid, leaving the theorem unproven. *Why:* a published, cited "theorem" currently without a proof.
Source claims: Lipka-Bartosik–Skrzypczyk (PRX 11, 011061, 2021) vs RMP 96 (2024) — gpt — open-confidence: **med-high**.
Status: `RECORDED QTD004`

**5. Equal-entropy catalysis with an infinite catalyst** — `equal-entropy-infinite-catalyst` · *OQP 49 residue*
Prove or disprove: an infinite-dimensional catalyst can exactly convert between inequivalent states of equal entropy — the leftover of the otherwise-solved catalytic entropy conjecture. *Why:* decides whether entropy is the only catalytic invariant in the infinite limit.
Source claims: Wilming (Quantum 6, 858, 2022) · IQOQI OQP Problem 49 — gpt — open-confidence: **high**.
Status: `RECORDED QTD005`

**6. Markovian thermal processes with coherence** — `markovian-thermal-processes-coherence-laws`
Extend the complete memoryless-thermalization laws ("continuous thermomajorization", solved for populations in 2022) to coherent states. *Why:* memoryless dynamics is what experiments actually implement.
Source claims: Lostaglio–Korzekwa (PRA 106, 012426, 2022) — opus — open-confidence: **med-high**.
Status: `RECORDED QTD006`

**7. Cooling maps = thermal operations?** — `cooling-maps-thermal-operations-equivalence` · *status thinly tracked*
Prove or disprove that the low-temperature "cooling maps" (closed-form transition conditions known) coincide with physical thermal operations — proven only for qubits; posed as the "main open question" in 2015. *Why:* exact transition rules in the regime where the third law lives.
Source claims: Narasimhachar–Gour (Nat. Commun. 6, 7689, 2015) — opus — open-confidence: **medium**.
Status: `LEAD`

**8. Elementary thermal operations: strong = weak?** — `eto-weto-quasi-uniform-equivalence`
Prove the conjectured criterion (Hack–Mendl 2025, Conjecture 1): two-level thermal swaps with and without randomness reach the same states exactly when the Gibbs weights take at most two distinct values. *Why:* pins down when fine-grained control adds power; necessity direction proven.
Source claims: Hack–Mendl (J. Phys. A 58, 315302, 2025) — gpt — open-confidence: **high**.
Status: `LEAD`

**9. Which Lindblad generators are thermal?** — `markovian-thermal-generator-classification`
Prove the conjectured classification (one inclusion proven): every Markovian generator inside the thermal-operations set arises from an energy-conserving coupling to a finite thermal bath. *Why:* the continuous-time counterpart of "what is a thermal operation".
Source claims: vom Ende–Malvetti–Dirr–Schulte-Herbrüggen (OSID 30, 2350005, 2023), Conjecture 1 — gpt — open-confidence: **medium**.
Status: `LEAD`

**10. Thermal Recall protocol conjectures** — `finite-temperature-memtp-extreme-point-convergence`
Prove two conjectures about a concrete memory-assisted thermalization protocol: its truncated version converges to the extreme reachable states, and it never does worse than the full version. *Why:* memory as a provably sufficient substitute for non-Markovian control.
Source claims: Czartowski–de Oliveira Junior–Korzekwa (PRX Quantum 4, 040304, 2023), Conjectures 1–2 — gpt — open-confidence: **medium**.
Status: `LEAD`

**11. Geometric limits of strict catalysis** — `catalytic-future-region-exactness`
Prove that the known geometric outer region equals exactly the set of states reachable with a strict catalyst (inclusion proven 2024; a 2026 result certifies part of the interior). *Why:* a picture-complete map of catalytic power for populations.
Source claims: Czartowski–de Oliveira Junior (PRR 6, 033203, 2024) — gpt — open-confidence: **med-high**.
Status: `LEAD`

**12. Local thermal operations + classical communication** — `infinite-round-ltocc-equals-slto` · *fresh 2026*
Prove two 2026 conjectures: unlimited rounds of local thermal operations with classical communication reach exactly the "semilocal" thermal class; and general (POVM) measurements stay inside it. *Why:* starts the LOCC-style distributed program for thermodynamics.
Source claims: Bistroń–Czartowski (PRE 113, 034114, 2026), Conjectures 1–2 — gpt — open-confidence: **high**.
Status: `LEAD`

**13. Quantum dichotomies beyond commuting targets** — `noncommuting-quantum-dichotomy-achievability`
Prove that the known optimal rates for converting one pair of states into another remain achievable when the target pair does not commute (one regime settled in 2026 with a different error metric). *Why:* the second-order rates of coherent thermodynamics.
Source claims: Lipka-Bartosik–Chubb–Renes–Tomamichel–Korzekwa (PRX Quantum 5, 020335, 2024), Conjecture 2 — gpt — open-confidence: **med-high**.
Status: `LEAD`

**14. Two-bath resource engines fill the simplex?** — `two-temperature-resource-engine-universality`
Prove that alternating contact with two finite-temperature baths eventually reaches every population distribution for dimension ≥ 3 (known for qubits, and when one bath is infinitely hot). *Why:* the minimal model of what a temperature difference alone can synthesize.
Source claims: Wojewódka-Ściążko–Puchała–Korzekwa (Quantum 8, 1222, 2024) — opus — open-confidence: **med-high**.
Status: `LEAD`

## §B Machines: clocks, batteries, engines (15–20)

*Autonomous machine: runs on temperature differences alone, no external control. Precision, power, and efficiency are the payoffs; dissipated entropy is the price.*

**15. Ultimate clock precision** — `quantum-clock-precision-dissipation-bound` · *models disagree*
Prove any universal bound tying a ticking clock's precision to its dissipation — now that the old linear bound is refuted and exponential precision-per-entropy is constructed (2025–26). *Why:* the thermodynamic price of timekeeping; currently no ceiling is known.
Source claims: Meier et al. (Nat. Phys. 2025; arXiv:2601.10785) — opus — open-confidence: **med-high**.
Status: `LEAD`

**16. Can precision become thermodynamically free?** — `vanishing-precision-cost-coherent-quantum-dynamics` · *models disagree*
Prove or disprove the 2021 conjecture that the entropy-cost-per-precision of a steady current can be pushed arbitrarily close to zero using enough quantum degrees of freedom. *Why:* decides whether any quantum remnant of the classical precision-dissipation trade-off survives.
Source claims: Menczel–Loisa–Brandner–Flindt (J. Phys. A 54, 314002, 2021) — opus — open-confidence: **med-high**.
Status: `LEAD`

**17. Entanglement as battery fuel — residual case** — `entanglement-depth-charging-rate-bound` · *partially resolved 2026*
Prove the entanglement-depth requirement for fast charging in the general setting (time-dependent drives, approximate charging) — the exact time-independent case was settled in 2026. *Why:* entanglement as a necessary resource for battery speed, not just sufficient.
Source claims: Shi et al. (arXiv:2503.02667) · exact frontier (arXiv:2605.16935) — opus — open-confidence: **medium**.
Status: `LEAD`

**18. Star graphs charge fastest** — `star-topology-battery-minimal-overlap-conjecture` · *fresh 2026*
Prove a clean spectral-graph inequality (checked to N = 7, proven in a special case) implying the star topology maximizes charging power of graph-structured fermionic batteries. *Why:* crisp and self-contained, but single-paper provenance so far.
Source claims: Sarkis–Prośniak–Nigro–Tkatchenko (arXiv:2603.10853, 2026), Conjecture 1 — opus — entry-grade: **medium**.
Status: `LEAD`

**19. Thermal marginals of two identical systems** — `optimal-thermal-marginals-identical-systems`
Prove that two copies of a thermal state can always be jointly rotated so each marginal is exactly thermal at any chosen hotter temperature — proven up to dimension 4, conjectured for all. *Why:* the core lemma of work-versus-correlations trade-offs.
Source claims: Bakhshinezhad et al. (J. Phys. A 52, 465303, 2019) — gpt — open-confidence: **med-high**.
Status: `LEAD`

**20. Carnot efficiency at nonzero power** — `carnot-efficiency-finite-power-quantum` · *borderline*
Prove or rule out a genuinely quantum engine (degeneracy, collective effects) that beats the classical no-go and reaches Carnot efficiency at strictly positive power. *Why:* a century-old boundary — but both surveyors flag that the engine class must be pinned before the question has a truth value.
Source claims: Shiraishi–Saito–Tasaki (PRL 117, 190601, 2016) + 2026 proposals — both models — **borderline**.
Status: `LEAD`

## §C Landauer & cooling (21–22)

*Landauer's principle: erasing one bit dissipates at least kT·ln 2 of heat. Finite baths and finite time make the true price higher — by exactly how much is the game.*

**21. Perfect oscillator cooling at the Landauer price** — `landauer-limit-oscillator-perfect-cooling`
Construct or rule out a protocol cooling a harmonic oscillator exactly to its ground state at exactly the Landauer heat cost, using finite means at every step ("remains an open question", verbatim). *Why:* is the Landauer limit reachable for infinite-dimensional systems?
Source claims: Taranto et al. (PRX Quantum 4, 010332, 2023) — opus — open-confidence: **med-high**.
Status: `LEAD`

**22. The finite-bath Landauer constant** — `finite-bath-landauer-sharp-constant`
Prove the sharp constant in finite-bath erasure: with an n-qubit bath the extra dissipation scales as c/n², and after a 2025 construction the answer is pinned to 2 ≤ c ≤ 2π². Close the gap. *Why:* the exact price of erasure with finite resources; both models converged here independently.
Source claims: Reeb–Wolf (NJP 16, 103011, 2014) · Lipka-Bartosik–Perarnau-Llobet (PRL 135, 170404, 2025) — both models — open-confidence: **medium**.
Status: `LEAD`

## §D Gibbs states & thermalization (23–38)

*Gibbs state: the equilibrium state at a given temperature. Clustering: far-apart regions look independent. Mixing: how fast dynamics reaches equilibrium. The qmb survey's 73 items are excluded by design.*

**23. Global Markov property of thermal states** — `gibbs-global-quantum-markov-property`
Prove that in 2D+ lattices at any temperature, two far-apart regions are nearly independent given the region between them, with error only polynomial in region sizes (local case proven 2025; high temperature settled at STOC 2026 — only arbitrary temperature survives). *Why:* the headline structural question about thermal states; strongest cross-model convergence in the survey.
Source claims: Kuwahara (PRX 15, 041010, 2025) + arXiv:2504.02208/2504.02235 — both models — open-confidence: **high**.
Status: `LEAD`

**24. Boundary Hamiltonians of thermal marginals** — `quasi-local-gibbs-marginal-hamiltonians`
Prove that tracing out part of a high-temperature thermal state leaves a state that is itself thermal for a quasi-local effective Hamiltonian — open beyond commuting interactions. *Why:* makes "temperature is local" an operator-level statement.
Source claims: Bluhm–Capel–Pérez-Hernández (J. Stat. Phys. 192, 134, 2025) — gpt — open-confidence: **high**.
Status: `LEAD`

**25. Fast thermalization in 2D+** — `quantum-gibbs-sampler-rapid-mixing-2d`
Prove size-independent relaxation speed of local thermalizing dynamics for non-commuting lattice systems all the way to the phase transition (1D settled 2025; high temperature settled; slow-mixing counterexamples show the frontier is real). *Why:* nature thermalizes efficiently — and quantum computers get a licensed Gibbs-sampling primitive.
Source claims: Kastoryano–Brandão program + arXiv:2510.08533/2510.04954/2411.04300 — opus — open-confidence: **high**.
Status: `LEAD`

**26. Strong relaxation for chains (MLSI)** — `uniform-mlsi-for-one-dimensional-gibbs-samplers`
1D non-commuting thermalizers now have a uniform spectral gap (STOC 2026); prove the stronger uniform entropy-decay constant, giving true rapid mixing from any starting state. *Why:* the explicitly-left-open half of this year's headline spin-chain result.
Source claims: Bergamaschi–Chen (STOC 2026, arXiv:2510.08533) — gpt — open-confidence: **high**.
Status: `LEAD`

**27. Clustering ⇔ fast mixing** — `clustering-mixing-equivalence-noncommuting`
Prove the equivalence — correlation decay in the thermal state if and only if fast relaxation of its dynamics — beyond commuting Hamiltonians (commuting case is a 2016 theorem). *Why:* welds statics to dynamics.
Source claims: Kastoryano–Brandão (CMP 344, 915, 2016) — opus — open-confidence: **high**.
Status: `LEAD`

**28. No phase transition ⇒ exponential clustering** — `gibbs-clustering-locality-temperature-subcritical`
Prove that in 2D+ thermal states, correlations decay exponentially at every temperature above the actual phase transition — not just above the much higher threshold current proofs reach. *Why:* the assumption every 2024–25 rigorous paper imports; quantum analogue of a classical cornerstone.
Source claims: Kliesch et al. (PRX 4, 031019, 2014) + Capel-line conditional results — opus — open-confidence: **high**.
Status: `LEAD`

**29. Uniform clustering under a spectral gap** — `temperature-uniform-clustering-in-gapped-chains`
For gapped 1D chains, prove correlation decay with constants uniform in temperature — bridging the fixed-temperature theorem (1969) and the zero-temperature theorem (2006), asked in 2026. *Why:* controls the low-temperature limit where all current constants blow up.
Source claims: Alhambra–Capel–Gondolf–Ruiz-de-Alarcón–Scalet (Ann. Henri Poincaré 2026, arXiv:2402.18500) — gpt — open-confidence: **high**.
Status: `LEAD`

**30. Araki's theorem for bosons** — `arbitrary-temperature-clustering-for-one-dimensional-lattice-bosons`
Prove all-temperature exponential clustering for 1D repulsive lattice bosons (proven only above a threshold temperature; the authors' stated primary open question, June 2026). *Why:* unbounded local dimensions break every spin-chain technique — and bosons are what experiments hold.
Source claims: Tong–Kuwahara–Gong (CMP, arXiv:2411.10759) — gpt — open-confidence: **high**.
Status: `LEAD`

**31. When does thermal entanglement die?** — `thermal-entanglement-sudden-death-threshold`
Prove the sharp temperature threshold below which thermal states are entangled (one-sided bound proven at FOCS 2024), and classify the symmetries that keep entanglement alive at all temperatures (2025–26 constructions). *Why:* the quantum/classical frontier of equilibrium matter.
Source claims: Bakshi–Liu–Moitra–Tang (FOCS 2024) + arXiv:2508.20166, 2607.12710 — opus — open-confidence: **med-high**.
Status: `LEAD`

**32. Average eigenstate entanglement** — `eigenstate-entanglement-universality` · *needs model-pinning*
Prove the conjectured universal entanglement curves of energy eigenstates — the free-fermion curve for quadratic models (explicit target, both surveyors), the maximal curve for chaotic ones. *Why:* the sharpest provable fingerprint separating chaos from integrability.
Source claims: Vidmar–Rigol (PRL 119, 220603, 2017) · Bianchi–Hackl–Kieburg–Rigol–Vidmar (PRX Quantum 3, 030201, 2022) — both models — open-confidence: **medium**.
Status: `LEAD`

**33. Floquet heating: matching bounds & bosons** — `floquet-heating-bound-tightness`
Heating under fast periodic driving is at most exponentially slow (2015–17 theorems). Two open flanks: prove it is also at least that fast (tightness), and prove any rigorous bound for lattice bosons, where unbounded occupations break the proofs. *Why:* can prethermal matter outlive its exponential guarantee — and does the guarantee cover the platforms experiments use?
Source claims: Abanin–De Roeck–Ho–Huveneers (CMP 354, 809, 2017) · Ho–Mori–Abanin–Dalla Torre (Ann. Phys. 454, 169297, 2023) — both models — open-confidence: **med-high**.
Status: `LEAD`

**34. A checkable criterion for the Mpemba effect** — `triplet-obstruction-to-markovian-mpemba` · *fresh 2026*
Prove the 2026 triplet conjecture: if no energy triplet of a detailed-balance master equation satisfies an explicit rate inequality, then no anomalous relaxation crossing (Mpemba effect) can occur. *Why:* turns a fashionable phenomenon into a verifiable criterion; proven at zero temperature.
Source claims: Avitan–Factor–Gelbwaser-Klimovsky (arXiv:2603.04567, 2026) — gpt — open-confidence: **high**.
Status: `LEAD`

**35. Diffusion is positive** — `closed-system-diffusion-positivity` · *needs model-pinning*
For one named chaotic spin chain, prove the closed-system diffusion constant is strictly positive and finite — "there exists no proof" for any clean chain (proven in 2025 only with added noise). *Why:* no theorem yet says any clean quantum chain genuinely diffuses.
Source claims: Žnidarič et al. (arXiv:2501.07749, 2025) — opus — open-confidence: **med-high**.
Status: `LEAD`

**36. Thermal states hard at constant temperature** — `finite-temperature-thermal-nlts` · *complexity*
Construct local Hamiltonians whose thermal states require deep circuits at some fixed temperature (known only for temperature shrinking with size; ground-state analogue proven 2022). *Why:* the finite-temperature quantum PCP frontier. Bucket: QuantumComputation.
Source claims: Eldar (arXiv:1911.04461) · Anshu–Breuckmann–Nirkhe (2022) — opus — open-confidence: **med-high**.
Status: `LEAD`

**37. Optimal thermal area law** — `thermal-area-law-optimal-scaling` · *sharpening*
Prove the optimal temperature-dependence in thermal area laws (remove log factors; decide whether a temperature-independent bound holds away from transitions). *Why:* a sharpening of a proven result — weakest shape in this group by the credit test.
Source claims: Wolf–Verstraete–Hastings–Cirac (PRL 100, 070502, 2008) · Kuwahara–Kato–Brandão (PRX 11, 011047, 2021) — opus — open-confidence: **medium**.
Status: `LEAD`

**38. Gaussian fluctuations at all temperatures** — `gibbs-berry-esseen-all-temperatures` · *thin provenance*
Prove optimal central-limit rates for extensive observables in 2D+ thermal states below the high-temperature regime. *Why:* ensemble equivalence at its sharpest — but posed as a natural next step, not a named conjecture.
Source claims: Kuwahara–Saito line (arXiv:2605.03829, 2026) — opus — open-confidence: **med-low**.
Status: `LEAD`

## §E Boundary of scope (39)

*Matrix-analytic rather than quantum-thermodynamic in character. (Three classical kinetic-theory leads were removed at the maintainer's direction: quantum only.)*

**39. Refined BMV inequality — surviving half** — `refined-bmv-lower-bound` · *OQP 40 residue · matrix analysis*
Prove the lower trace inequality of IQOQI Problem 40, \(\operatorname{tr}\exp(n\log A+m\log B)\le p_{n,m}(A,B)\) (its upper half was refuted in March 2026; the solved BMV conjecture is the weaker positivity statement). *Why:* OQP-listed and rooted in partition functions — but pure matrix analysis in character.
Source claims: OQP 40 (Hägele/Werner, 2017) · Cha (arXiv:2603.19927) — gpt — open-confidence: **med-high**.
Status: `LEAD`

---

## Where the two surveyor models disagree

- **15 · Clocks:** Opus treats "is there any ultimate precision–dissipation bound" as open with standard scaffolding; GPT filed it as a near-miss — the answer depends on the admitted clock class and readout accounting.
- **16 · Precision cost:** Opus keeps the Menczel conjecture open; GPT argues the exponential-clock constructions may already answer it constructively, up to convention differences.
- **17 · Battery depth bound:** GPT says the conjecture as posed was resolved by the 2026 exact frontier; Opus keeps the general (time-dependent, approximate) case as the surviving problem.
- **7 · Cooling maps:** Opus verified openness through review citations; GPT couldn't confirm current status within its lane window. Verify first if picked.

## Recently closed (verified rejections — evidence the front is moving)

- **Refuted:** linear clock precision–entropy bound (Meier et al., Nat. Phys. 2025)
- **Proven:** generalized quantum Stein's lemma (Lami; Hayashi–Yamasaki, 2024–25; Lean-formalized)
- **Proven:** exact catalytic entropy conjecture (Wilming 2022 — OQP 49 core)
- **Refuted:** thermal operations = closure of enhanced ones (Ding–Ding–Hu 2021)
- **Classified:** universality of elementary thermal operations (Hack–Mendl 2023–25)
- **Proven:** high-temperature thermal states are unentangled (Bakshi–Liu–Moitra–Tang, FOCS 2024)
- **Proven:** local Markov property of thermal states, any temperature (2025)
- **Proven:** uniform gap for 1D non-commuting thermalizers (Bergamaschi–Chen, STOC 2026)
- **Achieved:** 1/n² finite-bath Landauer rate (Lipka-Bartosik–Perarnau-Llobet, PRL 2025)
- **Refuted:** refined-BMV upper half (Cha, March 2026)
- **Proven:** battery depth–speed frontier, exact case (2026)

## How this was compiled

Six independent surveyor agents (three lanes × Claude Opus 4.8 / GPT-5.6-Sol max), each forbidden from spawning subagents, each verifying still-open status via live web checks against arXiv, journal pages, and the IQOQI OQP site through 2026-07-16. Reports were merged and deduplicated by the orchestrating agent; cross-model convergence and disagreements are recorded per entry. The rendered pick-list lives as a private claude.ai artifact; this file is the version-controlled source of truth and should be updated as entries move through the `add` → dual-`review` pipeline.
