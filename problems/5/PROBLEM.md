# 5 Qubit Formula for Relative Entropy of Entanglement

```xml
<problem id="5">
  <status>open</status>
  <kind>construction</kind>
  <field>QIT</field>
  <name>Qubit Formula for Relative Entropy of Entanglement</name>
  <keys>relative entropy of entanglement ree two-qubit two qubits qubit formula closest separable state css closest ppt state positive partial transpose ppt separable states peres horodecki criterion rains bound umegaki relative entropy closed formula eisert problem 8 open quantum problems oqp entanglement theory entanglement monotone</keys>

  <claim>
    There is a closed-form rule which, for every density operator \(\rho\) on
    \(\mathbb{C}^2\otimes\mathbb{C}^2\), returns the common value
    \[
      E_R^{\mathrm{sep}}(\rho)
      =
      \min_{\sigma\in\mathrm{Sep}_{2,2}} S(\rho\Vert\sigma)
      =
      E_R^{\mathrm{PPT}}(\rho)
      =
      \min_{\sigma\in\mathrm{PPT}_{2,2}} S(\rho\Vert\sigma)
    \]
    and at least one minimizing state \(\sigma_*(\rho)\).
  </claim>

  <def>
    A two-qubit density operator is a positive semidefinite operator \(\rho\)
    with \(\operatorname{tr}\rho=1\) on
    \(\mathbb{C}^2\otimes\mathbb{C}^2\). The quantum relative entropy is
    \[
      S(\rho\Vert\sigma)
      =
      \operatorname{tr}\rho(\log\rho-\log\sigma)
    \]
    when \(\operatorname{supp}(\rho)\subseteq\operatorname{supp}(\sigma)\),
    and \(+\infty\) otherwise. Logs are natural logarithms; using another
    fixed base only rescales the value.

    \(\mathrm{Sep}_{2,2}\) is the convex hull of product density operators
    \(\alpha\otimes\beta\), where \(\alpha\) and \(\beta\) are one-qubit
    density operators. With matrix elements in a product basis, partial
    transpose on the second tensor factor is
    \[
      \langle ij|\sigma^\Gamma|\ell m\rangle
      =
      \langle im|\sigma|\ell j\rangle .
    \]
    \(\mathrm{PPT}_{2,2}\) is the set of two-qubit density operators
    satisfying \(\sigma^\Gamma\succeq 0\). By the Peres-Horodecki criterion,
    \(\mathrm{Sep}_{2,2}=\mathrm{PPT}_{2,2}\), so the separable and PPT
    variants in the claim agree in this dimension. This equality is not used
    outside \(2\otimes2\).

    For this entry, a closed-form rule means an explicit finite analytic rule
    in terms of the entries of \(\rho\), roots of finite-degree polynomials
    whose coefficients are explicit functions of those entries, spectral
    decompositions of explicitly specified finite matrices, standard scalar
    functions such as logarithms, and finitely many case distinctions. It must
    not introduce a problem-specific special function defined only as the
    solution of the relative-entropy minimization, and it must not leave a
    minimization over \(\mathrm{Sep}_{2,2}\), \(\mathrm{PPT}_{2,2}\),
    entanglement witnesses, or a closest-state parameter space; it also must
    not be an iterative numerical scheme or a semidefinite or convex program.

    The Rains optimization is a different relaxation, commonly written as a
    minimization over positive semidefinite operators \(\tau\) satisfying
    \(\|\tau^\Gamma\|_1\le 1\). It is not part of the claim unless a proposed
    solution separately proves that it yields the same closest separable or
    PPT state and value for the two-qubit \(E_R\) above.
  </def>

  <known>
    Open Quantum Problems records Problem 8, "Qubit formula for Relative
    Entropy of Entanglement", in Entanglement theory and asks for a closed
    formula for systems consisting of two qubits. The OQP problem page defines
    \(D\) as the convex set of separable states or PPT states; in the
    two-qubit domain of this entry those two choices coincide, but this entry
    keeps the two variants named separately.

    Miranowicz and Ishizaka, "Closed formula for the relative entropy of
    entanglement", Phys. Rev. A 78, 032310 (2008), address the OQP problem but
    solve the inverse problem: for a given closest separable state
    \(\sigma\), characterize the entangled states \(\rho\) for which
    \(\sigma\) is closest. They state that the direct problem is to solve
    their relation conversely for \(\sigma\) from \(\rho\), and that this is
    possible in special cases but not in general by their method. Thus the
    2008 result is progress on the claim, not a closed-form rule for arbitrary
    input \(\rho\). Following this formulation, this entry treats solving the
    optimization as returning both the value \(E_R(\rho)\) and at least one
    closest separable, equivalently closest PPT, state. The 2008 formula builds
    on Ishizaka's earlier result characterizing states associated with a given
    closest disentangled state.

    Friedland and Gour, "An explicit expression for the relative entropy of
    entanglement in all dimensions", J. Math. Phys. 52, 052201 (2011),
    generalize the same inverse problem to multipartite systems in all finite
    dimensions: given a state on the boundary of the separable set, they give
    an explicit expression for the entangled states for which it is a closest
    separable state. Their two-qubit specialization reduces to Miranowicz and
    Ishizaka, Phys. Rev. A 78, 032310 (2008), so it likewise does not provide
    the direct closed-form rule demanded in the claim.

    Park, "Relative Entropy of Entanglement for Two-Qubit State with
    \(z\)-directional Bloch Vectors", Int. J. Quantum Inf. 8, 869 (2010),
    records that no closed formula was known for arbitrary two-qubit states
    and derives special-family results. Kim, Hwang, Jung, and Park,
    "Difficulties in analytic computation for relative entropy of
    entanglement", Phys. Rev. A 81, 052325 (2010), analyze why simple
    geometric methods for special families do not extend to arbitrary
    two-qubit states.
  </known>

  <refs>
    <ref>Open Quantum Problems, Problem 8, Qubit formula for Relative Entropy of Entanglement, https://oqp.iqoqi.oeaw.ac.at/qubit-formula-for-relative-entropy-of-entanglement; collection provenance for this entry.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems; lists Problem 8 in Entanglement theory.</ref>
    <ref>O. Krueger and R. F. Werner, Some Open Problems in Quantum Information Theory, arXiv:quant-ph/0504166, Problem 8.</ref>
    <ref>V. Vedral and M. B. Plenio, Entanglement measures and purification procedures, Phys. Rev. A 57, 1619 (1998), DOI 10.1103/PhysRevA.57.1619.</ref>
    <ref>M. Horodecki, P. Horodecki, and R. Horodecki, Separability of mixed states: necessary and sufficient conditions, Phys. Lett. A 223, 1 (1996), DOI 10.1016/S0375-9601(96)00706-2.</ref>
    <ref>S. Ishizaka, Analytical formula connecting entangled state and the closest disentangled state, Phys. Rev. A 67, 060301(R) (2003), DOI 10.1103/PhysRevA.67.060301, arXiv:quant-ph/0301107.</ref>
    <ref>A. Miranowicz and S. Ishizaka, Closed formula for the relative entropy of entanglement, Phys. Rev. A 78, 032310 (2008), DOI 10.1103/PhysRevA.78.032310, arXiv:0805.3134.</ref>
    <ref>S. Friedland and G. Gour, An explicit expression for the relative entropy of entanglement in all dimensions, J. Math. Phys. 52, 052201 (2011), DOI 10.1063/1.3591132, arXiv:1007.4544.</ref>
    <ref>D. Park, Relative Entropy of Entanglement for Two-Qubit State with \(z\)-directional Bloch Vectors, Int. J. Quantum Inf. 8, 869 (2010), DOI 10.1142/S0219749910006629, arXiv:1005.4777.</ref>
    <ref>H. Kim, M.-R. Hwang, E. Jung, and D. Park, Difficulties in analytic computation for relative entropy of entanglement, Phys. Rev. A 81, 052325 (2010), DOI 10.1103/PhysRevA.81.052325, arXiv:1002.4695.</ref>
  </refs>

  <ask>
    Prove the claim by giving a closed-form rule in the sense defined above
    and proving that, for every two-qubit density operator \(\rho\), the
    returned \(\sigma_*(\rho)\) lies in \(\mathrm{Sep}_{2,2}\) and
    \(\mathrm{PPT}_{2,2}\), satisfies the support condition for finite
    \(S(\rho\Vert\sigma_*(\rho))\), and globally minimizes the relative
    entropy over the equivalent separable and PPT feasible sets. For separable
    \(\rho\), the rule may return \(\sigma_*(\rho)=\rho\) and value \(0\).

    Because "closed form" is not a formal language independent of convention,
    the constructive direction is the canonical resolution of this entry. A
    claimed disproof must first specify a mathematically precise class
    \(\mathcal{C}\) of candidate rules, prove that \(\mathcal{C}\) captures
    the closed-form notion used in this entry, and then prove that no rule in
    \(\mathcal{C}\) can return both the value and a minimizing state for all
    two-qubit density operators. Showing that a particular proposed formula or
    a narrower formal class fails is progress or a gap, not a disproof of the
    entry, unless it rules out the stated closed-form notion.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
