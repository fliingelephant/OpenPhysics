# QF032J One-bit communication witnesses for partially entangled qubits

```xml
<problem id="QF032J">
  <status>open</status>
  <name>One-bit communication witnesses for partially entangled qubits</name>
  <keys>bell inequalities one bit communication one cbit communication communication assisted bell inequality communication cost classical simulation partially entangled two qubits projective measurements bidirectional communication alice bob toner bacon renner quintino zambrini cruzeiro gisin open quantum problems oqp problem 32j quantum foundations</keys>

  <claim>
    There exist positive integers \(m_A,m_B\), a real Bell functional \(\ell\)
    on the finite binary-output bipartite scenario \(X=[m_A]\),
    \(Y=[m_B]\), \(A=B=\{0,1\}\), a parameter
    \(0\lt\theta\lt\pi/4\), and rank-one projective qubit measurements such
    that
    \[
      \ell(p)\le \beta_1
      \quad\text{for every }p\in\mathcal C^{\leftrightarrow}_1(m_A,m_B),
    \]
    where \(\mathcal C^{\leftrightarrow}_1(m_A,m_B)\) is the one-bit
    communication polytope defined below, but the behavior obtained from
    \[
      |\psi_\theta\rangle
      =
      \cos\theta |00\rangle+\sin\theta |11\rangle
    \]
    satisfies \(\ell(q_\theta)\gt \beta_1\).
  </claim>

  <def>
    For a positive integer \(r\), write \([r]=\{1,\ldots,r\}\).
    A behavior is an array
    \[
      p=\{p(a,b|x,y):a,b\in\{0,1\},\ x\in[m_A],\ y\in[m_B]\}
    \]
    of nonnegative probabilities normalized for every pair \((x,y)\).

    The bidirectional one-bit communication polytope
    \(\mathcal C^{\leftrightarrow}_1(m_A,m_B)\) is the convex hull of all
    deterministic one-bit strategies of either fixed direction. In an
    Alice-to-Bob vertex there are functions
    \[
      \alpha:[m_A]\to\{0,1\},\qquad
      \gamma:[m_A]\to\{0,1\},\qquad
      \beta:[m_B]\times\{0,1\}\to\{0,1\}
    \]
    and the outputs are \(a=\alpha(x)\), \(c=\gamma(x)\), and
    \(b=\beta(y,c)\). In a Bob-to-Alice vertex there are functions
    \[
      \beta:[m_B]\to\{0,1\},\qquad
      \delta:[m_B]\to\{0,1\},\qquad
      \alpha:[m_A]\times\{0,1\}\to\{0,1\}
    \]
    and the outputs are \(b=\beta(y)\), \(c=\delta(y)\), and
    \(a=\alpha(x,c)\). Convex combinations represent shared randomness before
    the inputs are received. This convention is stronger than a fixed
    Alice-to-Bob convention: any inequality valid for
    \(\mathcal C^{\leftrightarrow}_1(m_A,m_B)\) is valid for both one-way
    subpolytopes and for mixtures of the two directions.

    A Bell functional is
    \[
      \ell(p)=
      \sum_{x,y,a,b} c_{a,b,x,y}p(a,b|x,y),
    \]
    and \(\beta_1=\max_{p\in\mathcal C^{\leftrightarrow}_1(m_A,m_B)}\ell(p)\).
    A behavior violates the one-bit communication inequality
    \(\ell\le\beta_1\) when its value is strictly larger than \(\beta_1\).

    A rank-one projective qubit measurement is a two-outcome projective
    measurement on \(\mathbb C^2\). Equivalently, for each Alice input \(x\)
    and Bob input \(y\) there are unit vectors \(u_x,v_y\in\mathbb R^3\) such
    that
    \[
      E^x_a=\frac{I+(-1)^a u_x\cdot\sigma}{2},\qquad
      F^y_b=\frac{I+(-1)^b v_y\cdot\sigma}{2},
    \]
    where \(\sigma=(\sigma_x,\sigma_y,\sigma_z)\) is the Pauli vector. The
    quantum behavior in the claim is
    \[
      q_\theta(a,b|x,y)
      =
      \langle\psi_\theta|E^x_a\otimes F^y_b|\psi_\theta\rangle .
    \]
    The restriction \(0\lt\theta\lt\pi/4\) means that the state is entangled
    but not maximally entangled.
  </def>

  <known>
    Open Quantum Problems, Problem 32, "Bell inequalities: many questions, a
    few answers", lists subquestion J: whether there is a Bell inequality valid
    for all correlations simulable with a single bit of communication and
    violated by some partially entangled two-qubit states. The Open Quantum
    Problems index lists Problem 32 in the Quantum foundations category, with
    Nicolas Gisin as contact and last progress dated 2016/12/01. This entry
    records the binary-output projective-qubit version and uses the
    bidirectional one-bit polytope so that a proof gives a direction-independent
    lower bound.

    Gisin, "Bell inequalities: many questions, a few answers", arXiv:quant-ph/0702021,
    poses the same resource question in a list about nonlocality as a resource.
    Gisin also records the surrounding facts that one bit of communication can
    simulate maximally entangled two-qubit projective correlations, and asks
    whether partially entangled qubit pairs can require more than this resource.

    Toner and Bacon, "Communication cost of simulating Bell correlations",
    Physical Review Letters 91, 187904 (2003), prove that one classical bit of
    communication plus shared randomness exactly simulates local projective
    measurements on a maximally entangled two-qubit Bell pair. That result
    excludes maximally entangled two-qubit projective measurements as witnesses
    for the claim, but it does not simulate all nonmaximally entangled
    two-qubit projective correlations.

    Bacon and Toner, "Bell inequalities with auxiliary communication",
    Physical Review Letters 90, 157904 (2003), and Maxwell and Chitambar,
    "Bell inequalities with communication assistance", Physical Review A 89,
    042108 (2014), analyze complete small communication-assisted Bell
    polytopes. These results constrain small finite scenarios, but they do not
    prove or disprove the existence claim for arbitrary \(m_A,m_B\).

    Zambrini Cruzeiro and Gisin, "Bell inequalities with one bit of
    communication", Entropy 21, 171 (2019), develop a method for binary-output
    Bell scenarios supplemented by one bit of classical communication. They find
    668 inequivalent facet inequalities in the \(3,3,+1\) scenario, conjecture
    completeness for that list, and report no two-qubit quantum violation of
    the one-bit bound in that scenario. This is negative evidence for small
    input numbers, not a disproof of the present arbitrary-input claim.

    Renner and Quintino, "The minimal communication cost for simulating
    entangled qubits", Quantum 7, 1149 (2023), prove that a single classical
    trit suffices to simulate all local projective measurements on every pure
    entangled two-qubit state
    \(\sqrt p |00\rangle+\sqrt{1-p}|11\rangle\), and that one bit suffices in
    the weakly entangled range where
    \[
      \frac{2p(1-p)}{2p-1}\log\!\left(\frac{p}{1-p}\right)+2(1-p)\le 1,
    \]
    approximately \(0.835\le p\le 1\). This settles a weak-entanglement
    subrange positively for simulation, but leaves open whether the remaining
    partially entangled two-qubit projective correlations require more than one
    bit.

    Marton, Bene, Divianszky, and Vertesi, "Beating one bit of communication
    with and without quantum pseudo-telepathy", npj Quantum Information 10, 79
    (2024), give finite Bell-type constructions whose high-dimensional quantum
    correlations beat one-bit classical bounds. Those constructions prove that
    one bit is not enough for some finite-dimensional quantum correlations, but
    they do not settle the present two-qubit partially entangled projective
    claim. Marton, Bene, Divianszky, and Vertesi also cite recent numerical
    evidence that all two-qubit projective correlations may be simulable with
    one bit of communication; this is relevant negative evidence for the
    present claim, not a proof.
  </known>

  <refs>
    <ref>Open Quantum Problems, Bell inequalities: many questions, a few answers, Problem 32, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/bell-inequalities-many-questions-a-few-answers, used for subquestion J and the listed partial progress.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is derived from Problem 32, Bell inequalities: many questions, a few answers, listed in Quantum foundations.</ref>
    <ref>N. Gisin, Bell inequalities: many questions, a few answers, in Essays in Honour of Abner Shimony, Western Ontario Series in Philosophy of Science, Springer, 2009, pp. 125-140, DOI: 10.1007/978-1-4020-9107-0_9, arXiv:quant-ph/0702021, used for the original one-bit communication resource question.</ref>
    <ref>B. F. Toner and D. Bacon, Communication cost of simulating Bell correlations, Physical Review Letters 91, 187904 (2003), DOI: 10.1103/PhysRevLett.91.187904, arXiv:quant-ph/0304076, used for one-bit simulation of maximally entangled two-qubit projective correlations.</ref>
    <ref>D. Bacon and B. F. Toner, Bell inequalities with auxiliary communication, Physical Review Letters 90, 157904 (2003), DOI: 10.1103/PhysRevLett.90.157904, arXiv:quant-ph/0208057, used for small communication-assisted Bell polytopes.</ref>
    <ref>K. Maxwell and E. Chitambar, Bell inequalities with communication assistance, Physical Review A 89, 042108 (2014), DOI: 10.1103/PhysRevA.89.042108, arXiv:1401.5439, used for fixed-direction and bidirectional communication-assisted inequalities in small scenarios.</ref>
    <ref>E. Zambrini Cruzeiro and N. Gisin, Bell inequalities with one bit of communication, Entropy 21(2), 171 (2019), DOI: 10.3390/e21020171, arXiv:1812.05107, used for the \(3,3,+1\) facet search and no-violation evidence.</ref>
    <ref>M. J. Renner and M. T. Quintino, The minimal communication cost for simulating entangled qubits, Quantum 7, 1149 (2023), DOI: 10.22331/q-2023-10-24-1149, arXiv:2207.12457, used for the trit protocol and weak-entanglement one-bit protocol.</ref>
    <ref>I. Marton, E. Bene, P. Divianszky, and T. Vertesi, Beating one bit of communication with and without quantum pseudo-telepathy, npj Quantum Information 10, 79 (2024), DOI: 10.1038/s41534-024-00874-1, arXiv:2308.10771, used for high-dimensional one-bit lower-bound progress distinct from the two-qubit claim.</ref>
  </refs>

  <ask>
    Prove or disprove the claim under the bidirectional one-bit convention in
    the definition. A proof must give finite input numbers \(m_A,m_B\), an
    explicit Bell functional \(\ell\), a certified value of
    \(\beta_1=\max_{\mathcal C^{\leftrightarrow}_1}\ell\), a parameter
    \(0\lt\theta\lt\pi/4\), and explicit projective qubit measurements with a
    rigorous strict gap \(\ell(q_\theta)-\beta_1\gt 0\).

    A disproof must prove that for every finite binary-output scenario, every
    real Bell functional, every nonmaximally entangled two-qubit pure state, and
    every choice of local projective measurements, the resulting behavior lies
    in \(\mathcal C^{\leftrightarrow}_1(m_A,m_B)\). Equivalently, it may give a
    universal one-bit bidirectional simulation protocol for all projective
    measurements on all pure entangled two-qubit states. A fixed small-scenario
    facet enumeration, a fixed-direction-only inequality, a numerical quantum
    search without certified bounds, a high-dimensional quantum violation, a
    trit simulation, or a one-bit simulation for only a subrange of
    \(\theta\) is progress unless it implies the stated bidirectional claim or
    its negation.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
