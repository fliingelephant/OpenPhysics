# QC031 Ratio-Optimal Individual Measurements on Geometrically Uniform States

```xml
<problem id="QC031">
  <status>open</status>
  <name>Ratio-Optimal Individual Measurements on Geometrically Uniform States</name>
  <keys>geometrically uniform states gu states cyclic geometrically uniform states cyclic gu states symmetric states multiple-copy discrimination n-copy discrimination minimum error discrimination square-root measurement srm pretty good measurement pgm collective measurement individual measurement adaptive measurement sequential measurement local measurement locc no quantum memory bayesian updating trine ensemble double trine quantum communication open quantum problems oqp problem 31</keys>

  <claim>
    For every finite-dimensional cyclic geometrically uniform pure-state
    ensemble with equiprobable labels, pairwise distinct rank-one states, and
    nonzero collective minimum error for all copy numbers, adaptive individual
    measurements without quantum memory are asymptotically as good as the
    optimal collective minimum-error measurement in the following strengthened
    ratio sense:
    \[
      \lim_{N\to\infty}
      \frac{P_{\mathrm{ind}}^{\mathrm{err}}(N)}
           {P_{\mathrm{col}}^{\mathrm{err}}(N)}
      =1 .
    \]
  </claim>

  <def>
    A cyclic geometrically uniform pure-state ensemble consists of an integer
    \(d\ge 2\), a finite-dimensional Hilbert space \(\mathcal{H}\), a unit
    vector \(|\psi_0\rangle\in\mathcal{H}\), and a unitary \(U\) such that
    \(U^d=\mathbf 1\) on the span of the states below and
    \(U\Pi_jU^\dagger=\Pi_{j+1\bmod d}\) for the physical states
    \[
      \Pi_j=|\psi_j\rangle\langle\psi_j|,
      \qquad |\psi_j\rangle=U^j|\psi_0\rangle,
      \qquad j\in\mathbb{Z}_d ,
    \]
    which are pairwise distinct. The label \(j\) is sampled uniformly from
    \(\mathbb{Z}_d\). For copy number \(N\), the receiver is given
    \(|\psi_j\rangle^{\otimes N}\). The nontrivial cases covered by the claim
    have \(P_{\mathrm{col}}^{\mathrm{err}}(N)\gt 0\) for all \(N\), equivalently
    at least one pair of the rank-one states has nonzero overlap.

    The collective success probability is
    \[
      P_{\mathrm{col}}^{\mathrm{succ}}(N)
      =
      \max_{\{E_j\}_{j\in\mathbb{Z}_d}}
      \frac{1}{d}\sum_{j\in\mathbb{Z}_d}
      \langle\psi_j|^{\otimes N}
        E_j
      |\psi_j\rangle^{\otimes N},
    \]
    where the maximum is over POVMs on \(\mathcal{H}^{\otimes N}\). The
    collective error probability is
    \(P_{\mathrm{col}}^{\mathrm{err}}(N)=1-P_{\mathrm{col}}^{\mathrm{succ}}(N)\).
    Since the \(N\)-copy ensemble is again cyclic geometrically uniform under
    \(U^{\otimes N}\), the square-root measurement, or pretty good
    measurement,
    \[
      E_j^{\mathrm{SRM}}
      =
      \rho_N^{-1/2}
      \frac{1}{d}
      (|\psi_j\rangle\langle\psi_j|)^{\otimes N}
      \rho_N^{-1/2},
      \qquad
      \rho_N=\frac{1}{d}\sum_{k\in\mathbb{Z}_d}
      (|\psi_k\rangle\langle\psi_k|)^{\otimes N},
    \]
    with the inverse taken on \(\operatorname{supp}\rho_N\) and arbitrary POVM
    completion on \((\operatorname{supp}\rho_N)^\perp\), attains
    \(P_{\mathrm{col}}^{\mathrm{succ}}(N)\).

    An admissible individual adaptive strategy without quantum memory is a
    sequential protocol that measures one copy at a time. At step \(t\), after
    observing a classical history \(h_{t-1}\), the protocol chooses a POVM
    \(\{M^{(t,h_{t-1})}_y\}_y\) on a single copy of \(\mathcal{H}\), records
    the classical outcome \(y\), and discards the measured quantum system. The
    protocol may keep unlimited classical memory and may use arbitrary final
    classical postprocessing to guess \(j\). It may not store a quantum system
    between steps, entangle two copies, or couple a persistent quantum ancilla
    to more than one copy. Let \(P_{\mathrm{ind}}^{\mathrm{err}}(N)\) be the
    infimum of the average error probability over all such \(N\)-step
    protocols.

    The phrase "asymptotically as good as the collective measurement" is
    ambiguous in the OQP source. This entry records the strengthened
    ratio-limit version in the claim. Mere convergence
    \(|P_{\mathrm{ind}}^{\mathrm{succ}}(N)-P_{\mathrm{col}}^{\mathrm{succ}}(N)|\to 0\)
    is weaker and is not the canonical target, because both success
    probabilities can converge to \(1\) for many distinguishable finite
    ensembles. Equality of Chernoff exponents is also weaker unless it implies
    the ratio limit above.
  </def>

  <known>
    Open Quantum Problems, Problem 31, "Individual measurement strategies on
    geometrically uniform states", asks whether suitably designed individual
    measurements can asymptotically reach the minimum error of the collective
    square-root measurement for \(N\)-copy cyclic geometrically uniform states.
    The Open Quantum Problems list records Problem 31 in the Quantum
    communication category, with date 2005/10/06 and no last-progress date.
    Open Quantum Problems, Problem 31 does not specify whether "asymptotically
    reaches" means equality of error probabilities, ratio \(1\), equality of
    error exponents, or only success-probability convergence. This entry
    records the strengthened nontrivial ratio-\(1\) version; exponent-only or
    success-only results are progress unless they imply the ratio statement.

    Eldar and Forney, "On quantum detection and the square-root measurement",
    IEEE Transactions on Information Theory 47(3), 858-872 (2001), define
    geometrically uniform pure-state sets generated by finite abelian unitary
    groups and prove that the square-root measurement minimizes the probability
    of detection error for such sets. Applying their theorem to the cyclic group
    generated by \(U^{\otimes N}\) gives the collective optimum used in this
    entry.

    Acin, Bagan, Baig, Masanes, and Munoz-Tapia, "Multiple copy 2-state
    discrimination with individual measurements", Physical Review A 71, 032338
    (2005), solve the binary case. For two pure states they give a fixed
    unanimity-vote individual measurement with the same asymptotic error
    exponent as the collective Helstrom measurement, up to a prefactor, and an
    adaptive Bayesian-updating sequence of individual von Neumann measurements
    whose error probability is exactly the collective optimum for every copy
    number \(N\). This proves the claim for \(d=2\).

    Brandsen, Lian, Stubbs, Rengaswamy, and Pfister, "Adaptive procedures for
    discriminating between arbitrary tensor-product quantum states", Physical
    Review A 106, 012408 (2022), generalize the binary adaptive result: a
    locally greedy Bayesian-updating scheme optimally distinguishes any two
    tensor-product pure states. This strengthens the binary side of Open
    Quantum Problems, Problem 31 but does not address cyclic geometrically
    uniform ensembles with \(d\ge 3\).

    Chitambar, Duan, and Hsieh, "When Do Local Operations and Classical
    Communication Suffice for Two-Qubit State Discrimination?", IEEE
    Transactions on Information Theory 60(3), 1549-1561 (2014), study
    minimum-error LOCC discrimination for two-qubit ensembles and the \(N\)-copy
    trine ensemble. They prove that the rotationally invariant trine ensemble
    cannot be optimally identified by LOCC for any finite \(N\), while LOCC can
    discriminate the states optimally in the limit \(N\to\infty\) in the sense
    discussed there. This is directly relevant to the \(d=3\) cyclic case, but
    it is neither a finite-\(N\) equality theorem nor a general proof for all
    cyclic geometrically uniform ensembles in the ratio sense fixed here.

    Zhou, Chessa, Chitambar, and Leditzky, "On the distinguishability of
    geometrically uniform quantum states", Journal of Physics A: Mathematical
    and Theoretical 58, 415303 (2025), analyze GU discrimination using
    representation theory and semidefinite programming. For pure GU ensembles
    they give a streamlined proof of pretty-good-measurement optimality and
    discuss the \(n\)-copy setting through lower bounds on PGM success
    probability. Their results update the collective-measurement side of this
    entry but do not give a no-quantum-memory individual adaptive strategy
    matching the collective optimum for every cyclic pure-state ensemble.

    Achenbach, Leppajarvi, Lee, and Heinosaari, "Nonclassical traits in
    multi-copy state discrimination", arXiv:2604.26647v1 (2026), compare global,
    separable, LOCC, adaptive, non-adaptive, and fixed local strategies for
    multi-copy minimum-error discrimination and include geometrically uniform
    qubit examples. This recent work shows that the restricted-strategy
    hierarchy remains active in current literature, but it does not settle Open
    Quantum Problems, Problem 31 as formulated here.
  </known>

  <refs>
    <ref>Open Quantum Problems, Individual measurement strategies on geometrically uniform states, Problem 31, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/individual-measurement-strategies-on-geometrically-uniform-states, used for the cyclic GU problem statement, collective square-root-measurement provenance, no-quantum-memory motivation, and the cited \(d=2\) partial result.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 31, Individual measurement strategies on geometrically uniform states, in Quantum communication.</ref>
    <ref>Y. C. Eldar and G. D. Forney Jr., On quantum detection and the square-root measurement, IEEE Transactions on Information Theory 47(3), 858-872 (2001), DOI: 10.1109/18.915636, arXiv:quant-ph/0005132, especially Theorem 4, used for optimality of the square-root measurement for geometrically uniform pure-state sets generated by finite abelian unitary groups.</ref>
    <ref>A. Acin, E. Bagan, M. Baig, L. Masanes, and R. Munoz-Tapia, Multiple copy 2-state discrimination with individual measurements, Physical Review A 71, 032338 (2005), DOI: 10.1103/PhysRevA.71.032338, arXiv:quant-ph/0410097, used for the fixed unanimity-vote exponent result and the adaptive Bayesian-updating exact optimality result for \(d=2\).</ref>
    <ref>S. Brandsen, M. Lian, K. D. Stubbs, N. Rengaswamy, and H. D. Pfister, Adaptive procedures for discriminating between arbitrary tensor-product quantum states, Physical Review A 106, 012408 (2022), DOI: 10.1103/PhysRevA.106.012408, arXiv:1912.05087, used for the modern extension of binary pure-state Bayesian-updating optimality.</ref>
    <ref>E. Chitambar, R. Duan, and M.-H. Hsieh, When Do Local Operations and Classical Communication Suffice for Two-Qubit State Discrimination?, IEEE Transactions on Information Theory 60(3), 1549-1561 (2014), DOI: 10.1109/TIT.2013.2295356, arXiv:1308.1737, used for finite-\(N\) nonoptimality and asymptotic LOCC behavior of the \(N\)-copy trine ensemble.</ref>
    <ref>J. Zhou, S. Chessa, E. Chitambar, and F. Leditzky, On the distinguishability of geometrically uniform quantum states, Journal of Physics A: Mathematical and Theoretical 58(41), 415303 (2025), DOI: 10.1088/1751-8121/ae0a95, arXiv:2501.12376v2, used for current GU-discrimination status, pure-GU PGM optimality, and \(n\)-copy PGM bounds.</ref>
    <ref>T. Achenbach, L. Leppajarvi, H. Lee, and T. Heinosaari, Nonclassical traits in multi-copy state discrimination, arXiv:2604.26647v1 (2026), used for current literature on global, separable, LOCC, adaptive, non-adaptive, and fixed local strategy hierarchies in multi-copy minimum-error discrimination.</ref>
  </refs>

  <ask>
    Prove or disprove the strengthened ratio-limit version of OQP Problem 31
    for every finite-dimensional cyclic geometrically uniform pure-state
    ensemble satisfying the definitions above.

    A proof must give, for every such ensemble, a sequence of no-quantum-memory
    individual adaptive strategies whose average error probabilities
    \(P_{\mathrm{strat}}^{\mathrm{err}}(N)\) satisfy
    \[
      \lim_{N\to\infty}
      \frac{P_{\mathrm{strat}}^{\mathrm{err}}(N)}
           {P_{\mathrm{col}}^{\mathrm{err}}(N)}
      =1 .
    \]
    The proof may use arbitrary one-copy POVMs, adaptive choice of later POVMs
    from earlier classical outcomes, and arbitrary final classical
    postprocessing, but it may not use a persistent quantum memory, entangling
    operations across copies, or a collective measurement hidden inside a
    larger implementation.

    A disproof must give a finite-dimensional cyclic geometrically uniform
    pure-state ensemble satisfying the definitions above and prove that no
    admissible individual adaptive strategy has the stated ratio limit. Showing
    a gap at a fixed finite \(N\), showing failure of a fixed non-adaptive
    measurement, showing failure of a locally greedy rule, or matching only the
    collective error exponent is progress unless it rules out all admissible
    adaptive strategies in the ratio sense.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
