# QCT044 Complexity of the Separability Problem

```xml
<problem id="QCT044">
  <status>open</status>
  <name>Complexity of the Separability Problem</name>
  <keys>complexity of separability problem quantum separability problem weak membership sep d d separable states trace norm trace distance constant precision inverse logarithmic precision inverse polynomial precision gurvits gharibian brandao christandl yard harrow montanaro best separable state bss qma(2) unentanglement locc norm frobenius norm euclidean norm symmetric extension de finetti open quantum problems oqp problem 44 quantum complexity theory</keys>

  <claim>
    Classify, under standard classical bit-complexity reductions, the
    asymptotic complexity of the trace-norm weak membership promise problem
    \(\mathrm{WMEM}^{1}_{\epsilon}(\mathrm{Sep}(d,d))\) defined below, as a
    function of the local dimension \(d\) and the precision
    \(\epsilon=\epsilon(d)\). The unresolved core includes the regimes
    \(\epsilon(d)\ge 1/\operatorname{polylog}(d)\), and especially fixed
    constants \(\epsilon_0\gt 0\): determine whether the trace-norm problem
    has a classical algorithm running in \(\operatorname{poly}(d)\) time
    on dense matrix input in those regimes, or prove a hardness theorem for
    the same trace-norm promise regime.
  </claim>

  <def>
    For \(d\ge 2\), let
    \[
      \mathcal D(d^2)=
      \{\rho\in L(\mathbb C^d\otimes\mathbb C^d):
        \rho\succeq 0,\ \operatorname{tr}\rho=1\}
    \]
    be the bipartite density operators on
    \(\mathbb C^d\otimes\mathbb C^d\). The set of separable states is
    \[
      \mathrm{Sep}(d,d)
      =
      \operatorname{conv}
      \{ |a\rangle\langle a|\otimes |b\rangle\langle b|:
         a,b\in\mathbb C^d,\ \|a\|_2=\|b\|_2=1\}.
    \]
    Equivalently, \(\mathrm{Sep}(d,d)\) is the convex hull of
    \(\alpha\otimes\beta\), where \(\alpha,\beta\) are local density
    operators.

    For an operator \(X\), write
    \[
      \|X\|_1=\operatorname{tr}\sqrt{X^\dagger X},
      \qquad
      \|X\|_2=(\operatorname{tr}X^\dagger X)^{1/2}.
    \]
    The first is the trace norm. The second is the Frobenius, or
    Hilbert-Schmidt, norm; after choosing matrix coordinates it is the
    Euclidean norm used in computational convex geometry. This entry uses
    \(\|X\|_1\), without the conventional factor \(1/2\), as the trace-norm
    distance. Replacing it by \(\|X\|_1/2\) changes only constant factors.
    For \(d^2\times d^2\) matrices,
    \[
      \|X\|_2\le \|X\|_1\le d\|X\|_2 .
    \]
    Hence inverse-polynomial Frobenius- and trace-norm promises are
    polynomially interconvertible, but constant or inverse-logarithmic
    promises are not preserved by this norm comparison.

    Define
    \[
      \Delta_1(\rho,\mathrm{Sep}(d,d))
      =
      \inf_{\sigma\in\mathrm{Sep}(d,d)}\|\rho-\sigma\|_1 .
    \]
    The promise problem
    \(\mathrm{WMEM}^{1}_{\epsilon}(\mathrm{Sep}(d,d))\) is:
    given \(d\), a rational or otherwise polynomial-bit classical
    description of a \(d^2\times d^2\) density matrix \(\rho\), and a
    precision convention for \(\epsilon\), decide between
    \[
      \rho\in\mathrm{Sep}(d,d)
      \quad\text{and}\quad
      \Delta_1(\rho,\mathrm{Sep}(d,d))\ge \epsilon,
    \]
    promised that one case holds. Running time is measured in the ordinary
    bit model for the dense matrix description; a claimed algorithm must state
    its dependence on \(d\), the entry bit length, and \(\epsilon\) or on the
    specified precision function \(\epsilon(d)\).

    The LOCC norm is a different norm. One convenient definition is
    \[
      \|X\|_{\mathrm{LOCC}}
      =
      \max_{M\in\mathrm{LOCC}}
      \operatorname{tr}((2M-I)X),
    \]
    where \(M\) ranges over two-outcome effects implementable by local
    operations and classical communication. It is bounded above by the trace
    norm and can be much smaller. A weak-membership algorithm in the LOCC norm
    is therefore not a trace-norm weak-membership algorithm.

    The normalized Best Separable State problem,
    \(\mathrm{BSS}_{\epsilon}\), asks, for a given measurement effect
    \(0\preceq M\preceq I\), for an additive-\(\epsilon\) estimate of
    \[
      \max_{\sigma\in\mathrm{Sep}(d,d)}\operatorname{tr}(M\sigma)
    \].
    For an unnormalized Hermitian \(M\), the additive error and runtime must be
    scaled by a specified norm of \(M\). Weak optimization over
    \(\mathrm{Sep}(d,d)\) and weak membership are equivalent in the
    Groetschel-Lovasz-Schrijver framework only with dimension-polynomial loss
    in precision. Thus constant-error BSS and constant-trace-norm weak
    membership are related but not interchangeable without a reduction that
    preserves the relevant norm and precision regime.
  </def>

  <known>
    Open Quantum Problems, Problem 44, "Complexity of the separability
    problem", asks for the computational complexity of deciding whether a
    bipartite density matrix on \(\mathbb C^d\otimes\mathbb C^d\), given as
    \(d^2\times d^2\) matrix entries, is separable or trace-distance
    \(\epsilon\)-far from \(\mathrm{Sep}(d,d)\). The Open Quantum Problems
    index lists this as Problem 44 in Quantum complexity theory, with contact
    Henry Yuen and date 2023/01/16.

    Gurvits, "Classical deterministic complexity of Edmonds' problem and
    quantum entanglement", STOC 2003, proves NP-hardness of weak membership
    for normalized bipartite separable density matrices in the computational
    convex-geometry formulation. Gharibian, "Strong NP-Hardness of the
    Quantum Separability Problem", Quantum Information &amp; Computation 10,
    343-360 (2010), strengthens this to inverse-polynomial Euclidean, or
    Frobenius/Bloch-vector, precision. By the norm comparison above, these
    results also give trace-norm NP-hardness at inverse-polynomial precision.
    Gharibian explicitly notes that NP-hardness at constant weak-membership
    radius remains open; the norm comparison does not upgrade the
    inverse-polynomial theorem to a constant trace-norm theorem.

    Brandao, Christandl, and Yard, "A quasipolynomial-time algorithm for the
    quantum separability problem", STOC 2011, give an
    \[
      \exp(O(\epsilon^{-2}\log |A|\log |B|))
    \]
    time algorithm for weak membership in the Euclidean/Frobenius norm and in
    the LOCC norm, and related algorithms for optimizing over separable
    states. This is quasipolynomial for constant \(\epsilon\) and
    \(|A|=|B|=d\), but it does not decide constant-\(\epsilon\) trace-norm
    weak membership. Their trace-norm symmetric-extension estimates have
    dimension dependence that prevents this conclusion, and the LOCC norm can
    be strictly weaker than trace norm because of quantum data hiding.

    Shi and Wu, "Epsilon-net method for optimizations over separable states",
    Theoretical Computer Science 598, 51-63 (2015), study the optimization
    problem \(\max_{\rho\in\mathrm{Sep}}\operatorname{tr}(Q\rho)\). They give
    algorithms better than brute force for several regimes, including a time
    bound exponential in \(\|Q\|_F\) for \(Q\succeq 0\), and PSPACE upper
    bounds for certain restricted QMA(2) protocols. These are results about
    weak optimization and special operator norms, not a polynomial-time
    solution of constant trace-norm weak membership.

    Harrow and Montanaro, "Testing product states, quantum Merlin-Arthur
    games and tensor optimization", Journal of the ACM 60, Article 3 (2013),
    prove the product test, \(\mathrm{QMA}(k)=\mathrm{QMA}(2)\) for
    \(k\ge 2\), and many equivalences and lower bounds around QMA(2), tensor
    optimization, BSS, and separability testing. In particular, they show that
    if a convex family \(K_d\) is within universal-constant Hausdorff trace
    distance of \(\mathrm{Sep}(d,d)\) and
    \(\mathrm{WMEM}_{1/\operatorname{poly}(d)}(K_d)\) is decidable in
    \(\operatorname{poly}(d)\) time, then 3-SAT has a deterministic algorithm
    running in \(\exp(\sqrt n\,\operatorname{polylog} n)\) time. This is
    strong conditional evidence against efficient constant-precision
    trace-norm separability testing, but it is not an unconditional
    NP-hardness theorem for
    \(\mathrm{WMEM}^{1}_{\epsilon_0}(\mathrm{Sep}(d,d))\) as defined here.

    Blier and Tapp, "All languages in NP have very short quantum proofs",
    ICQNM 2009; Aaronson, Beigi, Drucker, Fefferman, and Shor, "The Power of
    Unentanglement", Theory of Computing 5, 1-42 (2009); Chen and Drucker,
    "Short Multi-Prover Quantum Proofs for SAT without Entangled
    Measurements", arXiv:1011.0716; and Harrow and Montanaro's JACM paper
    establish the QMA(2)/unentanglement side recorded by Open Quantum
    Problems: short unentangled quantum proofs for NP problems and the
    connection between QMA(2) acceptance probabilities and optimization over
    separable states. These results motivate the problem but do not classify
    the dense-input trace-norm weak-membership problem.

    Later work sharpens adjacent algorithmic and proof-system problems without
    settling this entry. Brandao and Harrow, "Estimating operator norms using
    covering nets", arXiv:1509.05065 (2015), give polynomial- and
    quasipolynomial-time approximation schemes for generalized operator norms,
    including support functions of separable states. Harrow, Natarajan, and
    Wu, "An improved semidefinite programming hierarchy for testing
    entanglement", Communications in Mathematical Physics 352, 881-904
    (2017), obtain a hierarchy giving separability testing singly exponential
    in dimension and polylogarithmic in accuracy. Barak, Kothari, and
    Steurer, "Quantum entanglement, sum of squares, and the log rank
    conjecture", STOC 2017, give an
    \(\exp(\widetilde O(\sqrt n)/\epsilon^2)\)-time algorithm for the
    \(1\) versus \(1-\epsilon\) BSS problem on \(n^2\times n^2\) measurement
    matrices. Gutoski, Hayden, Milner, and Wilde, "Quantum Interactive Proofs
    and the Complexity of Separability Testing", Theory of Computing 11,
    59-103 (2015), classify several circuit-specified separability-output
    variants in classes such as QMA, QMA(2), and QSZK. Jeronimo and Wu,
    "Dimension Independent Disentanglers from Unentanglement and
    Applications", CCC 2024, advance the QMA(2)/disentangler side. None of
    these results gives either a polynomial-time algorithm or a hardness
    theorem for constant-precision trace-norm weak membership from an explicit
    dense density matrix.
  </known>

  <refs>
    <ref>Open Quantum Problems, Problem 44, Complexity of the separability problem, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/complexity-of-the-separability-problem, used for the trace-norm weak-membership statement, the \(d^2\times d^2\) input convention, and the listed partial results.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: Problem 44 is listed as Complexity of the separability problem in Quantum complexity theory, dated 2023/01/16.</ref>
    <ref>L. Gurvits, Classical deterministic complexity of Edmonds' problem and quantum entanglement, Proceedings of the 35th Annual ACM Symposium on Theory of Computing, 10-19 (2003), DOI: 10.1145/780542.780545, arXiv:quant-ph/0303055, used for NP-hardness of weak membership for separable normalized bipartite density matrices.</ref>
    <ref>S. Gharibian, Strong NP-Hardness of the Quantum Separability Problem, Quantum Information &amp; Computation 10(3&amp;4), 343-360 (2010), arXiv:0810.4507, used for inverse-polynomial strong NP-hardness and the statement that constant-radius weak membership remains open.</ref>
    <ref>F. G. S. L. Brandao, M. Christandl, and J. Yard, A quasipolynomial-time algorithm for the quantum separability problem, Proceedings of the 43rd Annual ACM Symposium on Theory of Computing, 343-352 (2011), DOI: 10.1145/1993636.1993683, arXiv:1011.2751, used for the Euclidean/Frobenius and LOCC norm quasipolynomial algorithms and separable-state optimization results.</ref>
    <ref>A. W. Harrow and A. Montanaro, Testing product states, quantum Merlin-Arthur games and tensor optimization, Journal of the ACM 60(1), Article 3 (2013), DOI: 10.1145/2432622.2432625, arXiv:1001.0017, used for the product test, QMA(2)/BSS connections, and conditional constant-trace-distance convex-approximation lower bounds.</ref>
    <ref>Y. Shi and X. Wu, Epsilon-net method for optimizations over separable states, Theoretical Computer Science 598, 51-63 (2015), DOI: 10.1016/j.tcs.2015.03.031, arXiv:1112.0808, used for epsilon-net algorithms for \(\max_{\rho\in\mathrm{Sep}}\operatorname{tr}(Q\rho)\) and the Frobenius-norm-dependent optimization runtime.</ref>
    <ref>H. Blier and A. Tapp, All languages in NP have very short quantum proofs, 2009 Third International Conference on Quantum, Nano and Micro Technologies, 34-37 (2009), DOI: 10.1109/ICQNM.2009.21, arXiv:0709.0738, used for the short-unentangled-proof background.</ref>
    <ref>S. Aaronson, S. Beigi, A. Drucker, B. Fefferman, and P. Shor, The Power of Unentanglement, Theory of Computing 5(1), 1-42 (2009), DOI: 10.4086/toc.2009.v005a001, arXiv:0804.0802, used for QMA(k), short unentangled proofs, disentanglers, and BSS/separability motivation.</ref>
    <ref>J. Chen and A. Drucker, Short Multi-Prover Quantum Proofs for SAT without Entangled Measurements, arXiv:1011.0716 (2010), used for the BellQMA/QMA(2) short-proof line cited by Open Quantum Problems.</ref>
    <ref>F. G. S. L. Brandao and A. W. Harrow, Estimating operator norms using covering nets, arXiv:1509.05065 (2015), used for later polynomial and quasipolynomial approximation schemes for support functions of separable states and related operator norms.</ref>
    <ref>A. W. Harrow, A. Natarajan, and X. Wu, An improved semidefinite programming hierarchy for testing entanglement, Communications in Mathematical Physics 352(3), 881-904 (2017), DOI: 10.1007/s00220-017-2859-0, arXiv:1506.08834, used for the singly-exponential-in-dimension hierarchy and accuracy dependence.</ref>
    <ref>B. Barak, P. K. Kothari, and D. Steurer, Quantum entanglement, sum of squares, and the log rank conjecture, Proceedings of the 49th Annual ACM Symposium on Theory of Computing, 975-988 (2017), DOI: 10.1145/3055399.3055488, arXiv:1701.06321, used for the subexponential-time algorithm for the \(1\) versus \(1-\epsilon\) Best Separable State problem.</ref>
    <ref>G. Gutoski, P. Hayden, K. Milner, and M. M. Wilde, Quantum Interactive Proofs and the Complexity of Separability Testing, Theory of Computing 11(3), 59-103 (2015), DOI: 10.4086/toc.2015.v011a003, arXiv:1308.5788, used to distinguish circuit-specified separability-testing variants from dense matrix weak membership.</ref>
    <ref>F. G. Jeronimo and P. Wu, Dimension Independent Disentanglers from Unentanglement and Applications, 39th Computational Complexity Conference (CCC 2024), LIPIcs 300, 26:1-26:28 (2024), DOI: 10.4230/LIPIcs.CCC.2024.26, arXiv:2402.15282, used for recent adjacent progress on QMA(2) and disentanglers.</ref>
  </refs>

  <ask>
    Give a complexity classification for
    \(\mathrm{WMEM}^{1}_{\epsilon}(\mathrm{Sep}(d,d))\) in the trace norm
    for the precision regimes not covered by inverse-polynomial hardness,
    especially fixed \(\epsilon_0\gt 0\) and inverse-logarithmic
    \(\epsilon(d)\). Under the \(\|X\|_1\) convention in this entry, fixed
    trace-norm constants satisfy \(0\lt\epsilon_0\le 2\). An algorithm for gap
    \(\epsilon_0\) also decides larger promised gaps, while hardness at gap
    \(\epsilon_0\) propagates to smaller gaps. A full solution should either
    give algorithms with stated dependence on \(d\) and \(\epsilon\), or prove
    hardness or conditional lower bounds for exactly the trace-norm dense-input
    promise problem above.

    Results for the Frobenius norm, Euclidean norms on Bloch vectors or
    matrix entries, the LOCC norm, support-function or BSS optimization,
    circuit-generated states or channels, mean-field Hamiltonians, tensor
    norms, or QMA(2) proof systems count as progress only if accompanied by a
    reduction that preserves the trace-norm weak-membership promise and the
    claimed precision regime. In particular, a constant-error LOCC-norm
    algorithm, a constant-error BSS algorithm, or inverse-polynomial
    trace-norm hardness does not by itself solve the constant-trace-norm
    weak-membership problem.

    A disproof of an asserted algorithmic classification must identify the
    failed regime and prove the contradictory lower or upper bound in the same
    input model. A disproof of the open status of this entry must cite a
    theorem that either decides
    \(\mathrm{WMEM}^{1}_{\epsilon_0}(\mathrm{Sep}(d,d))\) in
    \(\operatorname{poly}(d)\) time for every fixed admissible constant
    \(\epsilon_0\), or proves the corresponding hardness statement, with all
    norm and promise conventions matched.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    For this classification problem, proof means a complete complexity
    classification in the stated input model and norm convention; disproof
    means a theorem showing that the stated open-status target is already
    settled or misformulated.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
