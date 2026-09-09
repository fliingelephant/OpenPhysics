# 29 Thermodynamic Implementation of Gibbs-Preserving Maps

```xml
<problem id="29">
  <status>open</status>
  <kind>construction</kind>
  <field>QIT</field>
  <name>Thermodynamic Implementation of Gibbs-Preserving Maps</name>
  <keys>gibbs-preserving maps gibbs preserving operations gp maps gpm thermal operations thermodynamic implementation one-shot thermodynamics quantum thermodynamics time covariance time-covariant channels non-time-covariant channels coherence cost clock resource quantum clock time reference work cost information battery pure-state ancilla noisy operations unital channels quantum birkhoff thermodynamic capacity channel implementation process implementation open quantum problems oqp problem 46</keys>

  <claim>
    For every finite-dimensional input system \(X\) and output system \(X'\)
    with Hamiltonians \(H_X,H_{X'}\), inverse temperature \(\beta\), and
    Gibbs states
    \[
      \gamma_X=\frac{e^{-\beta H_X}}{\operatorname{tr}e^{-\beta H_X}},
      \qquad
      \gamma_{X'}=\frac{e^{-\beta H_{X'}}}{\operatorname{tr}e^{-\beta H_{X'}}},
    \]
    characterize the infimal additional resources required, as a function of
    a Gibbs-preserving CPTP map
    \(\mathcal E:\mathcal B(X)\to\mathcal B(X')\) satisfying
    \(\mathcal E(\gamma_X)=\gamma_{X'}\) and an accuracy
    \(\epsilon\ge 0\), to implement \(\mathcal E\) as a quantum process by
    physically implementable free thermodynamic operations such as thermal
    operations.

    The characterization must include work or purity resources, such as an
    explicit battery or pure-state ancillas, and time-translation-symmetry
    breaking resources, such as a finite clock or coherence resource, whenever
    they are necessary. It must also decide which Gibbs-preserving processes
    have finite cost and which require infinite or unbounded resources in the
    chosen one-shot framework.
  </claim>

  <def>
    A thermal operation on a system \(S\) with Hamiltonian \(H_S\) is a CPTP
    map of the form
    \[
      \Phi_S(\rho)=
      \operatorname{tr}_B\!\left[
        U_{SB}(\rho\otimes\gamma_B)U_{SB}^\dagger
      \right],
      \qquad
      [U_{SB},H_S\otimes I_B+I_S\otimes H_B]=0,
    \]
    where \(B\) is an ancillary bath system, \(H_B\) is its Hamiltonian, and
    \(\gamma_B=e^{-\beta H_B}/\operatorname{tr}e^{-\beta H_B}\). A thermal
    operation between different input and output systems means a protocol
    assembled from Gibbs-state ancillas, energy-conserving unitaries,
    discarding systems, and relabelling of retained subsystems; the effective
    channel from \(X\) to \(X'\) is compared with the target process.

    A channel \(\mathcal E:\mathcal B(X)\to\mathcal B(X')\) is
    Gibbs-preserving if it is completely positive, trace-preserving, and
    \(\mathcal E(\gamma_X)=\gamma_{X'}\). In the equal-system case
    \(X=X'\) and \(H_X=H_{X'}\), this says that the Gibbs state is a fixed
    point. Every thermal operation is Gibbs-preserving, but not every
    Gibbs-preserving channel is a thermal operation.

    A channel is time-covariant, or time-translation covariant, if for all
    real \(t\) and all operators \(\rho\),
    \[
      \mathcal E\!\left(e^{-itH_X}\rho e^{itH_X}\right)
      =
      e^{-itH_{X'}}\mathcal E(\rho)e^{itH_{X'}} .
    \]
    Thermal operations are time-covariant. Therefore a Gibbs-preserving
    process that is not time-covariant cannot be implemented by thermal
    operations alone; an implementation must consume or use a
    time-translation-symmetry-breaking resource.

    A process implementation with resource systems \(R\) is a thermodynamic
    protocol whose effective channel
    \(\mathcal T_{X\to X'}(\rho)=
    \operatorname{tr}_{R'}\Lambda(\rho\otimes\omega_R)\) satisfies
    \(\|\mathcal T-\mathcal E\|_\diamond\le\epsilon\), or an explicitly
    stated equivalent stabilized process-norm condition. This is stronger
    than implementing \(\mathcal E(\rho)\) for a fixed input state \(\rho\).
    Resource accounting may include a battery transition
    \(\eta_W^{\rm in}\to\eta_W^{\rm out}\), with work cost measured by
    energy change or by information-battery purity in pure nats, and a clock
    or coherence system \(C\), with cost measured for example by Hilbert-space
    dimension, energy spread, quantum Fisher information, asymmetry monotones,
    or copies of a fixed coherent qubit state such as
    \(|+\rangle=(|0\rangle+|1\rangle)/\sqrt 2\). A proposed solution must
    state the precise cost convention and prove that the corresponding free
    operations are physically implementable.

    If \(H_X=H_{X'}=0\), then \(\gamma_X,\gamma_{X'}\) are maximally mixed and
    Gibbs-preserving channels are exactly unital channels, after the input and
    output dimensions are matched. This trivial-Hamiltonian case is the
    noisy-operation or quantum-Birkhoff special case. It is not the same as
    the nontrivial-Hamiltonian quantum-coherent case, because the latter has
    nontrivial time covariance and coherence constraints.
  </def>

  <known>
    Open Quantum Problems records Problem 46, "Thermodynamic Implementation
    of Gibbs-Preserving Maps", and asks for the minimal resources required to
    implement a general Gibbs-preserving map in a one-shot thermodynamic
    framework whose free operations can reasonably be implemented physically,
    for example thermal operations. The Open Quantum Problems index lists
    Problem 46 in Quantum thermodynamics with contact Philippe Faist and date
    2023/01/16.

    Faist, Oppenheim, and Renner, "Gibbs-Preserving Maps outperform Thermal
    Operations in the quantum regime", New Journal of Physics 17, 043003
    (2015), prove that the classical equivalence between Gibbs-preserving maps
    and thermal operations fails in the quantum regime. They exhibit
    Gibbs-preserving maps that create coherence between energy eigenspaces,
    while thermal operations cannot do so because thermal operations are
    time-covariant. This is the basic obstruction that makes OQP46 a
    channel-implementation problem, not merely a state-transformation problem.

    In the trivial-Hamiltonian case, OQP46 records that Gibbs-preserving maps
    become unital maps and thermal operations reduce to noisy operations.
    Landau and Streater, "On Birkhoff's theorem for doubly stochastic
    completely positive maps of matrix algebras", Linear Algebra and its
    Applications 193, 107-127 (1993), and Haagerup and Musat, "Factorization
    and Dilation Problems for Completely Positive Maps on von Neumann
    Algebras", Communications in Mathematical Physics 303, 555-594 (2011),
    show that the quantum unital/noisy-operation setting is already strictly
    subtler than the classical Birkhoff theorem. Faist, Dupuis, Oppenheim,
    and Renner, "The Minimal Work Cost of Information Processing", Nature
    Communications 6, 7669 (2015), building on del Rio, Aberg, Renner,
    Dahlsten, and Vedral, "The thermodynamic meaning of negative entropy",
    Nature 474, 61-63 (2011), gives the work-cost and decoupling technology
    used by OQP46 for the one-shot unital special case. This does not solve
    the nontrivial-Hamiltonian, coherence-sensitive problem.

    Faist, Berta, and Brandao, "Thermodynamic Implementations of Quantum
    Processes", Communications in Mathematical Physics 384, 1709-1750 (2021),
    give universal i.i.d. implementations and identify the thermodynamic
    capacity of a process as the optimal work-cost rate in their settings.
    Their arXiv version includes a time-covariant thermal-operation result
    for block-diagonal or time-covariant inputs, recorded by OQP46 as a
    partial result. These theorems do not implement arbitrary
    non-time-covariant Gibbs-preserving processes using thermal operations
    with minimal clock and coherence resources.

    Woods, Silva, and Oppenheim, "Autonomous Quantum Machines and
    Finite-Sized Clocks", Annales Henri Poincare 20, 125-218 (2019), provide
    finite-clock constructions where clock dimension and energy spread control
    accuracy and back-reaction. Marvian, "Coherence distillation machines are
    impossible in quantum thermodynamics", Nature Communications 11, 25
    (2020), shows that coherence between energy eigenspaces is a resource
    independent of work and that generic full-rank mixed clock states cannot
    yield pure coherent states at nonzero asymptotic rate. These results
    identify candidate clock and coherence costs, but not the full OQP46
    classification for arbitrary Gibbs-preserving channels.

    Sagawa, Faist, Kato, Matsumoto, Nagaoka, and Brandao, "Asymptotic
    Reversibility of Thermal Operations for Interacting Quantum Spin Systems
    via Generalized Quantum Stein's Lemma", Journal of Physics A:
    Mathematical and Theoretical 54, 495303 (2021), characterize asymptotic
    state convertibility for translation-invariant ergodic spin-system states
    by thermal operations aided by a small coherence source. This is important
    state-transformation progress, but it is not a general finite-dimensional
    channel-implementation resource classification.

    Tajima and Takagi, "Gibbs-Preserving Operations Requiring Infinite Amount
    of Quantum Coherence", Physical Review Letters 134, 170201 (2025), prove
    major progress directly relevant to OQP46: they present a family of
    Gibbs-preserving operations that cannot be implemented by thermal
    operations aided by any finite amount of quantum coherence, and they give
    almost-tight lower bounds for the approximate coherence cost for a class
    of Gibbs-preserving operations. This rules out the naive finite-resource
    hope for all Gibbs-preserving processes, but it is not a complete
    minimal-resource classification across arbitrary Gibbs-preserving CPTP
    maps, work resources, clock conventions, and approximation regimes.

    Shiraishi, "Quantum Thermodynamics with Coherence: Covariant
    Gibbs-Preserving Operation Is Characterized by the Free Energy", Physical
    Review Letters 134, 160402 (2025), proves a correlated-catalytic
    state-convertibility result for covariant Gibbs-preserving operations.
    Faist, "Universal thermodynamic implementation of a process with a
    variable work cost", arXiv:2601.20155 (2026), constructs a thermal-
    operation implementation of many i.i.d. copies of time-covariant quantum
    channels with variable work cost, up to process decoherence associated
    with revealing the work cost. These later results sharpen adjacent
    covariant and asymptotic regimes, but they do not settle the general
    non-time-covariant OQP46 process question.
  </known>

  <refs>
    <ref>Open Quantum Problems, Thermodynamic Implementation of Gibbs-Preserving Maps, Problem 46, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/thermodynamic-implementation-of-gibbs-preserving-maps, used for the problem statement, physical-free-operation requirement, work and coherence resource discussion, and partial results.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems index, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 46, Thermodynamic implementation of Gibbs-Preserving Maps, in Quantum thermodynamics.</ref>
    <ref>P. Faist, J. Oppenheim, and R. Renner, Gibbs-Preserving Maps outperform Thermal Operations in the quantum regime, New Journal of Physics 17, 043003 (2015), DOI: 10.1088/1367-2630/17/4/043003, arXiv:1406.3618, used for the strict quantum gap between Gibbs-preserving maps and thermal operations.</ref>
    <ref>P. Faist, M. Berta, and F. G. S. L. Brandao, Thermodynamic Implementations of Quantum Processes, Communications in Mathematical Physics 384, 1709-1750 (2021), DOI: 10.1007/s00220-021-04107-w, arXiv:1911.05563, used for thermodynamic capacity, universal i.i.d. process implementations, and the time-covariant thermal-operation partial result in the arXiv version.</ref>
    <ref>P. Faist, F. Dupuis, J. Oppenheim, and R. Renner, The Minimal Work Cost of Information Processing, Nature Communications 6, 7669 (2015), DOI: 10.1038/ncomms8669, arXiv:1211.1037, used for one-shot work-cost accounting and decoupling-based implementations.</ref>
    <ref>L. del Rio, J. Aberg, R. Renner, O. Dahlsten, and V. Vedral, The thermodynamic meaning of negative entropy, Nature 474, 61-63 (2011), DOI: 10.1038/nature10123, arXiv:1009.1630, used for one-shot erasure and work-extraction background.</ref>
    <ref>M. P. Woods, R. Silva, and J. Oppenheim, Autonomous Quantum Machines and Finite-Sized Clocks, Annales Henri Poincare 20, 125-218 (2019), DOI: 10.1007/s00023-018-0736-9, arXiv:1607.04591, used for finite-clock resource and accuracy tradeoffs.</ref>
    <ref>I. Marvian, Coherence distillation machines are impossible in quantum thermodynamics, Nature Communications 11, 25 (2020), DOI: 10.1038/s41467-019-13846-3, arXiv:1805.01989, used for coherence-as-resource and coherence-distillation limits.</ref>
    <ref>L. J. Landau and R. F. Streater, On Birkhoff's theorem for doubly stochastic completely positive maps of matrix algebras, Linear Algebra and its Applications 193, 107-127 (1993), DOI: 10.1016/0024-3795(93)90274-R, used for the unital/noisy-operation special case.</ref>
    <ref>U. Haagerup and M. Musat, Factorization and Dilation Problems for Completely Positive Maps on von Neumann Algebras, Communications in Mathematical Physics 303, 555-594 (2011), DOI: 10.1007/s00220-011-1216-y, arXiv:1009.0778, used for the failure of quantum Birkhoff-type factorizability in the unital special case.</ref>
    <ref>T. Sagawa, P. Faist, K. Kato, K. Matsumoto, H. Nagaoka, and F. G. S. L. Brandao, Asymptotic Reversibility of Thermal Operations for Interacting Quantum Spin Systems via Generalized Quantum Stein's Lemma, Journal of Physics A: Mathematical and Theoretical 54, 495303 (2021), DOI: 10.1088/1751-8121/ac333c, arXiv:1907.05650, used for asymptotic state-convertibility context under thermal operations with coherence assistance.</ref>
    <ref>H. Tajima and R. Takagi, Gibbs-Preserving Operations Requiring Infinite Amount of Quantum Coherence, Physical Review Letters 134, 170201 (2025), DOI: 10.1103/PhysRevLett.134.170201, arXiv:2404.03479, used for the finite-coherence impossibility family and approximate coherence-cost lower bounds.</ref>
    <ref>N. Shiraishi, Quantum Thermodynamics with Coherence: Covariant Gibbs-Preserving Operation Is Characterized by the Free Energy, Physical Review Letters 134, 160402 (2025), DOI: 10.1103/PhysRevLett.134.160402, arXiv:2406.06234, used for correlated-catalytic covariant state-convertibility context.</ref>
    <ref>A. Munson, N. B. T. Kothakonda, J. Haferkamp, N. Yunger Halpern, J. Eisert, and P. Faist, Complexity-Constrained Quantum Thermodynamics, PRX Quantum 6, 010346 (2025), DOI: 10.1103/PRXQuantum.6.010346, arXiv:2403.04828, used for modern status language that general Gibbs-preserving implementation costs remain unclear.</ref>
    <ref>P. Faist, Universal thermodynamic implementation of a process with a variable work cost, arXiv:2601.20155 (2026), used for later time-covariant i.i.d. channel implementation with thermal operations and variable work cost.</ref>
  </refs>

  <ask>
    Give a complete one-shot resource classification for implementing
    Gibbs-preserving CPTP maps by physically implementable thermodynamic free
    operations. A solution must specify the admissible resource systems and
    cost measures, prove that the free operations are operationally
    implementable, and give necessary and sufficient conditions or exact
    infimal costs for every finite-dimensional Gibbs-preserving
    \(\mathcal E\) and accuracy \(\epsilon\). The classification must cover
    both time-covariant and non-time-covariant maps, exact and approximate
    implementation, and finite versus infinite resource requirements.

    A solution may choose a precise framework, such as an explicit work
    battery plus a clock or asymmetry resource, but it must state the
    framework locally and explain which parts of the classification are
    framework-dependent. It must not treat Gibbs preservation alone as
    physical implementability, and it must not replace process implementation
    by fixed-input state transformation unless it also proves the reduction
    from the process question.

    Results limited to unital maps with trivial Hamiltonians, to block-
    diagonal inputs, to time-covariant channels, to asymptotic state
    transformations, or to a proper subclass of Gibbs-preserving maps count as
    progress unless they are combined with a reduction covering arbitrary
    finite-dimensional Gibbs-preserving CPTP maps. A family requiring infinite
    coherence cost is progress on the classification and rules out finite-cost
    universality in that framework, but it is not by itself a complete answer
    unless all finite, infinite, and tradeoff regimes are characterized.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
