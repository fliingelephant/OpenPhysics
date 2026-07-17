# QTD016 Vanishing Minimal Thermodynamic Uncertainty Product in the Large-Dimension Limit

```xml
<problem id="QTD016">
  <status>open</status>
  <name>Vanishing Minimal Thermodynamic Uncertainty Product in the Large-Dimension Limit</name>
  <keys>thermodynamic uncertainty relation TUR quantum thermodynamic uncertainty relation coherently driven open quantum systems Lindblad GKSL master equation Markovian open quantum system steady-state current current fluctuations entropy production rate dissipation Fano factor uncertainty product precision-dissipation tradeoff precision dissipation trade-off vanishing precision cost thermodynamic cost of precision arbitrarily small uncertainty product large dimension limit large-N limit N-level quantum system quantum thermal machine quantum heat engine precision quantum coherence resource thermodynamics Menczel Loisa Brandner Flindt conjecture minimal uncertainty product dimension dependence full counting statistics counting field cumulant generating function classical bound violation</keys>

  <claim>
    For an integer $N\ge2$, let $\mathcal P^N_{min}$ denote the infimum of
    the thermodynamic uncertainty product $\mathcal P_\mu$ (defined below)
    over every admissible coherently driven $N$-level configuration and
    every dissipation channel $\mu$ therein. Then
    \[
    \lim_{N\to\infty}\mathcal P^N_{min}=0.
    \]
    Since $\mathcal P^N_{min}\ge0$ for every $N$ and $\mathcal P^N_{min}$ is
    non-increasing in $N$ (known, see below), this limit exists and equals
    $\inf_{N\ge2}\mathcal P^N_{min}$, so the claim is equivalent to: for
    every $\epsilon\gt0$ there exists an integer $N$ and an admissible
    $N$-level configuration with a dissipation channel $\mu$ satisfying
    $\mathcal P_\mu\lt\epsilon$.
  </claim>

  <def>
    Coherently driven $N$-level configuration. Fix an integer $N\ge1$ and a
    finite, non-empty index set $M$ of "dissipation channels." A
    configuration is a tuple $\bigl(H_0,X,(\gamma_\mu,L_\mu,n_\mu)_{\mu\in
    M}\bigr)$ where $H_0,X\in\mathbb C^{N\times N}$ are Hermitian (the
    "free Hamiltonian" and a "driving contribution"; $H:=H_0+X$ is the
    effective Hamiltonian), and for each $\mu\in M$: $\gamma_\mu\in
    (0,\infty)$ is a rate, $L_\mu\in\mathbb C^{N\times N}$ is a Lindblad
    (jump) operator satisfying
    \[
    [H_0,L_\mu]=-\Delta E_\mu L_\mu\quad\text{for some }\Delta E_\mu\ge0,
    \]
    and $n_\mu\in(0,\infty)$ is a Bose-Einstein-type occupation factor. The
    density operator $\rho_t$ on $\mathbb C^N$ evolves by the GKSL master
    equation
    \[
    \partial_t\rho_t=\frac1{i\hbar}[H,\rho_t]+\sum_{\mu\in M}\gamma_\mu
    (n_\mu+1)\Bigl(L_\mu\rho_tL_\mu^\dagger-\tfrac12\{L_\mu^\dagger
    L_\mu,\rho_t\}\Bigr)+\sum_{\mu\in M}\gamma_\mu n_\mu\Bigl(L_\mu^\dagger
    \rho_tL_\mu-\tfrac12\{L_\mu L_\mu^\dagger,\rho_t\}\Bigr).
    \]
    The configuration is admissible if this generator has a unique
    stationary state $\rho_\infty$ (i.e. $\rho_t\to\rho_\infty$ as
    $t\to\infty$ for every initial $\rho_0$); it has dimension $N$ whenever
    $H_0,X\in\mathbb C^{N\times N}$. (The claim is stated for $N\ge2$,
    matching the source's own analysis, which begins its systematic
    dimension-by-dimension study at two-level systems, Section 3.2.)

    Currents and entropy production. For an admissible configuration, the
    particle current through channel $\mu$ is
    \[
    j_\mu:=\gamma_\mu(n_\mu+1)\operatorname{tr}[L_\mu\rho_\infty
    L_\mu^\dagger]-\gamma_\mu n_\mu\operatorname{tr}[L_\mu^\dagger\rho_\infty
    L_\mu],
    \]
    and the total entropy production rate is
    \[
    \sigma:=\sum_{\mu\in M}j_\mu\log\!\Bigl[\frac{n_\mu+1}{n_\mu}\Bigr]\ge0
    \]
    (Boltzmann's constant is set to $1$ throughout, following the source).

    Current fluctuations. Introduce one counting field $s_\mu$ per channel
    and the tilted generator
    \[
    \mathsf L[s]\rho:=\frac1{i\hbar}[H,\rho]+\sum_{\mu\in M}\gamma_\mu
    (n_\mu+1)\Bigl(e^{s_\mu}L_\mu\rho L_\mu^\dagger-\tfrac12\{L_\mu^\dagger
    L_\mu,\rho\}\Bigr)+\sum_{\mu\in M}\gamma_\mu n_\mu\Bigl(e^{-s_\mu}
    L_\mu^\dagger\rho L_\mu-\tfrac12\{L_\mu L_\mu^\dagger,\rho\}\Bigr),
    \]
    with characteristic polynomial $P[s,\lambda]:=\det[\mathsf L[s]-\lambda
    \mathbb 1]$ (writing $s=(s_\mu)_{\mu\in M}$ and $\partial_\mu:=\partial/
    \partial s_\mu$). Then
    \[
    j_\mu=-\frac{\partial_\mu P[s,\lambda]}{\partial_\lambda P[s,\lambda]}
    \bigg|_{s,\lambda=0},\qquad
    d_\mu=\frac{-1}{\partial_\lambda P[s,\lambda]}\Bigl(\partial_\mu^2+2j_\mu
    \partial_\lambda\partial_\mu+j_\mu^2\partial_\lambda^2\Bigr)P[s,\lambda]
    \bigg|_{s,\lambda=0}
    \]
    reproduce the current above and define its long-time fluctuation
    $d_\mu\ge0$ (a scaled variance of the channel-$\mu$ particle count).

    Uncertainty product. For a channel $\mu$ with $j_\mu\neq0$,
    \[
    \mathcal P_\mu:=\frac{\sigma d_\mu}{j_\mu^2}\ \ (\ge0).
    \]
    $\mathcal P^N_{min}$ is the infimum of $\mathcal P_\mu$ over every
    admissible $N$-level configuration and every $\mu\in M$ with
    $j_\mu\neq0$ in that configuration.
  </def>

  <known>
    Classical bound (context, not part of the claim). For classical,
    time-homogeneous Markov-jump processes obeying local detailed balance,
    the analogous uncertainty product obeys $\mathcal P\ge2$ (Menczel,
    Loisa, Brandner, and Flindt, Eqs. (1)-(2), restating the classical
    thermodynamic uncertainty relation of the prior literature cited there;
    this entry does not independently verify that prior literature). The
    claim above concerns approach to $0$, strictly below this classical
    floor.

    Monotonicity in $N$ (proven). Menczel, Loisa, Brandner, and Flindt,
    Section 3.1 (the paragraph following Eq. (35)), prove $\mathcal
    P^{N+1}_{min}\le\mathcal P^N_{min}$ for every $N\ge1$: given an
    $N$-level configuration attaining $\mathcal P_\mu=\mathcal P^N_{min}$
    for some channel $\mu$, they adjoin one decoupled level together with
    an auxiliary channel of occupation factor $n_\nu\to0^+$, and show the
    resulting $(N+1)$-level system has a stationary state supported
    entirely on the original $N$-dimensional subspace, so it reproduces
    $\mathcal P_\mu=\mathcal P^N_{min}$. Combined with $\mathcal
    P^N_{min}\ge0$ (immediate from $\sigma\ge0$, $d_\mu\ge0$), this
    guarantees $\lim_{N\to\infty}\mathcal P^N_{min}$ exists and equals
    $\inf_{N\ge2}\mathcal P^N_{min}$; the open question is only whether
    this existing limit equals $0$ or a strictly positive number.

    Numerically established minimal values. Using a differential-evolution
    global-optimization algorithm, Menczel, Loisa, Brandner, and Flindt
    find $\mathcal P^2_{min}\approx1.2459$ (Eq. (36); for $N=2$ this follows
    an explicit symmetry reduction to a small parameter family, Section
    3.2), $\mathcal P^3_{min}\approx0.47242$ (Eq. (43), Section 3.3), and
    $\mathcal P^4_{min}\approx0.2625$ (Eq. (49), Section 3.4, summarized in
    Section 4's Eq. (50)) -- the three-point decreasing trend
    $1.25,\,0.47,\,0.26$ that motivates the conjecture. For $N=3,4$ the
    search is explicitly restricted to configurations with at most two
    dissipation channels, a restriction the authors justify only by the
    empirical pattern observed at $N=2$ (Section 3.3: "we found that the
    addition of extra dissipation channels generally increases the
    uncertainty products"), not by a general proof that more channels
    cannot help at every $N$; the quoted $N=3,4$ values are therefore
    minima within that restricted search, not values with a closed-form
    global-optimality certificate.

    Coherence is necessary, and high dimension alone is not sufficient. In
    the semiclassical (weak-driving) limit of the two-level example,
    $\mathcal P=(2n+1)\log[(n+1)/n]+O(\Omega^2)\gt2$ (Eq. (13)), so beating
    the classical bound requires genuinely coherent driving. Conversely,
    the driven-cavity example (Section 2.2, a bosonic mode with an
    intrinsically infinite-dimensional Hilbert space) gives $\mathcal
    P=(2n+1)\log[(n+1)/n]\gt2$ strictly for every choice of its parameters
    (Eq. (20); its long-time current cumulants coincide exactly with those
    of a classical biased random walk, Eq. (21)) -- an explicit
    infinite-dimensional family that never beats the classical bound at
    all, let alone approaches zero. Menczel, Loisa, Brandner, and Flindt
    state this as an explicit caveat on their own conjecture, in the
    sentence immediately following it (Section 4): a complicated,
    specifically coherent structure -- as in the "clock-like" optimal
    two-, three-, and four-level configurations of Sections 3.2-3.4 -- is
    needed in every dimension checked so far, not high dimension by itself.

    The conjecture is stated, not proved, in the source. Two paragraphs
    after Eq. (50), in Section 4 ("Conclusion and Perspectives"), Menczel,
    Loisa, Brandner, and Flindt write that they conjecture the lower bound
    "approaches zero as the dimension $N$ becomes large." No proof,
    partial proof, or numerical evidence beyond $N\le4$ is given anywhere
    in the paper, and this sentence is part of the paper's own concluding
    discussion, with no later section revisiting or resolving it.
  </known>

  <refs>
    <ref>Paul Menczel, Eetu Loisa, Kay Brandner, and Christian Flindt,
    "Thermodynamic uncertainty relations for coherently driven open quantum
    systems," Journal of Physics A: Mathematical and Theoretical 54, 314002
    (2021). DOI: 10.1088/1751-8121/ac0c8f. arXiv:2104.12712v2 (21 June
    2021). Used: Eqs. (1)-(2) (uncertainty product and classical bound);
    Eq. (13) (semiclassical limit of the two-level example); Section 2.2,
    Eqs. (19)-(21) (driven-cavity example); Section 3.1, Eqs. (28)-(35)
    (general $N$-level Lindblad setup; definitions of $j_\mu$, $\sigma$,
    $d_\mu$, $\mathcal P_\mu$; and the proof that $\mathcal P^N_{min}$ is
    non-increasing in $N$); Section 3.2, Eq. (36) ($N=2$); Section 3.3, Eq.
    (43) ($N=3$); Section 3.4 and Section 4, Eqs. (49)-(50) ($N=4$ and
    summary); Section 4, "Conclusion and Perspectives," the paragraph
    beginning "We thus conjecture," two paragraphs after Eq. (50) (the
    conjecture).</ref>
  </refs>

  <ask>
    Settle whether $\lim_{N\to\infty}\mathcal P^N_{min}=0$.

    A proof must exhibit, for every $\epsilon\gt0$, an integer $N$ and an
    admissible $N$-level configuration (as defined above) with a channel
    $\mu$ satisfying $\mathcal P_\mu\lt\epsilon$ -- equivalently, an
    explicit sequence of configurations along some subsequence of
    dimensions $N_k\to\infty$ with $\mathcal P^{N_k}_{min}\to0$; since
    $\mathcal P^N_{min}$ is already known to be non-increasing in $N$,
    convergence along any subsequence suffices and need not be exhibited
    at every $N$.

    A disproof must exhibit a constant $c\gt0$ together with a proof that
    $\mathcal P^N_{min}\ge c$ for every $N\ge2$ -- a uniform lower bound,
    valid across every dimension and every admissible configuration in
    that dimension, strictly separated from zero. Verifying $\mathcal
    P^N_{min}\gt0$ for any finite list of dimensions (e.g. reproducing or
    extending the known $N=2,3,4$ values, or computing $N=5,6,\dots$) does
    not disprove the claim, which concerns only the $N\to\infty$ limit.

    The following count as progress, not as resolving the claim: (i) a
    closed-form analytic proof, in place of numerical optimization, that
    the known $N=3$ or $N=4$ values (Eqs. (43),(49)) are the true global
    minima among all configurations of that dimension (with no
    restriction on the number of channels), without addressing
    $N\to\infty$; (ii) extending the numerical search to further fixed
    dimensions, with or without the at-most-two-channel restriction of
    Sections 3.3-3.4, without an argument covering every $N$; (iii) an
    explicit sequence of configurations giving an upper bound on $\mathcal
    P^N_{min}$ that is shown, or only observed numerically, to decrease
    with $N$ without a proof that its limit is exactly $0$ rather than
    some unidentified strictly positive constant; (iv) a proof or
    disproof restricted to a strict subclass of admissible configurations
    (e.g. a fixed single-dissipation-channel family, or the driven-cavity
    ladder alone, which by itself gives $\mathcal P\gt2$ in every
    dimension and does not bound $\mathcal P^N_{min}$, defined by
    optimizing over all admissible configurations); (v) an asymptotic rate
    bound such as $\mathcal P^N_{min}=\Theta(N^{-\alpha})$ for some
    $\alpha\gt0$ asserted without a rigorous proof that the implied limit
    is exactly zero.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
