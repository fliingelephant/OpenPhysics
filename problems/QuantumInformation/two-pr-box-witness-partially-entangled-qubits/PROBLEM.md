# QF032K Common-order two-PR-box witness variant

```xml
<problem id="QF032K">
  <status>open</status>
  <name>Common-order two-PR-box witness variant</name>
  <keys>oqp problem 32k two pr boxes two pr-boxes common-order sequential common order variant popescu rohrlich box nonlocal box nl bit resource simulation two pr assisted bell inequality partially entangled two qubits projective measurements shared randomness local wirings brunner gisin scarani cerf massar popescu quantum foundations</keys>

  <claim>
    There exist positive integers \(m_A,m_B\), a real Bell functional \(\ell\)
    on the finite binary-output bipartite scenario \(X=[m_A]\),
    \(Y=[m_B]\), \(A=B=\{0,1\}\), a parameter
    \(0\lt\theta\lt\pi/4\), and rank-one projective qubit measurements such
    that
    \[
      \ell(p)\le \beta_2
      \quad\text{for every }p\in\mathcal P_2(m_A,m_B),
    \]
    where \(\mathcal P_2(m_A,m_B)\) is the common-order sequential
    two-PR-box-assisted correlation set defined below, but the behavior
    obtained from
    \[
      |\psi_\theta\rangle
      =
      \cos\theta |00\rangle+\sin\theta |11\rangle
    \]
    satisfies \(\ell(q_\theta)\gt\beta_2\).
  </claim>

  <def>
    For a positive integer \(r\), write \([r]=\{1,\ldots,r\}\). A behavior is
    an array
    \[
      p=\{p(a,b|x,y):a,b\in\{0,1\},\ x\in[m_A],\ y\in[m_B]\}
    \]
    of nonnegative probabilities normalized for every pair \((x,y)\).

    A PR box is the nonsignalling binary-input binary-output behavior
    \[
      \operatorname{PR}(r,s|u,v)
      =
      \begin{cases}
        1/2, &amp; r\oplus s=uv,\\
        0, &amp; r\oplus s\ne uv,
      \end{cases}
    \]
    where \(u,v,r,s\in\{0,1\}\) and \(\oplus\) denotes addition modulo \(2\).

    A deterministic two-PR-box local wiring in the scenario
    \(([m_A],[m_B],\{0,1\},\{0,1\})\) consists of functions
    \[
      u_1:[m_A]\to\{0,1\},\qquad
      v_1:[m_B]\to\{0,1\},
    \]
    \[
      u_2:[m_A]\times\{0,1\}\to\{0,1\},\qquad
      v_2:[m_B]\times\{0,1\}\to\{0,1\},
    \]
    and final-output functions
    \[
      A:[m_A]\times\{0,1\}^2\to\{0,1\},\qquad
      B:[m_B]\times\{0,1\}^2\to\{0,1\}.
    \]
    Given inputs \(x,y\), Alice and Bob query the first PR box with
    \(u_1(x),v_1(y)\), obtaining outputs \(r_1,s_1\). They then query the
    second PR box with \(u_2(x,r_1),v_2(y,s_1)\), obtaining \(r_2,s_2\), and
    return
    \[
      a=A(x,r_1,r_2),\qquad b=B(y,s_1,s_2).
    \]
    The induced behavior is
    \[
      p(a,b|x,y)
      =
      \sum_{r_1,s_1,r_2,s_2}
      \operatorname{PR}(r_1,s_1|u_1(x),v_1(y))\,
      \operatorname{PR}(r_2,s_2|u_2(x,r_1),v_2(y,s_1))\,
      \mathbf 1\{a=A(x,r_1,r_2)\}\mathbf 1\{b=B(y,s_1,s_2)\}.
    \]
    The set \(\mathcal P_2(m_A,m_B)\) is the convex hull of all such
    deterministic two-PR-box local wirings. Convex combinations represent
    shared randomness chosen before \(x,y\) are received. The definition allows
    either party to ignore one or both PR boxes, and it allows the second local
    PR input to depend on that party's first PR output. This entry adopts the
    common-order sequential convention: both parties query one shared PR box
    first and then query the second shared PR box. More general asynchronous or
    differently ordered two-box wiring conventions define adjacent variants;
    a separation for the present \(\mathcal P_2\) is a certified lower bound
    for this convention.

    A Bell functional is
    \[
      \ell(p)=\sum_{x,y,a,b} c_{a,b,x,y}p(a,b|x,y),
    \]
    and
    \[
      \beta_2=\max_{p\in\mathcal P_2(m_A,m_B)}\ell(p).
    \]
    A two-PR-box resource inequality is an inequality
    \(\ell(p)\le\beta_2\) valid on \(\mathcal P_2(m_A,m_B)\).

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
    few answers", lists subquestion K: find inequalities satisfied by all
    correlations that can be simulated by two PR boxes. The Open Quantum
    Problems index lists Problem 32 in the Quantum foundations category, with
    Nicolas Gisin as contact and last progress dated 2016/12/01. This entry
    records a common-order sequential two-PR-box variant in the finite
    binary-output projective-qubit setting, because a bare request to "find
    inequalities" has no mathematical stopping criterion without a target
    scenario and a validity class. The unqualified OQP/Gisin phrase "two
    PR boxes" may also be formalized using broader local wiring conventions;
    those variants are adjacent to, but not identical with, the claim here.

    Gisin, "Bell inequalities: many questions, a few answers", in Essays in
    Honour of Abner Shimony, Springer, 2009, arXiv:quant-ph/0702021, places
    this question in the resource-simulation subsection. Gisin notes that one
    PR box plus shared randomness simulates maximally entangled two-qubit
    projective correlations, that some partially entangled two-qubit
    correlations are not simulable with one PR box, and then asks for
    inequalities valid for two PR boxes, motivated by the fact that two bits of
    classical communication can simulate two-qubit states.

    Cerf, Gisin, Massar, and Popescu, "Simulating maximal quantum entanglement
    without communication", Physical Review Letters 94, 220403 (2005), prove
    that a single PR box plus shared randomness exactly simulates all local
    projective measurements on a maximally entangled two-qubit state. This
    excludes maximally entangled two-qubit projective correlations as witnesses
    for the claim.

    Brunner, Gisin, and Scarani, "Entanglement and non-locality are different
    resources", New Journal of Physics 7, 88 (2005), prove that one PR box is
    insufficient for pure states
    \(\cos\alpha |00\rangle+\sin\alpha |11\rangle\) in a weak-entanglement
    range. Brunner, Scarani, and Gisin, "Bell-type inequalities for non-local
    resources", Journal of Mathematical Physics 47, 112101 (2006), construct
    resource inequalities for one use of generalized nonlocal machines. These
    results are one-resource lower bounds and do not characterize
    \(\mathcal P_2(m_A,m_B)\).

    Regev and Toner, "Simulating quantum correlations with finite
    communication", SIAM Journal on Computing 39, 1562-1580 (2009), prove that
    two classical bits of communication suffice to simulate the correlation
    functions \(E[\alpha\beta]=\operatorname{Tr}(A\otimes B\rho)\) of
    bipartite projective measurements. Toner and Bacon, "Communication cost of
    simulating Bell correlations", Physical Review Letters 91, 187904 (2003),
    give a classical-teleportation argument for two-bit simulation of
    projective measurements on two-qubit states. These communication protocols
    motivate the two-PR-box question but do not answer it, because PR boxes are
    nonsignalling and do not implement classical communication.

    Brunner, Gisin, Popescu, and Scarani, "Simulation of partial entanglement
    with nonsignaling resources", Physical Review A 78, 052111 (2008), give a
    nonsignalling simulation of partially entangled two-qubit projective
    correlations using PR boxes together with a continuous-input Millionaire
    box. Their protocol uses resources outside \(\mathcal P_2(m_A,m_B)\) and
    explicitly leaves open whether the continuous-input ingredient can be
    replaced by finite PR-box resources or a finite-input nonlocal box.

    Christensen, McCusker, Altepeter, Calkins, Gerrits, Lita, Miller,
    Shalm, Zhang, Nam, Brunner, Lim, Gisin, and Kwiat, "Exploring the limits of
    quantum nonlocality with entangled photons", Physical Review X 5, 041052
    (2015), experimentally study inequalities whose bounds hold for local
    correlations supplemented by one PR box and observe violations by weakly
    entangled two-qubit states. These inequalities witness more than one PR box
    in their scenarios, but they are not inequalities valid against all
    two-PR-box wirings and therefore do not solve the present two-PR target.
  </known>

  <refs>
    <ref>Open Quantum Problems, Bell inequalities: many questions, a few answers, Problem 32, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/bell-inequalities-many-questions-a-few-answers, used for subquestion K and the listed resource-simulation context.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is derived from Problem 32, Bell inequalities: many questions, a few answers, listed in Quantum foundations.</ref>
    <ref>N. Gisin, Bell inequalities: many questions, a few answers, in Essays in Honour of Abner Shimony, Western Ontario Series in Philosophy of Science, Springer, 2009, pp. 125-140, DOI: 10.1007/978-1-4020-9107-0_9, arXiv:quant-ph/0702021, Section III.3, used for the two-PR-box resource question and its relation to two-bit simulation.</ref>
    <ref>N. J. Cerf, N. Gisin, S. Massar, and S. Popescu, Simulating maximal quantum entanglement without communication, Physical Review Letters 94, 220403 (2005), DOI: 10.1103/PhysRevLett.94.220403, used for one-PR-box exact simulation of maximally entangled two-qubit projective correlations.</ref>
    <ref>N. Brunner, N. Gisin, and V. Scarani, Entanglement and non-locality are different resources, New Journal of Physics 7, 88 (2005), DOI: 10.1088/1367-2630/7/1/088, arXiv:quant-ph/0412109, used for one-PR-box lower bounds for weakly entangled two-qubit states.</ref>
    <ref>N. Brunner, V. Scarani, and N. Gisin, Bell-type inequalities for non-local resources, Journal of Mathematical Physics 47, 112101 (2006), DOI: 10.1063/1.2352857, arXiv:quant-ph/0603094, used for resource inequalities for one generalized nonlocal machine.</ref>
    <ref>O. Regev and B. Toner, Simulating quantum correlations with finite communication, SIAM Journal on Computing 39(4), 1562-1580 (2009), DOI: 10.1137/080723909, arXiv:0708.0827, used for the two-classical-bit simulation of bipartite projective-measurement correlation functions motivating Gisin's two-PR-box question.</ref>
    <ref>B. F. Toner and D. Bacon, Communication cost of simulating Bell correlations, Physical Review Letters 91, 187904 (2003), DOI: 10.1103/PhysRevLett.91.187904, arXiv:quant-ph/0304076, used for the two-bit two-qubit projective-measurement simulation motivation.</ref>
    <ref>N. Brunner, N. Gisin, S. Popescu, and V. Scarani, Simulation of partial entanglement with nonsignaling resources, Physical Review A 78, 052111 (2008), DOI: 10.1103/PhysRevA.78.052111, arXiv:0803.2359, used for the four-PR-box plus continuous-input Millionaire-box protocol and the finite-resource caveat.</ref>
    <ref>B. G. Christensen, K. T. McCusker, J. B. Altepeter, B. Calkins, T. Gerrits, A. E. Lita, A. Miller, L. K. Shalm, Y. Zhang, S. W. Nam, N. Brunner, C. C. W. Lim, N. Gisin, and P. G. Kwiat, Exploring the limits of quantum nonlocality with entangled photons, Physical Review X 5, 041052 (2015), DOI: 10.1103/PhysRevX.5.041052, used for one-PR-box resource-inequality progress distinct from the two-PR-box target.</ref>
  </refs>

  <ask>
    Prove or disprove the claim for the common-order sequential set
    \(\mathcal P_2(m_A,m_B)\) as defined above. A proof must give finite input
    numbers \(m_A,m_B\), an explicit Bell functional \(\ell\), a certified
    value of
    \(\beta_2=\max_{\mathcal P_2(m_A,m_B)}\ell\), a parameter
    \(0\lt\theta\lt\pi/4\), and explicit projective qubit measurements with a
    rigorous strict gap \(\ell(q_\theta)-\beta_2\gt 0\).

    A disproof must prove that for every finite binary-output scenario, every
    nonmaximally entangled two-qubit pure state, and every choice of local
    projective measurements, the resulting behavior lies in
    \(\mathcal P_2(m_A,m_B)\). It would suffice to give a universal
    two-PR-box local-wiring protocol for all projective measurements on all
    pure entangled two-qubit states. A one-PR-box lower bound, an inequality
    valid only for local-plus-one-PR correlations, a fixed small-scenario
    numerical search without certified two-PR bounds, a protocol using
    classical communication, a protocol using a continuous-input box, or a
    protocol using more than two PR boxes is progress unless it implies the
    stated two-PR separation or its negation.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
