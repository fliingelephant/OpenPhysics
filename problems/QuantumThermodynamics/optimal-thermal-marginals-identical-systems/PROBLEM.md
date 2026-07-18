# QTD019 Existence of Symmetrically Thermalizing Unitaries Beyond Local Dimension Four

```xml
<problem id="QTD019">
  <status>open</status>
  <name>Existence of Symmetrically Thermalizing Unitaries Beyond Local Dimension Four</name>
  <keys>symmetrically thermalizing unitaries STU STUs thermal marginals hotter marginal higher local temperature two identical thermal quantum systems thermodynamically optimal creation of correlations optimally correlating unitaries optimal correlation creation work cost of creating correlations Bakhshinezhad Clivaz Vitagliano Erker Rezakhani Huber Friis locally classical subspaces LCS majorised marginals majorisation majorization passing on the norm geometric approach doubly stochastic matrices unistochastic matrices Gibbs state marginal two qubits two qutrits two ququarts qubit qutrit ququart d=5 dimension five matching Hamiltonians symmetric case correlating unitaries quantum thermodynamics open conjecture mutual information creation cost local dimension d</keys>

  <claim>
    Fix an integer \(d\ge5\). Fix real numbers \(E_0,E_1,\ldots,E_{d-1}\) with
    \(E_0=0\) and \(E_0\le E_1\le\cdots\le E_{d-1}\), and set
    \(H:=\sum_{i=0}^{d-1}E_i|i\rangle\langle i|\) on \(\mathbb C^d\) in a fixed
    orthonormal basis \(\{|i\rangle\}_{i=0}^{d-1}\). Let
    \(\mathcal H_A\cong\mathcal H_B\cong\mathbb C^d\), each carrying a copy of
    \(H\). Then: for every \(\beta\in(0,\infty)\) and every
    \(\beta'\in[0,\beta]\), there exists a unitary \(U_{AB}\) on
    \(\mathcal H_A\otimes\mathcal H_B\) such that
    \[
      \operatorname{Tr}_B\!\bigl[U_{AB}\,(\tau(\beta)\otimes\tau(\beta))\,U_{AB}^\dagger\bigr]
      =\tau(\beta')=
      \operatorname{Tr}_A\!\bigl[U_{AB}\,(\tau(\beta)\otimes\tau(\beta))\,U_{AB}^\dagger\bigr],
    \]
    where \(\tau(\gamma):=e^{-\gamma H}/\operatorname{Tr}(e^{-\gamma H})\) for
    \(\gamma\ge0\) (so \(\tau(0)=\mathbb 1/d\)).
  </claim>

  <def>
    Local dimension and Hamiltonian. \(d\ge5\) is an integer, the common
    dimension of the two identical subsystems. \(0=E_0\le E_1\le\cdots\le
    E_{d-1}\) are real numbers (the ground-state energy is fixed to \(0\)
    without loss of generality; consecutive levels may coincide, i.e.
    degenerate spectra are allowed). \(H:=\sum_{i=0}^{d-1}E_i|i\rangle\langle
    i|\) acts on \(\mathbb C^d\) in the fixed orthonormal basis
    \(\{|i\rangle\}_{i=0}^{d-1}\). \(\mathcal H_A,\mathcal H_B\) are each a
    copy of \(\mathbb C^d\) with the same basis, and
    \(H_A:=\sum_iE_i|i\rangle\langle i|_A\) on \(\mathcal H_A\),
    \(H_B:=\sum_iE_i|i\rangle\langle i|_B\) on \(\mathcal H_B\) are identical
    ("matching") local Hamiltonians under this identification; \(H_A=H_B\)
    denotes this matching, and \(H\) is used generically for either.
    \(\mathcal H_{AB}:=\mathcal H_A\otimes\mathcal H_B\cong\mathbb C^{d^2}\).

    Thermal (Gibbs) state. For \(\gamma\in[0,\infty)\),
    \(\tau(\gamma):=e^{-\gamma H}/Z(\gamma)\), \(Z(\gamma):=
    \operatorname{Tr}(e^{-\gamma H})=\sum_{i=0}^{d-1}e^{-\gamma E_i}\), in
    units with \(k_B=1\), so \(\gamma\) is an inverse temperature and
    \(\tau(0)=\mathbb 1/d\) (maximally mixed, infinite temperature). The
    initial joint state is the product \(\tau_{AB}(\beta):=\tau(\beta)\otimes
    \tau(\beta)\) on \(\mathcal H_{AB}\): both subsystems start in the
    identical thermal state at the same inverse temperature \(\beta\).

    Unitary and marginals. \(U_{AB}\) ranges over the full unitary group on
    \(\mathcal H_{AB}\) (any \(d^2\times d^2\) unitary matrix); no constraint
    ties \(U_{AB}\) to \(H_A,H_B\), or to any bath, and in particular
    \(U_{AB}\) need not commute with \(H_A\otimes\mathbb 1+\mathbb 1\otimes
    H_B\) (this is the fully-controlled, no-bath setting used to derive
    optimal/idealized bounds, not a thermal operation in the sense of an
    energy-conserving unitary on system plus explicit bath). Given
    \(U_{AB}\), the post-unitary marginals are
    \(\tilde\varrho_A:=\operatorname{Tr}_B[U_{AB}\,\tau_{AB}(\beta)\,
    U_{AB}^\dagger]\) on \(\mathcal H_A\) and
    \(\tilde\varrho_B:=\operatorname{Tr}_A[U_{AB}\,\tau_{AB}(\beta)\,
    U_{AB}^\dagger]\) on \(\mathcal H_B\), with \(\operatorname{Tr}_A,
    \operatorname{Tr}_B\) the partial traces over \(\mathcal H_A,\mathcal
    H_B\).

    Symmetrically thermalizing unitary (STU). For fixed \(H\), \(\beta\), and
    \(\beta'\in[0,\beta]\), a unitary \(U_{AB}\) is a symmetrically
    thermalizing unitary (the term used by the source below) if
    \(\tilde\varrho_A=\tau(\beta')=\tilde\varrho_B\), i.e. both marginals are
    exactly thermal at the (weakly) higher local temperature
    \(T'=1/\beta'\ge T=1/\beta\) (with \(T'=\infty\) understood at
    \(\beta'=0\)). The claim above asserts that STUs exist for every
    admissible \(H\), \(\beta\), \(\beta'\), at every local dimension
    \(d\ge5\).
  </def>

  <known>
    Bakhshinezhad, Clivaz, Vitagliano, Erker, Rezakhani, Huber, and Friis,
    "Thermodynamically optimal creation of correlations," Journal of Physics
    A: Mathematical and Theoretical 52, 465303 (2019), Section II.1.2
    ("Symmetric case"), Eq. (11), pose exactly the existence question above,
    their "Question 2: Existence of STUs": "Does there exist a unitary
    \(U_{AB}\) on \(\mathcal H_{AB}\) such that
    \(\tilde\varrho_A=\operatorname{Tr}_B(U_{AB}\tau_{AB}(\beta)U_{AB}^\dagger)
    =\tau_A(\beta')\),
    \(\tilde\varrho_B=\operatorname{Tr}_A(U_{AB}\tau_{AB}(\beta)U_{AB}^\dagger)
    =\tau_B(\beta')\), for every pair of local Hamiltonians \(H_A=H_B\), for
    all final temperatures \(T'=1/\beta'\) and all initial temperatures
    \(T=1/\beta\le T'\)?" -- posed there without an explicit lower bound on
    \(d\). Bakhshinezhad et al. resolve \(d\in\{3,4\}\) within the same
    paper (below); \(d=2\) is resolved only by prior work that they cite
    there (below), not by an independent derivation of their own. In their
    Conclusions (Section IV), they restate the residual case as a
    conjecture over "all local dimensions." Since \(d\le4\) is already
    settled -- \(d=2\) by the cited prior result, \(d=3\) and \(d=4\) by
    their own theorems -- that conjecture has content only for \(d\ge5\);
    the claim above restricts to exactly this residual scope (noted here
    as the one place this entry narrows the source's literal quantifier,
    per the weaker-precise-claim preference).

    \(d=2\) is resolved prior to this paper: Bakhshinezhad et al., Section
    II.1.2 ("Symmetric case"), report that the existence question "can be
    answered affirmatively when the subsystem Hamiltonians are equally spaced,"
    citing Huber, Perarnau-Llobet, Hovhannisyan, Skrzypczyk, Brunner, and
    Acín, "Thermodynamic cost of creating correlations," New Journal of
    Physics 17, 065008 (2015) (their Ref. [11]), and state "this implies
    that such optimally correlating unitaries always exist for two qubits,
    i.e., when \(d=2\)" -- because a single energy gap is trivially
    "equally spaced," this covers every Hamiltonian at \(d=2\), not only
    equally-spaced ones.

    \(d=3\) is resolved unconditionally: Bakhshinezhad et al., Section III.1
    ("Majorised marginals approach"), Theorem 1: "For every pair of states
    \(\varrho\) and \(\bar\varrho\) in a 3-dimensional Hilbert space which
    satisfy the condition \(\lambda(\bar\varrho)\prec\lambda(\varrho)\)
    [\(\lambda\) the eigenvalue vector, \(\prec\) majorisation], there
    exists a unitary \(U_{AB}\) on \(\mathcal H_{AB}\) such that
    \(\tilde\varrho_A=\operatorname{Tr}_B(U_{AB}\varrho\otimes\varrho\,
    U_{AB}^\dagger)=\bar\varrho\),
    \(\tilde\varrho_B=\operatorname{Tr}_A(U_{AB}\varrho\otimes\varrho\,
    U_{AB}^\dagger)=\bar\varrho\)." Immediately after the proof, Bakhshinezhad
    et al. draw the corollary used here: "the existence of STUs is proven
    for the two-qutrit case for initial and final thermal states with
    inverse temperatures \(\beta\) and \(\beta'\le\beta\) ... since
    \(p(\beta')\prec p(\beta)\) holds whenever \(\beta'\le\beta\)" -- i.e.
    every Hamiltonian, unconditionally.

    \(d=4\) is resolved unconditionally only via a second route:
    Bakhshinezhad et al., Section III.2 ("passing on the norm"), Theorem 2,
    prove existence only "for every set of energy eigenvalues
    \(\{E_i\}_{i=0}^3\) with \(E_{i+1}\ge E_i\) and \(\delta_{i+1}\le
    \delta_i\)" (energy gaps \(\delta_i:=E_{i+1}-E_i\) required
    non-increasing) -- a genuine restriction on \(H\). Section III.3
    ("Geometric approach"), Theorem 3, removes this restriction: "In
    \(d=3\) and \(d=4\) systems, for every choice of Hamiltonians and
    initial inverse temperature \(\beta\), the set of thermal states with
    \(\beta'\le\beta\) is contained within the polytope with vertices
    defined in Eq. (59), which proves the existence of STUs in the
    symmetric two-qutrit and two-ququart cases." Theorem 3 is the
    unconditional resolution of both \(d=3\) and \(d=4\); Theorem 1 and
    Theorem 2 give two further, independent (and for \(d=4\), weaker or
    equal) routes to the same or a subset of that conclusion.

    Bakhshinezhad et al., Section IV ("Conclusions"), summarize: "we have
    shown that STUs exist for all (locally matching) Hamiltonians in local
    dimensions \(d=3\) and \(d=4\), and we conjecture that STUs exist in
    all local dimensions." This sentence is the direct source of the claim
    above. No proof, sufficient condition, or general-\(d\) argument is
    given anywhere in the paper for \(d\ge5\); it is presented purely as an
    open conjecture.

    Even \(d=5\) is only partially resolved: Section III.3 ("Geometric
    approach"), in the paragraph immediately preceding the Section IV
    heading, reports "we outline a possible route for such an approach for
    the case \(d=5\) ... where we show the existence of STUs for \(d=5\)
    for a subset of all possible Hamiltonians" (their Appendix A.XIII).
    This is a restricted-Hamiltonian
    partial result, not a resolution of the \(d=5\) instance of the claim
    above (which quantifies over every admissible \(H\)); the claim is thus
    open already at \(d=5\).

    A separate, already-resolved variant should not be confused with the
    claim above: Bakhshinezhad et al., Section II.2.3 ("Asymptotic case"),
    show that if each of \(A\) and \(B\) is replaced by \(n\) identical
    copies of the \(d\)-dimensional subsystem, then STUs relating
    \(\tau_{AB}(\beta)^{\otimes n}\) to marginals \(\tau(\beta')^{\otimes
    n}\) exist "for all \(\beta',\beta\) such that \(\beta'\le\beta\), and
    for all local Hamiltonians," at every \(d\), in the limit \(n\to
    \infty\). This resolves only the many-copy asymptotic limit; it does
    not bear on the claim above, which is the \(n=1\) (single copy of each
    subsystem) case for finite \(d\ge5\).

    The PDF read for this entry is arXiv:1904.07942v2 (23 October 2019),
    matching the published Journal of Physics A version (DOI
    10.1088/1751-8121/ab3932); no later corrective revision was consulted.
    Within this source, the claim above is stated only as an unproven
    conjecture, with no internal proof sketch, sufficient condition, or
    numerical evidence covering general \(d\ge5\).
  </known>

  <refs>
    <ref>F. Bakhshinezhad, F. Clivaz, G. Vitagliano, P. Erker, A. T.
    Rezakhani, M. Huber, and N. Friis, "Thermodynamically optimal creation
    of correlations," Journal of Physics A: Mathematical and Theoretical 52,
    465303 (2019). DOI: 10.1088/1751-8121/ab3932. arXiv:1904.07942v2. Used
    for: Question 2 (Section II.1.2, Eq. (11)); Theorem 1 (Section III.1,
    "Majorised marginals approach", and its corollary establishing STUs for
    \(d=3\)); Theorem 2 (Section III.2, "passing on the norm", conditional
    \(d=4\) result, Eq. (56)); Theorem 3 (Section III.3, "Geometric
    approach", unconditional \(d=3\) and \(d=4\) result, Eq. (59)), and, in
    that same section's final paragraph, the report of partial,
    Hamiltonian-subset progress at \(d=5\) (Appendix A.XIII); Section
    IV ("Conclusions"), the sentence "we conjecture that STUs exist in all
    local dimensions"; Section II.2.3 ("Asymptotic
    case"), the many-copy (\(n\to\infty\)) result distinguished above from
    the claim; and Section II.1.2 ("Symmetric case"), the report of the
    pre-existing \(d=2\) result.</ref>
    <ref>M. Huber, M. Perarnau-Llobet, K. V. Hovhannisyan, P. Skrzypczyk, N.
    Brunner, and A. Acín, "Thermodynamic cost of creating correlations," New
    Journal of Physics 17, 065008 (2015). arXiv:1404.2169. Cited by
    Bakhshinezhad et al. (2019), Section II.1.2 (their Ref. [11]), as the source
    proving the existence question affirmatively for equally-spaced-gap
    Hamiltonians, which trivially covers every Hamiltonian at \(d=2\) (a
    single energy gap) and thus establishes the \(d=2\) instance of the
    claim's underlying question.</ref>
  </refs>

  <ask>
    A proof must establish the claim exactly as stated: for every integer
    \(d\ge5\), every admissible Hamiltonian \(H\) (every non-decreasing
    sequence \(0=E_0\le E_1\le\cdots\le E_{d-1}\)), and every
    \(\beta\in(0,\infty)\), \(\beta'\in[0,\beta]\), prove the existence of a
    unitary \(U_{AB}\) on \(\mathbb C^d\otimes\mathbb C^d\) with
    \(\tilde\varrho_A=\tau(\beta')=\tilde\varrho_B\); either by extending
    one of Bakhshinezhad et al.'s three approaches (majorised marginals,
    passing on the norm, geometric) to arbitrary \(d\ge5\), or by any other
    valid construction or existence argument.

    A disproof requires only a single counterexample instance: one integer
    \(d\ge5\), one admissible energy sequence \(E_0,\ldots,E_{d-1}\), and
    one pair \(\beta\in(0,\infty)\), \(\beta'\in[0,\beta]\), together with a
    proof that no unitary on \(\mathbb C^d\otimes\mathbb C^d\) produces
    \(\tilde\varrho_A=\tilde\varrho_B=\tau(\beta')\). Because the claim
    universally quantifies over \(d,H,\beta,\beta'\), one such failing
    instance disproves it; the claim already excludes \(d\le4\) (proved by
    Bakhshinezhad et al.), so only a counterexample at \(d\ge5\) is
    relevant.

    The following count as progress, not as a full solution: (i) resolving
    a single dimension \(d\ge5\) for every Hamiltonian (e.g. completing
    \(d=5\) beyond the Hamiltonian-subset result in Bakhshinezhad et al.'s
    Appendix A.XIII) without a construction valid for every \(d\ge5\); (ii)
    existence results restricted to a Hamiltonian subclass (e.g.
    equally-spaced levels, or monotone energy gaps as in Theorem 2's
    \(d=4\) route) at fixed or all \(d\ge5\), rather than every admissible
    \(H\); (iii) results only in the many-copy asymptotic limit
    (\(n\to\infty\) copies of each subsystem, already resolved by
    Bakhshinezhad et al.'s Section II.2.3 for every \(d\)) or only as
    \(d\to\infty\), rather than for finite fixed \(d\ge5\) with \(n=1\);
    (iv) numerical or computational evidence for specific finite instances
    \((d,H,\beta,\beta')\) without an analytic existence argument covering
    an infinite family.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
