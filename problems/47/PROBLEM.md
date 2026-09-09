# 47 Entanglement-Depth Charging-Rate Bound Under Time-Dependent Driving

```xml
<problem id="47">
  <status>open</status>
  <kind>proof</kind>
  <field>QTD</field>
  <name>Entanglement-Depth Charging-Rate Bound Under Time-Dependent Driving</name>
  <keys>entanglement depth quantum battery quantum battery charging charging rate charging power complete charging exact charging fully charging scheme quantum speed limit QSL Mandelstam-Tamm bound Margolus-Levitin bound time-dependent Hamiltonian time-dependent driving time-dependent control k-producible k-producibility h-separable multipartite entanglement entanglement-depth speed frontier exact entanglement-depth speed frontier block orthogonalization fixed-partition lemma battery fuel entanglement as battery fuel Shi Gan Zhang Wang Yang Sun Lu Jin quantum charging advantage genuine multipartite entanglement GHZ charging cluster-flip staircase frontier charging Hamiltonian quench residual case QTD017</keys>

  <claim>
    Fix an integer \(N\ge1\), an integer \(k\) with \(1\le k\le N\), and a
    real \(T\gt0\). Let \(\mathcal P(N,T)\) be the set of piecewise
    continuous Hamiltonian paths \(H:[0,T]\to\mathrm{Herm}((\mathbb
    C^2)^{\otimes N})\) on the \(N\)-qubit register (def below), and for
    \(H\in\mathcal P(N,T)\) let \(|\psi_H(t)\rangle\), \(\mathrm{Ent}[H]\),
    and \(\eta_{\rm MT}(H)\) be as in def below. Define
    \[
      \mathcal P^{\rm exact}_k(N,T):=\Bigl\{H\in\mathcal P(N,T):\
      |\psi_H(T)\rangle=e^{i\phi}|\uparrow\rangle^{\otimes N}\text{ for
      some }\phi\in\mathbb R,\ \ \mathrm{Ent}[H]\le k\Bigr\}
    \]
    (piecewise-continuous drives that exactly completely charge the
    battery while never exceeding trajectory entanglement depth \(k\)).
    Determine whether
    \[
      \eta_{\rm MT}(H)\ \le\ \eta_{\max}(k):=\frac{1}{\sqrt{\lceil
      N/k\rceil}}
      \qquad\text{for every }N\ge1,\ 1\le k\le N,\ T\gt0,\ H\in\mathcal
      P^{\rm exact}_k(N,T),
    \]
    i.e. whether the exact time-independent entanglement-depth speed
    frontier of Sun, Lu, and Jin, "Exact Entanglement-Depth Speed Frontier
    for Complete Quantum Charging," arXiv:2605.16935 (2026) [full citation
    in refs], Eq. (4), continues to upper-bound the Mandelstam-Tamm-branch
    charging rate once the restriction to a time-independent driving
    Hamiltonian is dropped and \(H(t)\) is allowed to vary with \(t\) — or
    whether instead some \((N,k,T,H)\) with \(H\in\mathcal P^{\rm
    exact}_k(N,T)\) achieves \(\eta_{\rm MT}(H)\gt\eta_{\max}(k)\).

    This claim isolates one precise, self-contained piece of the "general
    setting (time-dependent drives, approximate charging)" named by the
    survey lead seeding this entry: it drops time-independence of the
    driving Hamiltonian but keeps the charging endpoint exact and
    restricts the quantum-speed-limit rate to its Mandelstam-Tamm branch
    only (def below). It is a deliberately weaker, source-grounded reading
    chosen among several ways to formalize "the general setting"; see
    known below, "Scope of the generalization chosen here," for the
    ambiguity this leaves unresolved.
  </claim>

  <def>
    Battery register and endpoints. The battery is \(N\ge1\) qubits,
    Hilbert space \((\mathbb C^2)^{\otimes N}\), with each cell's
    computational basis written \(|\downarrow\rangle,|\uparrow\rangle\).
    The empty state is \(|\downarrow\rangle^{\otimes N}\) and the fully
    charged state is \(|\uparrow\rangle^{\otimes N}\); these are exactly
    orthogonal, \(\langle\downarrow|\uparrow\rangle=0\) (Sun et al.,
    Setting and theorem).

    Hamiltonian path and induced trajectory. \(\mathcal P(N,T)\) is the
    set of piecewise continuous maps \(H:[0,T]\to\mathrm{Herm}((\mathbb
    C^2)^{\otimes N})\) (finite-dimensional Hermitian operators, so
    automatically bounded); \(H\) need not satisfy \(H(t_1)=H(t_2)\) for
    \(t_1\ne t_2\). For \(H\in\mathcal P(N,T)\) the induced trajectory is
    \[
      |\psi_H(t)\rangle:=\mathcal T\exp\!\Bigl(-i\int_0^t
      H(s)\,ds\Bigr)|\downarrow\rangle^{\otimes N},\qquad t\in[0,T],
    \]
    with \(\mathcal T\) the time-ordering symbol (units \(\hbar=1\)). When
    \(H(t)\equiv H_c\) is constant this reduces to
    \(|\psi_H(t)\rangle=e^{-iH_ct}|\downarrow\rangle^{\otimes N}\), the
    quench dynamics of both cited sources.

    Entanglement depth. For a pure \(N\)-qubit state \(|\phi\rangle\),
    \(\mathrm{Ent}(|\phi\rangle)\) is the least integer \(k'\) such that
    there is a partition of the \(N\) qubits into disjoint blocks
    \(B_1,\ldots,B_m\), each of size at most \(k'\), and states
    \(|\phi_\alpha\rangle\) on \(B_\alpha\), with
    \(|\phi\rangle=\bigotimes_{\alpha=1}^m|\phi_\alpha\rangle\) (a block
    may be internally entangled; it is required only to be product with
    the other blocks). This is the standard \(k\)-producibility depth
    used by both Sun et al. (Setting and theorem, citing Sørensen and
    Mølmer, "Entanglement and extreme spin squeezing," Phys. Rev. Lett.
    86, 4431 (2001), and the entanglement-depth literature) and Shi et
    al. ("A new tool for estimating entanglement depth": \(|\phi\rangle\)
    is \(k\)-producible and \(h\)-separable if
    \(|\phi\rangle=\bigotimes_{\ell=1}^h|\phi_\ell\rangle\) with each
    factor on at most \(k\) particles; \(|\phi\rangle\) exhibits
    \(k\)-particle entanglement, i.e. \(\mathrm{Ent}[|\phi\rangle]=k\), if
    it is \(k\)-producible but not \((k-1)\)-producible). For
    \(H\in\mathcal P(N,T)\), the trajectory depth is
    \[
      \mathrm{Ent}[H]:=\max_{0\le t\le T}\mathrm{Ent}\bigl(|\psi_H(t)\rangle\bigr),
    \]
    the same "pointwise maximum over time" convention Sun et al. use for
    \(\mathrm{Ent}[U]\) (Setting and theorem). This definition does not by
    itself require a single partition to work at every \(t\); that a
    single fixed partition can be chosen for the whole trajectory is, in
    the time-independent case, a proved lemma (known below), not part of
    the definition.

    Mandelstam-Tamm-branch charging rate. For \(H\in\mathcal P(N,T)\)
    with trajectory \(|\psi_H(t)\rangle\), the instantaneous energy
    uncertainty is
    \[
      \Delta H(t):=\sqrt{\langle\psi_H(t)|H(t)^2|\psi_H(t)\rangle-
      \langle\psi_H(t)|H(t)|\psi_H(t)\rangle^2},
    \]
    its time average is \(\overline{\Delta H}:=\frac1T\int_0^T\Delta
    H(t)\,dt\), and the Mandelstam-Tamm-branch QSL-normalized rate is
    \[
      \eta_{\rm MT}(H):=\frac{\pi}{2\,T\,\overline{\Delta H}}.
    \]
    For time-independent \(H(t)\equiv H_c\), energy moments
    \(\langle\psi_H(t)|H_c^n|\psi_H(t)\rangle=\langle\downarrow^{\otimes
    N}|H_c^n|\downarrow^{\otimes N}\rangle\) are conserved along the
    orbit, so \(\Delta H(t)\equiv\Delta H_c\) is constant,
    \(\overline{\Delta H}=\Delta H_c\), and \(\eta_{\rm
    MT}(H)=\pi/(2T\Delta H_c)=\tau_{\rm MT}/T\) exactly, matching the
    Mandelstam-Tamm branch of \(\tau_{\rm QSL}=\max\{\pi/(2\Delta
    H_c),\pi/(2\langle H_c\rangle_{\rm ML})\}\) used by Shi et al.
    (Eq. (2)) and Sun et al. (Eq. (3)). \(\eta_{\rm MT}(H)\) does not
    include the Margolus-Levitin branch \(\pi/(2\langle H\rangle_{\rm
    ML})\); see known, "Scope of the generalization chosen here."

    \(\eta_{\max}(k):=1/\sqrt{\lceil N/k\rceil}\) is Sun et al.'s Eq. (4)
    value, restated here only as the candidate bound tested by claim
    above; its proof (known below) is restricted to time-independent
    \(H\).
  </def>

  <known>
    The proved time-independent frontier. Sun, Lu, and Jin, "Exact
    Entanglement-Depth Speed Frontier for Complete Quantum Charging,"
    arXiv:2605.16935 (2026) [full citation in refs], Setting and theorem,
    Eqs. (4)-(5), prove: for \(N\)-qubit complete charging generated by a
    time-independent Hamiltonian
    (\(|\psi(t)\rangle=e^{-iHt}|\downarrow\rangle^{\otimes N}\)), "If the
    complete-charging orbit satisfies \(\mathrm{Ent}[U]\le k\), then
    \(\eta\le\frac{1}{\sqrt{\lceil N/k\rceil}}\). The bound is attainable,
    and hence \(\eta_{\max}(k)=\frac{1}{\sqrt{\lceil N/k\rceil}}\),
    \(1\le k\le N\)" (their \(\eta=\tau_{\rm QSL}/T\), the full
    max-of-both-branches rate, not \(\eta_{\rm MT}\) alone), and
    conversely "for any observed rate \(0\lt\eta\le1\), one certifies
    \(\mathrm{Ent}[U]\ge\bigl\lceil N/\lfloor\eta^{-2}\rfloor\bigr\rceil\)."
    Because \(\eta_{\rm MT}(H)\le\eta(H)\) always (the Mandelstam-Tamm
    branch is at most the max of both branches), and because for
    time-independent \(H\) the definitions of \(\eta_{\rm MT}\) and
    \(\mathrm{Ent}[H]\) used in claim above agree exactly with Sun et
    al.'s \(\eta\) (Mandelstam-Tamm branch) and \(\mathrm{Ent}[U]\) (def
    above), the claim above restricted to time-independent \(H\) is
    already an immediate corollary of this proved theorem — it is not
    open. The open content of claim above is entirely the extension from
    time-independent to time-dependent \(H\).

    Relation to Shi et al.'s conjecture, and resolution of the survey's
    internal disagreement. Shi, Gan, Zhang, Wang, and Yang, "Quantum
    Charging Advantage from Multipartite Entanglement," J. Phys. B: At.
    Mol. Opt. Phys. 58, 055401 (2025); arXiv:2503.02667 [full citation in
    refs], Eq. (6), conjectured \(\mathrm{Ent}[U]\ge\lceil
    N\eta^2\rceil,\ \forall U\in\mathcal U(\eta)\), where their
    \(\mathcal U(\eta)\), Eq. (4), is explicitly defined as "all fully
    charging schemes \(U(t)\equiv\exp(-iH_ct)\)" — i.e. their conjecture
    was already quantified only over time-independent quench dynamics,
    not over time-dependent drives (their Eq. (1) writes a general
    \(H(t)=H_b+\theta(t)(H_c-H_b)\) but fixes \(\theta(t)\) to the
    Heaviside step function throughout, so the analyzed dynamics is the
    \(H_c\)-quench). Sun et al., Introduction, identify this relation
    directly: "A closely related recent work by Shi et al. proposed the
    observed-rate witness \(\mathrm{Ent}[U]\ge\lceil N\eta^2\rceil\) and
    supported it in representative complete-charging settings. Here we
    solve the corresponding depth-constrained speed problem exactly
    under the present assumptions," and state explicitly that their own
    Eq. (5) implies Shi et al.'s conjectured bound: "Since
    \(\lfloor\eta^{-2}\rfloor\le\eta^{-2}\), Eq. (5) implies the smoother
    bound \(\mathrm{Ent}[U]\ge\lceil N\eta^2\rceil\). The difference is
    the integer count." Consequently, Shi et al.'s Eq. (6) conjecture,
    exactly as they quantified it, is a proved corollary of Sun et al.'s
    theorem and is not open. This entry's survey provenance
    (docs/Quantum-Thermodynamics-Open-Problems-Survey.md, "Where the two
    surveyor models disagree") records: "GPT says the conjecture as
    posed was resolved by the 2026 exact frontier; Opus keeps the general
    (time-dependent, approximate) case as the surviving problem." Reading
    the sources directly, both are correct about different objects: GPT
    is correct about Shi et al.'s Eq. (6) exactly as quantified
    (resolved); Opus is correct that the broader
    time-dependent/approximate-charging question is not addressed by
    either source and remains open. This entry records only the latter,
    narrowed to the single axis in claim above.

    Explicit scope exclusion in the source that proves the
    time-independent case. Sun et al., Operational meaning and scope,
    state verbatim: "Approximate charging, mixed states, open dynamics,
    time-dependent controls, work extraction, and thermodynamic figures
    of merit are outside this benchmark and may require different
    resource measures." Time-dependent driving is thus explicitly named
    by the very paper that settles the time-independent case as outside
    its proof, not merely omitted by oversight.

    Why the known proof does not immediately transfer. Sun et al.'s
    Proof sketch derives \(m\ge\lceil N/k\rceil\) independent blocks
    from a "fixed-partition lemma," obtained as follows: "Since there
    are only finitely many product partitions, the corresponding closed
    time sets cover the interval \([0,T]\). By the Baire category
    argument, at least one of them contains a nonempty relatively open
    subinterval of \([0,T]\)... By analyticity of the finite-dimensional
    time-independent orbit, these equations then vanish along the whole
    orbit. The fixed-partition lemma... yields one product partition
    supporting the entire trajectory." This step uses that
    \(t\mapsto e^{-iHt}|\downarrow\rangle^{\otimes N}\) is real-analytic
    in \(t\) for time-independent \(H\), which need not hold for a
    piecewise-continuous time-dependent \(H(t)\) as in claim above; a
    trajectory could a priori realize \(\mathrm{Ent}[H]\le k\) pointwise
    via different partitions at different times without ever admitting
    one partition valid throughout \([0,T]\). By contrast, the remaining
    half of the proof (each block spans an orthogonal-ray endpoint pair,
    so its Fubini-Study path length is at least \(\pi/2\); the blocks'
    Fubini-Study speeds combine as
    \(v^2(t)=\sum_\alpha v_\alpha^2(t)\); Minkowski's integral
    inequality; and \(v(t)=\Delta H(t)\) with
    \(L=\int_0^Tv(t)\,dt=T\overline{\Delta H}\)) does not visibly invoke
    time-independence and generalizes verbatim to \(\overline{\Delta H}\)
    once a fixed partition with \(m\ge\lceil N/k\rceil\) blocks is
    available. This entry records this as the identified obstruction in
    the known proof route, not as a proof that no fixed-partition-type
    reduction exists for time-dependent \(H\), nor as a proof that a
    counterexample exists; both remain to be shown.

    Scope of the generalization chosen here (ambiguity note). The issue
    and survey lead for this entry name "the general setting
    (time-dependent drives, approximate charging)" as a single residual
    case. Sun et al.'s exclusion sentence quoted above names at least six
    distinct relaxations (approximate charging, mixed states, open
    dynamics, time-dependent controls, work extraction, thermodynamic
    figures of merit) as jointly out of scope, without proposing a
    combined formalization of any two of them together, and neither
    cited source defines a quantum-speed-limit rate for approximate
    (fidelity-threshold) charging targets or for the Margolus-Levitin
    branch under time-dependent driving. Claim above deliberately
    isolates the single relaxation (time-dependent driving,
    Mandelstam-Tamm branch, exact charging endpoint) that can be stated
    using only definitions already fixed by the cited sources, without
    this entry inventing an unverified formula for fidelity-threshold
    charging or for a time-dependent Margolus-Levitin bound. Approximate
    (fidelity-threshold) charging and the Margolus-Levitin branch under
    time-dependent driving are therefore related but distinct open
    directions not covered by claim above.

    Residual doubt (recorded, not resolved by further search per this
    entry's instructions). Whether some paper other than the two cited
    here already proves or disproves an entanglement-depth speed bound
    for time-dependent complete-charging drives has not been checked;
    this entry relies only on Shi et al. (2025) and Sun et al. (2026)
    themselves, per the scope of this review.
  </known>

  <refs>
    <ref>Hai-Long Shi, Li Gan, Kun Zhang, Xiao-Hui Wang, and Wen-Li Yang, Quantum Charging Advantage from Multipartite Entanglement, Journal of Physics B: Atomic, Molecular and Optical Physics 58, 055401 (2025), arXiv:2503.02667, used for: Eq. (1) (driving Hamiltonian \(H(t)=H_b+\theta(t)(H_c-H_b)\) with \(\theta(t)\) the Heaviside step function); Eq. (2) (quantum speed limit \(\tau_{\rm QSL}=\max\{\pi/(2\Delta H_c),\pi/(2\langle H_c\rangle)\}\)); Eqs. (3)-(5) (charging rate \(\eta\) and the class \(\mathcal U(\eta)\), explicitly restricted to \(U(t)\equiv\exp(-iH_ct)\)); Eq. (6) (the conjectured bound \(\mathrm{Ent}[U]\ge\lceil N\eta^2\rceil\)); "A new tool for estimating entanglement depth" section (the \(k\)-producible/\(h\)-separable definition of entanglement depth); Conclusion (no statement of, or claim to resolve, a time-dependent-driving or approximate-charging generalization).</ref>
    <ref>Wenlong Sun, Gang Lu, and Yuanfeng Jin, Exact Entanglement-Depth Speed Frontier for Complete Quantum Charging, arXiv:2605.16935 (2026), no journal reference as of 2026-07-18, used for: Introduction (relation to and implication of Shi et al.'s Eq. (6), quoted in known above); Setting and theorem, Eqs. (3)-(5) (cyclic-subspace \(\tau_{\rm QSL}\), the proved frontier \(\eta_{\max}(k)=1/\sqrt{\lceil N/k\rceil}\), and the converse depth certificate); Proof sketch (the fixed-partition lemma and its use of analyticity of the time-independent orbit, and the Minkowski-inequality/Fubini-Study-speed argument, both quoted in known above); Operational meaning and scope (the verbatim exclusion of "approximate charging... time-dependent controls..." from the benchmark, quoted in claim and known above).</ref>
  </refs>

  <ask>
    A proof must show, for every \(N\ge1\), every integer \(k\) with
    \(1\le k\le N\), every \(T\gt0\), and every \(H\in\mathcal P^{\rm
    exact}_k(N,T)\) (def above), that \(\eta_{\rm MT}(H)\le\eta_{\max}(k)\).
    A proof restricted to real-analytic \(H(\cdot)\), to piecewise
    time-independent (multi-quench) \(H(\cdot)\) with finitely many
    pieces, or to any other strict subclass of piecewise continuous paths
    does not establish the claim as quantified, since it would leave open
    whether a genuinely non-analytic drive can beat \(\eta_{\max}(k)\);
    record such a restricted result as progress (below), not as a proof.

    A disproof must exhibit one explicit tuple \((N,k,T,H)\) with
    \(H\in\mathcal P^{\rm exact}_k(N,T)\) and \(\eta_{\rm
    MT}(H)\gt\eta_{\max}(k)\), i.e. a time-dependent drive that completely
    charges the battery, never exceeds entanglement depth \(k\) at any
    time, and beats the time-independent frontier's Mandelstam-Tamm-branch
    rate at that same \(N,k\). Because the claim is universal over
    \((N,k,T,H)\), one such tuple suffices; no argument about other
    values of \(N\) or \(k\) is additionally needed.

    The following count as progress, not as a solution to the claim,
    unless combined with an argument closing the remaining gap: (i) a
    proof of the fixed-partition lemma (known above) for time-dependent
    \(H\), or an explicit counterexample to it, without a matching
    resolution of the rate inequality itself; (ii) a bound \(\eta_{\rm
    MT}(H)\le f(N,k)\) for some universal but non-matching \(f\) (i.e. one
    not equal to \(\eta_{\max}(k)=1/\sqrt{\lceil N/k\rceil}\) for all
    admissible \(N,k\)), showing time-dependence cannot help unboundedly
    without pinning the exact frontier; (iii) a construction achieving
    \(\eta_{\rm MT}(H)\gt\eta_{\max}(k)\) only in an asymptotic sense (e.g.
    a sequence of tuples with \(\eta_{\rm MT}\to\eta_{\max}(k)^+\) as
    \(N\to\infty\)) rather than one explicit finite counterexample; (iv) a
    resolution of the analogous question for the Margolus-Levitin branch
    or for approximate (fidelity-threshold) charging (known above, "Scope
    of the generalization chosen here"), since neither is part of claim
    above as stated.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
