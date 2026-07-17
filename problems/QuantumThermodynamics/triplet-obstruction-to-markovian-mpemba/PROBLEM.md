# QTD034 Triplet Obstruction to the Markovian Mpemba Effect

```xml
<problem id="QTD034">
  <status>open</status>
  <name>Triplet Obstruction to the Markovian Mpemba Effect</name>
  <keys>mpemba effect markovian mpemba effect quantum mpemba effect anomalous relaxation anomalous thermalization thermalization crossing relaxation crossing overtaking detailed balance master equation pauli master equation classical master equation rate equation transition rates energy triplet three-level system three level system N-level system slowest eigenmode second eigenmode b2 coefficient spectral gap necessary condition obstruction triplet condition strong violation weak violation zero-temperature theorem finite-temperature conjecture Avitan Factor Gelbwaser-Klimovsky Gelbwaser-Klimovsky inverse temperature quasistatic locus geometric criterion 2026 triplet conjecture checkable criterion</keys>

  <claim>
    Avitan, Factor, and Gelbwaser-Klimovsky, "Necessary conditions for the
    Markovian Mpemba effect," arXiv:2603.04567 (2026), state as a conjecture,
    immediately after their numerical tests (subsection "Numerics for
    non-zero temperature baths"): for every admissible detailed-balance
    \(N\)-level system \(\mathcal S=(N,(E_l)_{l=1}^N,(a_{ij})_{i\neq j\in
    [N]},\beta_b)\) with \(N\ge3\) (def below), if every triplet \(T\in
    \binom{[N]}{3}\) violates condition (3) (i.e. \(\neg\mathrm{Sat}(T)\)
    for every \(T\), def below), then \(\mathrm{Mpemba}(\mathcal S)\) does
    not hold.

    The \(N=3\) instance of this statement is already established at every
    \(\beta_b\in(0,\infty)\) (known fact 1), and the zero-temperature limit
    \(\beta_b\to\infty\) is already established for every \(N\) under a
    strictly stronger hypothesis (known fact 2). The mathematically open
    content is the case \(N\ge4\) at finite \(\beta_b\in(0,\infty)\), under
    the plain-violation hypothesis stated above; see known, "Reading note
    (scope of the open claim)."
  </claim>

  <def>
    Write \([N]:=\{1,\ldots,N\}\). A population vector is \(p(t)=(p_1(t),
    \ldots,p_N(t))\in\Delta^{N-1}:=\{x\in\mathbb R^N_{\ge0}:\sum_lx_l=1\}\).
    For nonnegative rates \((a_{ij})_{i\neq j\in[N]}\) (\(a_{ij}\) is the
    rate of the jump \(j\to i\), i.e. the rate at which population flows
    into level \(i\) from level \(j\)), the generator \(M\in\mathbb
    R^{N\times N}\) is \(M_{ii}:=-\sum_{k\neq i}a_{ki}\), \(M_{ij}:=a_{ij}\)
    (\(i\neq j\)), and the master equation is \(\dot p(t)=Mp(t)\).

    A detailed-balance \(N\)-level system is a tuple \(\mathcal S=(N,
    (E_l)_{l=1}^N,(a_{ij})_{i\neq j\in[N]},\beta_b)\) with: non-degenerate
    energies \(E_1\lt E_2\lt\cdots\lt E_N\); a bath inverse temperature
    \(\beta_b\in(0,\infty)\); rates obeying detailed balance, \(a_{ij}=
    a_{ji}\,e^{-\beta_b(E_i-E_j)}\) for every ordered pair \(i\neq j\) in
    \([N]\); and, as an \emph{admissibility} requirement, the generator
    \(M\) built from these rates has \(N\) pairwise-distinct real
    eigenvalues (real spectrum is automatic under detailed balance, by
    symmetrization with the equilibrium measure; distinctness is an
    additional, genuine restriction, matching the explicit hypothesis "M
    eigenvalues are not degenerate" imposed by the source's own
    zero-temperature theorem, known fact 2). "Admissible" below always
    means this full tuple of requirements, with \(N\ge3\) throughout.

    The Gibbs (thermal) state at inverse temperature \(\beta\) is \(\Pi
    (\beta)_l:=e^{-\beta E_l}/Z(\beta)\), \(Z(\beta):=\sum_{m=1}^Ne^{-\beta
    E_m}\); \(\Pi(\beta_b)\) is the eigenvector of \(M\) for eigenvalue
    \(0\), and \(0\) is the largest eigenvalue of \(M\) (standard fact for
    an irreducible detailed-balance generator). Write the full spectrum,
    under admissibility, as \(\lambda_1=0\gt\lambda_2\gt\cdots\gt\lambda_N\),
    with right eigenvectors \(\Pi(\beta_b),V_2,\ldots,V_N\), each \(V_i\)
    fixed with an arbitrary nonzero normalization (every statement below is
    invariant under rescaling any single \(V_i\) by a nonzero constant,
    since that rescales the corresponding \(b_i(\cdot)\) below by the same
    \(\beta_{in}\)-independent constant).

    For \(\beta_{in}\in(0,\infty)\), define \(b_2(\beta_{in}),\ldots,
    b_N(\beta_{in})\in\mathbb R\) by expanding the initial Gibbs state in
    the eigenbasis of \(M\),
    \[
      \Pi(\beta_{in}) = \Pi(\beta_b) + \sum_{i=2}^N b_i(\beta_{in})\,V_i .
    \]
    The Mpemba-effect predicate (Avitan, Factor, and
    Gelbwaser-Klimovsky's own criterion) is: \(\mathrm{Mpemba}(\mathcal S)\)
    holds iff there exists \(\beta_{in}\in(0,\infty)\), \(\beta_{in}\neq
    \beta_b\), with
    \[
      \frac{d}{d\beta_{in}}\bigl|b_2(\beta_{in})\bigr| = 0 .
    \]

    Triplet notation: for \(T=\{l_1,l_2,l_3\}\subseteq[N]\) (any
    \(3\)-element subset; \(\binom{[N]}{3}\) denotes the set of all such
    subsets, of which there are \(\binom N3\)), relabel so \(E_{l_1}\lt
    E_{l_2}\lt E_{l_3}\) and set
    \[
      a^T_{-+}:=a_{l_1l_3},\qquad a^T_{0+}:=a_{l_2l_3},\qquad
      a^T_{-0}:=a_{l_1l_2},
    \]
    \[
      \Delta^T_{0-}:=E_{l_2}-E_{l_1}\gt0,\quad
      \Delta^T_{+-}:=E_{l_3}-E_{l_1}\gt0,\quad
      \Delta^T_{+0}:=E_{l_3}-E_{l_2}\gt0,
    \]
    \[
      C^T_{+-}:=1+\tfrac12e^{-\beta_b\Delta^T_{+-}},\qquad
      C^T_{+0}:=1+\tfrac12e^{-\beta_b\Delta^T_{+0}},\qquad
      k_T:=\frac{a^T_{-+}+a^T_{0+}}{a^T_{-0}\bigl(1+e^{-\beta_b
      \Delta^T_{0-}}\bigr)} .
    \]
    Triplet \(T\) satisfies condition (3) [Avitan, Factor, and
    Gelbwaser-Klimovsky's own equation number], written \(\mathrm{Sat}
    (T)\), iff
    \[
      \Bigl[k_T\ge1\ \text{and}\ a^T_{-+}C^T_{+-}-a^T_{0+}C^T_{+0} \gt
      a^T_{-0}\bigl(1-e^{-\beta_b\Delta^T_{0-}}\bigr)\Bigr]
      \ \ \text{or}\ \
      \Bigl[k_T\le1\ \text{and}\ a^T_{-+}e^{-\beta_b\Delta^T_{0-}} \gt
      a^T_{0+}\Bigr] .
    \]
    Triplet \(T\) violates condition (3) iff \(\neg\mathrm{Sat}(T)\).
    (At \(k_T=1\) both bracketed clauses are evaluated, matching the
    source's own closed inequalities \(k\ge1\), \(k\le1\) on the two
    branches.)
  </def>

  <known>
    1. Established necessity for \(N=3\), at every finite \(\beta_b\)
    (Avitan, Factor, and Gelbwaser-Klimovsky, subsection "3-Level system
    necessary conditions"). For \(N=3\) (so \([N]\) has the single triplet
    \(T=[N]\)), fulfilment of condition (3) is derived as a necessary
    condition for \(\mathrm{Mpemba}\) to hold, via a geometric argument
    identifying \(\mathrm{Mpemba}\) with tangency of the eigenvector
    \(V_3\) to a "quasistatic locus" in a two-simplex representation of the
    dynamics; the source states explicitly that fulfilment of condition
    (3) "is necessary for the Mpemba effect" and becomes sufficient "only"
    in a stated further asymptotic regime (sufficiency is not used here).
    Equivalently, \(\neg\mathrm{Sat}(T)\Rightarrow\neg\mathrm{Mpemba}
    (\mathcal S)\) for every admissible \(3\)-level \(\mathcal S\) and
    every \(\beta_b\in(0,\infty)\) — exactly the \(N=3\) instance of claim
    above, already established directly rather than conjecturally.

    2. Proven zero-temperature theorem for general \(N\) (Avitan, Factor,
    and Gelbwaser-Klimovsky, subsection "Analytical proof for
    zero-temperature baths"). In the \(\beta_b\to\infty\) limit (bath at
    zero temperature; \(M\) becomes lower-triangular in energy order,
    since every upward rate \(a_{ij}\) with \(E_i\gt E_j\) vanishes as
    \(\beta_b\to\infty\)), if \(M\)'s eigenvalues are non-degenerate and
    every triplet \(T\) \emph{strongly violates} condition (3), meaning
    \(a^T_{-0}\gt a^T_{-+}+a^T_{0+}\), then the \(N\)-level system does
    not exhibit \(\mathrm{Mpemba}\). Strong violation is a strictly
    stronger hypothesis than plain violation \(\neg\mathrm{Sat}(T)\):
    substituting \(\beta_b\to\infty\) into \(\mathrm{Sat}(T)\)'s
    \(k_T\ge1\) branch gives \(C^T_{+-},C^T_{+0}\to1\), so that branch
    reduces to \(a^T_{-+}-a^T_{0+}\gt a^T_{-0}\), whose negation is only
    \(a^T_{-+}\le a^T_{-0}+a^T_{0+}\) — weaker than \(a^T_{-0}\gt
    a^T_{-+}+a^T_{0+}\); the \(k_T\le1\) branch is automatically false as
    \(\beta_b\to\infty\), since its right side would require \(0\gt
    a^T_{0+}\ge0\). (This zero-temperature reduction is carried out here
    from the source's own condition (3) and is not stated in this closed
    form by the source itself.)

    3. Numerical evidence offered for the finite-temperature conjecture
    (Avitan, Factor, and Gelbwaser-Klimovsky, subsection "Numerics for
    non-zero temperature baths"). The conjecture in claim above is stated
    immediately after, and on the basis of, numerical tests on
    randomly-sampled \(N=4\)-to-\(7\)-level systems (rates and energies
    drawn at random, bath parameters scanned up to \(\beta_bE_i=11\)) that
    found no violation of the implication in claim above; the source's own
    text introduces the general, finite-\(\beta_b\) statement with "we
    conjecture," not "we prove" or "we show," in explicit contrast to its
    own "Theorem" label on known fact 2.

    4. Reading note (scope of the open claim). The mathematically new
    content of claim above is exactly \(N\ge4\) at finite \(\beta_b\in
    (0,\infty)\): the \(N=3\) case is covered unconditionally in
    \(\beta_b\) by known fact 1, and the \(\beta_b\to\infty\) endpoint is
    covered for every \(N\) by known fact 2, but only under the strictly
    stronger "strongly violated" hypothesis. A full proof of claim above
    must in particular handle every \(N\ge4\) at every finite \(\beta_b\in
    (0,\infty)\) under the weaker plain-"violates condition (3)"
    hypothesis; see ask.

    5. Residual doubt (recorded, not resolved by further search, per this
    entry's instructions). (i) No search was performed for work after this
    paper's 4 March 2026 posting that might already prove or disprove the
    \(N\ge4\), finite-\(\beta_b\) conjecture; the survey stage is treated
    as having already screened for this. (ii) The algebraic form of
    condition (3), the definitions of \(C_{ij}\) and \(k\), and the exact
    wordings quoted in known facts 1-3 were read from the arXiv HTML
    rendering of the paper via automated extraction, not from the original
    PDF/LaTeX source; the extraction was checked once for internal
    consistency, by verifying that the \(\beta_b\to\infty\) limit of
    condition (3) reduces exactly to known fact 2's "strongly violated"
    inequality (done in known fact 2 above), but the equation was not
    independently cross-checked against the PDF itself. (iii) The domain
    \(\beta_{in}\in(0,\infty)\) for the initial inverse temperature, and
    the implicit allowance for any pair of levels to carry a (possibly
    zero) direct rate consistent with detailed balance, were taken as the
    paper's standing setup rather than re-confirmed verbatim against the
    source. (iv) Known fact 1's geometric derivation for \(N=3\) was read
    only through the same HTML-extraction channel and not independently
    re-derived; its companion sufficiency clause ("sufficient only for
    \(r\to\infty\)") is noted but its parameter \(r\) is not otherwise used
    in this entry, since only the necessity direction is needed for claim
    above.
  </known>

  <refs>
    <ref>I. Avitan, R. Factor, and D. Gelbwaser-Klimovsky, Necessary
    conditions for the Markovian Mpemba effect, arXiv:2603.04567
    [cond-mat.stat-mech, quant-ph] (2026), DOI:
    10.48550/arXiv.2603.04567, used for: the master equation and generator
    \(M\) (subsection "N-Level system thermalization"); the eigenmode
    decomposition of the initial Gibbs state and the \(|b_2|\)-derivative
    criterion for the Mpemba effect (same subsection); the three-level
    rate condition and its two branches, labeled equation (3), together
    with the definitions of \(C_{ij}\) and \(k\) and the adjoining
    necessity/sufficiency statement (subsection "3-Level system necessary
    conditions"), used in def and known fact 1; the zero-temperature
    theorem for general \(N\)-level systems (subsection "Analytical proof
    for zero-temperature baths"), used in known fact 2; and the
    finite-temperature conjecture for general \(N\)-level systems together
    with its supporting numerical tests (subsection "Numerics for
    non-zero temperature baths"), used in claim and known fact 3.</ref>
  </refs>

  <ask>
    A proof of the claim must show, for every integer \(N\ge4\) and every
    finite \(\beta_b\in(0,\infty)\) (the case \(N=3\) being already covered
    by known fact 1, and \(\beta_b\to\infty\) by known fact 2), that for
    every admissible detailed-balance \(N\)-level system \(\mathcal S\),
    if every triplet \(T\in\binom{[N]}3\) violates condition (3) then
    \(\mathrm{Mpemba}(\mathcal S)\) does not hold. Recovering only \(N=3\),
    or only the \(\beta_b\to\infty\) limit, does not establish the claim,
    since known facts 1-2 already give exactly these. A proof that
    substitutes the strictly stronger "strongly violated" hypothesis of
    known fact 2 (\(a^T_{-0}\gt a^T_{-+}+a^T_{0+}\) for every \(T\)) for
    plain violation of condition (3) settles a different, a priori easier
    statement, not this claim.

    A disproof must exhibit a single explicit counterexample: an integer
    \(N\ge4\), non-degenerate energies, and detailed-balance rates at some
    finite \(\beta_b\in(0,\infty)\), forming an admissible \(\mathcal S\)
    (\(M\)'s eigenvalues pairwise distinct), such that every triplet
    \(T\in\binom{[N]}3\) violates condition (3) (i.e. \(\neg\mathrm{Sat}
    (T)\) for every \(T\)) and yet \(\mathrm{Mpemba}(\mathcal S)\) holds
    (some \(\beta_{in}\in(0,\infty)\), \(\beta_{in}\neq\beta_b\), with
    \(\frac{d}{d\beta_{in}}|b_2(\beta_{in})|=0\)).

    The following count as progress, not as a resolution of the claim,
    unless combined with a matching general argument as above: (i) a proof
    or disproof restricted to \(N=3\) (already known fact 1) or to the
    \(\beta_b\to\infty\) limit (already known fact 2); (ii) a proof under
    the strictly stronger "strongly violated" hypothesis rather than plain
    violation of condition (3); (iii) additional numerical tests, however
    extensive, without an accompanying proof; (iv) a proof restricted to a
    structurally special sub-family of rate networks (e.g.
    nearest-neighbor-only coupling, or a specific system-bath spectral
    density such as Ohmic or sub-Ohmic) rather than the general
    detailed-balance \(N\)-level system of def above.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
