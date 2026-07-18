# QTD025 Rapid Mixing of Non-Commuting Quantum Gibbs Samplers in Two and Higher Dimensions

```xml
<problem id="QTD025">
  <status>open</status>
  <name>Rapid Mixing of Non-Commuting Quantum Gibbs Samplers in Two and Higher Dimensions</name>

  <keys>quantum gibbs sampler davies generator davies map davies generator spectral gap rapid mixing fast mixing quantum thermalization thermalization time relaxation time mixing time spectral gap size-independent spectral gap system-size independent gap lindbladian open quantum system dynamics quantum detailed balance gns detailed balance kubo-martin-schwinger condition kms condition gibbs state thermal state preparation clustering of correlations exponential clustering strong clustering condition phase transition critical temperature critical inverse temperature kastoryano brandao quantum gibbs samplers commuting case rouze franca alhambra chen kastoryano gilyen noncommutative quantum gibbs sampler non-commuting hamiltonian two-dimensional lattice higher-dimensional lattice dissipative state preparation quantum metropolis sampling modified log-sobolev inequality mlsi entropy decay constant primitive lindbladian bergamaschi chen fast mixing quantum spin chains smid meister berta bondesan weakly-interacting quantum gibbs samplers gamarnik kiani zlokapa slow mixing quantum gibbs samplers transverse field ising model ferromagnetic phase bottleneck lemma high temperature threshold</keys>

  <claim>
    Fix an integer \(D\ge2\) and a finite-range interaction \(\Phi=\{h_X\}\)
    on \(\mathbb Z^D\) (Def section: bounded, self-adjoint, finite-range
    terms, not assumed to commute or to be translation-invariant), giving
    Hamiltonians \(H_{\Lambda_L}=\sum_{X\subseteq\Lambda_L}h_X\) on cubes
    \(\Lambda_L\subset\mathbb Z^D\) of side length \(L\)
    (\(n:=n(L)=L^D=|\Lambda_L|\)) and Gibbs states
    \(\rho_{\beta,\Lambda_L}=e^{-\beta H_{\Lambda_L}}/\operatorname{Tr}
    e^{-\beta H_{\Lambda_L}}\) at inverse temperature \(\beta\ge0\). Let
    \(\beta_c(\Phi)\in[0,\infty]\) be the strong-clustering threshold of the
    Def section.

    For every such \(D\) and \(\Phi\), and every \(\beta\in[0,\beta_c(\Phi))\),
    there exists a Davies generator \(\mathcal L_{\beta,\Lambda_L}\) for
    \(\Phi\) (a choice of local coupling operators and a
    Kubo-Martin-Schwinger-detailed-balanced weight, Def section) such that
    its spectral gap \(\Delta(\mathcal L_{\beta,\Lambda_L})\), Def section, is
    bounded below uniformly in \(L\):
    \[
      \inf_L \Delta(\mathcal L_{\beta,\Lambda_L})\ =:\ \gamma(\beta,\Phi)\ \gt\ 0,
    \]
    with \(\gamma(\beta,\Phi)\) depending only on \(\beta\), \(D\), the range
    \(r\) and norm bound \(J\) of \(\Phi\), the local dimension \(q\), and the
    locality radius and detailed-balance weight of the chosen generator --
    never on \(L\) or \(n\).

    Quantifier note (weaker reading chosen; ambiguity flagged). This claim is
    existential in the generator: it asks only that some Davies generator for
    \((\Phi,\beta)\) have a size-independent gap, matching how every source in
    the Known section below actually states its results, each for one
    explicit generator, not for every admissible choice of coupling operators
    and weight simultaneously. A strictly stronger reading -- every Davies
    generator for \((\Phi,\beta)\) has this gap, or the gap holds together
    with a size-independent modified logarithmic Sobolev inequality (giving
    convergence in polylogarithmic time from an arbitrary initial state,
    rather than only a positive gap of the fixed generator) -- is a different,
    harder statement not adopted here. The survey document this entry's lead
    is drawn from separately lists the \(D=1\) case of exactly that stronger,
    log-Sobolev statement as its own explicitly open companion lead
    ("Strong relaxation for chains (MLSI)"), confirming that "relaxation
    speed" in the lead is meant at the spectral-gap level fixed above, not the
    log-Sobolev level, and this entry follows that reading for \(D\ge2\) as
    the weaker, precise formulation of the two.
  </claim>

  <def>
    Lattice, interaction, Gibbs state. Fix an integer \(D\ge2\) and an
    integer \(q\ge2\) (local dimension); every site of \(\mathbb Z^D\) carries
    a Hilbert space \(\mathbb C^q\). For \(x,y\in\mathbb Z^D\),
    \(\|x-y\|_1:=\sum_{i=1}^D|x_i-y_i|\); for finite \(X,Y\subset\mathbb Z^D\),
    \(\operatorname{dist}(X,Y):=\min\{\|x-y\|_1:x\in X,y\in Y\}\) and
    \(\operatorname{diam}(X):=\max\{\|x-y\|_1:x,y\in X\}\). An interaction is
    a family \(\Phi=\{h_X\}\), indexed by finite \(X\subset\mathbb Z^D\), of
    Hermitian operators \(h_X\) acting on \(\bigotimes_{x\in X}\mathbb C^q\)
    (extended by the identity elsewhere), with a fixed range \(r\)
    (\(h_X=0\) whenever \(\operatorname{diam}(X)\gt r\)) and a fixed norm
    bound \(J\) (\(\|h_X\|\le J\) for every \(X\)); \(\Phi\) is not assumed
    commuting (\([h_X,h_Y]\ne0\) is allowed for every \(X,Y\)) or
    translation-invariant (\(h_{X+v}\) need not relate to \(h_X\) for lattice
    translations \(v\)). For a cube \(\Lambda_L:=\{0,\ldots,L-1\}^D\)
    (\(L\in\mathbb N\), \(n=n(L)=L^D=|\Lambda_L|\)), the associated Hamiltonian
    is \(H_{\Lambda_L}:=\sum_{X\subseteq\Lambda_L}h_X\) (free boundary
    condition; the claim above does not depend on this choice beyond the
    value of \(\gamma(\beta,\Phi)\)), and the Gibbs state at inverse
    temperature \(\beta\ge0\) is
    \(\rho_{\beta,\Lambda_L}:=e^{-\beta H_{\Lambda_L}}/\operatorname{Tr}
    e^{-\beta H_{\Lambda_L}}\).

    Davies generator. Write the spectral decomposition
    \(H_{\Lambda_L}=\sum_\epsilon\epsilon\,P_\epsilon\) (\(P_\epsilon\) the
    orthogonal projector onto the \(\epsilon\)-eigenspace). For a bounded
    operator \(S\) on \(\Lambda_L\) and a Bohr frequency
    \(\omega\in\{\epsilon'-\epsilon\}\), set
    \(S(\omega):=\sum_{\epsilon'-\epsilon=\omega}P_\epsilon SP_{\epsilon'}\),
    so \(S=\sum_\omega S(\omega)\) and
    \(e^{iH_{\Lambda_L}t}S(\omega)e^{-iH_{\Lambda_L}t}=e^{-i\omega t}S(\omega)\).
    Fix a locality radius \(r'\ge0\) (a constant, independent of \(L\)) and,
    for each \(x\in\Lambda_L\), a finite set of Hermitian coupling operators
    \(\{S_x^a\}_{a=1}^k\) (\(k\) a fixed integer independent of \(L\)), each
    supported on the ball \(B(x,r')\cap\Lambda_L\), chosen so that
    \(\{S_x^a\}_{x\in\Lambda_L,\,a=1,\ldots,k}\) spans the full space of
    Hermitian operators on every such ball (e.g. \(k=q^{2m}-1\) with
    \(m:=|B(x,r')|\) and \(\{S_x^a\}_a\) an orthonormal traceless-Hermitian
    basis of the local algebra on \(B(x,r')\) always suffices; \(r'=0\)
    recovers the classical single-site Davies construction). Fix a weight
    \(w:\mathbb R\to(0,\infty)\) satisfying the Kubo-Martin-Schwinger (KMS)
    condition \(w(-\omega)=e^{-\beta\omega}w(\omega)\) for every \(\omega\).
    The Davies generator for \(\Phi\) at \((\beta,\Lambda_L)\) built from
    \(\{S_x^a\}\) and \(w\) is the Lindbladian
    \[
      \mathcal L_{\beta,\Lambda_L}(\rho)\ =\ -i[H_{LS},\rho]
      \ +\ \sum_{x\in\Lambda_L}\sum_{a=1}^k\sum_\omega w(\omega)
      \left(S_x^a(\omega)\,\rho\,S_x^a(\omega)^\dagger
      -\tfrac12\{S_x^a(\omega)^\dagger S_x^a(\omega),\rho\}\right),
    \]
    with \(H_{LS}\) a Hermitian Lamb-shift term commuting with
    \(H_{\Lambda_L}\), chosen by the standard weak-coupling-limit
    construction (Davies 1974, Known section) so that
    \(\mathcal L_{\beta,\Lambda_L}\) generates a completely positive,
    trace-preserving semigroup with
    \(\mathcal L_{\beta,\Lambda_L}(\rho_{\beta,\Lambda_L})=0\). The KMS
    condition on \(w\) makes \(\mathcal L_{\beta,\Lambda_L}\) self-adjoint for
    the inner product
    \((X,Y)_{\rho_{\beta,\Lambda_L}}:=\operatorname{Tr}
    [\rho_{\beta,\Lambda_L}^{1/2}X^\dagger\rho_{\beta,\Lambda_L}^{1/2}Y]\)
    on operators (quantum detailed balance, GNS-symmetry), and the spanning
    property of \(\{S_x^a\}\) makes \(\mathcal L_{\beta,\Lambda_L}\) primitive:
    \(\rho_{\beta,\Lambda_L}\) is its unique fixed point and
    \(e^{t\mathcal L_{\beta,\Lambda_L}}(\rho)\to\rho_{\beta,\Lambda_L}\) as
    \(t\to\infty\) for every initial state \(\rho\).

    Spectral gap. Since \(\Lambda_L\) is finite, \(\mathcal L_{\beta,\Lambda_L}\)
    has finitely many eigenvalues, all with \(\operatorname{Re}(\lambda)\le0\)
    and \(\lambda=0\) simple by primitivity. The spectral gap is
    \[
      \Delta(\mathcal L_{\beta,\Lambda_L})\ :=\
      \min\{-\operatorname{Re}(\lambda):\lambda\in
      \operatorname{spec}(\mathcal L_{\beta,\Lambda_L})\setminus\{0\}\}\ \gt\ 0.
    \]
    A positive \(\Delta(\mathcal L_{\beta,\Lambda_L})\) controls the
    exponential rate at which \(e^{t\mathcal L_{\beta,\Lambda_L}}(\rho)\)
    approaches \(\rho_{\beta,\Lambda_L}\) for every fixed initial \(\rho\);
    size-independence of \(\Delta\) is the precise sense of "relaxation speed
    independent of system size" used throughout this entry, following
    Kastoryano and Brandão (Known section). It does not by itself give a
    size-independent, or even polylogarithmic, bound on the time needed to
    reach a fixed trace-distance error starting from an arbitrary (worst-case)
    initial state; that stronger property is a modified-logarithmic-Sobolev
    statement, out of scope here per the Claim section's quantifier note.

    Strong clustering and the threshold \(\beta_c(\Phi)\). Say \(\Phi\) has
    strong clustering at \(\beta\), in the sense used throughout this entry,
    if there exist \(\xi=\xi(\beta)\gt0\) and
    \(C=C(\beta)\lt\infty\) such that, for every \(L\), every
    \(\Lambda'\subseteq\Lambda_L\), every fixed state on
    \(\Lambda_L\setminus\Lambda'\), every finite \(X,Y\subset\Lambda'\), and
    every operators \(A\) on \(X\), \(B\) on \(Y\) with \(\|A\|,\|B\|\le1\),
    \[
      \bigl|\operatorname{Tr}[\rho'(A\otimes B)]
      -\operatorname{Tr}[\rho'(A\otimes\mathbb 1)]\,
      \operatorname{Tr}[\rho'(\mathbb 1\otimes B)]\bigr|
      \ \le\ C\,e^{-\operatorname{dist}(X,Y)/\xi},
    \]
    where \(\rho'\) is the Gibbs state on \(\Lambda'\) at inverse temperature
    \(\beta\) with the given fixed state on \(\Lambda_L\setminus\Lambda'\)
    (i.e. clustering uniform over every sub-region and every boundary
    condition, not merely for the full Gibbs state on \(\Lambda_L\) itself).
    Set \(\beta_c(\Phi):=\sup\{\beta_0\ge0:\Phi\text{ has strong clustering at
    every }\beta\lt\beta_0\}\in[0,\infty]\). This operationalizes "the phase
    transition" of the survey lead as the rigorous clustering threshold used
    throughout this literature, rather than a separately defined physical
    critical temperature, which is not established in general for
    non-commuting \(D\ge2\) lattice models in the generality of this entry.
  </def>

  <known>
    E. B. Davies, Markovian Master Equations, Communications in Mathematical
    Physics 39, 91-110 (1974), gives the general weak-coupling-limit
    construction of the Lamb-shift term \(H_{LS}\) and the Lindbladian form
    used in the Def section above; it is the foundational reference for the
    generator's form, not a source of any rapid-mixing bound used below.

    Chi-Fang Chen, Michael J. Kastoryano, and András Gilyén, An Efficient and
    Exact Noncommutative Quantum Gibbs Sampler, arXiv:2311.09207 (2023),
    construct the first efficiently implementable, exactly
    detailed-balanced Lindbladian for Gibbs states of arbitrary
    (non-commuting) local Hamiltonians at arbitrary inverse temperature
    \(\beta\); this is the "quantum Gibbs sampler" construction (cited as
    "[CKG23]") that the two 2025 papers below build on and analyze the
    mixing properties of. It supplies the generator and its
    detailed-balance/primitivity properties, not a system-size-independent
    spectral-gap bound in general dimension.

    Michael J. Kastoryano and Fernando G. S. L. Brandão, Quantum Gibbs
    Samplers: The Commuting Case, Communications in Mathematical Physics 344,
    915-957 (2016), Theorem 1, prove, for commuting interactions
    (\([h_X,h_Y]=0\) for all \(X,Y\)) on \(\mathbb Z^D\) at any \(D\), that
    \(\inf_L\Delta(\mathcal L_{\beta,\Lambda_L})\gt0\) if and only if \(\Phi\)
    has strong clustering at \(\beta\) in their own, conditional-covariance
    sense, which this entry does not verify against the plain
    correlation-decay bound of the Def section above. Modulo that
    verification, this resolves the claim above completely whenever
    \([h_X,h_Y]=0\) for every \(X,Y\); the open content of the claim is the
    extension from commuting to non-commuting \(\Phi\), together with
    confirming that identification in the commuting case.

    Cambyse Rouzé, Daniel Stilck França, and Álvaro M. Alhambra, Efficient
    Thermalization and Universal Quantum Computing with Quantum Gibbs
    Samplers, Nature Physics (2026), DOI: 10.1038/s41567-026-03246-y,
    arXiv:2403.12691, Theorem II.1 and Corollary II.2, prove, for every
    bounded-body, bounded-coordination-number local Hamiltonian on any graph
    satisfying a Lieb-Robinson bound -- in particular every finite-range
    \(\Phi\) on \(\mathbb Z^D\) at any \(D\), not assumed commuting -- the
    existence of a threshold \(\beta^*\gt0\), independent of system size and
    given in closed form as the inverse of a constant multiple of the product
    of the operator-norm bound, the interaction-body size, and the lattice
    coordination number (their own notation, distinct from this entry's
    \(J\), \(r\), \(k\) above), such that for every \(\beta\le\beta^*\) the
    spectral gap of the (CKG23-type) Davies generator is bounded below by the
    absolute constant \(1/(2\sqrt2\,e^{1/4})\). This is exactly the general,
    non-commuting, any-\(D\) "high temperature settled" leg of the survey
    lead this entry records: it extends Kastoryano-Brandão's commuting-case
    high-temperature
    regime to arbitrary local Hamiltonians. Their threshold \(\beta^*\) is a
    crude perturbative-style bound with no stated relation to
    \(\beta_c(\Phi)\) of the Def section above; Rouzé, França, and Alhambra's
    introduction and conclusion do not compare \(\beta^*\) to any clustering
    or phase-transition threshold, and their conclusion instead flags, as its
    own open direction, strengthening the gap bound to a modified logarithmic
    Sobolev inequality -- not extending \(\beta^*\) up to \(\beta_c(\Phi)\).
    This entry does not
    independently verify, beyond the automated extraction used here, whether
    \(\beta^*\) is provably strictly smaller than \(\beta_c(\Phi)\) for every
    \(\Phi\), or merely unshown to reach it; either way \(\beta^*\ge\beta_c(\Phi)\)
    is not asserted by Rouzé, França, and Alhambra, so their result does not
    resolve the claim above at any \(\beta\in[\beta^*,\beta_c(\Phi))\)
    whenever that interval is non-empty.

    Thiago Bergamaschi and Chi-Fang Chen, Fast Mixing of Quantum Spin Chains
    at All Temperatures, arXiv:2510.08533 (v2, 2026), Theorem I.1, prove, for
    \(D=1\) and every finite-range \(\Phi\) (not assumed commuting or
    translation-invariant; a general range reduces to the nearest-neighbor
    case by blocking sites into an \(O(1)\)-dimensional local space), that
    \(\inf_L\Delta(\mathcal L_{\beta,\Lambda_L})\gt0\) for every finite
    \(\beta\ge0\) -- i.e. \(\beta_c(\Phi)=\infty\) throughout \(D=1\) and the
    claim above holds at every temperature there. This resolves the \(D=1\)
    case of the claim completely; it does not address \(D\ge2\), and its
    introduction states that in higher dimensions "complex-time evolution
    generically diverges below a finite temperature," so its own 1D technique
    does not directly extend.

    Štěpán Šmíd, Richard Meister, Mario Berta, and Roberto Bondesan, Rapid
    Mixing of Quantum Gibbs Samplers for Weakly-Interacting Quantum Systems,
    arXiv:2510.04954 (2025), Theorem I.2 (informal) and Theorem III.3 with
    Corollary III.3.1 (quantitative), prove, for any fixed \(D\), any
    constant \(\beta\), and any quasi-local perturbation of a non-interacting
    reference Hamiltonian (free qudits or free fermions) of
    strength \(|\lambda|\) below an explicit computable threshold
    \(\lambda_{\max}(\beta,D,\ldots)\), that the resulting generator satisfies
    a contraction bound of the form
    \(\mathrm{poly}(n)e^{-\alpha t}\) with \(\alpha\gt0\) independent of \(n\),
    which in particular gives \(\inf_L\Delta(\mathcal L_{\beta,\Lambda_L})\gt0\).
    This resolves the claim above in a perturbative neighborhood of a
    solvable point, at every temperature and in every dimension, but not for
    generic finite-range \(\Phi\) of order-1 interaction strength, which is
    the regime relevant near \(\beta_c(\Phi)\) for an interacting model.

    David Gamarnik, Bobak T. Kiani, and Alexander Zlokapa, Slow Mixing of
    Quantum Gibbs Samplers, arXiv:2411.04300 (2024), Theorem 1.4 (informal)
    and the formal statement in Section 4.4, prove, for the
    ferromagnetic two-dimensional transverse-field Ising Hamiltonian
    \(H=-\sum_{\langle i,j\rangle}Z_iZ_j-h\sum_iX_i\) on an \(L\times L\)
    lattice (\(n=L^2\), non-commuting), that
    \(T_{\mathrm{mix}}=\exp(\Omega(\sqrt n))\) at every constant
    \(\beta\ge\beta^*\) and every \(0\le h\le h^*(\beta)\) -- a low-temperature,
    weak-field region of the ferromagnetically ordered phase, symmetric under
    global spin flip with no symmetry-breaking field, directly analogous to
    classical zero-field Glauber dynamics below its critical temperature --
    via a generic free-energy bottleneck argument that Gamarnik, Kiani, and
    Zlokapa state applies to every local Gibbs-reversible generator for that
    \((\Phi,\beta,h)\), not to one fixed choice of coupling operators. Because
    this region lies in the ordered phase, below the model's own
    order-disorder transition, it
    does not fall inside \(\beta\lt\beta_c(\Phi)\) as defined in the Def
    section above (the strong-clustering condition used there is expected to
    fail throughout the ordered phase for exactly this reason), so it is
    consistent with, and does not disprove, the claim above; it does show
    that the claim genuinely fails somewhere in the \(D=2\) phase diagram and
    so cannot be strengthened to hold at every \(\beta\ge0\).

    Together, these results leave exactly the gap recorded by the claim
    above: for \(D\ge2\), a system-size-independent spectral gap throughout
    the entire strong-clustering regime \(\beta\lt\beta_c(\Phi)\) is proven
    only for commuting \(\Phi\) (Kastoryano-Brandão) or in a perturbative
    neighborhood of a solvable point (Šmíd-Meister-Berta-Bondesan); for
    generic non-commuting, order-1-interaction-strength \(\Phi\), only a
    cruder, model-independent high-temperature threshold \(\beta^*\) is
    proven (Rouzé-França-Alhambra), with no source found establishing or
    refuting \(\beta^*=\beta_c(\Phi)\) or extending the gap to the full
    interval \([\beta^*,\beta_c(\Phi))\). Theorem numbers and statements for
    the four 2023-2026 papers above were obtained from automated fetches of
    arXiv abstract and HTML pages rather than from a manual read of the
    typeset PDFs, and are reported here with that caveat.
  </known>

  <refs>
    <ref>E. B. Davies, Markovian Master Equations, Communications in Mathematical Physics 39, 91-110 (1974), DOI: 10.1007/BF01608389, used for the general weak-coupling-limit construction of the Davies generator (Lamb-shift term, Lindblad form, Bohr-frequency decomposition) fixed in the Def section above.</ref>
    <ref>Chi-Fang Chen, Michael J. Kastoryano, and András Gilyén, An Efficient and Exact Noncommutative Quantum Gibbs Sampler, arXiv:2311.09207 (2023), used for the construction ("[CKG23]") of an efficiently implementable, exactly detailed-balanced Lindbladian for Gibbs states of arbitrary non-commuting local Hamiltonians at arbitrary temperature, which the two 2025 sources below analyze the mixing properties of.</ref>
    <ref>Michael J. Kastoryano and Fernando G. S. L. Brandão, Quantum Gibbs Samplers: The Commuting Case, Communications in Mathematical Physics 344, 915-957 (2016), DOI: 10.1007/s00220-016-2641-8, arXiv:1409.3435, Theorem 1, used for the iff-equivalence between a system-size-independent Davies-generator spectral gap and their own conditional-covariance notion of strong clustering, for commuting interactions in any dimension \(D\); see the Known section above for the unverified identification with the Def section's clustering condition.</ref>
    <ref>Cambyse Rouzé, Daniel Stilck França, and Álvaro M. Alhambra, Efficient Thermalization and Universal Quantum Computing with Quantum Gibbs Samplers, Nature Physics (2026), DOI: 10.1038/s41567-026-03246-y, arXiv:2403.12691, Theorem II.1 and Corollary II.2, used for the general (non-commuting), any-dimension, crude-high-temperature-threshold spectral-gap bound recorded in the Known section above -- the "high temperature settled" leg of the survey lead this entry records, for general local Hamiltonians rather than only the commuting case.</ref>
    <ref>Thiago Bergamaschi and Chi-Fang Chen, Fast Mixing of Quantum Spin Chains at All Temperatures, arXiv:2510.08533 (v2, revised 2026-01-23), Theorem I.1, used for the complete resolution of the claim above in \(D=1\), at every finite inverse temperature, for finite-range interactions not assumed commuting or translation-invariant -- the "1D settled 2025" leg of the survey lead this entry records.</ref>
    <ref>Štěpán Šmíd, Richard Meister, Mario Berta, and Roberto Bondesan, Rapid Mixing of Quantum Gibbs Samplers for Weakly-Interacting Quantum Systems, arXiv:2510.04954 (2025), Theorem I.2, Theorem III.3, and Corollary III.3.1, used for the resolution of the claim above in a perturbative neighborhood of a non-interacting reference Hamiltonian, at any temperature and in any fixed dimension \(D\).</ref>
    <ref>David Gamarnik, Bobak T. Kiani, and Alexander Zlokapa, Slow Mixing of Quantum Gibbs Samplers, arXiv:2411.04300 (2024), Theorem 1.4, Section 4.4, used for the generator-independent slow-mixing lower bound for the ferromagnetic 2D transverse-field Ising model in its low-temperature ordered phase -- the "slow-mixing counterexamples show the frontier is real" leg of the survey lead this entry records, and the basis for this entry's argument that the claim's domain \(\beta\lt\beta_c(\Phi)\) excludes this counterexample.</ref>
  </refs>

  <ask>
    A proof exhibits, for every \(D\ge2\) and every finite-range interaction
    \(\Phi\) on \(\mathbb Z^D\) (Def section), a Davies generator (a choice of
    local complete coupling operators and KMS-detailed-balanced weight)
    together with a function \(\gamma(\beta,\Phi)\gt0\) such that
    \(\Delta(\mathcal L_{\beta,\Lambda_L})\ge\gamma(\beta,\Phi)\) for every
    \(\beta\in[0,\beta_c(\Phi))\) and every \(L\). A single construction
    covering every \(D\), every \(\Phi\), and the full interval
    \([0,\beta_c(\Phi))\) is required; a proof of a uniform modified
    logarithmic Sobolev inequality throughout the same interval would imply
    the claim and counts as a full proof, since primitivity together with a
    positive log-Sobolev constant already forces a positive spectral gap for
    these generators.

    A disproof exhibits some \(D\ge2\), some finite-range \(\Phi\) on
    \(\mathbb Z^D\), and some \(\beta_0\in[0,\beta_c(\Phi))\), together with a
    proof that \(\liminf_{L\to\infty}\Delta(\mathcal L_{\beta_0,\Lambda_L})=0\)
    for every choice of local complete coupling operators and
    detailed-balanced weight (Def section) -- ruling out every Davies
    generator for that one \((\Phi,\beta_0)\), not merely one specific choice
    of coupling operators, since the claim is existential in the generator. A
    generic bottleneck argument in the style of Gamarnik, Kiani, and Zlokapa
    (Known section), which they state rules out every local Gibbs-reversible
    generator at once, is a template for arguments of the needed form; their
    own published instance lies at \(\beta_0\ge\beta_c(\Phi)\) and so does
    not itself supply a disproof.
    Because the claim is universally quantified over \(D\), \(\Phi\), and
    \(\beta\), one such triple with a matching all-generators impossibility
    proof disproves it, even leaving every other triple undecided.

    The following count as progress, not as a solution to the claim, unless
    combined with a proof removing the stated restriction: (i) a construction
    restricted to commuting \(\Phi\) (already resolved by Kastoryano and
    Brandão, Known section); (ii) a construction restricted to \(D=1\)
    (already resolved by Bergamaschi and Chen, Known section); (iii) a
    construction restricted to a perturbative neighborhood of a
    non-interacting reference Hamiltonian (already resolved by Šmíd, Meister,
    Berta, and Bondesan, Known section); (iv) an extension of the crude
    high-temperature threshold \(\beta^*\) of Rouzé, França, and Alhambra
    (Known section) to a larger, still model-independent constant, without a
    proof that it reaches \(\beta_c(\Phi)\) for the interactions covered; (v)
    a bound showing only that \(\Delta(\mathcal L_{\beta,\Lambda_L})\) closes
    no faster than an inverse polynomial in \(n\), rather than a genuine
    \(n\)-independent lower bound; (vi) a result restricted to a single
    explicit lattice model or a measure-zero family of \(\Phi\), rather than
    every finite-range interaction on \(\mathbb Z^D\) at fixed \(D\); (vii) an
    upper bound on \(\beta_c(\Phi)\) itself (narrowing the claimed clustering
    regime) without a matching gap construction throughout the narrowed
    regime.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
