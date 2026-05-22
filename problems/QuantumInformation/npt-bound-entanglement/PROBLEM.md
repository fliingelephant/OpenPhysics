# QI004 NPT Bound Entanglement

```xml
<problem id="QI004">
  <status>open</status>
  <name>NPT Bound Entanglement</name>
  <keys>npt nppt bound entanglement npt bound entangled negative partial transpose nondistillable entanglement undistillability implies ppt nondistillable n-copy distillability entanglement distillation ppt positive partial transpose werner state two-co-positive 2-co-positive 2-positive map tensor-stable positive map quantum information</keys>

  <claim>
    There exist an integer \(d \gt 2\) and a bipartite density operator
    \(\rho\) on \(\mathbb{C}^d\otimes\mathbb{C}^d\) such that
    \(\rho^\Gamma\) has a negative eigenvalue and \(\rho\) is not
    \(n\)-copy distillable for any finite \(n\ge 1\).
  </claim>

  <def>
    A bipartite density operator on
    \(\mathbb{C}^d\otimes\mathbb{C}^d\) is a positive semidefinite operator
    \(\rho\) with \(\operatorname{tr}\rho=1\). With matrix elements written
    in a product basis,
    \[
      \rho_{ij,\ell m}=\langle ij|\rho|\ell m\rangle ,
    \]
    its partial transpose on the second tensor factor is the operator
    \(\rho^\Gamma\) defined by
    \[
      \langle ij|\rho^\Gamma|\ell m\rangle
      =
      \langle im|\rho|\ell j\rangle .
    \]
    The state is PPT if \(\rho^\Gamma\succeq 0\), and NPT if
    \(\rho^\Gamma\) has at least one strictly negative eigenvalue.

    A state is separable if it is a convex combination of product states, and
    entangled otherwise. Every separable finite-dimensional bipartite state is
    PPT, hence every NPT state is entangled. In this entry, bound entangled
    means entangled and nondistillable under LOCC from arbitrarily many copies.

    For \(n\ge 1\), \(\rho\) is \(n\)-copy distillable if there exist
    rank-two projections \(P=P^\dagger=P^2\) and \(Q=Q^\dagger=Q^2\) on
    \((\mathbb{C}^d)^{\otimes n}\) such that
    \[
      (P\otimes Q)(\rho^\Gamma)^{\otimes n}(P\otimes Q)
    \]
    has a negative eigenvalue. The state is distillable if it is
    \(n\)-copy distillable for at least one finite \(n\); otherwise it is
    nondistillable.
  </def>

  <known>
    P. Horodecki, Ł. Rudnicki, and K. Życzkowski, Five Open Problems in
    Quantum Information Theory, PRX Quantum 3, 010101 (2022), Problem 4,
    state the task as establishing whether bound entangled states with
    negative partial transpose exist. They then restrict to equal local
    dimensions \(d\otimes d\), because the general \(d\otimes d'\) case can be
    reduced to that setting.

    Open Quantum Problems records the same decision problem as Problem 2,
    "Undistillability implies PPT?". Its converse formulation asks whether
    every nondistillable bipartite state is PPT. This entry's claim holds if
    and only if that OQP implication is false: a counterexample is a
    nondistillable NPT state, and every NPT state is entangled, so it is
    exactly NPT bound entanglement in the terminology used here. Conversely,
    proving the OQP implication would disprove the existence claim in this
    entry.

    In \(2\otimes 2\), every entangled state is distillable, so the claim
    cannot hold there. PPT entangled states are nondistillable and give bound
    entanglement in finite bipartite dimensions where such PPT entangled states
    exist, but they do not satisfy the NPT condition in the claim.

    Horodecki and Horodecki's Werner-state reduction says that NPT
    nondistillable entanglement exists if and only if there exists an NPT
    nondistillable Werner state in local dimension \(d \gt 2\). For
    Werner states
    \[
      \rho(d,\alpha)=\frac{I_d\otimes I_d+\alpha V}{d^2+\alpha d},
    \]
    where \(V\) is the swap operator. For this parametrization, Djokovic's
    Proposition 1.3, with \(t=-\alpha\), gives that the states are NPT for
    \(\alpha\in[-1,-1/d)\), are \(1\)-copy distillable for
    \(\alpha\lt -1/2\), and are \(1\)-copy nondistillable for
    \(\alpha\in[-1/2,1]\). The conjecturally nondistillable NPT part of this
    interval is \(\alpha\in[-1/2,-1/d)\). Horodecki, Rudnicki, and
    Życzkowski, PRX Quantum 3, 010101 (2022), Problem 5, single out the point
    \(\alpha=-2/d\), where \(\rho(d,\alpha)^\Gamma\) is proportional to a
    dichotomic unitary; this point coincides with the \(1\)-copy boundary only
    for \(d=4\). This Werner formulation is an equivalent route to the claim,
    not the separate Problem 5 target about the two-ququart state
    \(\rho(4,-1/2)\).
  </known>

  <refs>
    <ref>P. Horodecki, Ł. Rudnicki, and K. Życzkowski, Five Open Problems in Quantum Information Theory, PRX Quantum 3, 010101 (2022), Problems 4 and 5.</ref>
    <ref>https://doi.org/10.1103/PRXQuantum.3.010101</ref>
    <ref>https://arxiv.org/abs/2002.03233</ref>
    <ref>Open Quantum Problems, Problem 2, Undistillability implies PPT?, https://oqp.iqoqi.oeaw.ac.at/undistillability-implies-ppt; collection provenance for this entry.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 2, Undistillability implies ppt?, in Entanglement theory.</ref>
    <ref>D. Z. Djokovic, On two-distillable Werner states, Entropy 18, 216 (2016), Proposition 1.3, arXiv:1003.4337.</ref>
    <ref>M. Horodecki, P. Horodecki, and R. Horodecki, Mixed-state entanglement and distillation: Is there a 'bound' entanglement in Nature?, Phys. Rev. Lett. 80, 5239 (1998).</ref>
    <ref>M. Horodecki and P. Horodecki, Reduction criterion of separability and limits for a class of protocols of entanglement distillation, Phys. Rev. A 59, 4206 (1999).</ref>
    <ref>D. P. DiVincenzo, P. W. Shor, J. A. Smolin, B. M. Terhal, and A. V. Thapliyal, Evidence for bound entangled states with negative partial transpose, Phys. Rev. A 61, 062312 (2000).</ref>
  </refs>

  <ask>
    Prove the claim by giving some finite \(d \gt 2\) and a state \(\rho\)
    satisfying the NPT condition and proving that no finite-copy distillation
    witness \(n,P,Q\) as defined above exists. A proof may instead establish
    the equivalent Werner-state formulation by proving that some NPT Werner
    state in local dimension \(d \gt 2\) is nondistillable.

    To disprove the claim in the equal-dimension formulation, prove that for
    every \(d \gt 2\) and every NPT state on
    \(\mathbb{C}^d\otimes\mathbb{C}^d\), there are finite \(n\) and rank-two
    projectors \(P,Q\) satisfying the distillability condition above. Showing
    distillability only for a fixed dimension or for a proper subfamily, such
    as Werner states, is progress but not a disproof unless combined with a
    reduction covering all NPT states; the Horodecki-Horodecki Werner-state
    reduction is one such route if all NPT Werner states are proved
    distillable.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
