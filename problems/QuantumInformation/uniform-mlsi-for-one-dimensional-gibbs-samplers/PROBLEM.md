# QTD026 Uniform Modified Log-Sobolev Inequality for 1D Quantum Gibbs Samplers

```xml
<problem id="QTD026">
  <status>open</status>
  <name>Uniform Modified Log-Sobolev Inequality for 1D Quantum Gibbs Samplers</name>

  <keys>modified log-sobolev inequality, MLSI, log-sobolev inequality, quantum log-sobolev inequality, entropy decay, entropy decay constant, entropy production, relative entropy contraction, quantum Gibbs sampler, Lindbladian, quantum Markov semigroup, Davies generator, Davies semigroup, KMS detailed balance, KMS-detailed-balanced Lindbladian, exact detailed balance, spectral gap, uniform spectral gap, system-size independent spectral gap, rapid mixing, mixing time, thermalization time, one-dimensional spin chain, 1D spin chain, one-dimensional quantum lattice, non-commuting Hamiltonian, noncommutative quantum Gibbs sampler, quasi-local jump operators, primitive quantum Markov semigroup, Poincare inequality, Dirichlet form, Bergamaschi-Chen, Chen-Kastoryano-Gilyen, CKG23, quantum thermal state preparation, dissipative state preparation, purified Gibbs state, thermofield double state, adiabatic state preparation, complete MLSI, CMLSI, tensor-stabilized log-Sobolev, hypercontractivity, Kastoryano-Temme, commuting Hamiltonian gap implies MLSI, Bardet-Capel-Gao-Lucia-Perez-Garcia-Rouze, Kochanowski-Alhambra-Capel-Rouze, spectral conditional expectation not completely positive, uniform-mlsi-for-one-dimensional-gibbs-samplers</keys>

  <claim>
    Fix \(q\in\mathbb N_+\) (local dimension \(2^q\)) and \(\beta\in(0,\infty)\)
    (inverse temperature). For every \(n\in\mathbb N_+\) and every
    1D nearest-neighbor Hamiltonian \(H=\sum_{b=1}^{n-1}H_{b,b+1}\) on \(n\)
    qudits of local dimension \(2^q\), with open boundary conditions,
    \(H_{b,b+1}=H_{b,b+1}^*\), and \(\|H_{b,b+1}\|\le1\) for every \(b\)
    (\(H\) need not be commuting or translation invariant), let
    \(\mathcal L_{H,\beta}\) be the Chen-Kastoryano-Gilyén quantum Gibbs
    sampler used by Bergamaschi and Chen, Eq.\ (1.4) (defined below), and let
    \(\alpha(\mathcal L_{H,\beta})\) be its modified log-Sobolev constant
    (defined below).

    Claim: there exists a function \(\alpha_0:(0,\infty)\times\mathbb
    N_+\to(0,\infty)\) such that, for every \(n\in\mathbb N_+\) and every
    \(H\) in the class above,
    \[
      \alpha(\mathcal L_{H,\beta})\ \ge\ \alpha_0(\beta,q),
    \]
    where \(\alpha_0(\beta,q)\) depends only on \(\beta\) and \(q\) -- not on
    \(n\) or on the particular choice of \(H\).

    This is the modified-log-Sobolev-inequality (MLSI) analogue, for exactly
    the Hamiltonian class and temperature range of Bergamaschi and Chen,
    Theorem I.1 (spectral gap; restated in the known section below), of that
    already-proved spectral gap. It is stated for the ordinary
    (non-complete, "1-") MLSI constant of Kastoryano and Temme, which is the
    notion Bergamaschi and Chen's own discussion refers to. The complete
    (tensor-stabilized) MLSI constant -- the infimum in the definition below
    taken also over an arbitrary ancilla system and an arbitrary bipartite
    state -- is a generally stronger and formally different property that
    this entry does not address; a claim strength ambiguity is recorded
    here rather than resolved by fiat.
  </claim>

  <def>
    Sites, Hamiltonians, Gibbs state. Fix \(q\in\mathbb N_+\) and
    \(n\in\mathbb N_+\); let \(\mathcal H:=(\mathbb C^{2^q})^{\otimes n}\) be
    the Hilbert space of \(n\) qudits of local dimension \(2^q\). A 1D
    nearest-neighbor Hamiltonian on \(\mathcal H\) with open boundary
    conditions is \(H=\sum_{b=1}^{n-1}H_{b,b+1}\), where each
    \(H_{b,b+1}=H_{b,b+1}^*\) acts on sites \(b,b+1\) (as the identity
    elsewhere) with operator norm \(\|H_{b,b+1}\|\le1\); the terms need not
    commute and \(H\) need not be translation invariant (Bergamaschi and
    Chen, Eq.\ (1.2)). For \(\beta\in(0,\infty)\), the Gibbs state is
    \(\rho_{H,\beta}:=e^{-\beta H}/\mathrm{Tr}[e^{-\beta H}]\) (Eq.\ (1.3)), a
    full-rank density operator on \(\mathcal H\).

    The Lindbladian \(\mathcal L_{H,\beta}\). Bergamaschi and Chen, Eq.\
    (1.4) and Section II.E, equip \(\mathcal H\) with the quantum Gibbs
    sampler of Chen, Kastoryano, and Gilyén (arXiv:2311.09207):
    \[
      \mathcal L_{H,\beta}=\sum_{a\in S^1_{[n]}}\mathcal L_a ,
    \]
    where \(S^1_{[n]}\) indexes the family of single-site Pauli-type "jump"
    operators, each \(\mathcal L_a\) is quasi-local with
    \(\|\mathcal L_a\|_{1\to1}=O(1)\) independent of \(n\), and
    \(\mathcal L_a[\rho_{H,\beta}]=0\) for every \(a\). Here \(\mathcal
    L_{H,\beta}\) is written in the Schrödinger picture, i.e.\ it acts on
    states and \(e^{t\mathcal L_{H,\beta}}[\omega]\) is the state at time
    \(t\) of an initial state \(\omega\). \(\mathcal L_{H,\beta}\) generates
    a completely positive, trace-preserving one-parameter semigroup
    \((e^{t\mathcal L_{H,\beta}})_{t\ge0}\) on density operators, is exactly
    Kubo-Martin-Schwinger (KMS) \(\rho_{H,\beta}\)-detailed-balanced, and
    satisfies \(\mathcal L_{H,\beta}[\rho_{H,\beta}]=0\) exactly
    (Bergamaschi and Chen, Theorem II.1, citing Chen, Kastoryano, and
    Gilyén). This entry cites the exact construction of each \(\mathcal
    L_a\) by these defining properties rather than reproducing it; a proof
    attempt needs the construction of Chen, Kastoryano, and Gilyén itself,
    not merely these properties.

    Spectral gap (background only; Bergamaschi and Chen, Definition II.3).
    For a KMS-\(\rho\)-detailed-balanced Lindbladian \(\mathcal L\),
    \(\lambda_{gap}(\mathcal L)\) is defined there as the infimum, over
    observables \(O\), of the ratio of the Dirichlet form
    \(\langle O,-\mathcal L^\dagger[O]\rangle_\rho\) to the variance
    \(\|O-\mathbb 1\cdot\mathrm{Tr}[O\rho]\|_\rho^2\), with respect to the
    KMS inner product \(\langle\cdot,\cdot\rangle_\rho\) associated with
    \(\rho\) (Bergamaschi and Chen, Definition II.2). This entry does not
    reproduce that formula in full; it is cited only as background for the
    known section below and is not itself an object the claim above is
    stated in terms of.

    Quantum relative entropy and entropy production. For density operators
    \(\omega,\sigma\) on \(\mathcal H\) with \(\mathrm{supp}(\omega)\subseteq
    \mathrm{supp}(\sigma)\), \(D(\omega\|\sigma):=\mathrm{Tr}[\omega(\ln
    \omega-\ln\sigma)]\ge0\) (any fixed logarithm base may be used instead
    of \(\ln\); the ratio defining \(\alpha(\mathcal L)\) below is
    base-independent). Because \((e^{t\mathcal L})_{t\ge0}\) is a
    completely positive trace-preserving semigroup fixing \(\rho\), the
    data-processing inequality gives that \(t\mapsto D(e^{t\mathcal
    L}[\omega]\|\rho)\) is non-increasing for every state \(\omega\). Its
    initial rate of decrease is the entropy production functional
    \[
      \mathrm{EP}_{\mathcal L}(\omega):=-\frac{d}{dt}\Big|_{t=0^+}
      D\big(e^{t\mathcal L}[\omega]\,\|\,\rho\big)
      =-\mathrm{Tr}\big[\mathcal L[\omega]\,(\ln\omega-\ln\rho)\big]\ \ge0,
    \]
    defined for full-rank \(\omega\) (extended to general \(\omega\) by the
    usual closure/limiting argument).

    Modified log-Sobolev (MLSI) constant (Kastoryano and Temme,
    arXiv:1207.3261 / J. Math. Phys. 54, 052202 (2013)). For a
    KMS-\(\rho\)-detailed-balanced Lindbladian \(\mathcal L\) on
    \(\mathcal H\),
    \[
      \alpha(\mathcal L):=\inf\Big\{\ \frac{\mathrm{EP}_{\mathcal
      L}(\omega)}{D(\omega\|\rho)}\ :\ \omega\ \text{a full-rank density
      operator on }\mathcal H,\ \omega\ne\rho\ \Big\}.
    \]
    \(\mathcal L\) satisfies an \(\alpha\)-MLSI if \(\alpha(\mathcal
    L)\ge\alpha\gt0\); by Grönwall's inequality this is equivalent to
    \(D(e^{t\mathcal L}[\omega]\|\rho)\le e^{-\alpha t}D(\omega\|\rho)\) for
    every \(t\ge0\) and every state \(\omega\) on \(\mathcal H\). This is
    the ordinary ("1-", non-complete) MLSI constant: no ancilla or
    bipartite state is involved. The complete MLSI constant
    \(\alpha_{cb}(\mathcal L)\le\alpha(\mathcal L)\) instead takes the
    infimum also over an arbitrary ancilla Hilbert space \(\mathcal
    H_R\) and full-rank bipartite states \(\omega\) on \(\mathcal
    H\otimes\mathcal H_R\), with \(\mathcal L\) acting only on the
    \(\mathcal H\) factor; it is a distinct, generally strictly stronger
    quantity not addressed by the claim above.
  </def>

  <known>
    Bergamaschi and Chen, "Fast Mixing of Quantum Spin Chains at All
    Temperatures" (STOC 2026; arXiv:2510.08533), Theorem I.1, prove: for
    every \(n\in\mathbb N_+\) and every \(H\) in the class defined above,
    \(\lambda_{gap}(\mathcal L_{H,\beta})\ge\lambda_0(\beta,q)\gt0\), where
    \(\lambda_0\) depends only on \(\beta\) and \(q\), not on \(n\) or on
    \(H\) -- exactly the claim above with \(\alpha(\mathcal L_{H,\beta})\)
    replaced by \(\lambda_{gap}(\mathcal L_{H,\beta})\). Immediately after
    stating this theorem, the source draws only the following consequence
    for mixing: convergence to trace-distance error \(\epsilon\) from an
    arbitrary initial state is guaranteed only in time \(t_{mix}(\epsilon)=
    \lambda_{gap}^{-1}\cdot O\big(n+\log\tfrac1\epsilon\big)\) -- a bound
    with an explicit term linear in \(n\). By contrast, an
    \(n\)-independent MLSI constant \(\alpha_0(\beta,q)\) as in the claim
    above would, by a standard argument relating MLSI to mixing time (see
    Kastoryano and Temme, cited below), give trace-distance convergence in
    time logarithmic in \(n\) from an arbitrary initial state -- the "true
    rapid mixing" the claim above is about. A positive spectral gap is thus
    known already, but by itself gives only the weaker, size-dependent
    mixing guarantee just stated.

    Bergamaschi and Chen, Section I.D ("Discussion"), subsection "Mixing
    times beyond spectral gaps", state explicitly that the analogous
    all-temperature statement for their own family is open: "An intriguing
    open question is to identify if the [Chen-Kastoryano-Gilyén] Lindbladian
    family similarly admits such an implication [gap \(\Rightarrow\) MLSI].
    Unfortunately, it remains unclear to what extent the techniques of
    [Bardet-Capel-Gao-Lucia-Pérez-García-Rouzé, Kochanowski-Alhambra-Capel-
    Rouzé] are applicable in our context, since our spectral conditional
    expectation is not completely positive." This is the seed of the claim
    above, in the source's own words, together with a stated technical
    obstruction (loss of complete positivity of the spectral conditional
    expectation used in their proof of Theorem I.1) to directly transferring
    the commuting-case route.

    The two commuting-case precedents Bergamaschi and Chen cite for
    "gap \(\Rightarrow\) MLSI" differ in exactly how uniform the resulting
    constant is, which matters for the claim above:

    (i) Bardet, Capel, Gao, Lucia, Pérez-García, and Rouzé, "Entropy Decay
    for Davies Semigroups of a One Dimensional Quantum Lattice",
    Communications in Mathematical Physics 405, article 42 (2024)
    (arXiv:2112.00601), prove, for finite-range translation-invariant
    commuting Hamiltonians on a spin chain, that the Davies semigroup's
    relative-entropy decay is exponential "with an exponent that scales
    logarithmically with the length of the chain" -- i.e.\ a decay rate
    that degrades (slowly, as a function only of \(n\)) rather than one
    that is literally \(n\)-independent; the authors describe this as
    matching the classical Holley-Stroock result "up to a logarithmic
    overhead."

    (ii) Kochanowski, Alhambra, Capel, and Rouzé, "Rapid thermalization of
    dissipative many-body dynamics of commuting Hamiltonians",
    Communications in Mathematical Physics 406(8), article 176 (2025)
    (arXiv:2404.16780), prove the literally uniform statement: for 1D
    (nearest-neighbor) commuting Hamiltonians, they "prove rapid
    thermalization with a system size independent decay rate only from a
    positive gap in the generator" -- i.e.\ exactly the gap-to-MLSI
    implication of the claim above, with genuine \(n\)-independence,
    specialized to the commuting case. This is the closest existing
    precedent, in generality of conclusion, for what the claim above asks
    for the non-commuting family \(\mathcal L_{H,\beta}\).

    Kastoryano and Temme, "Quantum logarithmic Sobolev inequalities and
    rapid mixing", Journal of Mathematical Physics 54, 052202 (2013)
    (arXiv:1207.3261), introduce \(\alpha(\mathcal L)\) as defined above and
    prove a general upper bound on the MLSI constant in terms of the
    spectral gap of the same generator. Consequently the spectral gap of
    Theorem I.1 is necessary background but does not by itself yield a
    lower bound on \(\alpha(\mathcal L_{H,\beta})\); establishing the claim
    above requires a new argument beyond Theorem I.1, matching precisely
    what Bergamaschi and Chen flag as open.

    This entry does not search the post-2026-01-23 literature (the date of
    the arXiv v2 revision of Bergamaschi and Chen read for this entry) for
    work that may have resolved this question after that revision; residual
    doubt on this point is recorded here rather than investigated further,
    per this entry's verification scope.
  </known>

  <refs>
    <ref>T. Bergamaschi and C.-F. Chen, Fast Mixing of Quantum Spin Chains at All Temperatures, Proceedings of the 58th Annual ACM Symposium on Theory of Computing (STOC 2026), pp. 835-844, DOI: 10.1145/3798129.3800798, arXiv:2510.08533v2 (revised 23 Jan 2026), used for Eq. (1.2)-(1.4) (Hamiltonian, Gibbs state, Lindbladian), Theorem I.1 (spectral gap) and its stated mixing-time consequence, Theorem II.1 and Definitions II.2-II.3 (detailed balance, KMS inner product, spectral gap), and Section I.D, "Mixing times beyond spectral gaps" (explicit statement that the MLSI analogue of Theorem I.1 is open, and why).</ref>
    <ref>C.-F. Chen, M. J. Kastoryano, and A. Gilyén, An efficient and exact noncommutative quantum Gibbs sampler, arXiv:2311.09207 (2023), used for the exact construction of the jump operators \(\mathcal L_a\) making up \(\mathcal L_{H,\beta}\), cited by Bergamaschi and Chen as the source of Eq. (1.4).</ref>
    <ref>I. Bardet, A. Capel, L. Gao, A. Lucia, D. Pérez-García, and C. Rouzé, Entropy Decay for Davies Semigroups of a One Dimensional Quantum Lattice, Communications in Mathematical Physics 405, article 42 (2024), arXiv:2112.00601, used for the commuting-case, translation-invariant precedent with a logarithmically-degrading (not literally uniform) MLSI/entropy-decay exponent.</ref>
    <ref>J. Kochanowski, A. M. Alhambra, A. Capel, and C. Rouzé, Rapid thermalization of dissipative many-body dynamics of commuting Hamiltonians, Communications in Mathematical Physics 406(8), article 176 (2025), DOI: 10.1007/s00220-025-05353-y, arXiv:2404.16780, used for the commuting-case, 1D, literally system-size-independent gap-to-MLSI implication that the claim above generalizes to the non-commuting Chen-Kastoryano-Gilyén family.</ref>
    <ref>M. J. Kastoryano and K. Temme, Quantum logarithmic Sobolev inequalities and rapid mixing, Journal of Mathematical Physics 54, 052202 (2013), DOI: 10.1063/1.4804995, arXiv:1207.3261, used for the definitions of the entropy production functional and the (non-complete) modified log-Sobolev constant \(\alpha(\mathcal L)\), and for the general spectral-gap upper bound on \(\alpha(\mathcal L)\).</ref>
  </refs>

  <ask>
    Determine, for every \(q\in\mathbb N_+\) and every \(\beta\in(0,\infty)\),
    whether \(\alpha_0(\beta,q):=\inf_{n\in\mathbb N_+}\ \inf_{H}\
    \alpha(\mathcal L_{H,\beta})\) is strictly positive, where the second
    infimum ranges over 1D nearest-neighbor Hamiltonians \(H\) on \(n\)
    qudits of local dimension \(2^q\) with open boundary conditions and
    \(\|H_{b,b+1}\|\le1\).

    A proof must exhibit, for every \(q\in\mathbb N_+\) and every
    \(\beta\in(0,\infty)\), an explicit \(\alpha_0(\beta,q)\gt0\) and show
    \(\alpha(\mathcal L_{H,\beta})\ge\alpha_0(\beta,q)\) for every
    \(n\in\mathbb N_+\) and every \(H\) in the stated class; it may freely
    use Theorem I.1 of Bergamaschi and Chen (the spectral gap) as an input.
    A proof restricted to a single \((\beta,q)\) pair, to a proper subclass
    of the stated Hamiltonians (e.g.\ commuting or translation-invariant
    \(H\)), or to a bounded temperature range (e.g.\ sufficiently high
    temperature only) does not resolve the claim as stated, since the claim
    is universally quantified over \(q\in\mathbb N_+\) and
    \(\beta\in(0,\infty)\).

    A disproof must exhibit a single \((\beta,q)\in(0,\infty)\times\mathbb
    N_+\) and a sequence \((H_n)_{n\in\mathbb N_+}\) of Hamiltonians, each in
    the stated class on \(n\) qudits of local dimension \(2^q\), such that
    \(\alpha(\mathcal L_{H_n,\beta})\to0\) as \(n\to\infty\); equivalently,
    for every candidate \(c\gt0\) there is some \(n\) and some \(H\) in the
    class with \(\alpha(\mathcal L_{H,\beta})\lt c\). A single such
    \((\beta,q)\) and sequence disproves the claim as stated, since the
    claim asserts existence of \(\alpha_0(\beta,q)\) for every \((\beta,q)\)
    individually; no argument about other \((\beta,q)\) pairs is needed
    once one pair is disproved.

    The following count as progress, not as resolving the claim: (i) an
    MLSI-type bound \(\alpha(\mathcal L_{H,\beta})\ge c(\beta,q)/
    \mathrm{polylog}(n)\) with an explicit \(n\)-dependence (matching the
    Bardet-Capel-Gao-Lucia-Pérez-García-Rouzé precedent's logarithmic
    overhead rather than the literally uniform claim above); (ii) a proof
    restricted to commuting, translation-invariant, or high-temperature-only
    \(H\); (iii) a proof of a uniform lower bound on the complete MLSI
    constant \(\alpha_{cb}(\mathcal L_{H,\beta})\) restricted to a subclass
    as in (ii), rather than on the full stated class of \(H\) -- note that
    since \(\alpha_{cb}\le\alpha\) always, a uniform lower bound on
    \(\alpha_{cb}(\mathcal L_{H,\beta})\) over the entire stated class of
    \(H\) (not merely a subclass) would resolve the claim above a fortiori,
    without further argument; (iv)
    numerical evidence of non-degrading entropy decay for finite \(n\)
    without an analytic, \(n\)-uniform bound.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
