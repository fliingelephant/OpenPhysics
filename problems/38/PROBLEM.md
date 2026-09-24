# 38 Global Markov Property of Thermal States

```xml
<problem id="38">
  <status>open</status>
  <kind>proof</kind>
  <field>QIT</field>
  <name>Global Markov Property of Thermal States</name>
  <keys>global markov property quantum global markov property local markov property quantum markov property gibbs state thermal state conditional mutual information quantum conditional mutual information CMI QCMI hammersley-clifford theorem quantum hammersley-clifford approximate quantum markov chain quantum markov chain markov network macroscopic region quasi-local recovery map recovery channel petz recovery map shielded region clustering of conditional mutual information clustering theorem belief-propagation channel BP channel quantum belief propagation dobrushin condition quantum dobrushin condition path coupling rapid mixing gibbs sampler quantum gibbs sampler high temperature arbitrary temperature low temperature 2D lattice higher-dimensional lattice D-dimensional lattice quantum spin lattice qubit lattice thermalization strong subadditivity von neumann entropy tripartite correlation topological order k-local hamiltonian bounded-range interactions finite-range interactions interaction degree Kuwahara Chen-Rouze Kato-Kuwahara Bakshi-Liu-Moitra-Tang Kuwahara-Kato-Brandao pairwise markov property threshold temperature cluster expansion gibbs-global-quantum-markov-property</keys>

  <claim>
    Fix an integer \(D\ge2\), an integer \(k\ge1\), an integer \(d\ge1\), a
    real \(J\gt0\), and a real \(r\gt0\). For every \(\beta\gt0\) there
    exist \(\xi=\xi(D,k,d,J,r,\beta)\in(0,\infty)\) and a two-variable
    polynomial \(p=p_{D,k,d,J,r,\beta}\) with real coefficients such that,
    for every finite \(\Lambda\subset\mathbb Z^D\), every
    \((k,d,J,r)\)-Hamiltonian \(H\) on \(\Lambda\) (defined below), and
    every partition \(\Lambda=A\sqcup B\sqcup C\) into three disjoint
    nonempty subsets,
    \[
      I(A:C\mid B)_{\rho_\beta}\ \le\ p(|A|,|C|)\,
      e^{-\operatorname{dist}(A,C)/\xi}.
    \]
    Here \(\xi\) and \(p\) may depend on \(D,k,d,J,r,\beta\) but not on
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
    degree), a real \(J\gt0\) (interaction-strength bound), and a real
    \(r\gt0\) (interaction range). A \((k,d,J,r)\)-Hamiltonian on
    \(\Lambda\) is
    \[
      H=\sum_{\gamma\in\Gamma}H_\gamma,
    \]
    where \(\Gamma\) is a finite index set and each \(\gamma\in\Gamma\) has
    a nonempty support \(\operatorname{supp}(\gamma)\subseteq\Lambda\) with
    \(|\operatorname{supp}(\gamma)|\le k\) and
    \(\operatorname{diam}(\operatorname{supp}(\gamma)):=\max\{\|x-y\|_1:
    x,y\in\operatorname{supp}(\gamma)\}\le r\); \(H_\gamma\) is a Hermitian
    operator on \(\mathcal H_\Lambda\) acting as the identity outside
    \(\operatorname{supp}(\gamma)\), with operator norm
    \(\|H_\gamma\|\le J\); and, writing \(\gamma\sim\gamma'\) for
    \(\operatorname{supp}(\gamma)\cap\operatorname{supp}(\gamma')\ne
    \emptyset\), every \(\gamma\in\Gamma\) satisfies
    \(|\{\gamma'\in\Gamma:\gamma'\sim\gamma,\ \gamma'\ne\gamma\}|\le d\)
    ("interaction degree at most \(d\)", following Chen and Rouzé,
    arXiv:2504.02208, Section III). The range bound matches Chen and
    Rouzé's finite-range assumption for \(D\)-dimensional lattices, Section
    III.2: interactions supported on regions of diameter larger than a
    fixed constant vanish.

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
    for constants \(\mu,\lambda\gt0\) depending on \(D,k,d,J,r\). This is
    local-Markov type: it only decays once \(\operatorname{dist}(A,C)\)
    exceeds a multiple of \(\min(|A|,|C|)\), and gives no control once
    \(\min(|A|,|C|)\) grows with \(|\Lambda|\). It proves the local Markov
    property at every \(\beta\gt0\) but not the claim above. Immediately
    after these results, Chen and Rouzé state explicitly that "finding a
    QCMI bound which depends only polynomially on \(|A|\) and \(|C|\),
    namely a global Markov property, remains open" -- exactly the claim
    above, in their own words, marked open. Chen and Rouzé's own footnote 1
    is not about concurrent work: it attaches to their reference [49],
    Kuwahara, Kato, and Brandão, "Clustering of Conditional Mutual
    Information for Quantum Gibbs States above a Threshold Temperature",
    Physical Review Letters 124, 220601 (2020) (arXiv:1910.09425) -- a 2020
    paper, five years earlier, not concurrent with either Chen-Rouzé or
    Kato-Kuwahara -- and states verbatim: "In [49], an even stronger
    notion of clustering of the QCMI at high enough temperature ... was
    considered using high temperature cluster expansions. However, the
    proof requires expansions of operator-valued partial trace
    functionals, whose correctness remains unclear." This footnote
    therefore flags an unresolved correctness question about part of a
    2020 paper's high-temperature argument; it neither proves nor
    endorses a global-type high-temperature bound, and it does not
    mention Bakshi, Liu, Moitra, and Tang.

    Chen and Rouzé's Section I.1 and Table 1 also classify prior QCMI-decay
    results into pairwise (\(|A|,|C|=O(1)\)), local
    (\(\min(|A|,|C|)=O(1)\)), and global (\(|A|,|C|\) unrestricted) tiers.
    They credit Kato and Brandão, "Quantum Approximate Markov Chains are
    Thermal", Communications in Mathematical Physics 370, 117 (2019), with
    the global tier in the restricted one-dimensional case, at
    subexponential decay; they state that Kuwahara, arXiv:2407.05835 (this
    entry's next reference), strengthens that one-dimensional bound to
    exponential decay, but that "in the general \(D\)-dimensional setting,
    the impressive work [Kuwahara, arXiv:2407.05835] proved the pairwise
    Markov property" -- i.e., only the weakest tier, strictly below even
    the local-Markov type of Chen and Rouzé's own Theorem III.1, for
    \(D\ge2\).

    Kuwahara, "Clustering of Conditional Mutual Information and Quantum
    Markov Structure at Arbitrary Temperatures", Physical Review X 15,
    041010 (2025) (arXiv:2407.05835), proves that quantum conditional
    mutual information decays exponentially with \(\operatorname{dist}
    (A,C)\) at every \(\beta\gt0\), including arbitrarily low temperature,
    with a correlation length that grows only polynomially in \(\beta\),
    and frames this as a quantum analog of the classical Hammersley-
    Clifford theorem. This establishes exponential-in-distance clustering
    at every temperature for the regions and models it covers. Per Chen
    and Rouzé's classification above, this bound is of pairwise type
    (\(|A|,|C|=O(1)\)) for \(D\ge2\) -- strictly weaker than global, and
    even weaker than local -- so it does not establish the claim above for
    \(D\ge2\). This entry relies on Chen and Rouzé's secondary
    classification for this point rather than an independent line-by-line
    check of Kuwahara's proof; a residual, low-weight possibility is that
    Kuwahara's paper contains an unremarked stronger bound for \(D\ge2\)
    that this secondary classification undercounts.

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
    condition holds. This paper is not among the three sources originally
    cited alongside this entry's seed problem (Kuwahara, Physical Review X
    15, 041010; arXiv:2504.02208; arXiv:2504.02235); it is identified here,
    by its venue, date, and content, as the source of the seed problem's
    "high temperature settled at STOC 2026" description. Its precise
    high-temperature threshold (a quantitative condition on
    \(D,k,d,J,r,\beta\) from their Dobrushin condition) is not reproduced in
    this entry.

    Together, these four results leave exactly the gap recorded by the
    claim above: the global Markov property (polynomial-in-\(|A|,|C|\)
    QCMI bound, unrestricted region sizes) is proven only at high
    temperature (Bakshi-Liu-Moitra-Tang). At every temperature and for
    general \(D\ge2\), the strongest unconditional result of local-Markov
    type (bounded, or unrestricted-but-exponential-prefactor, dependence
    on \(\min(|A|,|C|)\)) is Chen and Rouzé's Theorem III.1; Kuwahara's
    arXiv:2407.05835 is classified by Chen and Rouzé as reaching only the
    strictly weaker pairwise tier for \(D\ge2\), though it reaches the
    global tier in the one-dimensional case. None of the four cited
    results proves or claims to prove the global property at every
    \(\beta\gt0\) for \(D\ge2\); two of them (Chen-Rouzé and Kato-Kuwahara)
    state in their own words that this extension is open.
  </known>

  <refs>
    <ref>C.-F. Chen and C. Rouzé, Quantum Gibbs states are locally Markovian, arXiv:2504.02208 (2025), Theorem III.1 (quasi-local recovery maps at arbitrary temperature) and Corollary III.2 (resulting QCMI bound), used for the local Markov property at every temperature and for the explicit statement, in the discussion following these results in Section III, that a polynomial-in-region-size QCMI bound -- a global Markov property -- remains open; also Section I.1 and Table 1, used for the pairwise/local/global classification of prior results including Kuwahara's arXiv:2407.05835; also Section III.2, used for the finite-range (bounded-diameter) assumption on interactions that underlies the interaction-range parameter \(r\) in this entry's Hamiltonian definition; and footnote 1, used for the correctness caveat on Kuwahara-Kato-Brandão's high-temperature cluster-expansion argument.</ref>
    <ref>T. Kuwahara, Clustering of Conditional Mutual Information and Quantum Markov Structure at Arbitrary Temperatures, Physical Review X 15, 041010 (2025), DOI: 10.1103/9hx7-pzxw, arXiv:2407.05835, used for exponential-in-distance conditional-mutual-information clustering at every temperature, including low temperature, with correlation length polynomial in inverse temperature, framed as a quantum Hammersley-Clifford theorem.</ref>
    <ref>K. Kato and T. Kuwahara, Clustering of Conditional Mutual Information via Quantum Belief-Propagation Channels, arXiv:2504.02235 (2025), Theorem 2 (unconditional high-temperature QCMI decay) and the accompanying discussion, used for the belief-propagation-channel reduction, the conditional (assumption-dependent) all-temperature result, the unconditional high-temperature result, and the explicit statement that whether the regions can be taken arbitrarily large is a key open problem.</ref>
    <ref>A. Bakshi, A. Liu, A. Moitra, and E. Tang, A Dobrushin condition for quantum Markov chains: Rapid mixing and conditional mutual information at high temperature, Proceedings of the 58th Annual ACM Symposium on Theory of Computing (STOC 2026), DOI: 10.1145/3798129.3800859, arXiv:2510.08542, used for the unconditional, region-size-unrestricted exponential conditional-mutual-information decay at high temperature via a quantum Dobrushin condition and path-coupling argument, resolving the global Markov property in the high-temperature regime only.</ref>
    <ref>T. Kuwahara, K. Kato, and F. G. S. L. Brandão, Clustering of Conditional Mutual Information for Quantum Gibbs States above a Threshold Temperature, Physical Review Letters 124, 220601 (2020), DOI: 10.1103/PhysRevLett.124.220601, arXiv:1910.09425, identified as reference [49] in Chen and Rouzé (arXiv:2504.02208), used for the origin of the QCMI-decay conjecture stated in Chen and Rouzé's Section I.1 and for the target of their footnote 1, which flags unclear correctness in this paper's high-temperature cluster-expansion argument for an "even stronger" clustering claim; not used for any independent claim about this paper's own proven theorem.</ref>
  </refs>

  <ask>
    Settle, for every \(D\ge2\), \(k\ge1\), \(d\ge1\), \(J\gt0\), and
    \(r\gt0\), whether, for every \(\beta\gt0\), there exist
    \(\xi(D,k,d,J,r,\beta)\in(0,\infty)\) and a two-variable polynomial
    \(p_{D,k,d,J,r,\beta}\) such that the bound of the claim holds for
    every finite \(\Lambda\subset\mathbb Z^D\), every
    \((k,d,J,r)\)-Hamiltonian \(H\) on \(\Lambda\), and every partition
    \(\Lambda=A\sqcup B\sqcup C\).

    A proof must establish this for every \(\beta\gt0\), for every
    \(D\ge2,k,d,J,r\) in the stated domain. A proof may cite Bakshi, Liu,
    Moitra, and Tang (STOC 2026) for whatever high-temperature range their
    quantum Dobrushin condition already covers, but must newly establish
    the bound for every remaining \(\beta\) in that domain, up to
    arbitrarily low temperature (arbitrarily large \(\beta\)), to resolve
    the claim in full; a proof covering only \(\beta\) below a fixed or
    newly derived threshold, without covering arbitrarily large \(\beta\),
    does not resolve the claim.

    A disproof must exhibit one tuple \((D,k,d,J,r,\beta)\) with
    \(D\ge2\), \(k\ge1\), \(d\ge1\), \(J\gt0\), \(r\gt0\), \(\beta\gt0\),
    together with, for every \(\xi\lt\infty\) and every two-variable
    polynomial \(p\), infinitely many instances -- a finite
    \(\Lambda\subset\mathbb Z^D\), a \((k,d,J,r)\)-Hamiltonian \(H\) on
    \(\Lambda\), and a partition \(\Lambda=A\sqcup B\sqcup C\) -- with
    \(I(A:C\mid B)_{\rho_\beta}\gt p(|A|,|C|)\,
    e^{-\operatorname{dist}(A,C)/\xi}\). A single such tuple disproves the
    claim as stated, since the claim is universally quantified over
    \(D,k,d,J,r,\beta\); no argument about which tuples remain open is
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
    \(D,k,d,J,r,\beta\); (iv) a proof restricted to a proper subclass of
    \((k,d,J,r)\)-Hamiltonians, such as commuting Hamiltonians, without
    covering every \((k,d,J,r)\)-Hamiltonian in the stated class; (v)
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
