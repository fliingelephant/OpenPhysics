# QTD024 Quasi-Local Effective Hamiltonians of Thermal Marginals

```xml
<problem id="QTD024">
  <status>open</status>
  <name>Quasi-Local Effective Hamiltonians of Thermal Marginals</name>
  <keys>quasi-local effective hamiltonian, local effective hamiltonian, weak local effective hamiltonian, strong local effective hamiltonian, marginal hamiltonian, boundary hamiltonian, gibbs state marginal, thermal state marginal, conditional expectation, commuting hypothesis, non-commuting interactions, high temperature gibbs state, mixing condition, decay of correlations, temperature is local, quantum spin system, cluster expansion, non-commutative cluster expansion, quantum belief propagation, bluhm capel perez-hernandez, kuwahara kato brandao, threshold temperature clustering, conditional mutual information clustering</keys>

  <claim>
    Fix \(g\in\mathbb N\) and let \(V=\mathbb Z^g\) carry the sup-distance
    \(\mathrm{dist}(x,y):=\|x-y\|_\infty\). Let
    \(\Phi=(\Phi_X)_{X\in\mathcal P_f(V)}\) be a local interaction on \(V\),
    i.e. a family of Hermitian operators \(\Phi_X=\Phi_X^*\) supported on
    the finite sets \(X\subset V\) (fully specified in the definitions
    below),
    and suppose \(\Phi\) is quasi-local in the sense that
    \[
      \|\Phi\|_{\varepsilon,\mathbf b}
      :=\sup_{x\in V}\sum_{X\ni x}\|\Phi_X\|\,e^{\varepsilon|X|+\mathbf b(X)}
      \lt \infty
    \]
    for some \(\varepsilon\gt0\) and some subadditive
    \(\mathbf b:\mathcal P_f(V)\to[0,\infty)\) (the standard
    exponentially-decaying case is \(\mathbf b(X)=\mu\,\mathrm{diam}(X)\) for
    a fixed \(\mu\gt0\)). The claim covers every such \(\Phi\), whether or
    not it satisfies the Commuting Hypothesis defined below.

    Claim: for every such \(\Phi\) there exists
    \(\beta_*=\beta_*(\Phi,\varepsilon,\mathbf b)\gt0\) such that, for every
    real \(\beta\) with \(0\lt\beta\le\beta_*\), the quantum spin system
    with interaction \(\Phi\) has weak local effective Hamiltonians at
    inverse temperature \(\beta\) (defined below): for every \(L\subset V\)
    there is a local interaction \(\widehat\Phi^{L,\beta}\) on \(V\)
    satisfying properties (i)-(iii) of that definition, and moreover
    \[
      \|\widehat\Phi^{L,\beta}\|_{\mathbf b}
      :=\sup_{x\in V}\sum_{X\ni x}\|\widehat\Phi_X^{L,\beta}\|\,e^{\mathbf b(X)}
      \lt\infty .
    \]

    This is the weaker of two natural formalizations: it asks only for weak
    local effective Hamiltonians, not for strong local effective
    Hamiltonians; it asks only for a qualitative finite \(\mathbf b\)-decay
    bound, not for a quantitative constant matching Theorem 3.6 or Theorem
    3.8 of the source below; and it restricts \(\beta\) to the reals, where
    Theorem 3.6 of the source allows complex \(\beta\). Bluhm, Capel, and
    Pérez-Hernández, "Strong decay of correlations for Gibbs states in any
    dimension," Journal of Statistical Physics 192, 134 (2025), Section 1.2,
    judge that assuming existence of a strong effective Hamiltonian is
    likely too restrictive once the Commuting Hypothesis is dropped, which
    is why the weak version is the defensible target here.
  </claim>

  <def>
    Sites and algebras. Fix \(D\in\mathbb N\) and give every \(x\in
    V=\mathbb Z^g\) the local Hilbert space \(\mathcal H_x\equiv\mathbb
    C^D\). For a finite set \(X\subset V\), \(\mathcal
    H_X:=\bigotimes_{x\in X}\mathcal H_x\), \(D_X:=\dim\mathcal
    H_X=D^{|X|}\), and \(\mathfrak A_X:=\mathcal B(\mathcal H_X)\) is
    embedded into \(\mathfrak A_Y\) for finite \(X\subset Y\) via
    \(Q\mapsto Q\otimes\mathbb 1_{Y\setminus X}\). Write \(\mathcal
    P_f(V)\) for the finite subsets of \(V\).

    Interactions and quasi-locality. A local interaction is a family
    \(\Phi=(\Phi_X)_{X\in\mathcal P_f(V)}\) with \(\Phi_X=\Phi_X^*\in
    \mathfrak A_X\). A function \(\mathbf b:\mathcal P_f(V)\to[0,\infty)\)
    is subadditive if \(\mathbf b(X\cup Y)\le\mathbf b(X)+\mathbf b(Y)\)
    for all \(X,Y\in\mathcal P_f(V)\); \(\mathbf b(X)=\mu\,\mathrm{diam}(X)\)
    for fixed \(\mu\gt0\) is the standard example. For \(\varepsilon\ge0\),
    \(\|\Phi\|_{\varepsilon,\mathbf b}:=\sup_{x\in V}\sum_{X\ni
    x}\|\Phi_X\|e^{\varepsilon|X|+\mathbf b(X)}\); \(\Phi\) is quasi-local,
    or short-range, if \(\|\Phi\|_{\varepsilon,\mathbf b}\lt\infty\) for
    some \(\varepsilon\gt0\).

    Hamiltonians, Gibbs states, marginals. For finite \(\Lambda\subset V\),
    \(H_\Lambda:=\sum_{X\subset\Lambda}\Phi_X\in\mathfrak A_\Lambda\), and
    at inverse temperature \(\beta\gt0\) the Gibbs state is
    \(\rho_\beta^\Lambda:=e^{-\beta
    H_\Lambda}/\mathrm{Tr}_\Lambda[e^{-\beta H_\Lambda}]\). For
    \(L\subset\Lambda\), \(\mathrm{tr}_{\Lambda\setminus
    L}:=\mathrm{Tr}_{\Lambda\setminus L}\otimes\mathrm{id}_L:\mathfrak
    A_\Lambda\to\mathfrak A_L\) is the partial trace, and the marginal of
    \(\rho_\beta^\Lambda\) on \(L\) is
    \(\rho_{\beta,L}^\Lambda:=\mathrm{tr}_{\Lambda\setminus
    L}[\rho_\beta^\Lambda]\), a genuine quantum state on \(L\). For
    \(L\subset V\) possibly infinite and \(Q\in\mathfrak A_\Lambda\) with
    \(\Lambda\) finite, write \(\mathbb E_L[Q]:=\mathrm{tr}_{\Lambda\setminus
    L}[Q]/D_{\Lambda\setminus L}\in\mathfrak A_{\Lambda\cap L}\) for the
    normalized partial trace (conditional expectation) that keeps
    \(\Lambda\cap L\) and traces out \(\Lambda\setminus L\); only
    \(\Lambda\cap L\) matters, so this is well defined regardless of the
    size of \(L\).

    The trivial part of the claim. Because \(\mathfrak A_L\) is
    finite-dimensional, \(e^{-\beta H_\Lambda}\) is positive and
    invertible, so \(-\frac1\beta\log\mathbb E_L[e^{-\beta H_\Lambda}]\) is
    a well-defined Hermitian operator on \(L\) for every finite
    \(\Lambda\supset L\), and its Gibbs state at inverse temperature
    \(\beta\) reproduces the exact marginal:
    \(\rho_{\beta,L}^\Lambda=e^{-\beta\widetilde H_\Lambda^{L,\beta}}
    /\mathrm{Tr}_L[e^{-\beta\widetilde H_\Lambda^{L,\beta}}]\) where
    \(\widetilde H_\Lambda^{L,\beta}:=-\frac1\beta\log\mathbb E_L[e^{-\beta
    H_\Lambda}]\). So "the marginal of a thermal state is thermal for some
    Hamiltonian" holds for every \(\Phi\), every \(\beta\), and every
    finite \(\Lambda\), with no locality content: \(\widetilde
    H_\Lambda^{L,\beta}\) a priori depends on all of \(\Lambda\), not only
    on \(L\) and its neighborhood, and need not decompose into a
    \(\Lambda\)-independent sum of short-range terms. The open content of
    the claim above is entirely about this decomposition.

    Weak local effective Hamiltonians (Bluhm-Capel-Pérez-Hernández,
    Definition 3.3). For finite \(\Lambda\subset V\) and \(L\subset
    \Lambda\), set \(Z_{\Lambda\setminus L}:=\mathrm{Tr}_{\Lambda\setminus
    L}[e^{-\beta H_{\Lambda\setminus L}}]\) and
    \[
      \widehat H_\Lambda^{L,\beta}
      :=-\frac1\beta\log\mathrm{tr}_{\Lambda\setminus L}[e^{-\beta
      H_\Lambda}]+\frac1\beta\log(Z_{\Lambda\setminus L})\,\mathbb 1_L .
    \]
    This differs from \(\widetilde H_\Lambda^{L,\beta}\) only by an
    operator-trivial (identity-proportional) shift, so it too reproduces
    \(\rho_{\beta,L}^\Lambda\) as its Gibbs state at inverse temperature
    \(\beta\). The system has weak local effective Hamiltonians at inverse
    temperature \(\beta\) if for every \(L\subset V\) there is a local
    interaction \(\widehat\Phi^{L,\beta}=(\widehat\Phi_X^{L,\beta})_{X\in
    \mathcal P_f(V)}\) such that: (i) \(\widehat\Phi_X^{L,\beta}\) is
    supported in \(X\cap L\) for every finite \(X\subset V\); (ii)
    \(\widehat\Phi_X^{L,\beta}=\widehat\Phi_X^{L',\beta}\) whenever
    \(X\cap L'=X\cap L\); and (iii)
    \(\widehat H_\Lambda^{L,\beta}=\sum_{X\subset\Lambda,\,X\cap
    L\ne\emptyset}\widehat\Phi_X^{L,\beta}\) for every finite
    \(\Lambda\subset V\). Conditions (i)-(ii) say the terms of
    \(\widehat\Phi^{L,\beta}\) depend on \(L\) only through their
    intersection with \(L\), uniformly in \(\Lambda\); this is what makes
    \(\widehat\Phi^{L,\beta}\) a single, \(\Lambda\)-independent quasi-local
    interaction rather than a family re-derived for each \(\Lambda\).

    Strong local effective Hamiltonians (Bluhm-Capel-Pérez-Hernández,
    Definition 3.1). Defined identically but with
    \(\widetilde H_\Lambda^{L,\beta}=-\frac1\beta\log\mathbb E_L[e^{-\beta
    H_\Lambda}]\) in place of \(\widehat H_\Lambda^{L,\beta}\) in condition
    (iii), i.e. \(\widetilde H_\Lambda^{L,\beta}=\sum_{X\subset\Lambda}
    \widetilde\Phi_X^{L,\beta}\), with the sum unrestricted (terms with
    \(X\cap L=\emptyset\) are allowed and are then multiples of the
    identity). Existence of strong local effective Hamiltonians at inverse
    temperature \(\beta\) implies existence of weak ones at the same
    \(\beta\); the source states it has neither a proof nor a
    counterexample for the converse.

    Commuting Hypothesis (Bluhm-Capel-Pérez-Hernández, Definition 3.5).
    \(\Phi\) satisfies the Commuting Hypothesis if there is a commuting
    \(*\)-subalgebra \(\mathcal A\subset\mathfrak A_V\) (the inductive
    limit of the \(\mathfrak A_\Lambda\)) with \(\Phi_X\in\mathcal A\) for
    every \(X\in\mathcal P_f(V)\), and \(\mathbb E_L[\mathcal A]\subset
    \mathcal A\) for every \(L\subset V\).
  </def>

  <known>
    Bluhm, Capel, and Pérez-Hernández, "Strong decay of correlations for
    Gibbs states in any dimension," Journal of Statistical Physics 192, 134
    (2025), arXiv:2401.10147, Theorem 3.6, prove that if \(\Phi\) satisfies
    the Commuting Hypothesis and \(\|\Phi\|_{\varepsilon,\mathbf
    b}\lt\infty\) for some \(\varepsilon\gt0\) and subadditive \(\mathbf
    b\), then for every complex \(\beta\) with \(|\beta|\le
    \varepsilon/(2\|\Phi\|_{\varepsilon,\mathbf b})\) there exist strong
    local effective Hamiltonians with \(\|\widetilde\Phi^{L,\beta}\|_{
    \mathbf b}\lt\varepsilon/2\) for every \(L\subset V\). Their Theorem 3.8
    sharpens the decay to \(\|\widetilde\Phi^{L,\beta}\|_{\mathbf b}\le1\),
    matching \(\mathbf b\) exactly rather than only up to an
    \(\varepsilon\) margin, when \(\Phi\) additionally has finite degree
    (their Definition 3.7), which holds in particular for finite-range
    interactions on \(\mathbb Z^g\). Both results require the Commuting
    Hypothesis. Because strong local effective Hamiltonians imply weak
    ones, the claim above already holds whenever \(\Phi\) satisfies the
    Commuting Hypothesis; a proof of the claim needs only to newly cover
    interactions that do not satisfy it.

    Bluhm, Capel, and Pérez-Hernández, Section 1.2 and Section 7, state
    directly that whether local effective Hamiltonians of either kind
    exist once the Commuting Hypothesis is dropped is open and left to
    future work; in Section 7 they identify weak-effective-Hamiltonian
    existence as the one hypothesis still needed, in their approach, to
    get the equivalence of decay-of-correlations measures beyond one
    dimension.

    Kuwahara, Kato, and Brandão, "Clustering of conditional mutual
    information for quantum Gibbs states above a threshold temperature,"
    Physical Review Letters 124, 220601 (2020), arXiv:1910.09425,
    originally claimed quasi-locality of an effective Hamiltonian on
    subsystems above a threshold temperature, obtained through a cluster
    expansion, without restricting to commuting interactions. Bluhm,
    Capel, and Pérez-Hernández, Section 1.1, report a flaw in the
    non-commutative cluster expansion behind that claim (attributed to S.
    Scalet, personal communication, and corroborated by Kuwahara's 2024
    paper below), leaving the status of that construction's effective
    Hamiltonian unclear; this is why the source paper develops the
    strong/weak framework above instead of relying on the 2020
    construction.

    Kuwahara, "Clustering of conditional mutual information and quantum
    Markov structure at arbitrary temperatures," arXiv:2407.05835 (2024),
    proves decay of the conditional mutual information at any positive
    temperature. Bluhm, Capel, and Pérez-Hernández, Section 1.1, note
    explicitly that this does not imply decay of the mutual information
    and does not prove existence of an effective Hamiltonian of the kind
    claimed in the 2020 paper above; it does not resolve the claim here.

    Kochanowski, Alhambra, Capel, and Rouzé, "Rapid thermalization of
    dissipative many-body dynamics of commuting Hamiltonians,"
    arXiv:2404.16780 (2024), establish the equivalence of
    decay-of-correlations measures without assuming any effective
    Hamiltonian, but again only for commuting Hamiltonians. Bluhm, Capel,
    and Pérez-Hernández, Section 7, cite this as grounds for hoping that
    the effective-Hamiltonian assumption can eventually be removed, or
    shown unnecessary, beyond the commuting case as well, but this has not
    been done.

    Bluhm, Capel, and Pérez-Hernández, "Exponential decay of mutual
    information for Gibbs states of local Hamiltonians," Quantum 6, 650
    (2022), arXiv:2104.04419, prove the corresponding equivalence of
    decay-of-correlations measures in one spatial dimension at any
    positive temperature, without needing an effective-Hamiltonian
    assumption; the claim above concerns the higher-dimensional setting,
    where no such effective-Hamiltonian-free route is currently available
    beyond the commuting case.
  </known>

  <refs>
    <ref>A. Bluhm, Á. Capel, and A. Pérez-Hernández, Strong decay of correlations for Gibbs states in any dimension, Journal of Statistical Physics 192, 134 (2025), DOI: 10.1007/s10955-025-03512-y, arXiv:2401.10147, used for the problem statement (Section 1.2), Definitions 3.1 (strong local effective Hamiltonian), 3.3 (weak local effective Hamiltonian), 3.5 (Commuting Hypothesis), Theorem 3.6, Theorem 3.8, and the Section 7 discussion of what remains open.</ref>
    <ref>T. Kuwahara, K. Kato, and F. G. S. L. Brandão, Clustering of conditional mutual information for quantum Gibbs states above a threshold temperature, Physical Review Letters 124, 220601 (2020), DOI: 10.1103/PhysRevLett.124.220601, arXiv:1910.09425, used for the original claimed construction of a quasi-local effective Hamiltonian above a threshold temperature without a commuting restriction, whose non-commutative cluster expansion was later found flawed.</ref>
    <ref>T. Kuwahara, Clustering of conditional mutual information and quantum Markov structure at arbitrary temperatures, arXiv:2407.05835 (2024), used for the follow-up result on decay of conditional mutual information at arbitrary temperature that does not establish existence of a quasi-local effective Hamiltonian.</ref>
    <ref>J. Kochanowski, A. M. Alhambra, Á. Capel, and C. Rouzé, Rapid thermalization of dissipative many-body dynamics of commuting Hamiltonians, arXiv:2404.16780 (2024), used for the effective-Hamiltonian-free equivalence of decay-of-correlations measures in the commuting case.</ref>
    <ref>A. Bluhm, Á. Capel, and A. Pérez-Hernández, Exponential decay of mutual information for Gibbs states of local Hamiltonians, Quantum 6, 650 (2022), DOI: 10.22331/q-2022-02-10-650, arXiv:2104.04419, used for the one-dimensional, effective-Hamiltonian-free precursor result.</ref>
  </refs>

  <ask>
    Determine whether the claim holds. Because Theorem 3.6 of
    Bluhm-Capel-Pérez-Hernández already gives strong, hence weak, local
    effective Hamiltonians whenever \(\Phi\) satisfies the Commuting
    Hypothesis, a proof needs only to newly cover interactions that do not
    satisfy it; the claim as stated is the conjunction of that known case
    and the open one.

    A proof should, for every quasi-local \(\Phi\) on \(V=\mathbb Z^g\)
    with \(\|\Phi\|_{\varepsilon,\mathbf b}\lt\infty\) for some
    \(\varepsilon\gt0\) and subadditive \(\mathbf b\), exhibit
    \(\beta_*\gt0\) and, for every \(L\subset V\) and every
    \(\beta\in(0,\beta_*]\), a local interaction \(\widehat\Phi^{L,\beta}\)
    satisfying conditions (i)-(iii) of the weak local effective Hamiltonian
    definition with \(\|\widehat\Phi^{L,\beta}\|_{\mathbf b}\lt\infty\);
    equivalently, it may extend Theorem 3.6 (or its finite-degree
    sharpening, Theorem 3.8) by removing the Commuting Hypothesis from the
    statement while keeping some finite decay conclusion in its place. A
    proof that instead produces strong local effective Hamiltonians, or
    that reaches the quantitative constants of Theorem 3.6 or Theorem 3.8,
    resolves the claim a fortiori.

    A disproof should exhibit, for some \(g\in\mathbb N\), a quasi-local
    \(\Phi\) on \(\mathbb Z^g\) not satisfying the Commuting Hypothesis
    such that for every \(\beta_*\gt0\) there is a real \(\beta\) with
    \(0\lt\beta\le\beta_*\) and an \(L\subset V\) at which no local
    interaction satisfies conditions (i)-(iii) of the weak local effective
    Hamiltonian definition with finite \(\mathbf b\)-norm; because the
    claim asserts only the existence of some threshold \(\beta_*\), failure
    at a single fixed \(\beta\) does not by itself disprove it.

    Results restricted to finite-range \(\Phi\), to a fixed finite
    \(\Lambda\) rather than the \(\Lambda\)-independent family of
    conditions (i)-(iii), to a proper subclass of non-commuting
    interactions, or that let \(\beta_*\) depend on \(\Lambda\) or on
    \(L\) rather than being uniform over both, count as progress rather
    than a full resolution, unless accompanied by a proof that the
    restriction is removable.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
