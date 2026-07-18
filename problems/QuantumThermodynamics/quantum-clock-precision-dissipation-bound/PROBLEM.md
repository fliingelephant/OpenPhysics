# QTD015 Ultimate Clock Precision-Dissipation Bound

```xml
<problem id="QTD015">
  <status>open</status>
  <name>Ultimate Clock Precision-Dissipation Bound</name>
  <keys>clock precision dissipation bound precision-dissipation trade-off precision dissipation tradeoff clock accuracy entropy trade-off autonomous quantum clock ticking clock quantum clock thermodynamic uncertainty relation TUR dissipative TUR quantum TUR inverse Fano factor Fano factor entropy production per tick second law of thermodynamics timekeeping ring clock spin chain clock single-excitation clock coherent transport clock exponential precision-dissipation scaling linear precision bound fully dissipative clock Erker Mitchison Silva Woods Brunner Huber autonomous quantum clocks Barato-Seifert brownian clock cost and precision Pearson Guryanova thermodynamic cost of timekeeping Meier Minoguchi Sundelin Apollaro Erker Gasparinetti Huber precision is not limited by the second law universal bound clock precision ultimate precision-dissipation bound quantum thermodynamics of clocks autonomous ticking clock accuracy Knuth asymptotic notation big omega quantum correlated ticks Pauli exclusion clock</keys>

  <claim>
    For an integer \(n\ge2\), real nonzero couplings
    \(\mathbf g=(g_0,\ldots,g_{n-2})\), a rate \(\Gamma\gt0\), and a real
    \(\Sigma_{\rm tick}\gt0\), let
    \(\mathcal N_\Sigma(n,\mathbf g,\Gamma,\Sigma_{\rm tick})\) denote the
    steady-state precision (def below) of the \(n\)-site ring clock with
    these parameters, and let
    \[
      \mathcal C_{\rm ring}:=\bigl\{(n,\mathbf g,\Gamma,\Sigma_{\rm tick}):
      n\ge2,\ \mathbf g\in(\mathbb R\setminus\{0\})^{n-1},\ \Gamma\gt0,\
      \Sigma_{\rm tick}\gt0\bigr\}
    \]
    be the class of all such clocks (def below). For \(\Sigma\gt0\) define
    \[
      B(\Sigma):=\sup\bigl\{\mathcal N_\Sigma(n,\mathbf g,\Gamma,\Sigma):
      (n,\mathbf g,\Gamma,\Sigma)\in\mathcal C_{\rm ring}\bigr\}.
    \]

    Determine whether
    \[
      B(\Sigma)\lt\infty\quad\text{for every }\Sigma\in(0,\infty),
    \]
    i.e. whether steady-state clock precision at any fixed entropy
    production per tick is bounded by some function of \(\Sigma_{\rm
    tick}\) alone, uniformly over every ring size \(n\), coupling vector
    \(\mathbf g\), and rate \(\Gamma\) (existence of a universal,
    clock-design-independent precision-dissipation bound on \(\mathcal
    C_{\rm ring}\), of unspecified functional form) — or whether instead
    \(B(\Sigma_0)=\infty\) for some \(\Sigma_0\in(0,\infty)\) (precision at
    that fixed dissipation is unbounded on \(\mathcal C_{\rm ring}\), so no
    universal bound of any functional form exists there).

    This claim is restricted to the ring-clock family \(\mathcal C_{\rm
    ring}\) exactly as parametrized by Meier, Minoguchi, Sundelin,
    Apollaro, Erker, Gasparinetti, and Huber, "Precision is not limited by
    the second law of thermodynamics," Nature Physics 21, 1147 (2025)
    [full citation in refs], Eqs. (2)-(3). It is the weaker, precisely
    source-verified reading of the survey lead's broader "any ticking
    clock" phrasing; see known below, "Reading note (scope ambiguity)."
  </claim>

  <def>
    Ring clock (Meier et al. 2025, Eqs. (2)-(3)). Fix \(n\ge2\) and an
    orthonormal basis \(|0\rangle,\ldots,|n-1\rangle\) of \(\mathbb C^n\)
    (the single-excitation subspace of an \(n\)-site chain). For real
    nonzero \(g_0,\ldots,g_{n-2}\), the coherent chain Hamiltonian is
    \[
      H=\sum_{j=0}^{n-2}g_j\,|j\rangle\langle j+1|+\mathrm{h.c.}
    \]
    For \(\Gamma\gt0\), the forward tick jump operator is
    \(J=\sqrt\Gamma\,|0\rangle\langle n-1|\), closing the chain into a ring
    by coupling the last site back to the first. For \(\Sigma_{\rm
    tick}\gt0\), the reverse (anti-tick) jump operator is
    \(\bar J:=e^{-\Sigma_{\rm tick}/2}J^\dagger\); by local detailed
    balance, \(\Sigma_{\rm tick}\) is the entropy produced, in units of
    \(k_B\) (i.e. \(k_B=1\)), by one completed forward tick.

    With \(\rho(0)=|0\rangle\langle0|\), the state evolves by the Lindblad
    (GKSL) master equation
    \[
      \dot\rho=-i[H,\rho]+\mathcal D[J]\rho+\mathcal D[\bar J]\rho,\qquad
      \mathcal D[L]\rho:=L\rho L^\dagger-\tfrac12\{L^\dagger L,\rho\},
    \]
    in units \(\hbar=1\). Every jump of \(J\) is counted as \(+1\) tick and
    every jump of \(\bar J\) as \(-1\) tick; \(N(t)\in\mathbb Z\) is the net
    tick count up to time \(t\) on the corresponding quantum-trajectory
    unraveling. The clock precision is the steady-state inverse Fano
    factor of this counting process,
    \[
      \mathcal N_\Sigma(n,\mathbf g,\Gamma,\Sigma_{\rm
      tick}):=\lim_{t\to\infty}\frac{\mathrm E[N(t)]}{\mathrm{Var}[N(t)]}.
    \]
    Where the parameter list is clear from context, this is abbreviated
    \(\mathcal N_\Sigma\). The class \(\mathcal C_{\rm ring}\) and the
    function \(B(\Sigma)\) are as in claim above.

    Fully dissipative clocks (named exactly this way by Meier et al. 2025,
    Introduction, citing Erker et al. 2017 and Barato-Seifert 2016 below).
    A ticking clock, in the separate formalism of those two papers, whose
    entire dynamics is generated by classical/incoherent (population-only)
    transition rates, with no coherent term analogous to \(H\) above. This
    is not a special case (e.g. not the \(H=0\) case) of \(\mathcal C_{\rm
    ring}\) as fixed here; the known linear bound below is a theorem about
    this separate, externally defined class, recorded as Meier et al.
    (2025) themselves state it, without this entry re-deriving it from
    Erker et al. (2017) or Barato and Seifert (2016) directly (see known,
    residual-doubt note).

    Asymptotic lower-bound notation. For \(f,\phi:(0,\infty)\to(0,\infty)\),
    "\(f=e^{\Omega(\phi(\Sigma))}\) as \(\Sigma\to\infty\)" means: there
    exist \(c\gt0\) and \(\Sigma_0\gt0\) such that \(f(\Sigma)\ge
    e^{c\,\phi(\Sigma)}\) for every \(\Sigma\ge\Sigma_0\) (Knuth's
    asymptotic \(\Omega\), cited by Meier et al. 2025 for their Eq. (1));
    this is a lower-bound statement only and does not by itself imply any
    matching upper bound on \(f\).
  </def>

  <known>
    The linear bound. Meier, Minoguchi, Sundelin, Apollaro, Erker,
    Gasparinetti, and Huber, "Precision is not limited by the second law
    of thermodynamics," Nature Physics 21, 1147 (2025) [full citation in
    refs], Introduction, state (their notation \(\mathcal N\) for
    precision, \(\Sigma_{\rm tick}\) for entropy production per tick,
    matching def above): "For fully dissipative clocks, a linear bound
    \(\mathcal N\le\Sigma_{\rm tick}/2\) tightly bounds the clock
    precision. Such a bound has been confirmed both theoretically and
    experimentally." They attribute the theoretical result to Erker,
    Mitchison, Silva, Woods, Brunner, and Huber, "Autonomous Quantum
    Clocks: Does Thermodynamics Limit Our Ability to Measure Time?,"
    Physical Review X 7, 031022 (2017), and to Barato and Seifert, "Cost
    and precision of Brownian clocks," Physical Review X 6, 041053 (2016)
    [both full citations in refs], and the experimental confirmation to
    Pearson, Guryanova, Erker, Laird, Briggs, Huber, and Ares, "Measuring
    the Thermodynamic Cost of Timekeeping," Physical Review X 11, 021029
    (2021) [full citation in refs]. This linear bound is a theorem about
    the separate "fully dissipative clocks" class (def above), not about
    \(\mathcal C_{\rm ring}\) itself.

    Meier et al. (2025), same Introduction, add that quantum coherence had
    previously been reported to beat this linear bound only in limited
    ways: "In certain quantum scenarios, the linear bound can be beaten by
    using quantum coherence beyond the dissipative regime. So far,
    however, only small theoretical violations have been reported, and
    larger ones remain contested." For the classical dissipative-current
    thermodynamic uncertainty relation (TUR) that their Eq. (1) is
    compared against, they cite Barato and Seifert, "Thermodynamic
    Uncertainty Relation for Biomolecular Processes," Physical Review
    Letters 114, 158101 (2015) [full citation in refs].

    The exponential construction. Meier et al. (2025), Eq. (1), report an
    explicit family of ring clocks within \(\mathcal C_{\rm ring}\) (def
    above) whose precision satisfies
    \[
      \mathcal N_\Sigma=e^{\Omega(\Sigma_{\rm tick})},
    \]
    which they describe as "exponentially surpassing the dissipative
    TUR." By the \(\Omega\)-notation fixed in def above, this means: there
    exist \(c\gt0\), \(\Sigma_0\gt0\) such that for every \(\Sigma\ge
    \Sigma_0\) there is a choice of \((n,\mathbf g,\Gamma)\) with
    \((n,\mathbf g,\Gamma,\Sigma)\in\mathcal C_{\rm ring}\) and
    \(\mathcal N_\Sigma(n,\mathbf g,\Gamma,\Sigma)\ge e^{c\Sigma}\);
    equivalently \(B(\Sigma)\ge e^{c\Sigma}\) for every \(\Sigma\ge
    \Sigma_0\). This is a proved lower bound on \(B(\Sigma)\) for large
    \(\Sigma\); it is not an upper bound and not a finiteness statement
    about \(B(\Sigma)\) at any fixed \(\Sigma\).

    The open status. Meier et al. (2025), Abstract, state verbatim:
    "Classical and quantum models and experiments often show a linear
    relationship between precision and dissipation, but the ultimate
    bounds on this relationship are unknown." Their Introduction poses the
    same question directly in terms of \(B(\Sigma)\) above: "For fixed
    \(\Sigma_{\rm tick}\), one may ask what is the maximum possible clock
    precision. This question relates the fundamental limit of clock
    performance to the second law of thermodynamics." No theorem,
    proposition, or numbered equation anywhere else in the main text or
    appendices of this paper proves an upper bound on \(\mathcal N_\Sigma\)
    for \(\mathcal C_{\rm ring}\), or for any broader clock class. Their
    Sec. III Outlook discusses only a distinct, separate limitation
    (finite lifetime at intermediate ring sites bounding, for a fixed
    finite construction, how long the exponential scaling can be
    maintained in practice), explicitly calling this a technical, not a
    fundamental, limitation; it does not bear on \(B(\Sigma)\) as defined
    above, which is an idealized quantity with dissipation confined to the
    single closing link exactly as in def. No conjecture for the exact
    functional form of an ultimate bound, if one exists, is stated
    anywhere in the paper.

    A second, independent construction. Meier, Minoguchi, Blasi, Haack,
    and Huber, "Exponential gain in clock precision using quantum
    correlated ticks," arXiv:2601.10785 (2026) [full citation in refs],
    report a second, independent construction — submitted (15 Jan 2026)
    about eighteen months after arXiv:2407.07948's own posting (10 Jul
    2024), by an overlapping but different author list, using a different
    physical mechanism
    (autonomous self-correction of consecutive ticks via the Pauli
    exclusion principle among quantum-correlated ticks, rather than the
    ring clock's coherent single-excitation transport) — that likewise
    achieves, in their words, "an exponential advantage in precision."
    This model is not a special case of \(\mathcal C_{\rm ring}\) as fixed
    in def above, so it is recorded here only as corroborating context for
    the general phenomenon (the fully-dissipative linear bound being
    exponentially beaten by more than one construction), not as part of
    this entry's claim domain. Read in full, its Introduction and
    Conclusion state no universal precision-dissipation bound and no
    claim of resolving the open question above, describing their result
    only as "a fundamentally new route to precision."

    Citation correction to the issue-tracker lead. GitHub issue #19
    seeding this entry, and the QTD survey entry it cites, attribute the
    pairing "Meier et al. (Nat. Phys. 2025; arXiv:2601.10785)" as a single
    source. Fetching arXiv:2601.10785 directly shows it is the Meier,
    Minoguchi, Blasi, Haack, and Huber January-2026 preprint described
    above, which carries no journal reference as of 2026-07-18 and cannot
    be the Nature Physics 21, 1147 (2025) publication; that publication's
    own arXiv identifier, confirmed via its arXiv abstract page and its
    DOI-linked journal reference, is arXiv:2407.07948. The two are
    related but distinct papers: three authors (Meier, Minoguchi, and
    Huber) appear on both, out of seven authors total on the 2025 paper
    and five on the 2026 preprint. This entry's refs below cite each under
    its correct identifier. The underlying physical claim in the issue — old
    linear bound superseded by an exponential-scaling construction,
    ultimate bound left open — is corroborated directly by the correctly
    identified Nature Physics 21, 1147 (2025) paper and is not affected by
    this citation-key error.

    Reading note (scope ambiguity). The survey lead and issue #19 phrase
    the target as a bound for "a ticking clock" or "any universal bound"
    generally, not restricted to ring-shaped clocks. This entry's claim
    is deliberately narrower: it is restricted to \(\mathcal C_{\rm
    ring}\), the exact class in which both the linear bound (via the
    disjoint "fully dissipative" sub-case handled by Erker et al. 2017 and
    Barato-Seifert 2016) and its exponential violation (Meier et al. 2025,
    Eq. (1)) are directly established by the sources reviewed here. A
    broader formulation — arbitrary Hermitian \(H\) beyond nearest-neighbor
    coupling, additional dissipative channels, or a genuinely different
    architecture such as the quantum-correlated-tick mechanism of Meier et
    al. (arXiv:2601.10785, 2026) — is a strictly stronger and not
    source-verified claim that this entry does not cover.

    Residual doubt (recorded, not resolved by further search per this
    entry's instructions). The exact inequality \(\mathcal N\le\Sigma_{\rm
    tick}/2\), its precise domain of validity, and its precision-measure
    convention have not been independently confirmed against Erker et al.
    (2017) or Barato and Seifert (2016) directly; this entry relies on
    Meier et al. (2025)'s own restatement of those results in their
    Introduction. Likewise, whether some paper predating 2025 (e.g. Woods,
    Silva, Pütz, Stupar, and Renner, "Quantum Clocks are More Accurate
    Than Classical Ones," PRX Quantum 3, 010319 (2022), listed in Meier et
    al. 2025's own bibliography but not read for this entry) already bears
    on \(B(\Sigma)\) is not independently checked here; Meier et al.
    (2025) is a peer-reviewed 2025 paper by overlapping authors that cites
    this prior work and still states in its Abstract that the ultimate
    bound is unknown, which this entry treats as the operative,
    domain-expert assessment of the field's state.
  </known>

  <refs>
    <ref>F. Meier, Y. Minoguchi, S. Sundelin, T. J. G. Apollaro, P. Erker, S. Gasparinetti, and M. Huber, Precision is not limited by the second law of thermodynamics, Nature Physics 21, 1147-1152 (2025), DOI: 10.1038/s41567-025-02929-2, arXiv:2407.07948, used for: Abstract (statement that the ultimate precision-dissipation bound is unknown); Introduction (definition of the ring clock, Eqs. (2)-(3); the linear bound \(\mathcal N\le\Sigma_{\rm tick}/2\) for fully dissipative clocks and its theoretical/experimental provenance; the question "what is the maximum possible clock precision" for fixed \(\Sigma_{\rm tick}\)); Eq. (1) and surrounding text (the exponential construction \(\mathcal N_\Sigma=e^{\Omega(\Sigma_{\rm tick})}\)); Sec. III Outlook (discussion of a separate, technical finite-lifetime limitation only, not an ultimate bound).</ref>
    <ref>P. Erker, M. T. Mitchison, R. Silva, M. P. Woods, N. Brunner, and M. Huber, Autonomous Quantum Clocks: Does Thermodynamics Limit Our Ability to Measure Time?, Physical Review X 7, 031022 (2017), DOI: 10.1103/PhysRevX.7.031022, used for: the theoretical proof of the linear bound for fully dissipative clocks, as cited by Meier et al. (2025), Introduction (not independently re-read for this entry; see known, residual-doubt note).</ref>
    <ref>A. C. Barato and U. Seifert, Cost and precision of Brownian clocks, Physical Review X 6, 041053 (2016), DOI: 10.1103/PhysRevX.6.041053, used for: the classical theoretical proof of the linear bound for fully dissipative clocks, as cited by Meier et al. (2025), Introduction (not independently re-read for this entry; see known, residual-doubt note).</ref>
    <ref>A. N. Pearson, Y. Guryanova, P. Erker, E. A. Laird, G. A. D. Briggs, M. Huber, and N. Ares, Measuring the Thermodynamic Cost of Timekeeping, Physical Review X 11, 021029 (2021), DOI: 10.1103/PhysRevX.11.021029, used for: the experimental confirmation of the linear bound, as cited by Meier et al. (2025), Introduction.</ref>
    <ref>A. C. Barato and U. Seifert, Thermodynamic Uncertainty Relation for Biomolecular Processes, Physical Review Letters 114, 158101 (2015), DOI: 10.1103/PhysRevLett.114.158101, used for: the dissipative thermodynamic uncertainty relation (TUR) that Meier et al. (2025), Eq. (1), state their exponential construction surpasses.</ref>
    <ref>F. Meier, Y. Minoguchi, G. Blasi, G. Haack, and M. Huber, Exponential gain in clock precision using quantum correlated ticks, arXiv:2601.10785 (2026), no journal reference as of 2026-07-18, used for: the second, independent exponential-precision construction recorded as corroborating context in known above (not part of this entry's claim domain).</ref>
  </refs>

  <ask>
    A proof must exhibit a function \(g:(0,\infty)\to(0,\infty)\) and prove
    \[
      \mathcal N_\Sigma(n,\mathbf g,\Gamma,\Sigma_{\rm tick})\le
      g(\Sigma_{\rm tick})
    \]
    for every \((n,\mathbf g,\Gamma,\Sigma_{\rm tick})\in\mathcal C_{\rm
    ring}\) (def above), i.e. \(B(\Sigma)\le g(\Sigma)\lt\infty\) for every
    \(\Sigma\in(0,\infty)\) — establishing that some universal,
    ring-design-independent bound on precision exists at every fixed
    entropy production per tick. \(g\) need not be tight, minimal, or of
    any particular functional form (it need not itself be linear or
    exponential in \(\Sigma_{\rm tick}\)); any finite-valued \(g\) that
    works for every \(n\), \(\mathbf g\), and \(\Gamma\) suffices. A bound
    proved only for fixed \(n\), only for \(\Gamma\) restricted to a
    bounded range, or only in the \(\Sigma_{\rm tick}\to\infty\)
    asymptotic regime does not establish the claim as quantified above.

    A disproof must exhibit some \(\Sigma_0\in(0,\infty)\) and a sequence
    \((n_k,\mathbf g^{(k)},\Gamma_k,\Sigma_0)\in\mathcal C_{\rm ring}\),
    \(k=1,2,\ldots\), with
    \[
      \lim_{k\to\infty}\mathcal N_\Sigma(n_k,\mathbf
      g^{(k)},\Gamma_k,\Sigma_0)=\infty,
    \]
    proving \(B(\Sigma_0)=\infty\). Because the claim requires a single
    \(g\) finite at every \(\Sigma\), unboundedness at one \(\Sigma_0\)
    already rules out every candidate \(g\) and disproves the claim; no
    argument about other values of \(\Sigma\) is additionally needed.

    The following count as progress, not as a solution to the claim,
    unless combined with a proof removing the stated restriction: (i) an
    upper bound on the achievable asymptotic growth rate of \(B(\Sigma)\)
    as \(\Sigma\to\infty\) (e.g. ruling out super-exponential scaling, or
    pinning the constant \(c\) in \(B(\Sigma)\ge e^{c\Sigma}\)) without
    establishing finiteness of \(B(\Sigma)\) at any fixed, finite
    \(\Sigma\); (ii) a bound proved only for a restricted subfamily of
    \(\mathcal C_{\rm ring}\), such as fixed or bounded \(n\), constrained
    \(\mathbf g\) (e.g. equal couplings), or a fixed ratio between
    \(\Gamma\) and \(\Sigma_{\rm tick}\); (iii) an unboundedness or
    boundedness result for the quantum-correlated-tick construction of
    Meier et al. (arXiv:2601.10785, 2026), or for the general "fully
    dissipative clock" class of Erker et al. (2017) and Barato-Seifert
    (2016), rather than for \(\mathcal C_{\rm ring}\) as fixed in def
    above, unless accompanied by a proof that the result transfers; (iv) a
    bound stated only up to an unspecified or non-explicit constant (e.g.
    "\(B(\Sigma)=O(f(\Sigma))\)" for some named \(f\), without an
    explicit admissible \(g\)), unless the argument is readily made
    explicit and the explicit \(g\) is exhibited.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
