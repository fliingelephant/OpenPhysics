# 33 Exponential Clustering for One-Dimensional Lattice Bosons at Arbitrary Temperature

```xml
<problem id="33">
  <status>open</status>
  <kind>proof</kind>
  <field>QIT</field>
  <name>Exponential Clustering for One-Dimensional Lattice Bosons at Arbitrary Temperature</name>

  <keys>araki theorem araki's theorem gibbs states of a one dimensional quantum lattice bose-hubbard model bose-hubbard class hamiltonian lattice bosons bosonic lattice model repulsive bosons repulsive interaction repulsive potential unbounded local dimension unbounded local hilbert space unbounded bosonic operators infinite-dimensional local hilbert space interaction-picture cluster expansion cluster expansion technique low-boson-density inequality low-density assumption boson thermal area law quasi dulong-petit law exponential clustering clustering of correlations decay of correlations correlation decay correlation function two-point function connected correlation function covariance correlation length threshold temperature high-temperature threshold arbitrary temperature all-temperature any temperature every temperature one-dimensional bosonic chain 1d lattice bosons 1d chain finite-range hopping squeezing terms parametric driving creation and annihilation operators canonical commutation relations fock space particle number operator number operator moments absence of phase transition no phase transition kms state gibbs state thermal state tong kuwahara gong communications in mathematical physics</keys>

  <claim>
    Fix reals \(0\lt U_{\min}\le U_{\max}\lt\infty\) and \(J,\mu\ge0\) (Def
    section). For every inverse temperature \(\beta\in(0,\infty)\), the
    bound-tuple \((U_{\min},U_{\max},J,\mu)\) has exponential clustering at
    \(\beta\) in the sense of the Def section -- i.e.\ there exist a finite
    \(K(\beta)\) and a \(\xi(\beta)\gt0\), depending only on
    \(\beta,U_{\min},U_{\max},J,\mu\) (not on the interval \(V\), the
    admissible coupling data, the regions \(X,Y\), or the operators
    \(O_X,O_Y\)), such that the clustering inequality of the Def section
    holds for every finite interval \(V\subset\subset\mathbb Z\), every
    admissible coupling data on \(V\), every disjoint \(X,Y\subseteq V\),
    and every \(O_X\in\mathcal P_X(a,a^*)\), \(O_Y\in\mathcal P_Y(a,a^*)\).

    Quantifier and strength note (weaker reading chosen; ambiguity flagged).
    Tong, Kuwahara, and Gong (Known section) already prove exactly this
    property, with the same inequality and the same dependence structure
    for the constants, restricted to \(\beta\in(0,\beta_c]\) for a finite
    threshold \(\beta_c=\beta_c(U_{\min},U_{\max},J,\mu)\); this entry's
    claim is exactly their Theorem 2, specialized to the one-dimensional
    nearest-neighbor chain, with that restriction removed. Two weakenings
    are built in on purpose. First, \(\xi(\beta)\) is required to be finite
    at each individual \(\beta\), but is not required to stay bounded as
    \(\beta\to\infty\); a version additionally asserting
    \(\sup_{\beta\gt0}\xi(\beta)\lt\infty\), or an explicit rate for
    \(\xi(\beta)\) as \(\beta\to\infty\), is strictly stronger and not
    claimed here. Second, the claim is stated only for nearest-neighbor
    hopping and squeezing (nonzero \(J_{x,x+1},\widetilde J_{x,x+1}\) only
    for edges of the 1D chain, Def section); Tong, Kuwahara, and Gong,
    Sec.~7.2, and Tong and Kuwahara's power-law-range extension (Known
    section) discuss a finite-range/power-law-range generalization of the
    high-temperature theorem, and whether the arbitrary-temperature
    extension holds for that broader class is a separate, not-here-addressed
    question. The companion
    half of Tong, Kuwahara, and Gong's stated open question -- "the absence
    of phase transitions" -- is also not part of this claim: it would need
    its own operationalization (e.g.\ real-analyticity of the free energy
    density) distinct from, and not shown here to be equivalent to, the
    clustering property below.
  </claim>

  <def>
    One-dimensional lattice. Fix integers \(a\le b\) and set
    \(V:=\{a,a+1,\ldots,b\}\subset\mathbb Z\) (\(\subset\subset\) denotes
    "finite subset of"), \(n:=|V|=b-a+1\), and
    \(E:=\{\{x,x+1\}:a\le x\le b-1\}\). This is the \(d=1\) case of the
    "prototypical example" graph of Tong, Kuwahara, and Gong, Sec.~2.1
    (finite subset of a \(d\)-dimensional hypercubic lattice with all
    nearest-neighbor bonds); connectedness of \((V,E)\) forces \(V\) to be
    an interval of consecutive integers as written. For \(x,y\in V\),
    \(\operatorname{dist}(x,y):=|x-y|\); for \(X,Y\subseteq V\),
    \(\operatorname{dist}(X,Y):=\min_{x\in X,y\in Y}|x-y|\). Every vertex
    has degree at most \(\mathfrak d=2\), and (Tong, Kuwahara, and Gong,
    Sec.~2.1, citing Klarner and Penrose) the graph's growth constant
    \(\sigma\) obeys \(\sigma\le2e\) uniformly in \(n\).

    Local Hilbert space and operators. For each \(x\in V\), the local
    Hilbert space is \(\mathcal H_x\cong\ell^2(\mathbb N_0)\), spanned by
    number states \(\{|k\rangle_x\}_{k\in\mathbb N_0}\), with creation and
    annihilation operators \(a_x^*,a_x\) obeying the canonical commutation
    relations \([a_x,a_y^*]=\delta_{xy}\mathbb 1\),
    \([a_x,a_y]=[a_x^*,a_y^*]=0\), and local number operator
    \(n_x:=a_x^*a_x\). Set \(\mathcal H_V:=\bigotimes_{x\in V}\mathcal
    H_x\); all operators below act a priori on the dense finite-particle
    domain \(\mathcal D_{\mathrm{fin}}\) of Tong, Kuwahara, and Gong,
    Sec.~2.2, and are extended to self-adjoint operators by closure where
    stated.

    Model parameters and admissible coupling data. Given the fixed bounds
    \(J,\mu\ge0\) and \(0\lt U_{\min}\le U_{\max}\lt\infty\), admissible
    coupling data on \(V\) is a choice of real numbers
    \(J_{x,x+1},\widetilde J_{x,x+1}\) for each edge \(\{x,x+1\}\in E\) and
    real numbers \(\mu_x,U_x\) for each \(x\in V\), subject to
    \[
      |J_{x,x+1}|\le J,\quad|\widetilde J_{x,x+1}|\le J,\quad
      |\mu_x|\le\mu,\quad 0\lt U_{\min}\le U_x\le U_{\max}.
    \]
    ("Repulsive" refers to the strict positivity \(U_x\ge U_{\min}\gt0\)
    of the on-site interaction.)

    Hamiltonian and Gibbs state. For admissible coupling data on \(V\), the
    one-dimensional Bose-Hubbard-class Hamiltonian is
    \[
      H_V:=-\sum_{x=a}^{b-1}\Bigl(J_{x,x+1}a_x^*a_{x+1}
      +\widetilde J_{x,x+1}a_x^*a_{x+1}^*+\text{h.c.}\Bigr)
      +\sum_{x=a}^{b}\Bigl[\tfrac{U_x}{2}n_x(n_x-1)-\mu_x n_x\Bigr],
    \]
    the \(d=1\), nearest-neighbor case of Tong, Kuwahara, and Gong's
    Eq.~(8). \(H_V\) is essentially self-adjoint on
    \(\mathcal D_{\mathrm{fin}}\) (their Proposition 1), and for every
    \(\beta\in(0,\infty)\)
    (with no threshold restriction) \(e^{-\beta H_V}\) is trace-class
    (their Proposition 3), so the Gibbs state
    \(\rho_{\beta,V}:=e^{-\beta H_V}/\operatorname{Tr}(e^{-\beta H_V})\) is
    well-defined for every \(\beta\in(0,\infty)\) and every admissible
    \(H_V\); existence of \(\rho_{\beta,V}\) itself is not part of what is
    open here.

    Correlation function. For \(X\subseteq V\), let \(\mathcal
    P_X(a,a^*)\) be the algebra of polynomials in
    \(\{a_x,a_x^*\}_{x\in X}\) (Tong, Kuwahara, and Gong, Definition 1).
    For disjoint \(X,Y\subseteq V\), \(O_X\in\mathcal P_X(a,a^*)\), and
    \(O_Y\in\mathcal P_Y(a,a^*)\), their thermal correlation function at
    \(\beta\) is
    \[
      C_{\beta,V}(O_X,O_Y):=\operatorname{Tr}(O_XO_Y\rho_{\beta,V})
      -\operatorname{Tr}(O_X\rho_{\beta,V})\cdot
      \operatorname{Tr}(O_Y\rho_{\beta,V}),
    \]
    well-defined for every \(\beta\in(0,\infty)\) because operators growing
    at most polynomially in the total particle number have trace-class
    product with \(e^{-\beta H_V}\) (their Proposition 4).

    Regularized norm. For \(X\subseteq V\), set
    \(N_X:=\sum_{x\in X}n_x\); for \(O_X\in\mathcal P_X(a,a^*)\) and
    \(\gamma\gt0\), \(\|O_Xe^{-\gamma N_X}\|\) is the operator norm of the
    bounded operator \(O_Xe^{-\gamma N_X}\) on \(\mathcal H_X:=
    \bigotimes_{x\in X}\mathcal H_x\).

    Exponential clustering at \(\beta\) (property of a bound-tuple). Say
    the bound-tuple \((U_{\min},U_{\max},J,\mu)\) has exponential
    clustering at \(\beta\in(0,\infty)\) if there exist \(K=K(\beta)\lt
    \infty\) and \(\xi=\xi(\beta)\gt0\), depending only on
    \(\beta,U_{\min},U_{\max},J,\mu\), such that for every interval
    \(V\subset\subset\mathbb Z\), every admissible coupling data on \(V\),
    every disjoint \(X,Y\subseteq V\), and every \(O_X\in\mathcal
    P_X(a,a^*)\), \(O_Y\in\mathcal P_Y(a,a^*)\),
    \[
      \bigl|C_{\beta,V}(O_X,O_Y)\bigr|\ \le\
      K^{|X|+|Y|}\,\bigl\|O_Xe^{-\sqrt\beta N_X}\bigr\|\,
      \bigl\|O_Ye^{-\sqrt\beta N_Y}\bigr\|\,
      e^{-\operatorname{dist}(X,Y)/\xi}.
    \]
    This is Tong, Kuwahara, and Gong's Eq.~(15) specialized to
    \(\mathfrak d=2\) (the 1D chain), read as a property of the bound-tuple
    that must hold uniformly over every volume, every admissible coupling
    realization, and every pair of regions.
  </def>

  <known>
    Xin-Hai Tong, Tomotaka Kuwahara, and Zongping Gong, "Clustering Theorem
    for Bose-Hubbard class Gibbs states," accepted for publication in
    Communications in Mathematical Physics, arXiv:2411.10759v4 (revised 20
    Jun 2026; dated 23 Jun 2026), Theorem 2 (Sec.~2.4, p.~8), prove exactly
    the property defined above ("exponential clustering at \(\beta\)") for
    every bound-tuple \((U_{\min},U_{\max},J,\mu)\) and every graph of
    bounded degree \(\mathfrak d\) and growth constant \(\sigma\)
    (including, taking \(\mathfrak d=2\), \(\sigma\le2e\), the
    one-dimensional chain of the Def section) -- but only for
    \(\beta\in(0,\beta_c]\), where \(\beta_c\gt0\) is a threshold depending
    only on \(U_{\min},U_{\max},J,\mu,\mathfrak d\). Their explicit
    correlation length is
    \(\xi(\beta)=-\{\ln[\sigma C_1\beta^{1/2}/(1-C_2\beta^{1/2})]\}^{-1}\)
    (their Eq.~(15), constants \(C_1,C_2\) depending only on
    \(U_{\min},U_{\max},J,\mu,\mathfrak d\)), which is finite and positive
    only while \(\sigma C_1\beta^{1/2}\lt1-C_2\beta^{1/2}\); this is the
    structural reason their method stops at a finite \(\beta_c\) rather
    than reaching \(\beta=\infty\). Their Theorem 1 (same section, p.~8,
    the "Low-Boson-Density Inequality") bounds particle-number moments,
    \(\operatorname{Tr}(n_x^s\rho_{\beta,V})\le K_{\mathrm{low}}^s\cdot
    s!\cdot\beta^{-s/2}\), likewise only for \(\beta\in(0,\beta_c)\), and
    is the main technical input to Theorem 2. By contrast, existence of
    \(\rho_{\beta,V}\) itself is unconditional for every \(\beta\in
    (0,\infty)\) (their Propositions 1 and 3, Sec.~2.2-2.3, p.~5-6): no
    threshold appears there.

    In Sec.~8 ("Conclusion and Outlook," p.~42), Tong, Kuwahara, and Gong
    state: "A primary open question is the rigorous extension of Araki's
    one-dimensional results [1] to establish clustering properties and the
    absence of phase transitions for one-dimensional bosonic chains at
    arbitrary temperatures." Their reference [1] is Huzihiro Araki, "Gibbs
    states of a one dimensional quantum lattice," Communications in
    Mathematical Physics, 14:120-157, 1969 -- the classical result, for
    finite-dimensional (bounded) local Hilbert spaces, i.e.\ quantum spin
    chains rather than bosons, that one-dimensional finite-range
    interactions have no finite-temperature phase transition. This entry's
    claim is precisely the clustering half of that stated open question,
    restricted to the nearest-neighbor Bose-Hubbard class already defined
    and proved-at-high-temperature by the same paper's Theorem 2; this
    entry does not independently verify the content of Araki 1969 beyond
    Tong, Kuwahara, and Gong's own characterization of it as the
    finite-dimensional precedent their bosonic result seeks to extend.

    In Sec.~7.2 (p.~42), Tong, Kuwahara, and Gong note that "a subsequent
    work [43] by two of the authors of this article" -- Xin-Hai Tong and
    Tomotaka Kuwahara, "Toward a complexity classification of
    high-temperature bosons: Computational tractability and power-law
    clustering," arXiv:2509.25572v2 (2025) -- extends their cluster
    expansion to power-law-decaying long-range couplings, but, in their own
    words, "for such models at high temperatures, we have rigorously
    established a clustering theorem and a low-boson-density inequality":
    a broader-range but still threshold-restricted result. This entry's
    search did not extend to reading arXiv:2509.25572 itself (only Tong,
    Kuwahara, and Gong's own description of it in Sec.~7.2); on that
    description alone it does not resolve the arbitrary-temperature
    question, since it is explicitly restricted to "high temperatures" in
    the same sense as Theorem 2 above, for a different (longer-range, not
    nearest-neighbor) interaction class. No other resolution, partial or
    full, of the arbitrary-temperature clustering property for
    one-dimensional lattice bosons was located for this entry; per this
    entry's instructions, no broader literature search for solving papers
    beyond the sources the survey lead cites was carried out, so a proof
    published elsewhere and missed here remains a residual possibility.
    Section, theorem, and page numbers above are taken from a direct
    reading of the Tong, Kuwahara, and Gong arXiv:2411.10759v4 PDF; since
    that entry is only listed as "accepted for publication" and not yet
    assigned a Communications in Mathematical Physics volume, page, or DOI
    at the time of this entry, its final typeset numbering may differ from
    the arXiv numbering cited here.
  </known>

  <refs>
    <ref>Xin-Hai Tong, Tomotaka Kuwahara, and Zongping Gong, "Clustering Theorem for Bose-Hubbard class Gibbs states," accepted for publication in Communications in Mathematical Physics, arXiv:2411.10759v4 (2026), Theorem 1 and Theorem 2 (Sec. 2.4), Definition 1 and Eq. (11) (correlation function), Eq. (15) (clustering bound and correlation length), Sec. 2.1 (graph, growth constant, one-dimensional example), Sec. 2.2-2.3 (Hamiltonian, Propositions 1-4), Sec. 7.2, and Sec. 8 (Conclusion and Outlook, statement of the primary open question extending Araki's theorem to bosons), used for every definition and for the exact statement of what is proved and what is left open.</ref>
    <ref>Huzihiro Araki, "Gibbs states of a one dimensional quantum lattice," Communications in Mathematical Physics, 14:120-157, 1969, cited as reference [1] of Tong, Kuwahara, and Gong above, used only to identify the classical finite-dimensional precedent named in their stated open question; its content is not independently used or verified in this entry beyond that attribution.</ref>
    <ref>Xin-Hai Tong and Tomotaka Kuwahara, "Toward a complexity classification of high-temperature bosons: Computational tractability and power-law clustering," arXiv:2509.25572v2 (2025), cited as reference [43] of Tong, Kuwahara, and Gong above (Sec. 7.2); used only via that paper's own description of arXiv:2509.25572's high-temperature-restricted long-range extension, to confirm it does not resolve the claim above.</ref>
  </refs>

  <ask>
    A proof exhibits, for every \(\beta\in(0,\infty)\) and every
    bound-tuple \(0\lt U_{\min}\le U_{\max}\lt\infty\), \(J,\mu\ge0\),
    finite \(K(\beta)\) and \(\xi(\beta)\gt0\) (depending only on
    \(\beta,U_{\min},U_{\max},J,\mu\), not on \(V\), the admissible
    coupling data, \(X,Y\), or the operators) such that the clustering
    inequality of the Def section holds for every finite interval
    \(V\subset\subset\mathbb Z\), every admissible coupling data on \(V\),
    every disjoint \(X,Y\subseteq V\), and every \(O_X\in\mathcal
    P_X(a,a^*)\), \(O_Y\in\mathcal P_Y(a,a^*)\) -- i.e.\ extends Tong,
    Kuwahara, and Gong's Theorem 2 (Known section) from
    \(\beta\in(0,\beta_c]\) to \(\beta\in(0,\infty)\) for the
    one-dimensional, nearest-neighbor case. A proof additionally giving
    \(\sup_{\beta\gt0}\xi(\beta)\lt\infty\) or an explicit low-temperature
    rate for \(\xi(\beta)\) is strictly stronger and also counts as a full
    proof of the claim above.

    A disproof exhibits some bound-tuple \(U_{\min},U_{\max},J,\mu\), some
    admissible coupling data (on a single interval \(V\), or a sequence of
    intervals \(V_m\) of growing length with fixed coupling rule), some
    \(\beta_0\in(0,\infty)\), and a proof that no finite \(K,\xi\gt0\)
    make the clustering inequality of the Def section hold at \(\beta_0\)
    uniformly over all finite intervals and all disjoint region pairs --
    for instance a rigorous lower bound on some \(C_{\beta_0,V_m}(O_X,O_Y)\)
    that decays only polynomially, or does not decay, in
    \(\operatorname{dist}(X,Y)\) as \(m\to\infty\) with
    \(\operatorname{dist}(X,Y)\to\infty\) proportionally. This refutes the
    claim above for that one bound-tuple and \(\beta_0\), leaving every
    other bound-tuple undecided.

    The following count as progress, not as a solution, unless combined
    with a proof removing the stated restriction: (i) any extension of the
    proven range beyond \(\beta_c\) that still stops at a finite, however
    large, threshold \(\beta_c'\); (ii) a proof restricted to a strict
    submodel, e.g.\ \(\widetilde J_{x,x+1}\equiv0\) (particle-number
    conserving), a single fixed \(U_x\equiv U\) rather than the full
    bound-tuple family, or the hard-core limit in which the local Hilbert
    space is truncated to finite dimension (where Araki's original theorem
    already applies directly and is not itself the open extension); (iii)
    a proof of a weaker decay -- polynomial, or exponential only for
    \(\beta\) in a bounded or sparse subset of \((0,\infty)\) -- throughout
    the full temperature range, rather than exponential decay with some
    finite \(\xi(\beta)\) at every individual \(\beta\); (iv) a proof
    conditional on an unproven extra hypothesis, such as an assumed
    extension of the low-boson-density moment bound (Theorem 1, Known
    section) beyond \(\beta_c\), without proving that extension; (v) a
    proof or disproof of the finite-range or power-law-range
    generalization of Sec.~7.2 (Known section) instead of the
    nearest-neighbor case defined above; (vi) a proof or disproof of the
    companion "absence of phase transitions" half of Tong, Kuwahara, and
    Gong's stated open question, under some operationalization of that
    phrase, without also settling the clustering claim exactly as stated
    above.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
