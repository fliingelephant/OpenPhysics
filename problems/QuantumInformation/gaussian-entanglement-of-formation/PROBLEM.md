# ET029 Gaussian Entanglement of Formation Equality

```xml
<problem id="ET029">
  <status>open</status>
  <name>Gaussian Entanglement of Formation Equality</name>
  <keys>gaussian entanglement of formation geof gaussian eof entanglement of formation eof gaussian states bosonic gaussian states continuous variables cv bipartite multimode gaussian states covariance matrix convex roof borel decomposition countable decomposition pure gaussian decomposition displaced squeezed gaussian state gaussian weights gaussian convex roof two-mode symmetric two-mode akbari alijanzadeh boura marian ivan simon tserkis wolf giedke krueger werner cirac open quantum problems oqp problem 29 entanglement theory</keys>

  <claim>
    For every finite pair of local mode numbers \(m,n\ge 1\) and every
    bipartite \((m+n)\)-mode bosonic Gaussian state \(\rho_{AB}\) on
    \[
      \mathcal H_A\otimes\mathcal H_B
      =
      L^2(\mathbb R^m)\otimes L^2(\mathbb R^n),
    \]
    the ordinary Borel-measure entanglement of formation is equal to the
    Gaussian entanglement of formation:
    \[
      E_F^{\mathrm{Bor}}(\rho_{AB})
      =
      E_F^{\mathrm G}(\rho_{AB}) .
    \]
  </claim>

  <def>
    Let
    \[
      R=(Q_{A,1},P_{A,1},\ldots,Q_{A,m},P_{A,m},
         Q_{B,1},P_{B,1},\ldots,Q_{B,n},P_{B,n})
    \]
    be canonical quadratures satisfying \([R_j,R_k]=i\Omega_{jk}\). A
    Gaussian state is a density operator whose Weyl characteristic function is
    Gaussian. This entry uses the covariance convention
    \[
      V_{jk}=\operatorname{tr}\rho\{R_j-d_j\mathbf 1,R_k-d_k\mathbf 1\}_+,
      \qquad d_j=\operatorname{tr}\rho R_j ,
    \]
    so \(V\) is real symmetric and satisfies the uncertainty relation
    \(V+i\Omega\succeq 0\). A pure Gaussian covariance matrix \(\gamma\) has
    vacuum symplectic eigenvalue \(1\), equivalently
    \((\Omega\gamma)^2=-\mathbf 1\). First moments are omitted because
    phase-space displacements are local unitaries and do not change bipartite
    entanglement.

    For a pure state \(\psi_{AB}=|\psi\rangle\langle\psi|\), its entropy of
    entanglement is
    \[
      E(\psi_{AB})
      =
      S(\operatorname{tr}_B\psi_{AB}),
      \qquad
      S(\omega)=-\operatorname{tr}\omega\log_2\omega .
    \]
    The Borel-measure entanglement of formation is
    \[
      E_F^{\mathrm{Bor}}(\rho_{AB})
      =
      \inf_{\mu:\,\bar\rho(\mu)=\rho_{AB}}
      \int E(\psi_{AB})\,d\mu(\psi_{AB}),
    \]
    where \(\mu\) ranges over Borel probability measures on the pure-state
    space and \(\bar\rho(\mu)\) is the barycenter
    \(\int \psi_{AB}\,d\mu(\psi_{AB})\). The countable-discrete variant
    \(E_F^{\mathrm{disc}}\) restricts \(\mu\) to countable pure-state
    decompositions with exact barycenter \(\rho_{AB}\). Shirokov, Theory
    Probab. Appl. 52, 250 (2008), Section 8, implies that
    \(E_F^{\mathrm{Bor}}(\rho_{AB})=E_F^{\mathrm{disc}}(\rho_{AB})\) whenever
    at least one local entropy is finite. Every finite-mode Gaussian state has
    finite local entropies, so the ordinary countable-discrete and Borel
    definitions agree on the domain of this entry. Exact finite ensembles are
    available in finite-dimensional systems, but a generic mixed finite-mode
    Gaussian state is infinite rank and need not be an exact finite convex
    combination of pure states. The claim uses the Borel version because
    Gaussian decompositions are naturally continuous displacement ensembles.

    For a Gaussian state \(\rho_V\) with covariance matrix \(V\), the Gaussian
    entanglement of formation is the same convex-roof quantity with the pure
    states restricted to pure Gaussian states. By Wolf, Giedke, Krueger,
    Werner, and Cirac, Phys. Rev. A 69, 052320 (2004), Proposition 1, this is
    equivalently
    \[
      E_F^{\mathrm G}(\rho_V)
      =
      \inf_{\gamma:\,\gamma\ \mathrm{pure},\,V-\gamma\succeq 0}
      E(\psi_\gamma),
    \]
    where \(\psi_\gamma\) is any pure Gaussian state with covariance
    \(\gamma\). The condition \(V-\gamma\succeq0\) gives a decomposition of
    \(\rho_V\) into phase-space translates of \(\psi_\gamma\) with a Gaussian
    probability distribution of displacements. Thus \(E_F^{\mathrm G}\) is
    always an upper bound on \(E_F^{\mathrm{Bor}}\). The problem is whether
    non-Gaussian pure-state ensembles can ever lower the ordinary value below
    this Gaussian-restricted value.

    This entry is bipartite. It is not about multipartite alpha-entanglement of
    formation, Gaussian Renyi-\(2\) entanglement of formation, Gaussian
    intrinsic entanglement, logarithmic negativity, or the regularized
    entanglement cost, unless a proposed solution proves an implication for the
    one-copy von Neumann entropy convex roof above.
  </def>

  <known>
    Open Quantum Problems records Problem 29, "Entanglement of formation for
    Gaussian states", in the Entanglement theory category. The Open Quantum
    Problems index lists Problem 29 with contact O. Krueger and date
    2005/04/20. OQP Problem 29 asks whether the entanglement of formation of a
    Gaussian state can always be attained by decompositions into pure Gaussian
    states that are translates of one squeezed Gaussian state with Gaussian
    weights; in the terminology of this entry, this is the equality
    \(E_F^{\mathrm{Bor}}=E_F^{\mathrm G}\).

    Uhlmann, "Entropy and optimal decompositions of states relative to a
    maximal commutative subalgebra", Open Syst. Inf. Dyn. 5, 209-228 (1998),
    develops the finite-dimensional convex-roof framework underlying finite
    optimal decompositions. Shirokov, "On properties of quantum channels
    related to their classical capacity", Theory Probab. Appl. 52, 250-276
    (2008), Section 8, gives the infinite-dimensional Borel-measure convex
    closure formulation and the finite-entropy coincidence used above. These
    results justify treating the ordinary countable-discrete and Borel versions
    as the same quantity for finite-mode Gaussian states, but they do not
    impose Gaussianity of an optimal ensemble.

    Wolf, Giedke, Krueger, Werner, and Cirac, "Gaussian entanglement of
    formation", Phys. Rev. A 69, 052320 (2004), introduce
    \(E_F^{\mathrm G}\) for bipartite Gaussian states, prove the covariance
    matrix reduction in Proposition 1 for arbitrary finite numbers of modes,
    solve the corresponding one-mode-per-party Gaussian optimization, and prove
    \(E_F^{\mathrm G}=E_F\) for symmetric two-mode Gaussian states. Their
    result supplies a computable Gaussian upper bound in all finite multimode
    cases, not the general equality in the claim.

    Giedke, Wolf, Krueger, Werner, and Cirac, "Entanglement of formation for
    symmetric Gaussian states", Phys. Rev. Lett. 91, 107901 (2003), prove the
    ordinary entanglement of formation for symmetric two-mode Gaussian states
    by showing an extremal relation between EPR-type correlations and
    entanglement. This is the symmetric two-mode equality case recorded by OQP.

    Ivan and Simon, "Entanglement of Formation for Gaussian States",
    arXiv:0808.1658 (2008), compute arbitrary two-mode Gaussian-state
    entanglement of formation conditional on a conjectured extremal property of
    a generalized EPR correlation. Marian and Marian, "Entanglement of
    formation for an arbitrary two-mode Gaussian state", Phys. Rev. Lett. 101,
    220403 (2008), claim an optimal pure-state decomposition for arbitrary
    two-mode Gaussian states and equality with the Gaussian value. OQP records
    Akbari-Kourbolagh and Alijanzadeh-Boura, "On the entanglement of formation
    of two-mode Gaussian states: a compact form", Quantum Inf. Process. 14,
    4179-4199 (2015), as removing the symmetry assumption and proving the
    two-mode case while observing that the earlier Marian-Marian proof was
    incomplete. The Akbari-Kourbolagh and Alijanzadeh-Boura comparison table
    nevertheless reports values labelled \(E_F(\rho)\) below values labelled
    \(E_F^G(\rho)\), so this entry does not use that paper alone as a clean
    proof of the equality. Instead, it records that OQP and later two-mode
    literature, including Tserkis and Ralph, Phys. Rev. A 96, 062338 (2017),
    and Li, Das, Tserkis, Narang, Lam, Gu, and Assad, Sci. Rep. 13, 11722
    (2023), treat the one-mode-per-party equality status as settled. Separate
    closed-form or numerical evaluation questions for arbitrary two-mode
    covariance matrices remain active.

    Serafini, Adesso, and Illuminati, "Unitarily localizable entanglement of
    Gaussian states", Phys. Rev. A 71, 032349 (2005), show that bisymmetric
    \((m+n)\)-mode Gaussian states are locally unitarily equivalent to a
    tensor product of one two-mode state and \(m+n-2\) uncorrelated single-mode
    states, and identify a subset whose multimode entanglement of formation is
    analytically computable. Such high-symmetry reductions are partial
    multimode progress, not a proof of the equality for arbitrary finite
    bipartite multimode Gaussian states.

    Hayden, Horodecki, and Terhal, "The asymptotic entanglement cost of
    preparing a quantum state", J. Phys. A 34, 6891-6898 (2001), prove in
    finite dimensions that entanglement cost is the regularized entanglement of
    formation. OQP Problem 29 notes this because operational entanglement cost
    requires regularization. A statement about
    \(\lim_{k\to\infty} k^{-1}E_F(\rho^{\otimes k})\), or about the Gaussian
    version of that regularized quantity, does not by itself settle the
    one-copy equality in the claim.

    As of 2026-05-22, no accepted proof or counterexample was found for the
    all finite-mode bipartite multimode equality
    \(E_F^{\mathrm{Bor}}=E_F^{\mathrm G}\). Results about Gaussian Renyi-\(2\)
    entanglement of formation, Gaussian intrinsic entanglement, Gaussian key
    distillation bounds, multipartite Gaussian entanglement of formation, or
    two-mode entanglement potentials address related but different quantities
    unless they are explicitly converted to the one-copy von Neumann entropy
    convex roof in this entry.
  </known>

  <refs>
    <ref>Open Quantum Problems, Problem 29, Entanglement of formation for Gaussian states, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/entanglement-of-formation-for-gaussian-states, used for the problem statement, finite-versus-Borel background, Gaussian entanglement of formation formulation, two-mode partial results, and collection provenance.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 29, Entanglement of formation for Gaussian states, in Entanglement theory.</ref>
    <ref>A. Uhlmann, Entropy and optimal decompositions of states relative to a maximal commutative subalgebra, Open Systems and Information Dynamics 5, 209-228 (1998), DOI 10.1023/A:1009664331611, arXiv:quant-ph/9704017.</ref>
    <ref>M. E. Shirokov, On properties of quantum channels related to their classical capacity, Theory of Probability and its Applications 52, 250-276 (2008), DOI 10.1137/S0040585X97982980, arXiv:quant-ph/0411091, Section 8.</ref>
    <ref>M. M. Wolf, G. Giedke, O. Krueger, R. F. Werner, and J. I. Cirac, Gaussian entanglement of formation, Physical Review A 69, 052320 (2004), DOI 10.1103/PhysRevA.69.052320, arXiv:quant-ph/0306177, especially Proposition 1.</ref>
    <ref>G. Giedke, M. M. Wolf, O. Krueger, R. F. Werner, and J. I. Cirac, Entanglement of formation for symmetric Gaussian states, Physical Review Letters 91, 107901 (2003), DOI 10.1103/PhysRevLett.91.107901, arXiv:quant-ph/0304042.</ref>
    <ref>J. Solomon Ivan and R. Simon, Entanglement of Formation for Gaussian States, arXiv:0808.1658 (2008), used for the conditional arbitrary two-mode calculation based on a generalized EPR extremality conjecture.</ref>
    <ref>P. Marian and T. A. Marian, Entanglement of formation for an arbitrary two-mode Gaussian state, Physical Review Letters 101, 220403 (2008), DOI 10.1103/PhysRevLett.101.220403, arXiv:0809.0321.</ref>
    <ref>Y. Akbari-Kourbolagh and H. Alijanzadeh-Boura, On the entanglement of formation of two-mode Gaussian states: a compact form, Quantum Information Processing 14, 4179-4199 (2015), DOI 10.1007/s11128-015-1119-5, arXiv:1402.3321.</ref>
    <ref>S. Tserkis and T. C. Ralph, Quantifying entanglement in two-mode Gaussian states, Physical Review A 96, 062338 (2017), DOI 10.1103/PhysRevA.96.062338, arXiv:1705.03612.</ref>
    <ref>A. Serafini, G. Adesso, and F. Illuminati, Unitarily localizable entanglement of Gaussian states, Physical Review A 71, 032349 (2005), DOI 10.1103/PhysRevA.71.032349, arXiv:quant-ph/0411109.</ref>
    <ref>P. M. Hayden, M. Horodecki, and B. M. Terhal, The asymptotic entanglement cost of preparing a quantum state, Journal of Physics A: Mathematical and General 34, 6891-6898 (2001), DOI 10.1088/0305-4470/34/35/314, arXiv:quant-ph/0008134.</ref>
    <ref>B. Li, A. Das, S. Tserkis, P. Narang, P. K. Lam, M. Gu, and S. M. Assad, On the equivalence between squeezing and entanglement potential for two-mode Gaussian states, Scientific Reports 13, 11722 (2023), DOI 10.1038/s41598-023-38572-1.</ref>
  </refs>

  <ask>
    Prove or disprove the claim that ordinary entanglement of formation and
    Gaussian entanglement of formation coincide for every finite-mode
    bipartite Gaussian state.

    A proof must show that for every covariance matrix \(V\) of a bipartite
    Gaussian state \(\rho_V\), no Borel probability measure over arbitrary
    pure states with barycenter \(\rho_V\) has strictly smaller average
    entropy of entanglement than the best pure-Gaussian covariance matrix
    \(\gamma\preceq V\) in the Wolf-Giedke-Krueger-Werner-Cirac
    characterization. Equivalently, it must prove
    \(E_F^{\mathrm{Bor}}(\rho_V)\ge E_F^{\mathrm G}(\rho_V)\), since the
    reverse inequality follows from the Gaussian restriction. The proof may
    cite the accepted two-mode case for \(m=n=1\), but it must cover arbitrary
    finite \(m,n\).

    A disproof must exhibit finite local mode numbers \(m,n\) and a
    bipartite Gaussian state \(\rho_V\) for which
    \(E_F^{\mathrm{Bor}}(\rho_V)\lt E_F^{\mathrm G}(\rho_V)\), and must prove
    both a strict upper bound on the ordinary Borel-measure convex roof and a
    strictly larger lower bound on the Gaussian-restricted value. Giving a
    non-Gaussian ensemble that improves one proposed Gaussian decomposition is
    progress only, unless it beats the global Gaussian infimum. A counterexample
    for a Renyi-\(2\), logarithmic-negativity, multipartite, asymptotic,
    energy-constrained, or Gaussian-operations-only variant is progress only
    unless it is converted to the one-copy von Neumann entropy quantities
    defined above.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
