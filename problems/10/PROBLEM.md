# 10 Complexity of Product Preparations

```xml
<problem id="10">
  <status>open</status>
  <kind>construction</kind>
  <field>QCT</field>
  <name>Complexity of Product Preparations</name>
  <keys>complexity of product preparations product preparation complexity algorithmic complexity product state preparation multiple copies known state cloning known state copying no cloning theorem quantum mass production kretschmer cnot count clean ancilla trace distance state synthesis circuit complexity pauli string rotations pauli product rotations angle cost preparation complexity nonuniform state preparation independent preparation baseline error allocation open quantum problems oqp quantum computation</keys>

  <claim>
    In the Pauli-string rotation cost model defined below, characterize up to
    explicit asymptotic equivalence the state-dependent cost
    \[
      P_{m,n}(\psi,\epsilon)
      :=
      C_{mn}(\psi^{\otimes n};\epsilon)
    \]
    of preparing \(n\) product copies of a known \(m\)-qubit pure state
    \(\psi\), and the one-copy-conditioned worst-case envelope
    \[
      W_m(n,k,\delta,\epsilon)
      :=
      \sup\{P_{m,n}(\psi,\epsilon):
      \psi\in S_m,\ C_m(\psi;\delta)\le k\},
    \]
    as functions of \(m,n,\epsilon,\delta\), and one-copy preparation
    information about \(\psi\), including either the scalar value
    \(C_m(\psi;\delta)\) or the full approximation profile
    \(\alpha\mapsto C_m(\psi;\alpha)\). A complete characterization must
    determine in which regimes known-state mass production can asymptotically
    beat the independent-preparation baseline defined below and in which
    regimes independent preparation is optimal.
  </claim>

  <def>
    For \(L\ge 1\), let
    \(S_L\) be the unit sphere of \((\mathbb C^2)^{\otimes L}\), with pure
    states identified up to global phase. Let
    \[
      \mathcal P_L
      =
      \{I,X,Y,Z\}^{L}\setminus\{I^{\otimes L}\}.
    \]
    For \(u\in\mathcal P_L\), write \(\sigma_u\) for the corresponding
    \(L\)-qubit Pauli string. The allowed gates on \(L\) qubits are
    \[
      R_u(\phi)=\exp(i\phi\sigma_u),
      \qquad u\in\mathcal P_L,\quad \phi\in\mathbb R,
    \]
    and the cost of this written gate is \(|\phi|\). A circuit is a finite
    product \(U=R_{u_T}(\phi_T)\cdots R_{u_1}(\phi_1)\), with additive cost
    \[
      \ell(U)=\sum_{t=1}^T |\phi_t|.
    \]
    There is no parallel-time discount, no surcharge for Pauli weight, and no
    locality restriction: for \(L=mn\), a single Pauli string may act
    nontrivially across different output copies.

    The OQP page specifies the Pauli-string rotations and angle cost but does
    not fully specify workspace, uniformity, or error conventions. This entry
    fixes the following precision conventions. The primary complexity uses no
    extra workspace qubits. Starting from
    \(|0\rangle^{\otimes L}\), a circuit \(U\) \(\epsilon\)-prepares
    \(\eta\in S_L\) when
    \[
      D(U|0\rangle^{\otimes L},\eta)
      :=
      \sqrt{1-|\langle \eta|U|0\rangle^{\otimes L}|^2}
      \le \epsilon .
    \]
    Exact preparation is the case \(\epsilon=0\). Define
    \[
      C_L(\eta;\epsilon)
      :=
      \inf\{\ell(U):U\text{ is an }L\text{-qubit circuit that }
      \epsilon\text{-prepares }\eta\}.
    \]
    If the infimum is not attained, bounds on \(C_L\) are interpreted in the
    usual infimum sense. A clean-ancilla variant may also be studied, but it is
    not the primary convention here; such a variant must specify the number of
    ancillas, initialize them to \(|0\rangle\), and return them to
    \(|0\rangle\) or trace them out with an explicit error convention.

    The state \(\psi\in S_m\) is known or classically described. The circuit
    preparing \(\psi^{\otimes n}\) may depend on an exact description of
    \(\psi\) or on a chosen one-copy preparation circuit for \(\psi\). The
    model is nonuniform: the classical length of that description and the
    classical time needed to find the circuit are not included in \(C_L\).
    Uniform synthesis algorithms are progress only after their output circuits
    are interpreted in this nonuniform cost model. Because \(\psi\) is known
    and no unknown input copy is supplied, this is not an unknown-state
    cloning task.

    Throughout, \(m,n\in\mathbb N\), \(k\ge 0\), and
    \(0\le\epsilon,\delta\lt 1\), with \(\epsilon=0\) or \(\delta=0\)
    denoting exact preparation for the corresponding quantity. For
    \(m,n\ge 1\), define the product-copy cost
    \[
      P_{m,n}(\psi,\epsilon)=C_{mn}(\psi^{\otimes n};\epsilon).
    \]
    The admissible set in \(W_m(n,k,\delta,\epsilon)\) is nonempty because
    \(C_m(|0\rangle^{\otimes m};\delta)=0\).

    Define the independent-preparation baseline by
    \[
      I_{m,n}(\psi,\epsilon)
      =
      \inf
      \left\{
        \sum_{j=1}^n C_m(\psi;\alpha_j):
        0\le\alpha_j\lt 1,\ 
        \prod_{j=1}^n(1-\alpha_j^2)\ge 1-\epsilon^2
      \right\}.
    \]
    This is the best cost obtained by preparing the \(n\) copies independently,
    with an optimized allocation of one-copy errors. The constraint follows
    from the identity
    \[
      D(\eta_1\otimes\cdots\otimes\eta_n,\psi^{\otimes n})^2
      =
      1-\prod_{j=1}^n|\langle\psi|\eta_j\rangle|^2 .
    \]
    Hence \(P_{m,n}(\psi,\epsilon)\le I_{m,n}(\psi,\epsilon)\), and
    \(I_{m,n}(\psi,0)=nC_m(\psi;0)\). The simpler triangle-inequality bound
    \(P_{m,n}(\psi,\epsilon)\le nC_m(\psi;\epsilon/n)\) is valid but is not the
    independent baseline in the approximate case. The problem asks whether
    better asymptotics than \(I_{m,n}\) are possible when the global
    \(mn\)-qubit circuit is optimized jointly.

    Write \(A\asymp B\) for a claimed asymptotic equivalence only when
    \(cB\le A\le CB\) is proved with constants \(0\lt c\le C\) whose dependence
    on \(m,n,\epsilon,\delta,\psi\), and the asymptotic regime is explicitly
    stated. If logarithmic or subconstant factors are suppressed, the statement
    must use separate notation and state what is suppressed.
  </def>

  <known>
    Open Quantum Problems, Problem 16, "Complexity of product preparations",
    asks what can be said about the asymptotic algorithmic complexity of
    preparing \(|\psi\rangle^{\otimes n}\), where \(\psi\) is an \(m\)-qubit
    state, as a function of \(n\) and the algorithmic complexity of preparing
    \(\psi\) from \(|0\rangle\). The same OQP page proposes, for present
    purposes, the gate set of all rotations
    \(\exp(i\phi\sigma_u)\), where \(\sigma_u\) is a product of Pauli
    matrices, with gate cost \(|\phi|\), and says that a version with an
    approximation parameter may be useful. The Open Quantum Problems list
    records this as Problem 16 in the Quantum computation category.

    The OQP Problem 16 background states that, because \(\psi\) is known, it
    may be possible to produce copies of \(\psi\) more efficiently than by
    preparing every copy independently. This entry follows that known-state
    interpretation. It does not impose an unknown-input cloning transformation
    and does not use the no-cloning theorem as an obstruction.

    The independent-preparation construction gives
    \(P_{m,n}(\psi,\epsilon)\le I_{m,n}(\psi,\epsilon)\), with
    \(I_{m,n}(\psi,0)=nC_m(\psi;0)\) in the exact case. The elementary
    \(nC_m(\psi;\epsilon/n)\) estimate is only a simpler upper bound obtained
    by a triangle-inequality allocation of error. Neither the optimized
    independent baseline nor the simpler estimate is known to be tight in the
    OQP Pauli-string rotation cost model.

    Kretschmer, "Quantum Mass Production Theorems", TQC 2023, proves in a
    different circuit model that for every \(q\)-qubit pure state \(\eta\) and
    every \(r=2^{o(q/\log q)}\), there is a circuit with at most
    \((1+o(1))2^q\) CNOT gates preparing \(\eta^{\otimes r}\), using arbitrary
    one-qubit gates, CNOT gates with all-to-all connectivity, and clean
    ancillas. This matches the worst-case CNOT-count order for preparing a
    single arbitrary \(q\)-qubit state. Thus the broad standard-CNOT-count
    worst-case analogue has substantial modern progress, but Kretschmer's
    theorem does not determine \(P_{m,n}\) or \(W_m\) for the OQP
    Pauli-string angle-cost model, nor does it give a state-dependent
    characterization in terms of \(C_m(\psi;\delta)\).

    Shende, Bullock, and Markov, "Synthesis of Quantum Logic Circuits", IEEE
    Transactions on Computer-Aided Design of Integrated Circuits and Systems
    25(6), 1000-1010 (2006), give asymptotically efficient exact synthesis
    methods for arbitrary quantum computations and register initialization in
    the standard one-qubit-gate plus CNOT model, including state-preparation
    context used by later mass-production results. These results provide
    adjacent state-synthesis benchmarks, not a solution of the OQP
    Pauli-string angle-cost formulation.

    Nielsen, Dowling, Gu, and Doherty, "Quantum Computation as Geometry",
    Science 311(5764), 1133-1135 (2006), formulate circuit complexity using
    continuous paths generated by Hamiltonians expanded in the Pauli basis and
    show that the chosen cost function or penalties are part of the
    mathematical problem. This supports treating the OQP Pauli-string
    angle-cost convention as a distinct model rather than translating results
    from CNOT count without proof.
  </known>

  <refs>
    <ref>Open Quantum Problems, Complexity of product preparations, Problem 16, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/complexity-of-product-preparations, used for the problem statement, the Pauli-string rotation gate set, the \(|\phi|\) gate cost, the approximation-parameter suggestion, and the known-state copying background.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 16, Complexity of product preparations, in Quantum computation.</ref>
    <ref>W. Kretschmer, Quantum Mass Production Theorems, in 18th Conference on the Theory of Quantum Computation, Communication and Cryptography (TQC 2023), Leibniz International Proceedings in Informatics 266, 10:1-10:11 (2023), DOI: 10.4230/LIPIcs.TQC.2023.10, arXiv:2212.14399, used for modern mass-production results for states in the standard one-qubit-gate plus CNOT-count model.</ref>
    <ref>V. V. Shende, S. S. Bullock, and I. L. Markov, Synthesis of Quantum Logic Circuits, IEEE Transactions on Computer-Aided Design of Integrated Circuits and Systems 25(6), 1000-1010 (2006), DOI: 10.1109/TCAD.2005.855930, arXiv:quant-ph/0406176, used for arbitrary state-synthesis and circuit-synthesis benchmarks in the standard CNOT-count model.</ref>
    <ref>M. A. Nielsen, M. R. Dowling, M. Gu, and A. C. Doherty, Quantum Computation as Geometry, Science 311(5764), 1133-1135 (2006), DOI: 10.1126/science.1121541, arXiv:quant-ph/0603161, used for background on Pauli-basis Hamiltonian cost models and the convention dependence of geometric circuit complexity.</ref>
  </refs>

  <ask>
    Give theorems that determine \(P_{m,n}(\psi,\epsilon)\) and
    \(W_m(n,k,\delta,\epsilon)\), up to the stated asymptotic equivalence, in
    the Pauli-string rotation model above. A solution must give matching upper
    and lower bounds in the same model, with the allowed dependence on
    \(m,n,\epsilon,\delta\), and \(\psi\) stated explicitly. In particular, it
    must prove when the independent-preparation baseline
    \(I_{m,n}(\psi,\epsilon)\) is optimal and when a globally optimized
    \(mn\)-qubit circuit gives an asymptotic improvement.

    A formula depending only on the scalar one-copy number
    \(C_m(\psi;\delta)\) must prove that neither additional invariants of
    \(\psi\) nor the rest of the approximation profile
    \(\alpha\mapsto C_m(\psi;\alpha)\) changes the product-copy cost in the
    claimed regime. Otherwise, results should be stated as state-dependent
    bounds for \(P_{m,n}(\psi,\epsilon)\), as profile-dependent bounds using
    \(\alpha\mapsto C_m(\psi;\alpha)\), or as envelope bounds for
    \(W_m(n,k,\delta,\epsilon)\). Exhibiting two state families with the same
    one-copy complexity scale but inequivalent product-copy costs is a valid
    gap or disproof of any proposed one-copy-only characterization.

    Results using unknown input copies, probabilistic cloning machines,
    tomography from supplied copies, standard CNOT count, depth, finite
    universal gate sets, locality-restricted gates, clean ancillas, or
    classical compilation cost are progress only if they are translated into
    the definitions in this entry or are explicitly identified as a different
    variant. Results for exact preparation alone, approximate preparation
    alone, fixed \(m\), fixed \(n\), worst-case states only, or special
    structured families of \(\psi\) are progress unless they imply the stated
    state-dependent and envelope characterizations.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
