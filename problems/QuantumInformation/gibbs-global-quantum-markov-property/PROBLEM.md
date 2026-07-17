# QTD023 Global Markov Property of Thermal States

```xml
<problem id="QTD023">
  <status>open</status>
  <name>Global Markov Property of Thermal States</name>
  <keys>global markov property quantum global markov property local markov property quantum markov property gibbs state thermal state conditional mutual information quantum conditional mutual information CMI QCMI hammersley-clifford theorem quantum hammersley-clifford approximate quantum markov chain quantum markov chain markov network macroscopic region quasi-local recovery map recovery channel petz recovery map shielded region clustering of conditional mutual information clustering theorem belief-propagation channel BP channel quantum belief propagation dobrushin condition quantum dobrushin condition path coupling rapid mixing gibbs sampler quantum gibbs sampler high temperature arbitrary temperature low temperature 2D lattice higher-dimensional lattice D-dimensional lattice quantum spin lattice qubit lattice thermalization strong subadditivity von neumann entropy tripartite correlation topological order k-local hamiltonian bounded-range interactions finite-range interactions interaction degree Kuwahara Chen-Rouze Kato-Kuwahara Bakshi-Liu-Moitra-Tang gibbs-global-quantum-markov-property</keys>

  <claim>
    Fix an integer \(D\ge2\), an integer \(k\ge1\), an integer \(d\ge1\), and
    a real \(J\gt0\). For every \(\beta\gt0\) there exist
    \(\xi=\xi(D,k,d,J,\beta)\in(0,\infty)\) and a two-variable polynomial
    \(p=p_{D,k,d,J,\beta}\) with real coefficients such that, for every
    finite \(\Lambda\subset\mathbb Z^D\), every \((k,d,J)\)-Hamiltonian
    \(H\) on \(\Lambda\) (defined below), and every partition
    \(\Lambda=A\sqcup B\sqcup C\) into three disjoint nonempty subsets,
    \[
      I(A:C\mid B)_{\rho_\beta}\ \le\ p(|A|,|C|)\,
      e^{-\operatorname{dist}(A,C)/\xi}.
    \]
    Here \(\xi\) and \(p\) may depend on \(D,k,d,J,\beta\) but not on
    \(\Lambda\), \(H\), or the choice of \(A,B,C\) beyond \(|A|\), \(|C|\),
    and \(\operatorname{dist}(A,C)\) appearing explicitly in the bound.
    This is the global Markov property (defined below) asserted to hold at
    every positive \(\beta\) -- i.e. at every temperature -- for
    \(D\ge2\)-dimensional lattices.
  </claim>

  <def>
    Lattice and distance. Fix an integer \(D\ge2\). For a finite
    \(\Lambda\subset\mathbb Z^D\), let
    \(\mathcal H_\Lambda:=\bigotimes_{x\in\Lambda}\mathbb C^2\) (one qubit
    per site). For \(x,y\in\mathbb Z^D\), \(\|x-y\|_1:=\sum_{i=1}^D|x_i-y_i|\);
    for nonempty \(A,C\subset\Lambda\),
    \(\operatorname{dist}(A,C):=\min\{\|x-y\|_1:x\in A,\ y\in C\}\).

    Hamiltonians. Fix integers \(k\ge1\) (body size), \(d\ge1\) (interaction
    degree), and a real \(J\gt0\) (interaction-strength bound). A
    \((k,d,J)\)-Hamiltonian on \(\Lambda\) is
    \[
      H=\sum_{\gamma\in\Gamma}H_\gamma,
    \]
    where \(\Gamma\) is a finite index set and each \(\gamma\in\Gamma\) has
    a nonempty support \(\operatorname{supp}(\gamma)\subseteq\Lambda\) with
    \(|\operatorname{supp}(\gamma)|\le k\); \(H_\gamma\) is a Hermitian
    operator on \(\mathcal H_\Lambda\) acting as the identity outside
    \(\operatorname{supp}(\gamma)\), with operator norm
    \(\|H_\gamma\|\le J\); and, writing \(\gamma\sim\gamma'\) for
    \(\operatorname{supp}(\gamma)\cap\operatorname{supp}(\gamma')\ne
    \emptyset\), every \(\gamma\in\Gamma\) satisfies
    \(|\{\gamma'\in\Gamma:\gamma'\sim\gamma,\ \gamma'\ne\gamma\}|\le d\)
    ("interaction degree at most \(d\)", following Chen and Rouzé,
    arXiv:2504.02208, Section III).

    Gibbs state. For \(\beta\gt0\),
    \(\rho_\beta:=e^{-\beta H}/\operatorname{Tr}(e^{-\beta H})\). For
    \(Y\subseteq\Lambda\), \(\rho_{\beta,Y}:=\operatorname{Tr}_{\Lambda
    \setminus Y}(\rho_\beta)\) is the marginal on \(Y\).

    Entropy and quantum conditional mutual information (QCMI). For a
    density operator \(\sigma\), \(S(\sigma):=-\operatorname{Tr}(\sigma\ln
    \sigma)\) (von Neumann entropy, natural logarithm; any other fixed
    logarithm base rescales every quantity below by the same constant and
    does not affect the claim). For a partition \(\Lambda=A\sqcup B\sqcup
    C\) into three disjoint nonempty subsets,
    \[
      I(A:C\mid B)_{\rho_\beta}:=S(\rho_{\beta,AB})+S(\rho_{\beta,BC})
      -S(\rho_{\beta,B})-S(\rho_{\beta,ABC})\ \ (\ge0\ \text{by strong
      subadditivity}).
    \]

    Local versus global Markov property. Following Chen and Rouzé,
    "Quantum Gibbs states are locally Markovian" (arXiv:2504.02208),
    Section III: a bound on \(I(A:C\mid B)_{\rho_\beta}\) is of
    local-Markov type if it controls \(I(A:C\mid B)_{\rho_\beta}\) only
    when \(\min(|A|,|C|)\) stays bounded as \(|\Lambda|\to\infty\)
    (equivalently, its dependence on \(|A|,|C|\) may be exponential); it is
    of global-Markov type if it holds with \(|A|,|C|\) unrestricted, in
    particular allowing \(|A|,|C|=\Theta(|\Lambda|)\) (macroscopic
    regions), with a bound whose dependence on \(|A|,|C|\) is polynomial.
    The claim above asserts a global-Markov-type bound at every \(\beta
    \gt0\).
  </def>

  <known>
    Chen and Rouzé, "Quantum Gibbs states are locally Markovian"
    (arXiv:2504.02208), Theorem III.1, construct, for every region
    \(A\subseteq\Lambda\) and every \(\beta\gt0\), a time-averaged,
    detailed-balanced Lindbladian recovery channel built from single-qubit
    Pauli jump operators that approximately recovers \(\rho_\beta\) from
    \(\rho_{\beta,\Lambda\setminus A}\); their Corollary III.2 turns this
    into a QCMI bound of the schematic form \(I(A:C\mid B)_{\rho_\beta}
    \lesssim|A|\,|C|\,e^{\mu\min(|A|,|C|)-\lambda\operatorname{dist}(A,C)}\)
    for constants \(\mu,\lambda\gt0\) depending on \(D,k,d,J\). This is
    local-Markov type: it only decays once \(\operatorname{dist}(A,C)\)
    exceeds a multiple of \(\min(|A|,|C|)\), and gives no control once
    \(\min(|A|,|C|)\) grows with \(|\Lambda|\). It proves the local Markov
    property at every \(\beta\gt0\) but not the claim above. Immediately
    after these results, Chen and Rouzé state explicitly that "finding a
    QCMI bound which depends only polynomially on \(|A|\) and \(|C|\),
    namely a global Markov property, remains open" -- exactly the claim
    above, in their own words, marked open. A footnote there also notes
    that concurrent work proves the global property only at high
    temperature, consistent with the Bakshi-Liu-Moitra-Tang result below.

    Kuwahara, "Clustering of Conditional Mutual Information and Quantum
    Markov Structure at Arbitrary Temperatures", Physical Review X 15,
    041010 (2025) (arXiv:2407.05835), proves that quantum conditional
    mutual information decays exponentially with \(\operatorname{dist}
    (A,C)\) at every \(\beta\gt0\), including arbitrarily low temperature,
    with a correlation length that grows only polynomially in \(\beta\),
    and frames this as a quantum analog of the classical Hammersley-
    Clifford theorem. This establishes exponential-in-distance clustering
    at every temperature for the regions and models it covers. This entry
    does not independently verify, from the published abstract alone,
    whether its bound is of local or global type in the sense above --
    i.e. whether the prefactor multiplying \(e^{-\operatorname{dist}(A,C)/
    \xi}\) is polynomial or exponential in \(|A|,|C|\) once \(|A|,|C|\) are
    unrestricted -- and records this as a residual point not resolved by
    this verification pass.

    Kato and Kuwahara, "Clustering of Conditional Mutual Information via
    Quantum Belief-Propagation Channels" (arXiv:2504.02235), reduce QCMI
    decay to the quasi-locality of a "belief-propagation channel" and
    prove, via this route, conditional QCMI decay at every \(\beta\gt0\)
    under an additional uniform-clustering or uniform-rapid-mixing
    hypothesis, together with an unconditional QCMI decay bound restricted
    to high temperature. Independent of Chen and Rouzé, they state
    explicitly that "a key open problem is whether the regions \(A\) and
    \(C\) can be taken arbitrarily large" -- a second, independent
    statement, from a second cited source, of the same open question as
    the claim above.

    Bakshi, Liu, Moitra, and Tang, "A Dobrushin condition for quantum
    Markov chains: Rapid mixing and conditional mutual information at high
    temperature", Proceedings of the 58th Annual ACM Symposium on Theory
    of Computing (STOC 2026) (arXiv:2510.08542), prove unconditional
    exponential QCMI decay "without restrictions on the size of the probed
    subsystems" -- i.e. the global Markov property of the claim above --
    via a quantum Dobrushin condition and a path-coupling argument, but
    only in the high-temperature regime where their Dobrushin-type
    condition holds. This paper is not among the sources originally cited
    alongside this entry's seed problem; it is identified here as the
    likely source of the "high temperature settled at STOC 2026"
    description used to scope the ask below, found while checking the
    footnote in Chen and Rouzé noted above. Its precise high-temperature
    threshold (a quantitative condition on \(D,k,d,J,\beta\) from their
    Dobrushin condition) is not reproduced in this entry.

    Together, these four results leave exactly the gap recorded by the
    claim above: the global Markov property (polynomial-in-\(|A|,|C|\)
    QCMI bound, unrestricted region sizes) is proven only at high
    temperature (Bakshi-Liu-Moitra-Tang); at every temperature, only the
    local property (bounded, or unrestricted-body-but-exponential-prefactor,
    dependence on \(\min(|A|,|C|)\)) is proven unconditionally (Chen-Rouzé;
    Kuwahara). None of the four cited results proves or claims to prove the
    global property at every \(\beta\gt0\); two of them (Chen-Rouzé and
    Kato-Kuwahara) state in their own words that this extension is open.
  </known>

  <refs>
    <ref>C.-F. Chen and C. Rouzé, Quantum Gibbs states are locally Markovian, arXiv:2504.02208 (2025), Theorem III.1 (quasi-local recovery maps at arbitrary temperature) and Corollary III.2 (resulting QCMI bound), used for the local Markov property at every temperature and for the explicit statement, in the discussion following these results in Section III, that a polynomial-in-region-size QCMI bound -- a global Markov property -- remains open, together with a footnote on concurrent high-temperature-only progress.</ref>
    <ref>T. Kuwahara, Clustering of Conditional Mutual Information and Quantum Markov Structure at Arbitrary Temperatures, Physical Review X 15, 041010 (2025), DOI: 10.1103/9hx7-pzxw, arXiv:2407.05835, used for exponential-in-distance conditional-mutual-information clustering at every temperature, including low temperature, with correlation length polynomial in inverse temperature, framed as a quantum Hammersley-Clifford theorem.</ref>
    <ref>K. Kato and T. Kuwahara, Clustering of Conditional Mutual Information via Quantum Belief-Propagation Channels, arXiv:2504.02235 (2025), Theorem 2 (unconditional high-temperature QCMI decay) and the accompanying discussion, used for the belief-propagation-channel reduction, the conditional (assumption-dependent) all-temperature result, the unconditional high-temperature result, and the explicit statement that whether the regions can be taken arbitrarily large is a key open problem.</ref>
    <ref>A. Bakshi, A. Liu, A. Moitra, and E. Tang, A Dobrushin condition for quantum Markov chains: Rapid mixing and conditional mutual information at high temperature, Proceedings of the 58th Annual ACM Symposium on Theory of Computing (STOC 2026), DOI: 10.1145/3798129.3800859, arXiv:2510.08542, used for the unconditional, region-size-unrestricted exponential conditional-mutual-information decay at high temperature via a quantum Dobrushin condition and path-coupling argument, resolving the global Markov property in the high-temperature regime only.</ref>
  </refs>

  <ask>
    Settle, for every \(D\ge2\), \(k\ge1\), \(d\ge1\), and \(J\gt0\),
    whether, for every \(\beta\gt0\), there exist \(\xi(D,k,d,J,\beta)\in
    (0,\infty)\) and a two-variable polynomial \(p_{D,k,d,J,\beta}\) such
    that the bound of the claim holds for every finite \(\Lambda\subset
    \mathbb Z^D\), every \((k,d,J)\)-Hamiltonian \(H\) on \(\Lambda\), and
    every partition \(\Lambda=A\sqcup B\sqcup C\).

    A proof must establish this for every \(\beta\gt0\), for every
    \(D\ge2,k,d,J\) in the stated domain. A proof may cite Bakshi, Liu,
    Moitra, and Tang (STOC 2026) for whatever high-temperature range their
    quantum Dobrushin condition already covers, but must newly establish
    the bound for every remaining \(\beta\) in that domain, up to
    arbitrarily low temperature (arbitrarily large \(\beta\)), to resolve
    the claim in full; a proof covering only \(\beta\) below a fixed or
    newly derived threshold, without covering arbitrarily large \(\beta\),
    does not resolve the claim.

    A disproof must exhibit one tuple \((D,k,d,J,\beta)\) with \(D\ge2\),
    \(k\ge1\), \(d\ge1\), \(J\gt0\), \(\beta\gt0\), together with, for
    every \(\xi\lt\infty\) and every two-variable polynomial \(p\),
    infinitely many instances -- a finite \(\Lambda\subset\mathbb Z^D\), a
    \((k,d,J)\)-Hamiltonian \(H\) on \(\Lambda\), and a partition
    \(\Lambda=A\sqcup B\sqcup C\) -- with
    \(I(A:C\mid B)_{\rho_\beta}\gt p(|A|,|C|)\,
    e^{-\operatorname{dist}(A,C)/\xi}\). A single such tuple disproves the
    claim as stated, since the claim is universally quantified over
    \(D,k,d,J,\beta\); no argument about which tuples remain open is
    needed once one tuple is disproved.

    The following count as progress, not as resolving the claim: (i) a
    bound as in the claim with \(p\) replaced by a super-polynomial but
    sub-exponential function of \(|A|,|C|\) (e.g. quasi-polynomial), for
    some or all \(\beta\gt0\); (ii) a proof restricted to \(\beta\) in a
    bounded interval not extending to arbitrarily large \(\beta\),
    including any proof that only reproduces or marginally extends the
    high-temperature threshold of Bakshi, Liu, Moitra, and Tang (STOC
    2026); (iii) a proof in which \(\xi\) or \(p\) is allowed to depend on
    \(\Lambda\) (for example on \(|\Lambda|\)) rather than only on
    \(D,k,d,J,\beta\); (iv) a proof restricted to a proper subclass of
    \((k,d,J)\)-Hamiltonians, such as commuting Hamiltonians, without
    covering every \((k,d,J)\)-Hamiltonian in the stated class; (v)
    numerical or finite-size evidence that \(I(A:C\mid B)_{\rho_\beta}\) is
    small for sampled instances, without an analytic bound of the stated
    form.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
