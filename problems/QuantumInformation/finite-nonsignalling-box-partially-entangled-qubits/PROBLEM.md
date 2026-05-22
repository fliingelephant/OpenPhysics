# QF032L Uniform finite nonsignalling box variant

```xml
<problem id="QF032L">
  <status>open</status>
  <name>Uniform finite nonsignalling box variant</name>
  <keys>oqp problem 32l uniform finite nonsignalling box finite non-signaling box nonlocal box finite inputs finite outputs global continuum variant partially entangled two qubits projective measurements exact simulation millionaire box m-box pr box shared randomness brunner gisin popescu scarani ahanj joag quantum foundations</keys>

  <claim>
    There exists a bipartite nonsignalling box \(R\) with finite input sets
    \(U,V\) and finite output sets \(C,D\) such that, for every
    \(0\lt\theta\lt\pi/4\), all local rank-one projective qubit measurement
    correlations of
    \[
      |\psi_\theta\rangle
      =
      \cos\theta |00\rangle+\sin\theta |11\rangle ,
    \]
    are exactly simulable by one use of \(R\), shared randomness, and a
    globally defined local pre- and post-processing protocol as defined below.
  </claim>

  <def>
    A finite bipartite nonsignalling box is a conditional probability
    distribution
    \[
      R(c,d|u,v),\qquad u\in U,\ v\in V,\ c\in C,\ d\in D,
    \]
    where \(U,V,C,D\) are finite sets, \(R(c,d|u,v)\ge 0\), and
    \(\sum_{c,d}R(c,d|u,v)=1\) for every \(u,v\). It is nonsignalling when
    \[
      \sum_d R(c,d|u,v)
    \]
    is independent of \(v\) for every \(u,c\), and
    \[
      \sum_c R(c,d|u,v)
    \]
    is independent of \(u\) for every \(v,d\). A PR box is the special case
    \(U=V=C=D=\{0,1\}\) with outputs satisfying \(c\oplus d=uv\) uniformly.

    A rank-one projective qubit measurement is a two-outcome projective
    measurement on \(\mathbb C^2\), indexed by a Bloch vector
    \(r\in S^2\subset\mathbb R^3\):
    \[
      E_a(r)=\frac{I+(-1)^a r\cdot\sigma}{2},
      \qquad a\in\{0,1\},
    \]
    where \(\sigma=(\sigma_x,\sigma_y,\sigma_z)\) is the Pauli vector.

    A finite box \(R\) globally simulates the projective correlations of
    \(|\psi_\theta\rangle\) if there is a shared-randomness probability space
    \((\Lambda_\theta,\mu_\theta)\), measurable local input maps
    \[
      f_A^\theta:S^2\times\Lambda_\theta\to U,\qquad
      f_B^\theta:S^2\times\Lambda_\theta\to V,
    \]
    and measurable local output kernels
    \[
      K_A^\theta(a|r,\lambda,c),\qquad
      K_B^\theta(b|s,\lambda,d),
    \]
    with nonnegative values and
    \[
      \sum_{a\in\{0,1\}}K_A^\theta(a|r,\lambda,c)=1,\qquad
      \sum_{b\in\{0,1\}}K_B^\theta(b|s,\lambda,d)=1
    \]
    for every admissible argument. The simulation condition is that, for every
    \(r,s\in S^2\) and \(a,b\in\{0,1\}\),
    \[
      \langle\psi_\theta|E_a(r)\otimes E_b(s)|\psi_\theta\rangle
      =
      \int_{\Lambda_\theta}
      \sum_{c,d}
      R(c,d|f_A^\theta(r,\lambda),f_B^\theta(s,\lambda))\,
      K_A^\theta(a|r,\lambda,c)K_B^\theta(b|s,\lambda,d)\,
      d\mu_\theta(\lambda).
    \]
    The simulator may depend on \(\theta\), but the finite box
    \(R(c,d|u,v)\) and its finite alphabets \(U,V,C,D\) must be the same for
    all \(0\lt\theta\lt\pi/4\). The shared randomness may be continuous; the
    open problem is about finiteness of the nonsignalling box, not about
    discretizing the shared randomness. A result that simulates each finite
    measurement list with unrelated finite-list-dependent maps is progress
    unless it yields a globally defined protocol of the form above.
  </def>

  <known>
    Open Quantum Problems, Problem 32, "Bell inequalities: many questions, a
    few answers", lists subquestion L: find a nonsignalling box with finitely
    many inputs and outputs with which one can simulate partially entangled
    states. The Open Quantum Problems index lists Problem 32 in the Quantum
    foundations category, with Nicolas Gisin as contact and last progress dated
    2016/12/01. This entry fixes a uniform global-continuum version: one finite
    box should simulate all projective-measurement correlations of all pure
    nonmaximally entangled two-qubit states, allowing the local protocol but
    not the box itself to depend on the entanglement parameter. A
    state-dependent finite-box formulation, or a finite-test-only formulation
    with incompatible maps for different measurement lists, is an adjacent
    variant and not the claim recorded here.

    Gisin, "Bell inequalities: many questions, a few answers", in Essays in
    Honour of Abner Shimony, Springer, 2009, arXiv:quant-ph/0702021, asks for
    a finite-input finite-output nonsignalling box in the same resource section
    that asks whether two PR boxes can replace two bits of classical
    communication. The surrounding text states that one PR box simulates
    maximally entangled two-qubit projective correlations and that partially
    entangled two-qubit states need stronger nonsignalling resources than one
    PR box in some regimes.

    Cerf, Gisin, Massar, and Popescu, "Simulating maximal quantum entanglement
    without communication", Physical Review Letters 94, 220403 (2005), give a
    finite nonsignalling resource, a single PR box, for exact simulation of all
    projective measurements on a maximally entangled two-qubit state. This is
    the maximally entangled endpoint, not a simulation of nonmaximally
    entangled states.

    Brunner, Gisin, and Scarani, "Entanglement and non-locality are different
    resources", New Journal of Physics 7, 88 (2005), prove that a single PR box
    is insufficient for a weak-entanglement range of nonmaximally entangled
    two-qubit states. This rules out the simplest finite box candidate for that
    range, but it does not rule out a different finite nonsignalling box.

    Jones and Masanes, "Interconversion of nonlocal correlations", Physical
    Review A 72, 052312 (2005), and Barrett and Pironio,
    "Popescu-Rohrlich correlations as a unit of nonlocality", Physical Review
    Letters 95, 140401 (2005), study conversion among finite nonsignalling
    boxes and PR-box simulation of finite nonsignalling behaviors. These finite
    scenario results show that PR-type resources are powerful once the
    measurement list is fixed, but they do not give one fixed finite box and
    globally defined local protocol that exactly simulate every projective
    measurement of every partially entangled two-qubit state.

    Brunner, Gisin, Popescu, and Scarani, "Simulation of partial entanglement
    with nonsignaling resources", Physical Review A 78, 052111 (2008), prove
    that all pure entangled two-qubit projective correlations can be simulated
    without communication by using PR boxes and a Millionaire box. The
    Millionaire box has finite outputs but inputs ranging over a continuous
    interval. Brunner, Gisin, Popescu, and Scarani explicitly identify as open
    whether this continuous-input resource can be replaced by finitely many
    PR boxes or, more generally, by a nonlocal box with finitely many possible
    inputs.

    Ahanj and Joag, "Simulation of a Partially Entangled Two Qubit State
    Correlation with one PR-Box and one M-box", arXiv:1104.2491 (2011),
    propose a protocol using one PR box and one Millionaire box. Because the
    Millionaire box has continuous inputs, this is not a finite-input
    nonsignalling box in the sense of the claim.
  </known>

  <refs>
    <ref>Open Quantum Problems, Bell inequalities: many questions, a few answers, Problem 32, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/bell-inequalities-many-questions-a-few-answers, used for subquestion L and the resource-simulation context.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is derived from Problem 32, Bell inequalities: many questions, a few answers, listed in Quantum foundations.</ref>
    <ref>N. Gisin, Bell inequalities: many questions, a few answers, in Essays in Honour of Abner Shimony, Western Ontario Series in Philosophy of Science, Springer, 2009, pp. 125-140, DOI: 10.1007/978-1-4020-9107-0_9, arXiv:quant-ph/0702021, Section III.3, used for the finite nonsignalling-box question.</ref>
    <ref>N. J. Cerf, N. Gisin, S. Massar, and S. Popescu, Simulating maximal quantum entanglement without communication, Physical Review Letters 94, 220403 (2005), DOI: 10.1103/PhysRevLett.94.220403, used for the single-PR-box exact simulation of the maximally entangled two-qubit endpoint.</ref>
    <ref>N. Brunner, N. Gisin, and V. Scarani, Entanglement and non-locality are different resources, New Journal of Physics 7, 88 (2005), DOI: 10.1088/1367-2630/7/1/088, arXiv:quant-ph/0412109, used for the insufficiency of one PR box for weakly entangled two-qubit states.</ref>
    <ref>N. S. Jones and Ll. Masanes, Interconversion of nonlocal correlations, Physical Review A 72, 052312 (2005), DOI: 10.1103/PhysRevA.72.052312, arXiv:quant-ph/0506182, used for finite nonsignalling-box interconversion in binary-output scenarios.</ref>
    <ref>J. Barrett and S. Pironio, Popescu-Rohrlich correlations as a unit of nonlocality, Physical Review Letters 95, 140401 (2005), DOI: 10.1103/PhysRevLett.95.140401, arXiv:quant-ph/0506180, used for PR-box simulation of broad classes of nonsignalling correlations and limitations in multipartite settings.</ref>
    <ref>N. Brunner, N. Gisin, S. Popescu, and V. Scarani, Simulation of partial entanglement with nonsignaling resources, Physical Review A 78, 052111 (2008), DOI: 10.1103/PhysRevA.78.052111, arXiv:0803.2359, used for the PR-box plus continuous-input Millionaire-box simulation and its finite-input open question.</ref>
    <ref>A. Ahanj and P. S. Joag, Simulation of a Partially Entangled Two Qubit State Correlation with one PR-Box and one M-box, arXiv:1104.2491 (2011), DOI: 10.48550/arXiv.1104.2491, used for a later continuous-input M-box protocol that does not settle the finite-input claim.</ref>
  </refs>

  <ask>
    Prove or disprove the global-continuum claim as stated. A proof must
    specify finite sets \(U,V,C,D\), a nonsignalling distribution
    \(R(c,d|u,v)\), and, for each \(0\lt\theta\lt\pi/4\), a globally defined
    simulation protocol over all Bloch-sphere projective measurements producing
    the exact Born probabilities in the definition.

    A disproof must prove that for every finite-input finite-output bipartite
    nonsignalling box \(R\), there is some nonmaximally entangled two-qubit
    state for which no globally defined one-use simulation protocol of the
    form in the definition exists. It is sufficient to exhibit, for every
    finite \(R\), a finite set of rank-one projective measurements whose
    behavior is not simulable by one use of \(R\) with shared randomness and
    local processing. A protocol using classical communication, a protocol
    using a continuous-input Millionaire or cosine box, a protocol depending on
    an unbounded number of PR boxes as the measurement list changes, an
    approximate simulation, an exact simulation only for one fixed finite
    measurement scenario, or finite-test simulations without a compatible
    global protocol is progress unless it implies the stated uniform finite-box
    claim or its negation.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
