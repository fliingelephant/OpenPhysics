# 31 The Entanglement Cost of f-Routing

```xml
<problem id="31">
  <status>open</status>
  <kind>construction</kind>
  <field>QIT</field>
  <name>The Entanglement Cost of f-Routing</name>
  <keys>f-routing f routing f-route f route entanglement cost non-local quantum computation nlqc quantum position verification qpv conditional disclosure of secrets cds conditional disclosure of quantum secrets cdqs garden hose garden-hose model span program code routing simultaneous exchange diamond norm explicit boolean functions polynomial lower bound one-sided perfect correctness nondeterministic rank non-deterministic rank quantum cryptography open quantum problems oqp problem 48</keys>

  <claim>
    There exist constants \(\epsilon\in(0,1)\) and \(c\gt 0\), a fixed
    finite-dimensional quantum system \(Q\) with \(\dim Q\ge 2\), and an
    explicit family of Boolean functions
    \[
      f_n:\{0,1\}^n\times\{0,1\}^n\to\{0,1\}
    \]
    such that every \(\epsilon\)-sound \(f_n\)-routing protocol in the full
    simultaneous-exchange model has entanglement cost at least \(n^c\) for
    all sufficiently large \(n\).
  </claim>

  <def>
    Explicit means uniformly polynomial-time computable: there is a
    deterministic classical algorithm that, on input \(1^n,x,y\), computes
    \(f_n(x,y)\) in time polynomial in \(n\). A proposed proof using another
    standard explicitness convention must state it and explain whether the
    resulting family is also uniformly polynomial-time computable.

    Fix \(n\), a Boolean function
    \(f:\{0,1\}^n\times\{0,1\}^n\to\{0,1\}\), and a finite-dimensional
    system \(Q\). An \(f\)-routing protocol is a two-party non-local quantum
    computation with one simultaneous exchange. Alice initially holds \(Q\)
    and register \(A_0\); Bob initially holds register \(B_0\). The registers
    \(A_0B_0\) are in a shared pure entangled state
    \(|\psi\rangle_{A_0B_0}\). Padding local registers if needed, write
    \(D=\dim A_0=\dim B_0\). The entanglement cost is
    \[
      \log_2 D .
    \]
    This is the logarithm of the local Hilbert-space dimension of the initial
    shared resource state, not an entropy of entanglement and not a
    communication cost.

    On input \(x\in\{0,1\}^n\), Alice applies a CPTP map
    \[
      \mathcal V_x^A:\mathcal B(QA_0)\to
      \mathcal B(A_1C_{A\to B}),
    \]
    and sends \(C_{A\to B}\) to Bob. On input
    \(y\in\{0,1\}^n\), Bob applies a CPTP map
    \[
      \mathcal V_y^B:\mathcal B(B_0)\to
      \mathcal B(B_1C_{B\to A}),
    \]
    and sends \(C_{B\to A}\) to Alice. These two messages are exchanged
    simultaneously: neither second-round operation can depend on the other
    party's message except through the received quantum system. Alice then
    applies
    \[
      \mathcal W_x^A:\mathcal B(A_1C_{B\to A})\to
      \mathcal B(A_{\rm out}A_{\rm junk}),
    \]
    and Bob applies
    \[
      \mathcal W_y^B:\mathcal B(B_1C_{A\to B})\to
      \mathcal B(B_{\rm out}B_{\rm junk}),
    \]
    where \(A_{\rm out}\) and \(B_{\rm out}\) are each identified with \(Q\).
    The message dimensions and local workspaces are unrestricted unless a
    proposed theorem states an additional resource measure.

    Let \(\Omega_{xy}^A:\mathcal B(Q)\to\mathcal B(A_{\rm out})\) be the
    channel induced by the protocol after tracing out Bob's final systems and
    Alice's junk systems. Let
    \(\Omega_{xy}^B:\mathcal B(Q)\to\mathcal B(B_{\rm out})\) be the analogous
    channel induced after tracing out Alice's final systems and Bob's junk
    systems. The phrase "arbitrary input quantum system \(Q\)" means that the
    protocol must implement the identity channel on the whole input system
    \(Q\), including inputs entangled with an external reference. It is not
    enough to route one fixed input state or one known ensemble.

    The protocol is \(\epsilon\)-sound if
    \[
      \|\Omega_{xy}^A-\mathcal I_{Q\to A_{\rm out}}\|_\diamond\le\epsilon
      \quad\text{for every }(x,y)\text{ with }f(x,y)=0,
    \]
    and
    \[
      \|\Omega_{xy}^B-\mathcal I_{Q\to B_{\rm out}}\|_\diamond\le\epsilon
      \quad\text{for every }(x,y)\text{ with }f(x,y)=1.
    \]
    There is no perfect-correctness promise on either side of \(f\), no
    restriction to unitary local operations, and no restriction to
    garden-hose, code-routing, Clifford, stabilizer, or other special
    protocols.

    Define \(E_\epsilon^Q(f)\) to be the infimum of \(\log_2 D\) over all
    \(\epsilon\)-sound \(f\)-routing protocols for the fixed system \(Q\) in
    the full model above. Since a protocol for a larger \(Q\) restricts to
    any two-dimensional subspace of \(Q\), a lower bound for a qubit input
    also lower-bounds protocols for any fixed \(Q\) with \(\dim Q\ge 2\),
    up to the convention used to identify the output subsystem.
  </def>

  <known>
    This entry was collected from Open Quantum Problems, Problem 48, "The
    entanglement cost of f-routing". The Open Quantum Problems index lists
    Problem 48 in Quantum cryptography, with contact Alex May and date
    2026/01/08. The Open Quantum Problems page asks for a fixed
    \(\epsilon\gt 0\) and an explicit sequence of Boolean functions
    \(f_n\) whose minimum \(\epsilon\)-sound f-routing entanglement cost
    scales as \(\operatorname{poly}(n)\). The OQP target is a polynomial
    lower bound in the full approximate channel model, not an upper-bound
    construction.

    Kent, Munro, and Spiller, "Quantum tagging: Authenticating location via
    quantum information and relativistic signaling constraints", Physical
    Review A 84, 012326 (2011), and Buhrman, Fehr, Schaffner, and Speelman,
    "The garden-hose model", ITCS 2013, introduced position-verification
    settings and the garden-hose model that motivated f-routing. Garden-hose
    protocols give efficient attacks for functions of low garden-hose
    complexity; they are upper bounds on \(E_\epsilon^Q(f)\), not lower
    bounds for explicit hard functions in the full model.

    Gertner, Ishai, Kushilevitz, and Malkin, "Protecting data privacy in
    private information retrieval schemes", STOC 1998 and Journal of
    Computer and System Sciences 60(3), 592-629 (2000), introduced
    conditional disclosure of secrets. Gay, Kerenidis, and Wee,
    "Communication complexity of conditional disclosure of secrets and
    attribute-based encryption", CRYPTO 2015, proved nontrivial CDS lower
    bounds. Applebaum and Vasudevan, "Placing Conditional Disclosure of
    Secrets in the Communication Complexity Universe", Journal of Cryptology
    34, article 11 (2021), related CDS lower bounds to communication
    complexity classes and emphasized that strong explicit lower bounds for
    imperfect CDS remain tied to major open problems. These are classical
    CDS results; they do not by themselves lower-bound f-routing unless they
    are connected to the same \(f\) and the same resource convention.

    Allerstorfer, Buhrman, May, Speelman, and Verduyn Lunel, "Relating
    non-local quantum computation to information theoretic cryptography",
    Quantum 8, 1387 (2024), proved that f-routing is equivalent, up to small
    resource overheads, to conditional disclosure of quantum secrets, and
    that classical CDS protocols give f-routing protocols. In particular,
    the randomness cost of CDS for \(f\) upper-bounds the entanglement cost
    of f-routing for \(f\). Thus an f-routing polynomial lower bound of the
    kind asked here would imply a polynomial lower bound on CDS randomness
    for the same explicit function. The same work also gives
    \(2^{O(\sqrt{n\log n})}\)-type upper bounds for f-routing for arbitrary
    Boolean functions and efficient protocols for some functions believed to
    be outside \(P/poly\). These are upper bounds and barrier evidence, not
    a solution of OQP48.

    Cree and May, "Code-routing: a new attack on position verification",
    Quantum 7, 1079 (2023), gave f-routing upper bounds from span-program
    and quantum secret-sharing constructions. Such code-routing attacks
    show that several explicit classes have low entanglement cost; they do
    not rule out explicit functions with polynomial lower bounds.

    Bluhm, Hofer, May, Stasiuk, Verduyn Lunel, and Yuen, "A complexity
    theory for non-local quantum computation", arXiv:2505.23893 (2025),
    proved resource-efficient reductions among f-route, f-measure, and
    related classically controlled non-local quantum computation tasks. The
    paper transfers upper bounds, lower-bound techniques, and barriers
    between these tasks, and records that robust linear lower bounds for
    classically controlled function tasks would also confront the CDS lower
    bound barrier. These reductions do not supply an explicit full-model
    polynomial lower bound for f-routing.

    Lower bounds for random functions, lower bounds on a memory or ancilla
    register, and lower bounds under restrictions such as purified or
    unitary local actions are adjacent progress. They do not prove the claim
    because OQP48 asks for an explicit Boolean family and arbitrary CPTP
    local operations in the full \(\epsilon\)-sound f-routing model.

    Asadi, Culf, and May, "Rank Lower Bounds on Non-Local Quantum
    Computation", ITCS 2025, proved the first nontrivial entanglement lower
    bounds for f-routing and f-BB84 in a one-sided-perfect model. Their
    bounds use ranks of matrices supported on \(f\) or on \(\neg f\) and
    imply linear lower bounds for explicit functions such as equality,
    non-equality, greater-than, and set-disjointness when the protocol is
    perfect on one side. This does not settle OQP48 because the OQP model
    allows two-sided constant error measured in diamond norm.

    Asadi, Kuroiwa, Leung, May, Pasterski, and Waddell, "Conditional
    disclosure of secrets with quantum resources", Quantum 9, 1885 (2025),
    developed CDQS systematically and proved lower bounds in terms of
    communication-complexity measures, including one-way and interactive
    quantum communication settings. These CDQS lower bounds are important
    evidence and are connected to f-routing, but they do not currently give
    an explicit \(n^{\Omega(1)}\) entanglement lower bound for the full
    approximate f-routing task.

    Later literature through 2026/05/22 that was checked for this entry
    does not appear to solve OQP48. Girish, May, Orshansky, and Waddell,
    "Comparing classical and quantum conditional disclosure of secrets",
    Quantum 10, 2049 (2026), gives additional classical-versus-quantum CDS
    separations and lower bounds, but not the explicit full f-routing lower
    bound. Cleve and May, "Lower bounds on non-local computation from
    controllable correlation", arXiv:2602.00255 (2026), gives robust lower
    bounds for unitary non-local quantum computations, including common
    two-qubit gates, but it is a unitary-action setting rather than the
    f-routing channel task with arbitrary input-controlled CPTP maps.
  </known>

  <refs>
    <ref>Open Quantum Problems, Problem 48, The entanglement cost of f-routing, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/large-lower-bounds-for-the-entanglement-cost-of-f-routing, used for the f-routing communication pattern, entanglement-cost convention, diamond-norm epsilon-soundness condition, exact question, background, and listed partial results.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems index, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 48, The entanglement cost of f-routing, in Quantum cryptography, dated 2026/01/08.</ref>
    <ref>A. Kent, W. J. Munro, and T. P. Spiller, Quantum tagging: Authenticating location via quantum information and relativistic signaling constraints, Physical Review A 84(1), 012326 (2011), DOI: 10.1103/PhysRevA.84.012326, arXiv:1008.2147, used for quantum-tagging and quantum-position-verification provenance.</ref>
    <ref>H. Buhrman, S. Fehr, C. Schaffner, and F. Speelman, The garden-hose model, Proceedings of the 4th Conference on Innovations in Theoretical Computer Science, ITCS 2013, 145-158, DOI: 10.1145/2422436.2422455, arXiv:1109.2563, used for garden-hose complexity and position-verification background.</ref>
    <ref>R. Allerstorfer, H. Buhrman, A. May, F. Speelman, and P. Verduyn Lunel, Relating non-local quantum computation to information theoretic cryptography, Quantum 8, 1387 (2024), DOI: 10.22331/q-2024-06-27-1387, arXiv:2306.16462, used for the f-routing and CDQS equivalence, CDS-to-f-routing upper bounds, sub-exponential f-routing upper bounds, and CDS lower-bound barrier.</ref>
    <ref>B. Applebaum and P. N. Vasudevan, Placing Conditional Disclosure of Secrets in the Communication Complexity Universe, Journal of Cryptology 34(2), article 11 (2021), DOI: 10.1007/s00145-021-09376-1, used for CDS lower-bound context and the communication-complexity barrier for imperfect CDS lower bounds.</ref>
    <ref>Y. Gertner, Y. Ishai, E. Kushilevitz, and T. Malkin, Protecting data privacy in private information retrieval schemes, Proceedings of the 30th Annual ACM Symposium on Theory of Computing, STOC 1998, 151-160, DOI: 10.1145/276698.276723; journal version Journal of Computer and System Sciences 60(3), 592-629 (2000), DOI: 10.1006/jcss.1999.1689, used for the introduction of conditional disclosure of secrets in private information retrieval.</ref>
    <ref>A. Bluhm, S. Hofer, A. May, M. Stasiuk, P. Verduyn Lunel, and H. Yuen, A complexity theory for non-local quantum computation, arXiv:2505.23893 (2025), DOI: 10.48550/arXiv.2505.23893, used for f-route and f-measure reductions and current status of lower-bound barriers for classically controlled NLQC tasks.</ref>
    <ref>V. R. Asadi, E. Culf, and A. May, Rank Lower Bounds on Non-Local Quantum Computation, in 16th Innovations in Theoretical Computer Science Conference, ITCS 2025, Leibniz International Proceedings in Informatics 325, 11:1-11:18 (2025), DOI: 10.4230/LIPIcs.ITCS.2025.11, arXiv:2402.18647, used for one-sided-perfect f-routing lower bounds, nondeterministic-rank methods, and explicit one-sided examples.</ref>
    <ref>R. Gay, I. Kerenidis, and H. Wee, Communication complexity of conditional disclosure of secrets and attribute-based encryption, in Advances in Cryptology -- CRYPTO 2015, Lecture Notes in Computer Science 9216, 485-502 (2015), DOI: 10.1007/978-3-662-48000-7_24, IACR ePrint 2015/665, used for classical CDS lower-bound background.</ref>
    <ref>V. R. Asadi, K. Kuroiwa, D. Leung, A. May, S. Pasterski, and C. Waddell, Conditional disclosure of secrets with quantum resources, Quantum 9, 1885 (2025), DOI: 10.22331/q-2025-10-16-1885, arXiv:2404.14491, used for CDQS definitions, amplification, lower bounds, and relation to f-routing.</ref>
    <ref>J. Cree and A. May, Code-routing: a new attack on position verification, Quantum 7, 1079 (2023), DOI: 10.22331/q-2023-08-09-1079, arXiv:2202.07812, used for span-program and quantum-secret-sharing upper bounds on f-routing entanglement cost.</ref>
    <ref>U. Girish, A. May, L. Orshansky, and C. Waddell, Comparing classical and quantum conditional disclosure of secrets, Quantum 10, 2049 (2026), DOI: 10.22331/q-2026-04-01-2049, arXiv:2505.02939, used for later CDQS and classical-versus-quantum CDS status through May 2026.</ref>
    <ref>R. Cleve and A. May, Lower bounds on non-local computation from controllable correlation, arXiv:2602.00255 (2026), DOI: 10.48550/arXiv.2602.00255, used for later robust unitary NLQC lower bounds and for distinguishing unitary-action results from f-routing channel lower bounds.</ref>
  </refs>

  <ask>
    Prove or disprove the claim above for the full approximate f-routing
    model. A proof must specify a fixed constant \(\epsilon\in(0,1)\), an
    exponent \(c\gt 0\), and a uniformly explicit function family
    \(f_n:\{0,1\}^n\times\{0,1\}^n\to\{0,1\}\), and must prove
    \[
      E_\epsilon^Q(f_n)\ge n^c
    \]
    for all sufficiently large \(n\), where \(E_\epsilon^Q\) is the minimum
    entanglement cost over arbitrary shared initial states, arbitrary CPTP
    local maps, unrestricted simultaneous quantum messages, and
    diamond-norm \(\epsilon\)-soundness on both \(f=0\) and \(f=1\) inputs.

    A proof limited to a random Boolean function, to a non-explicit family,
    to one-sided-perfect protocols, to perfect protocols only, to unitary or
    purified local actions, to a memory or gate-count lower bound, to a
    garden-hose or position-verification variant, or to a CDS or CDQS model
    must include a reduction showing that it lower-bounds \(E_\epsilon^Q\)
    in the full model above. Otherwise it is progress, not a proof.

    A disproof must show that the OQP48 target is impossible under the
    explicitness convention in this entry, for example by proving that every
    uniformly explicit \(f_n\) has \(E_\epsilon^Q(f_n)=n^{o(1)}\) for each
    fixed constant \(\epsilon\) in the relevant range, or by giving an
    equivalently strong universal upper-bound theorem that precludes
    polynomial lower bounds. An upper bound for a special family, a
    sub-exponential upper bound for all functions, or a criticism of a
    particular lower-bound technique is progress unless it rules out every
    admissible explicit family.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
