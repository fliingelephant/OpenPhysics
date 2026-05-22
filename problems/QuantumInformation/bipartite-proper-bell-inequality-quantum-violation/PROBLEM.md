# QF026 Bipartite proper Bell inequalities and quantum violation

```xml
<problem id="QF026">
  <status>open</status>
  <name>Bipartite proper Bell inequalities and quantum violation</name>
  <keys>bell inequalities holding for all quantum states oqp problem 26 proper bell inequality tight bell inequality facet bell inequality bipartite bell scenario local polytope classical polytope no-signalling polytope no signaling polytope quantum correlations quantum body quantum violation finite-dimensional tensor product correlations almost quantum correlations gyni guess your neighbors input masanes vertesi pal almeida bancal brunner acin gisin pironio slofstra quantum foundations</keys>

  <claim>
    For every pair of positive integers \(M,K\) and every proper facet Bell
    inequality of the bipartite local correlation polytope with two parties,
    \(M\) measurement choices per party, and \(K\) outcomes per measurement,
    finite-dimensional tensor-product quantum correlations strictly violate
    that inequality.
  </claim>

  <def>
    For positive integers \(M,K\), the uniform finite bipartite Bell scenario
    has input sets \(X=Y=[M]\) and output sets \(A=B=[K]\), where
    \([r]=\{1,\ldots,r\}\). Alice receives \(x\in[M]\) and returns
    \(a\in[K]\); Bob receives \(y\in[M]\) and returns \(b\in[K]\). A behavior is
    an array \(p=\{p(a,b|x,y)\}_{a,b,x,y}\) satisfying
    \[
      \sum_{a,b\in[K]}p(a,b|x,y)=1,\qquad x,y\in[M] .
    \]

    The no-signalling affine space is the set of normalized behaviors such
    that, for all \(a,b\in[K]\) and \(x,y,y',x'\in[M]\),
    \[
      \sum_{b\in[K]}p(a,b|x,y)=\sum_{b\in[K]}p(a,b|x,y'),
      \qquad
      \sum_{a\in[K]}p(a,b|x,y)=\sum_{a\in[K]}p(a,b|x',y).
    \]
    The no-signalling polytope \(P_{M,K}\) is the intersection of this affine
    space with the positivity constraints \(p(a,b|x,y)\ge 0\).

    The local polytope \(C_{M,K}\) is the convex hull of deterministic local
    behaviors. For functions \(f:[M]\to[K]\) and \(g:[M]\to[K]\), the
    deterministic behavior \(d^{f,g}\) is
    \[
      d^{f,g}(a,b|x,y)
      =
      \mathbf 1\{a=f(x)\}\mathbf 1\{b=g(y)\}.
    \]
    Thus
    \[
      C_{M,K}
      =
      \operatorname{conv}\{d^{f,g}:f:[M]\to[K],\ g:[M]\to[K]\}.
    \]

    The finite-dimensional tensor-product quantum set
    \(\mathcal Q_{\mathrm{fd}}(M,K)\) consists of all behaviors for which
    there are finite-dimensional Hilbert spaces
    \(\mathcal H_A,\mathcal H_B\), a density operator \(\rho\) on
    \(\mathcal H_A\otimes\mathcal H_B\), POVMs
    \(\{E_a^x\}_{a\in[K]}\) on \(\mathcal H_A\) for each \(x\in[M]\), and
    POVMs \(\{F_b^y\}_{b\in[K]}\) on \(\mathcal H_B\) for each \(y\in[M]\),
    such that
    \[
      p(a,b|x,y)
      =
      \operatorname{Tr}\!\left(\rho(E_a^x\otimes F_b^y)\right).
    \]
    Write \(\overline{\mathcal Q_{\mathrm{fd}}}\) for its Euclidean closure in
    the no-signalling affine space. For the strict-violation question in this
    entry, \(\mathcal Q_{\mathrm{fd}}\) and
    \(\overline{\mathcal Q_{\mathrm{fd}}}\) are equivalent: if a continuous
    linear functional has value strictly larger than the local bound on the
    closure, then some finite-dimensional quantum behavior already has value
    strictly larger than the local bound.

    A Bell functional is a real linear functional
    \[
      \ell(p)=\sum_{x,y\in[M]}\sum_{a,b\in[K]} c_{a,b,x,y}p(a,b|x,y).
    \]
    Let
    \[
      \beta_C=\max_{p\in C_{M,K}}\ell(p),
      \qquad
      \beta_P=\max_{p\in P_{M,K}}\ell(p).
    \]
    The inequality \(\ell(p)\le \beta_C\) is facet-defining for
    \(C_{M,K}\) when
    \(C_{M,K}\cap\{p:\ell(p)=\beta_C\}\) is a facet relative to
    \(\operatorname{aff}(C_{M,K})\). It is proper when it is facet-defining for
    \(C_{M,K}\) and \(\beta_P\gt\beta_C\). Equivalently,
    it is a tight local Bell inequality that is not valid for the
    no-signalling polytope and hence is not induced by positivity together with
    normalization and no-signalling.
  </def>

  <known>
    This entry was collected from Open Quantum Problems, Problem 26, "Bell
    inequalities holding for all quantum states". The Open Quantum Problems
    index lists Problem 26 in the Quantum foundations category. OQP Problem 26
    has two subproblems: 26.A asks whether nontrivial boundary points of the
    quantum body can always be reached with local Hilbert spaces of outcome
    dimension and complete von Neumann measurements, while 26.B asks whether
    every proper Bell inequality can be violated by quantum correlation data.
    This entry records only the remaining bipartite part of 26.B.

    OQP Problem 26 uses the same uniform \(N,M,K\) finite-scenario convention as
    OQP Problem 1, "All the Bell inequalities". The claim here is therefore
    restricted to the \(N=2\) case of that convention and does not include
    asymmetric bipartite scenarios with party-dependent input or output sets.

    Open Quantum Problems, Problem 26, records that Masanes,
    "Extremal quantum correlations for N parties with two dichotomic
    observables per site", arXiv:quant-ph/0512100, proves that in the
    \((N,2,2)\) dichotomic-observable setting all extreme quantum correlations are
    attainable by measuring \(N\)-qubit pure states with projective observables,
    and that any \(N\)-partite state violating a Bell inequality in that setting
    can be locally transformed to an \(N\)-qubit state with no smaller
    violation. This controls the minimal-dimension issue for that special
    two-observable, two-outcome setting; it does not classify or settle all
    proper bipartite facets in the full conditional-probability polytope.

    OQP Problem 26 records that 26.A was answered negatively by Vertesi and
    Pal, "Generalized Clauser-Horne-Shimony-Holt inequalities maximally
    violated by higher dimensional systems", Physical Review A 77, 042106
    (2008). Their generalized CHSH family shows that qubit systems do not
    always attain the maximum quantum violation of the relevant bipartite Bell
    inequalities. This is a solved dimension-attainment variant, not the
    bipartite proper-facet violation question in the claim.

    OQP Problem 26 records that 26.B was answered negatively for more than two
    parties by Almeida, Bancal, Brunner, Acin, Gisin, and Pironio, "Guess your
    neighbour's input: a multipartite non-local game with no quantum
    advantage", Physical Review Letters 104, 230404 (2010). Almeida et al.
    prove that quantum correlations do not outperform classical correlations in
    the GYNI game for any prior input distribution, while no-signalling
    correlations can outperform them for some distributions, and that some
    Bell inequalities from their construction are facets of the local polytope.
    Those multipartite facet counterexamples are why this entry restricts the
    open claim to bipartite scenarios.

    Slofstra, "The set of quantum correlations is not closed", Forum of
    Mathematics, Pi 7, e1 (2019), proves that finite-dimensional tensor-product
    quantum correlations are not closed. This affects exact boundary and
    maximum-attainment formulations. It does not change the present strict
    violation claim for a fixed Bell functional, because strict violation by a
    closure point implies strict violation by nearby finite-dimensional quantum
    correlations.

    Ramanathan, "Violation of all two-party facet Bell inequalities by
    almost-quantum correlations", Physical Review Research 3, 033100 (2021),
    proves that every proper two-party facet Bell inequality is violated by
    almost-quantum correlations, a semidefinite-programming relaxation of the
    quantum set. This is progress toward the geometric question but is not a
    proof for \(\mathcal Q_{\mathrm{fd}}\) or
    \(\overline{\mathcal Q_{\mathrm{fd}}}\). Escola-Farras, Calsamiglia, and
    Winter, "All tight correlation Bell inequalities have quantum violations",
    Physical Review Research 2, 012044 (2020), prove the analogous quantum
    violation statement for the nontrivial two-player XOR or full-correlation
    subcase. That subcase result is progress only unless it is extended to all
    proper bipartite conditional-probability facets in the uniform \(M,K\)
    setting.
  </known>

  <refs>
    <ref>Open Quantum Problems, Bell inequalities holding for all quantum states, Problem 26, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/bell-inequalities-holding-for-all-quantum-states, used for the two subproblem statements, the \(C\subseteq Q\subseteq P\) geometry, the definition of proper Bell inequality, and the recorded status of 26.A and multipartite 26.B.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 26, Bell inequalities holding for all quantum states, in Quantum foundations.</ref>
    <ref>Ll. Masanes, Extremal quantum correlations for N parties with two dichotomic observables per site, arXiv:quant-ph/0512100 (2005), DOI: 10.48550/arXiv.quant-ph/0512100, used for the \((N,2,2)\) pure-state and projective-measurement reduction recorded by OQP Problem 26.</ref>
    <ref>T. Vertesi and K. F. Pal, Generalized Clauser-Horne-Shimony-Holt inequalities maximally violated by higher dimensional systems, Physical Review A 77, 042106 (2008), DOI: 10.1103/PhysRevA.77.042106, arXiv:0712.4225, used for the negative answer to OQP Problem 26.A.</ref>
    <ref>M. L. Almeida, J.-D. Bancal, N. Brunner, A. Acin, N. Gisin, and S. Pironio, Guess your neighbour's input: a multipartite non-local game with no quantum advantage, Physical Review Letters 104, 230404 (2010), DOI: 10.1103/PhysRevLett.104.230404, arXiv:1003.3844, used for multipartite proper-facet Bell inequalities with no quantum violation.</ref>
    <ref>W. Slofstra, The set of quantum correlations is not closed, Forum of Mathematics, Pi 7, e1 (2019), DOI: 10.1017/fmp.2018.3, arXiv:1703.08618, used for the finite-dimensional nonclosure caveat.</ref>
    <ref>R. Ramanathan, Violation of all two-party facet Bell inequalities by almost-quantum correlations, Physical Review Research 3, 033100 (2021), DOI: 10.1103/PhysRevResearch.3.033100, arXiv:2004.07673, used for the proper-facet almost-quantum progress result and for distinguishing it from a quantum solution.</ref>
    <ref>L. Escola-Farras, J. Calsamiglia, and A. Winter, All tight correlation Bell inequalities have quantum violations, Physical Review Research 2, 012044 (2020), DOI: 10.1103/PhysRevResearch.2.012044, arXiv:1908.06669, used for the nontrivial two-player XOR or full-correlation subcase.</ref>
  </refs>

  <ask>
    Prove or disprove the claim. A proof must show that for every finite
    pair \(M,K\) and every proper facet inequality \(\ell\le \beta_C\) of
    \(C_{M,K}\), there is a finite-dimensional tensor-product quantum behavior
    \(q\) with \(\ell(q)\gt\beta_C\). It is enough to prove
    \(\sup_{q\in\overline{\mathcal Q_{\mathrm{fd}}}}\ell(q)\gt\beta_C\),
    because strict separation is then witnessed by a finite-dimensional
    approximant.

    A disproof must give positive integers \(M,K\) and a linear functional
    \(\ell\) such that \(\ell\le\beta_C\) defines a facet of
    \(C_{M,K}\), \(\beta_P\gt\beta_C\), and
    \[
      \sup_{q\in\mathcal Q_{\mathrm{fd}}(M,K)}\ell(q)=\beta_C .
    \]
    The disproof must prove all three facts: local facet-defining tightness,
    properness relative to \(P_{M,K}\), and absence of strict quantum
    violation. Numerical optimization, almost-quantum violation,
    no-signalling violation, fixed small-scenario facet enumeration, full
    correlation or XOR-only results, or multipartite counterexamples are
    progress, not a solution, unless they imply the stated bipartite claim or
    its negation.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
