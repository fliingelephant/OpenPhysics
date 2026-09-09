# 24 The PPT-Squared Conjecture

```xml
<problem id="24">
  <status>open</status>
  <kind>proof</kind>
  <field>QIT</field>
  <name>The PPT-Squared Conjecture</name>
  <keys>ppt squared conjecture ppt square conjecture ppt^2 conjecture positive partial transpose completely positive completely copositive ppt map ppt channel entanglement breaking entanglement-breaking separable choi matrix choi jamiolkowski entanglement swapping quantum repeater private states data hiding gaussian channels choi-type maps qutrit d=3 two-qutrit high dimensions real hilbert space open quantum problems oqp problem 38 quantum communication</keys>

  <claim>
    For all positive integers \(d_1,d_2,d_3\), let
    \(T_1:\mathcal{M}_{d_1}(\mathbb{C})\to\mathcal{M}_{d_2}(\mathbb{C})\)
    and
    \(T_2:\mathcal{M}_{d_2}(\mathbb{C})\to\mathcal{M}_{d_3}(\mathbb{C})\)
    be linear maps that are both completely positive and completely
    copositive. Then the composition \(T_2\circ T_1\) is
    entanglement-breaking.
  </claim>

  <def>
    Let \(\mathcal{M}_d(\mathbb{C})\) be the algebra of \(d\times d\)
    complex matrices and let \(\mathcal{M}_d(\mathbb{C})^+\) be its cone of
    positive semidefinite matrices. The transpose map in the computational
    basis is denoted by \(\Theta_d(X)=X^T\).

    A linear map \(T:\mathcal{M}_{d_{\rm in}}(\mathbb{C})\to
    \mathcal{M}_{d_{\rm out}}(\mathbb{C})\) is positive if
    \(T(X)\succeq 0\) whenever \(X\succeq 0\). It is completely positive
    if \(\operatorname{id}_n\otimes T\) is positive for every \(n\ge 1\).
    It is completely copositive if \(\Theta_{d_{\rm out}}\circ T\) is
    completely positive; equivalently, \(T\circ\Theta_{d_{\rm in}}\) is
    completely positive. A PPT map means a map that is both completely
    positive and completely copositive. A PPT quantum channel is a PPT map
    that is trace-preserving.

    The Choi matrix of \(T\) is
    \[
      J(T)=\sum_{i,j=1}^{d_{\rm in}}
      |i\rangle\langle j|\otimes T(|i\rangle\langle j|)
      \in
      \mathcal{M}_{d_{\rm in}}(\mathbb{C})\otimes
      \mathcal{M}_{d_{\rm out}}(\mathbb{C}).
    \]
    Choi's theorem says that \(T\) is completely positive if and only if
    \(J(T)\succeq 0\). The map \(T\) is completely copositive if and only if
    the partial transpose of \(J(T)\) on either Choi tensor factor is
    positive semidefinite.

    A positive semidefinite operator \(X\) on
    \(\mathbb{C}^a\otimes\mathbb{C}^b\) is separable if it belongs to the
    conic hull of product positive semidefinite operators, i.e.
    \[
      X=\sum_k A_k\otimes B_k,\qquad
      A_k\in\mathcal{M}_a(\mathbb{C})^+,\quad
      B_k\in\mathcal{M}_b(\mathbb{C})^+ .
    \]
    For trace-one states this is the usual convex combination of product
    states. A completely positive map \(T\) is entanglement-breaking if
    \((\operatorname{id}_n\otimes T)(X)\) is separable for every
    \(n\ge 1\) and every \(X\succeq 0\) on
    \(\mathbb{C}^n\otimes\mathbb{C}^{d_{\rm in}}\). Equivalently, \(T\) is
    entanglement-breaking if and only if \(J(T)\) is separable across the
    Choi input-output bipartition.

    The equivalent entanglement-swapping formulation is as follows. Let
    \(\rho_{AB}\) and \(\tau_{CD}\) be finite-dimensional bipartite density
    operators that are PPT with respect to their indicated bipartitions.
    Let \(M_{BC}\) be a yes-outcome effect on \(B\otimes C\), so
    \(0\preceq M_{BC}\preceq I_{BC}\). Define the unnormalized conditional
    output on \(A\otimes D\) by
    \[
      \widetilde{\sigma}_{AD}
      =
      \operatorname{Tr}_{BC}\!\left[
        (I_A\otimes M_{BC}^{1/2}\otimes I_D)
        (\rho_{AB}\otimes\tau_{CD})
        (I_A\otimes M_{BC}^{1/2}\otimes I_D)
      \right].
    \]
    The probability of the yes outcome is
    \(p=\operatorname{Tr}\widetilde{\sigma}_{AD}\). If \(p\gt 0\), the
    normalized conditional state is
    \(\sigma_{AD}=\widetilde{\sigma}_{AD}/p\); if \(p=0\), that conditional
    state is undefined and gives no instance of the conjecture. Under the
    Choi-Jamiolkowski isomorphism, the claim above is equivalent to saying
    that every such \(\sigma_{AD}\) with \(p\gt 0\) is separable. The special
    choice of a maximally entangled effect on the middle systems recovers the
    Choi matrix of a composed map, and arbitrary effects correspond to the
    tensor-product reformulation of the conjecture.
  </def>

  <known>
    This entry was collected from Open Quantum Problems, Problem 38, "The
    PPT-squared conjecture". The OQP problem page states the entanglement
    swapping version: two PPT states are shared across \(AB\) and \(CD\), a
    positive yes-no measurement is performed on \(BC\), and the conjecture
    asks whether the conditional \(AD\) state is always separable. The OQP
    index lists Problem 38, "The PPT-squared conjecture", by M. Christandl in
    the Quantum communication category.

    Christandl, Muller-Hermes, and Wolf, "When Do Composed Maps Become
    Entanglement Breaking?", Annales Henri Poincare 20, 2295-2322 (2019),
    formulate the square version for a single PPT map
    \(T:\mathcal{M}_d\to\mathcal{M}_d\), prove it equivalent to the
    rectangular composition version used as this entry's claim, and prove it
    equivalent to a tensor-product entanglement-annihilation formulation. They
    also state that the conjecture is still open, prove the \(d=3\) case, and
    prove the conjecture for Gaussian channels.

    In dimension \(d=2\), a PPT Choi matrix on
    \(\mathbb{C}^2\otimes\mathbb{C}^2\) is separable by the
    Peres-Horodecki criterion, so every PPT map
    \(\mathcal{M}_2\to\mathcal{M}_2\) is already entanglement-breaking. The
    \(d=3\) case is proved by Christandl, Muller-Hermes, and Wolf, Annales
    Henri Poincare 20, 2295-2322 (2019), and independently by Chen, Yang, and
    Tang, "Positive-partial-transpose square conjecture for n=3", Physical
    Review A 99, 012337 (2019), using the fact that two-qutrit PPT states
    have Schmidt number at most two. These results do not settle dimensions
    \(d\ge 4\).

    Christandl and Ferrara, "Private States, Quantum Data Hiding, and the
    Swapping of Perfect Secrecy", Physical Review Letters 119, 220506 (2017),
    provide the quantum-key-repeater motivation: if the conjecture holds, PPT
    states cannot be used as a resource to swap entanglement or perfect
    secrecy through such an intermediate station in the OQP setting.

    Singh and Nechita, "The PPT^2 Conjecture Holds for All Choi-Type Maps",
    Annales Henri Poincare 23, 3311-3329 (2022), prove a stronger restricted
    result for diagonal-unitary-covariant and conjugate-diagonal-unitary-
    covariant maps, a class including Choi-type maps, depolarizing maps,
    dephasing maps, amplitude damping maps, and mixtures of these families.
    This is a restricted-class theorem, not a proof for arbitrary PPT maps.

    Jin, "Investigation of the PPT Squared Conjecture for High
    Dimensions", arXiv:2010.15554 (2020), propose higher-dimensional search
    schemes and describe a potential counterexample candidate from composing
    PPT channels in unsolved dimensions. They do not certify that the composed
    Choi matrix is entangled, and therefore do not disprove the conjecture.
    The low-rank and near-depolarizing examples in Christandl, Muller-Hermes,
    and Wolf, Annales Henri Poincare 20, 2295-2322 (2019), are likewise
    restricted progress, not disproofs.

    As of the cited 2026 sources, later works found further restricted positive
    results, for example random diagonal-orthogonal-covariant channels in
    Nechita and Park, "Random Covariant Quantum Channels", Annales Henri
    Poincare 27, 847-907 (2026), Cartan-covariant channels in Prudhoe,
    arXiv:2501.03959, and symplectic-covariant or conjugate-symplectic-covariant
    maps in Park, arXiv:2602.09860. These sources treat the original complex
    finite-dimensional conjecture as unresolved outside such classes.

    Chiribella, Davidson, Paulsen, and Rahaman, "Positive Maps and
    Entanglement in Real Hilbert Spaces", Annales Henri Poincare 24, 4139-4168
    (2023), show that a straightforward real-Hilbert-space version of the
    PPT-squared conjecture is false even in dimension two. This is a caveat
    about a different real theory; it is not a counterexample to the complex
    finite-dimensional claim in this entry.
  </known>

  <refs>
    <ref>Open Quantum Problems, The PPT-squared conjecture, Problem 38, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/the-ppt-squared-conjecture, used for the entanglement-swapping statement and collection provenance.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 38, The PPT-squared conjecture, in Quantum communication.</ref>
    <ref>M. Christandl and R. Ferrara, Private States, Quantum Data Hiding, and the Swapping of Perfect Secrecy, Physical Review Letters 119, 220506 (2017), DOI: 10.1103/PhysRevLett.119.220506, arXiv:1609.04696, used for quantum key repeater motivation and the swapping-of-secrecy setting.</ref>
    <ref>M. Christandl, A. Muller-Hermes, and M. M. Wolf, When Do Composed Maps Become Entanglement Breaking?, Annales Henri Poincare 20, 2295-2322 (2019), DOI: 10.1007/s00023-019-00774-7, arXiv:1807.01266, used for the rectangular map formulation, equivalences, the \(d=3\) result, Gaussian channels, and related partial results.</ref>
    <ref>L. Chen, Y. Yang, and W.-S. Tang, Positive-partial-transpose square conjecture for n=3, Physical Review A 99, 012337 (2019), DOI: 10.1103/PhysRevA.99.012337, arXiv:1807.03636, used for the independent \(d=3\) proof and \(d\ge 4\) status.</ref>
    <ref>R. Jin, Investigation of the PPT Squared Conjecture for High Dimensions, arXiv:2010.15554 (2020), used for higher-dimensional search schemes and the uncertified potential counterexample.</ref>
    <ref>S. Singh and I. Nechita, The PPT^2 Conjecture Holds for All Choi-Type Maps, Annales Henri Poincare 23, 3311-3329 (2022), DOI: 10.1007/s00023-022-01166-0, arXiv:2011.03809, used for the Choi-type and diagonal-covariant restricted-class theorem.</ref>
    <ref>I. Nechita and S.-J. Park, Random Covariant Quantum Channels, Annales Henri Poincare 27, 847-907 (2026), DOI: 10.1007/s00023-025-01558-y, arXiv:2403.03667, used for modern open-status language and random diagonal-orthogonal-covariant restricted results.</ref>
    <ref>S. Prudhoe, Cartan-covariant Quantum Channels and the PPT^2 conjecture, arXiv:2501.03959 (2025), used for a later restricted positive result.</ref>
    <ref>S.-J. Park, k-Positivity and high-dimensional bound entanglement under symplectic group symmetries, arXiv:2602.09860 (2026), used for a later restricted positive result.</ref>
    <ref>G. Chiribella, K. R. Davidson, V. I. Paulsen, and M. Rahaman, Positive Maps and Entanglement in Real Hilbert Spaces, Annales Henri Poincare 24, 4139-4168 (2023), DOI: 10.1007/s00023-023-01325-x, arXiv:2207.02510, used only for the real-Hilbert-space caveat.</ref>
  </refs>

  <ask>
    Prove the claim by showing that \(T_2\circ T_1\) is
    entanglement-breaking for every finite triple
    \((d_1,d_2,d_3)\) and every pair of PPT maps with the indicated source
    and target dimensions. Equivalently, prove that \(J(T_2\circ T_1)\) is
    separable for every such pair, or prove the equivalent entanglement-
    swapping statement for every finite-dimensional pair of PPT states and
    every yes-outcome effect with positive probability.

    Disprove the claim by giving finite dimensions \(d_1,d_2,d_3\) and PPT
    maps \(T_1,T_2\) for which \(T_2\circ T_1\) is not
    entanglement-breaking. A disproof must prove nonseparability of the Choi
    matrix \(J(T_2\circ T_1)\), for example by an entanglement witness or an
    equivalent separability obstruction. A numerical candidate, a full-rank
    PPT Choi matrix without an entanglement certificate, or a counterexample
    only to a real-Hilbert-space variant is progress or a variant result, not
    a disproof of this complex finite-dimensional claim.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
