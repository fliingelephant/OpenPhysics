# 28 Single-Prover Interactive Proofs for Quantum Computations

```xml
<problem id="28">
  <status>open</status>
  <kind>construction</kind>
  <field>QCT</field>
  <name>Single-Prover Interactive Proofs for Quantum Computations</name>
  <keys>single prover interactive proofs quantum computations qpip qpip0 classical verifier bqp prover bpp verifier information theoretic soundness classical communication delegated quantum computation classical verification quantum computation cvqc mahadev lwe cryptographic soundness blind quantum computation multi prover mip ruv post hoc verification trusted gates finite quantum verifier relativizing mip open quantum problems oqp problem 45 quantum complexity theory</keys>

  <claim>
    Every language \(L\in\mathsf{BQP}\) has a uniform interactive proof
    protocol with one prover and one verifier such that the verifier is a
    classical probabilistic polynomial-time machine, all messages are
    classical bit strings, the honest prover is implementable by a
    polynomial-time quantum computation, completeness is at least \(2/3\) on
    every \(x\in L\), and soundness is at most \(1/3\) on every
    \(x\notin L\) against every cheating prover, including computationally
    unbounded provers.
  </claim>

  <def>
    A language \(L\subseteq\{0,1\}^*\) is in \(\mathsf{BQP}\) if there is a
    uniform polynomial-size quantum circuit family, or equivalently a
    polynomial-time quantum Turing machine, that accepts \(x\in L\) with
    probability at least \(2/3\) and accepts \(x\notin L\) with probability
    at most \(1/3\). Replacing \(2/3,1/3\) by any fixed constants with a
    positive gap gives the same class by amplification.

    A single-prover classical-verifier quantum-prover interactive proof is a
    pair \((P,V)\) of interactive machines with common input \(x\). In
    Mahadev's resource notation this is the \(\tau=0\) verifier-quantum-memory
    setting, but this entry adds the stronger requirement of
    information-theoretic soundness. The verifier \(V\) is a classical
    probabilistic polynomial-time machine. The protocol has polynomially many
    rounds and every message has polynomial length in \(|x|\). The only
    communication channel carries classical bit strings. There is no quantum
    message, no verifier quantum register, no trusted center, no additional
    non-communicating prover, no pre-shared entanglement resource, and no
    computational hardness assumption.

    An efficient honest prover is a uniform polynomial-time quantum
    interactive strategy. It may keep a polynomial-size private quantum
    workspace between rounds and perform polynomial-time quantum operations,
    but its outgoing messages to \(V\) are classical strings obtained by
    measurement. The phrase "honest prover needs only \(\mathsf{BQP}\)
    power" means the honest prover's total running time over the interaction
    is polynomial in \(|x|\); it does not mean an all-powerful, \(\mathsf{PP}\),
    \(\mathsf{PostBQP}\), \(\mathsf{PSPACE}\), or exponentially precise
    prover.

    Completeness at level \(c\) means that for every \(x\in L\), the honest
    \(\mathsf{BQP}\) prover \(P\) makes \(V\) accept with probability at
    least \(c\), where the probability is over all randomness and quantum
    measurements in the honest interaction. Soundness at level \(s\) means
    that for every \(x\notin L\) and every cheating prover strategy \(P^*\),
    with unlimited classical or quantum computation and arbitrary private
    state but only classical communication with \(V\), the verifier accepts
    with probability at most \(s\). Information-theoretic soundness is this
    universal quantification over \(P^*\); it is not soundness against only
    polynomial-time quantum provers and it is not based on \(\mathsf{LWE}\),
    trapdoor claw-free functions, random oracles, indistinguishability
    obfuscation, or other cryptographic assumptions.
  </def>

  <known>
    Open Quantum Problems, Problem 45, "Single-prover interactive proofs for
    quantum computations", asks whether every \(L\in\mathsf{BQP}\) has an
    interactive protocol with a classical polynomial-time verifier and one
    untrusted prover such that a polynomial-time quantum prover can convince
    the verifier on yes-instances, while soundness holds against all provers,
    including computationally unbounded cheating provers. The Open Quantum
    Problems index lists Problem 45 in Quantum complexity theory, with
    contact Henry Yuen and date 2023/01/16.

    Lund, Fortnow, Karloff, and Nisan, "Algebraic Methods for Interactive
    Proof Systems", Journal of the ACM 39, 859-868 (1992), and Shamir,
    "IP = PSPACE", Journal of the ACM 39, 869-877 (1992), show that
    \(\mathsf{IP}=\mathsf{PSPACE}\). Since \(\mathsf{BQP}\subseteq
    \mathsf{PSPACE}\), these theorems imply classical-verifier interactive
    proofs for \(\mathsf{BQP}\) languages with information-theoretic
    soundness. They do not solve this entry because the honest prover in the
    resulting protocols may need \(\mathsf{PSPACE}\), \(\mathsf{P}^{\#P}\),
    or comparable power rather than \(\mathsf{BQP}\) power.

    Aharonov, Ben-Or, Eban, and Mahadev, "Interactive Proofs for Quantum
    Computations", arXiv:1704.04487 (2017), define quantum prover
    interactive proofs and prove that every \(\mathsf{BQP}\) language has a
    protocol with a \(\mathsf{BQP}\) prover and a verifier whose classical
    part is \(\mathsf{BPP}\) but who can store, process, and communicate a
    constant number of qubits. This gives information-theoretic soundness in
    a nearly classical single-verifier model, but it is not a \(\mathsf{QPIP}_0\)
    protocol because the verifier has quantum capability and quantum
    communication is used.

    Broadbent, Fitzsimons, and Kashefi, "Universal Blind Quantum
    Computation", FOCS 2009, give a blind and verifiable delegated-computation
    protocol in which a single server is used but the client can prepare and
    send randomly chosen single-qubit states. The same work also adapts the
    approach to a purely classical verifier with two non-communicating
    entangled servers. These are not solutions of this entry: the single-server
    protocol uses quantum messages from the verifier, and the purely classical
    protocol uses more than one prover.

    Reichardt, Unger, and Vazirani, "Classical command of quantum systems",
    Nature 496, 456-460 (2013), give a classical-verifier protocol based on
    rigidity of CHSH games and entangled non-communicating components. This is
    a central positive result for classical command of quantum systems, but it
    is a multi-prover or spatially separated-device result rather than a
    single-prover protocol.

    Fitzsimons, Hajdusek, and Morimae, "Post hoc Verification of Quantum
    Computation", Physical Review Letters 120, 040501 (2018), give post hoc
    verification protocols using either five entangled provers with a classical
    verifier or one prover with one-way quantum communication to a verifier
    that measures qubits in the \(X\) or \(Z\) basis. Morimae and Fitzsimons,
    "Post hoc verification with a single prover", arXiv:1603.06046 (2016),
    also show that similar constant-round protocols with one prover and a
    completely classical verifier would imply
    \(\mathsf{BQP}\subseteq\Sigma_3^{\mathsf P}\). This is evidence against
    constant-round solutions under the expectation that
    \(\mathsf{BQP}\not\subseteq\Sigma_3^{\mathsf P}\), but it does not rule
    out polynomial-round protocols and does not settle the claim above.

    Mahadev, "Classical Verification of Quantum Computations", SIAM Journal
    on Computing 51, 1172-1229 (2022), gives a single-prover protocol with an
    entirely classical verifier and an efficient quantum honest prover. Its
    soundness is computational: it assumes the Learning-with-Errors problem is
    intractable for efficient quantum machines. This does not solve Open
    Quantum Problems, Problem 45, because an unbounded cheating prover can
    break the cryptographic binding condition.

    Later work has strengthened adjacent cryptographic classical-verification
    results without removing the computational assumption. Zhang, "Classical
    Verification of Quantum Computations in Linear Time", Cryptology ePrint
    Archive 2022/432 and FOCS 2022, improves the asymptotic cost in a quantum
    random oracle model using noisy trapdoor claw-free functions. Bartusek,
    Kalai, Lombardi, Ma, Malavolta, Vaikuntanathan, Vidick, and Yang,
    "Succinct Classical Verification of Quantum Computation", CRYPTO 2022,
    give succinct classical arguments under post-quantum indistinguishability
    obfuscation and Learning-with-Errors assumptions. Barhoush, Morimae,
    Nishimaki, and Yamakawa, "Separating Non-Interactive Classical
    Verification of Quantum Computation from Falsifiable Assumptions",
    arXiv:2602.18034 (2026), prove, assuming a
    \(\mathsf{QMA}\)-\(\mathsf{QCMA}\) gap problem, that there is no quantum
    black-box reduction of a non-interactive classical-verification protocol
    for \(\mathsf{QMA}\), after instance-independent setup and one prover
    message, to falsifiable assumptions. These results are important for
    classical verification and cryptographic argument systems, but they do not
    give information-theoretic soundness against unbounded single provers for
    \(\mathsf{BQP}\).

    Kashefi, Leichtle, Music, and Ollivier, "Verification of Quantum
    Computations without Trusted Preparations or Measurements",
    arXiv:2403.10464 (2024), remove trusted preparations and measurements
    from information-theoretic delegated quantum computation by reducing
    verification to trusted single-qubit gates, and by giving a second
    construction with a verifier quantum register whose size is independent
    of the delegated computation. This is information-theoretic progress on
    weak-verifier models, but not a solution of this entry because the
    verifier is not entirely classical.

    Aaronson, Natarajan, Tal, and Villanyi, "A Relativizing MIP for BQP",
    arXiv:2604.11952 (2026), show that
    \(\mathsf{BQP}\subseteq\mathsf{MIP}\) holds relative to every classical
    oracle and describe relativization as a proxy for prover efficiency. This
    is multi-prover and oracle-relative progress, not a single-prover
    \(\mathsf{IP}\) protocol, and Aaronson, Natarajan, Tal, and Villanyi
    explicitly frame a non-cryptographic protocol for proving arbitrary
    quantum computations to a classical skeptic as a longstanding open
    problem.

    Green, "On Information-Theoretic Classical Verification of Quantum
    Computers", arXiv:2105.05942 (2021), studies a natural family of
    single-prover classical-verifier protocols called inexact linear scalar
    consistency checking protocols, including protocols inspired by earlier
    quantum-verification attempts. Green proves strong limitations for that
    family: in broad parameter regimes such protocols either admit cheating
    strategies or collapse to efficient classical simulation, and the analysis
    indicates that protocols in this family can require a prover stronger than
    \(\mathsf{BQP}\). These are barrier results for a class of approaches,
    not an impossibility theorem for all possible \(\mathsf{QPIP}_0\)
    protocols.
  </known>

  <refs>
    <ref>Open Quantum Problems, Problem 45, Single-prover interactive proofs for quantum computations, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/single-prover-interactive-proofs-for-quantum-computations, used for the exact \(\mathsf{QPIP}\) question, the completeness and soundness requirements, and the listed partial results.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: Problem 45 is listed as Single-prover interactive proofs for quantum computations in Quantum complexity theory, dated 2023/01/16.</ref>
    <ref>C. Lund, L. Fortnow, H. J. Karloff, and N. Nisan, Algebraic Methods for Interactive Proof Systems, Journal of the ACM 39(4), 859-868 (1992), DOI: 10.1145/146585.146605, used for algebraic interactive-proof methods and the \(\mathsf{P}^{\#P}\subseteq\mathsf{IP}\) route cited by Open Quantum Problems.</ref>
    <ref>A. Shamir, IP = PSPACE, Journal of the ACM 39(4), 869-877 (1992), DOI: 10.1145/146585.146609, used for \(\mathsf{IP}=\mathsf{PSPACE}\) and the all-powerful-prover baseline cited by Open Quantum Problems.</ref>
    <ref>E. Bernstein and U. Vazirani, Quantum Complexity Theory, SIAM Journal on Computing 26(5), 1411-1473 (1997), DOI: 10.1137/S0097539796300921, used for the \(\mathsf{BQP}\) model and standard containment context.</ref>
    <ref>D. Aharonov, M. Ben-Or, E. Eban, and U. Mahadev, Interactive Proofs for Quantum Computations, arXiv:1704.04487 (2017), used for the \(\mathsf{QPIP}_\kappa\) definition, \(\mathsf{BQP}\)-prover protocols with a constant-qubit verifier, and the statement that removing quantum communication remains open.</ref>
    <ref>A. Broadbent, J. Fitzsimons, and E. Kashefi, Universal Blind Quantum Computation, Proceedings of the 50th Annual IEEE Symposium on Foundations of Computer Science, 517-526 (2009), DOI: 10.1109/FOCS.2009.36, arXiv:0807.4154, used for the single-server protocol with client-prepared qubits and the two-prover classical-verifier adaptation.</ref>
    <ref>B. W. Reichardt, F. Unger, and U. Vazirani, Classical command of quantum systems, Nature 496, 456-460 (2013), DOI: 10.1038/nature12035, arXiv:1209.0448, used for the classical-verifier multi-prover or separated-device verification result.</ref>
    <ref>J. F. Fitzsimons, M. Hajdusek, and T. Morimae, Post hoc Verification of Quantum Computation, Physical Review Letters 120, 040501 (2018), DOI: 10.1103/PhysRevLett.120.040501, arXiv:1512.04375, used for post hoc verification variants with five entangled provers or one-way quantum communication to a weak quantum verifier.</ref>
    <ref>T. Morimae and J. F. Fitzsimons, Post hoc verification with a single prover, arXiv:1603.06046 (2016), used for the single-prover post hoc protocol and the constant-round single-prover classical-verifier complexity-theoretic barrier.</ref>
    <ref>U. Mahadev, Classical Verification of Quantum Computations, SIAM Journal on Computing 51(4), 1172-1229 (2022), DOI: 10.1137/20M1371828, arXiv:1804.01082, used for the single-prover classical-verifier protocol with \(\mathsf{LWE}\)-based computational soundness.</ref>
    <ref>J. Zhang, Classical Verification of Quantum Computations in Linear Time, Cryptology ePrint Archive, Paper 2022/432, https://eprint.iacr.org/2022/432, and 2022 IEEE 63rd Annual Symposium on Foundations of Computer Science, used for later cryptographic classical-verification improvements that retain computational assumptions.</ref>
    <ref>J. Bartusek, Y. T. Kalai, A. Lombardi, F. Ma, G. Malavolta, V. Vaikuntanathan, T. Vidick, and L. Yang, Succinct Classical Verification of Quantum Computation, Advances in Cryptology - CRYPTO 2022, arXiv:2206.14929, used for succinct cryptographic classical verification under post-quantum \(\mathsf{iO}\) and \(\mathsf{LWE}\) assumptions.</ref>
    <ref>M. Barhoush, T. Morimae, R. Nishimaki, and T. Yamakawa, Separating Non-Interactive Classical Verification of Quantum Computation from Falsifiable Assumptions, arXiv:2602.18034 (2026), used for a later limitation on non-interactive cryptographic classical verification variants.</ref>
    <ref>E. Kashefi, D. Leichtle, L. Music, and H. Ollivier, Verification of Quantum Computations without Trusted Preparations or Measurements, arXiv:2403.10464 (2024), used for information-theoretic weak-verifier progress that still requires trusted quantum gates or finite verifier-side quantum capability.</ref>
    <ref>S. Aaronson, A. Natarajan, A. Tal, and A. Villanyi, A Relativizing MIP for BQP, arXiv:2604.11952 (2026), used for classical-oracle \(\mathsf{MIP}\) progress and the explicit distinction from the longstanding non-cryptographic single-prover classical-skeptic problem.</ref>
    <ref>A. Green, On Information-Theoretic Classical Verification of Quantum Computers, arXiv:2105.05942 (2021), used for barrier results on inexact linear scalar consistency checking approaches to single-prover information-theoretic classical verification.</ref>
  </refs>

  <ask>
    Prove or disprove the claim exactly as stated. A proof must give, for
    every \(L\in\mathsf{BQP}\), a uniform polynomial-time classical verifier
    and a uniform polynomial-time quantum honest prover using only classical
    communication, with constant completeness-soundness gap and
    information-theoretic soundness against arbitrary unbounded cheating
    provers. A proof may use any protocol structure, but it must not rely on
    computational hardness assumptions, trusted setup, computationally binding
    trapdoors, quantum communication, a verifier quantum register, a trusted
    center, multiple non-communicating provers, or an honest prover stronger
    than \(\mathsf{BQP}\). Ordinary private randomness of the classical
    verifier is allowed.

    A disproof must prove that at least one \(\mathsf{BQP}\) language lacks
    such a protocol, or must otherwise prove that the positive claim implies a
    false or explicitly stated complexity-theoretic consequence. Conditional
    lower bounds, oracle separations, round lower bounds, or impossibility
    theorems for restricted protocol families count as progress unless their
    assumptions and scope rule out every protocol satisfying the definition
    above.

    Results for protocols with small quantum verifiers, verifier-prepared
    qubits, verifier quantum measurements, quantum prover-to-verifier
    messages, multiple entangled provers, blind delegation with quantum
    clients, \(\mathsf{LWE}\)- or random-oracle-based soundness, public-coin
    cryptographic arguments, trusted-center models, trusted gate models,
    finite verifier-side quantum registers, post hoc quantum-proof protocols,
    oracle-relative \(\mathsf{MIP}\) protocols, or \(\mathsf{PSPACE}\)-power
    provers solve this entry only if they are accompanied by a reduction or
    transformation that removes the extra resource while preserving a
    \(\mathsf{BQP}\) honest prover and information-theoretic soundness
    against unbounded single provers.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    For this existence problem, proof means a protocol satisfying every
    condition in the claim and definitions; disproof means a theorem excluding
    such protocols for at least one \(\mathsf{BQP}\) language or otherwise
    contradicting the claim's exact universal assertion.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
