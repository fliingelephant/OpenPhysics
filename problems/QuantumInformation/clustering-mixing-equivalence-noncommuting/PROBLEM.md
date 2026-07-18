# QTD027 Clustering-Mixing Equivalence for Non-Commuting Quantum Gibbs Samplers

```xml
<problem id="QTD027">
  <status>open</status>
  <name>Clustering-Mixing Equivalence for Non-Commuting Quantum Gibbs Samplers</name>

  <keys>clustering mixing equivalence, clustering-mixing equivalence, correlation decay implies rapid mixing, rapid mixing implies clustering, weak clustering, strong clustering, conditional covariance, conditional expectation, quantum Gibbs sampler, local Gibbs sampler, Davies generator, Davies map, heat-bath generator, heat-bath Monte Carlo generator, spectral gap, system-size independent spectral gap, size-independent gap, Kastoryano-Brandao, Kastoryano Brandao commuting case, Theorem 1 Kastoryano Brandao, Theorem 23, Theorem 26, non-commuting Hamiltonian, noncommuting Hamiltonian, non-commuting potential, r-local potential, Lindbladian, Liouvillian, primitive Liouvillian, locally primitive, reversible Liouvillian, locally reversible, detailed balance, GNS detailed balance, quantum lattice Gibbs state, thermal state, thermalization, mixing time, dissipative state preparation, non-commutative Lp space, statics-dynamics equivalence, welds statics to dynamics, Outlook open question, clustering-mixing-equivalence-noncommuting</keys>

  <claim>
    Fix an integer \(d\ge1\). For every integer \(r\ge1\), every
    \(\beta\in(0,\infty)\), every \(r\)-local bounded potential \(\Phi\) on
    \(\mathbb Z^d\) (not assumed commuting, i.e. \([\Phi(i),\Phi(j)]\ne0\)
    is allowed for every \(i,j\)), and every coherent family of local
    Gibbs samplers \((L_l,E_l)_{l\in\mathbb N_+}\) of
    \((\rho_{\Lambda_l})_{l\in\mathbb N_+}\) (all defined below): the
    following two statements are equivalent.

    (i) \(\inf_{l\in\mathbb N_+}\lambda_{\Lambda_l}(L_l)\gt0\) (the
    spectral gap of the local Gibbs sampler's Liouvillian \(L_l\) is
    bounded below independently of the cube side length \(l\), i.e.
    independently of system size).

    (ii) \((\rho_{\Lambda_l})_{l\in\mathbb N_+}\) satisfies strong
    clustering with respect to \((E_l)_{l\in\mathbb N_+}\) uniformly in
    \(l\) (the constants \(c,\xi\) of Definition 15 below can be chosen
    independently of \(l\)).

    This is Kastoryano and Brandão, "Quantum Gibbs Samplers: The Commuting
    Case," Communications in Mathematical Physics 344, 915-957 (2016),
    Theorem 1 (formally Theorems 23 and 26), with the hypothesis that
    \(\Phi\) is a commuting potential dropped. Every other object in the
    claim -- \(r\)-local potential, Gibbs state, spectral gap
    \(\lambda_\Lambda\), conditional expectation, weak and strong
    clustering -- is defined by Kastoryano-Brandão (2016) without
    reference to commutativity of \(\Phi\) (their Definitions 5-8 and
    14-15), so dropping only the commuting hypothesis from Theorems 23 and
    26 yields a claim that is well posed using solely Kastoryano-Brandão
    (2016)'s own machinery.

    Ambiguity flagged. Kastoryano-Brandão (2016) never state this
    generalization as a numbered conjecture; their Outlook (Sec. IX)
    remarks only in prose that extending their framework to non-commuting
    \(\Phi\) "would be very interesting," while separately noting the
    concrete obstruction (quoted in the Known section) that their own
    conditional expectations and generators "become non-local" outside the
    commuting case. This entry therefore records the weakest precise
    reading available: the same theorem statement with the commuting
    hypothesis on \(\Phi\) alone removed, universally quantified over
    whichever coherent family of local Gibbs samplers is given, rather
    than asserting that the specific Davies or heat-bath construction of
    Kastoryano-Brandão (2016) itself remains a local Gibbs sampler for
    general \(\Phi\) -- a strictly stronger claim that source neither
    makes nor discusses proving. Existence of at least one coherent family
    of local Gibbs samplers for every \(r\)-local bounded \(\Phi\) is a
    distinct, logically prior question not addressed here; the claim above
    is conditional on such a family being given and is vacuous for any
    \(\Phi\) for which none exists.
  </claim>

  <def>
    Lattice and local algebras. Fix an integer \(d\ge1\) and, once and for
    all, finite-dimensional site Hilbert spaces \(\mathbb C^{n_x}\)
    (\(n_x\lt\infty\)) for every \(x\in\mathbb Z^d\). For finite
    \(\Lambda\subseteq\mathbb Z^d\), \(\mathcal H_\Lambda:=\bigotimes_{x
    \in\Lambda}\mathbb C^{n_x}\); \(B_\Lambda\) is the algebra of bounded
    operators on \(\mathcal H_\Lambda\); \(A_\Lambda\subseteq B_\Lambda\)
    is its Hermitian subset ("observables"); \(S_\Lambda:=\{X\in
    A_\Lambda:X\ge0,\ \mathrm{tr}[X]=1\}\) ("states") and
    \(S_\Lambda^+\subset S_\Lambda\) is its full-rank subset
    (Kastoryano-Brandão (2016), Sec. II.A). An operator \(f\in B_\Lambda\)
    has support \(\Sigma_f=A\), for \(A\subseteq\Lambda\), if
    \(f=f_A\otimes\mathrm{id}_{A^c}\) for some \(f_A\in B_A\). For
    \(i,j\in\mathbb Z^d\), \(d(i,j)\) is Euclidean distance; for finite
    \(A,B\subseteq\mathbb Z^d\), \(d(A,B):=\min\{d(i,j):i\in A,j\in B\}\).

    \(r\)-local bounded potential and Gibbs state. Fix an integer
    \(r\ge1\) and a constant \(K\lt\infty\). An \(r\)-local bounded
    potential is a map \(\Phi\) assigning to every \(j\in\mathbb Z^d\) a
    Hermitian operator \(\Phi(j)\), supported on the ball \(B(j,r)\subset
    \mathbb Z^d\), with \(\|\Phi(j)\|\lt K\) (Kastoryano-Brandão (2016),
    Sec. IV, Eq. (25), extended here from a single finite \(\Lambda\) to
    all of \(\mathbb Z^d\) so that the finite-volume Hamiltonians below
    are mutually coherent as the volume grows; \(\Phi\) is not required to
    satisfy \([\Phi(i),\Phi(j)]=0\), which Kastoryano-Brandão (2016) call
    the commuting case and impose as an extra hypothesis on top of this
    definition). For finite \(\Lambda\subseteq\mathbb Z^d\),
    \(H_\Lambda:=\sum_{j\in\Lambda}\Phi(j)\), and for \(\beta\in
    (0,\infty)\),
    \[
      \rho_\Lambda:=e^{-\beta H_\Lambda}/\mathrm{tr}\bigl[e^{-\beta
      H_\Lambda}\bigr]\ \in\ S_\Lambda^+
    \]
    (Kastoryano-Brandão (2016), Eq. (27), with the inverse temperature
    \(\beta\) made explicit). For \(l\in\mathbb N_+\), \(\Lambda_l\subset\mathbb Z^d\) is the cube of
    side length \(l\) (\(|\Lambda_l|=l^d\)), identified with the periodic
    quotient \((\mathbb Z/l\mathbb Z)^d\) that Kastoryano-Brandão (2016),
    Sec. II.A, fix throughout and that Theorems 23, 26, and 28 are all
    stated for (this entry does not extend the claim to other boundary
    conditions), and \(\rho_{\Lambda_l}\) is as above with
    \(\Lambda=\Lambda_l\).

    \(L^2(\rho)\) inner product and covariance. For \(\rho\in S_\Lambda^+\)
    and \(f,g\in A_\Lambda\),
    \[
      \langle f,g\rangle_\rho:=\mathrm{tr}\bigl[\sqrt\rho\,f\,\sqrt\rho\,g
      \bigr],\qquad \|f\|_{2,\rho}:=\langle f,f\rangle_\rho^{1/2},\qquad
      \mathrm{Cov}_\rho(f,g):=\langle f,g\rangle_\rho-\mathrm{tr}[\rho f]\,
      \mathrm{tr}[\rho g]
    \]
    (Kastoryano-Brandão (2016), Eqs. (2), (1) at \(p=2\), and (5)).

    Liouvillians: primitive, reversible, \(r'\)-local. A Liouvillian on
    \(\Lambda\) is a linear map \(L:B_\Lambda\to B_\Lambda\) of Lindblad
    form \(L(f)=i[H,f]+\sum_k(L_k^\dagger fL_k-\tfrac12\{L_k^\dagger
    L_k,f\})\) for some Hermitian \(H\in A_\Lambda\) and \(\{L_k\}\subset
    B_\Lambda\) (Kastoryano-Brandão (2016), Eqs. (6)-(7)). \(L\) is
    primitive if it has a unique full-rank stationary state \(\rho\)
    (\(L^*(\rho)=0\) for the Hilbert-Schmidt adjoint \(L^*\), and no other
    full-rank state has this property); \(L\) is reversible with respect
    to \(\rho\in S_\Lambda^+\) (detailed balance) if \(\langle f,L(g)
    \rangle_\rho=\langle L(f),g\rangle_\rho\) for every \(f,g\in
    A_\Lambda\). For an integer \(r'\ge1\), \(L\) is \(r'\)-local if
    \(L(f)=\sum_{Z\subseteq\Lambda,\,|Z|\le r'}\bigl(i[H_Z,f]+D_Z(f)
    \bigr)\) with each \(H_Z,D_Z\) of the above Lindblad form and
    supported on \(Z\); the restricted Liouvillian on \(A\subseteq
    \Lambda\) is \(L_A(f):=\sum_{Z\subseteq\Lambda,\,|Z|\le r',\,Z\cap
    A\ne\emptyset}\bigl(i[H_Z,f]+D_Z(f)\bigr)\) (Kastoryano-Brandão
    (2016), Eq. (9) and surrounding text). \(L\) is locally primitive if,
    for every \(A\subseteq\Lambda\), \(L_A(f)=0\) implies \(f\) acts
    non-trivially only on \(A^c\); \(L\) is locally reversible with
    respect to \(\rho\) if, for every \(A\subseteq\Lambda\) and every
    \(f,g\in A_\Lambda\), \(\langle f,L_A(g)\rangle_\rho=\langle
    L_A(f),g\rangle_\rho\) (Kastoryano-Brandão (2016), Definition 5; none
    of these definitions reference commutativity of any Hamiltonian).

    Spectral gap. For finite \(\Lambda\) and \(L\) a primitive reversible
    Liouvillian on \(\Lambda\) with stationary state \(\rho\),
    \[
      \lambda_\Lambda(L):=\inf_{f\in A_\Lambda}\ \frac{-\langle f,L(f)
      \rangle_\rho}{\mathrm{Var}_\rho(f)},\qquad \mathrm{Var}_\rho(f):=
      \mathrm{Cov}_\rho(f,f)
    \]
    (Kastoryano-Brandão (2016), Definition 7); equivalently the smallest
    nonzero eigenvalue of \(-L\) acting on the \(L^2(\rho)\) space (loc.
    cit.).

    Conditional expectations, local and general. For \(\rho\in
    S_\Lambda^+\), \(E:B_\Lambda\to B_\Lambda\) is a conditional
    expectation of \(\rho\) if (a) \(E\) is completely positive and
    unital, (b) \(\mathrm{tr}[\rho E(f)]=\mathrm{tr}[\rho f]\) for every
    \(f\in A_\Lambda\), (c) \(\langle E(f),g\rangle_\rho=\langle f,E(g)
    \rangle_\rho\) for every \(f,g\in A_\Lambda\), and (d) \(\langle
    E^n(f),f\rangle_\rho\ge\langle E^{n+1}(f),f\rangle_\rho\) for every
    \(f\in A_\Lambda\), \(n\in\mathbb N\) (Kastoryano-Brandão (2016),
    Definition 8; stated for a general full-rank \(\rho\), with no
    commutativity hypothesis). A family \((E_A)_{A\subseteq\Lambda}\),
    with \(E_\Lambda=E\), of conditional expectations of \(\rho\) is
    \(r''\)-local, for an integer \(r''\ge1\) independent of \(\Lambda\),
    if \(E_A(f)\) has support in the \(r''\)-neighborhood of \(A\) for
    every \(A\subseteq\Lambda\) and every \(f\in A_\Lambda\).
    Kastoryano-Brandão (2016), Sec. III, construct two such families for a
    general \(\rho\) -- \(E^L\), from the infinite-time limit of a locally
    primitive Liouvillian's semigroup, and \(E^\rho\), from a
    coarse-graining map -- but prove \(r''\)-locality of \(E^\rho\) only
    when \(\rho\) is the Gibbs state of a commuting potential; this is the
    specific obstruction recorded in the Known section below.

    Local Gibbs sampler. A local Gibbs sampler of \(\rho\in S_\Lambda^+\)
    is a pair \((L,E)\) where \(L\) is an \(r'\)-local, locally primitive,
    locally reversible Liouvillian on \(\Lambda\) with stationary state
    \(\rho\), \(E=(E_A)_{A\subseteq\Lambda}\) is an \(r''\)-local family
    of conditional expectations of \(\rho\), and \(L,E\) share a kernel:
    for every \(A\subseteq\Lambda\) and \(f\in A_\Lambda\), \(L_A(f)=0\)
    if and only if \(E_A(f)=f\). (This assembles Kastoryano-Brandão
    (2016)'s two named instances -- \((L^D,E^L)\), the Davies generator
    with the local Liouvillian projector, and \((L^H,E^\rho)\), the
    heat-bath generator with the minimal conditional expectation, which
    the source states have "the same kernel" [Sec. VI.A] -- under the name
    "\((L_\Lambda,E)\) a local Gibbs sampler of \(\rho\)" that Theorem 26
    itself already uses, and for which Remark iv after Corollary 27 states
    the strong-clustering-gap equivalence "carries through with some
    modification for the pair \((L^H,E^\rho)\)" as well as for
    \((L^D,E^L)\); no third instance or general existence argument is
    given by Kastoryano-Brandão (2016).) A coherent family of local Gibbs
    samplers of \((\rho_{\Lambda_l})_{l\in\mathbb N_+}\) is a choice of
    local Gibbs sampler \((L_l,E_l)\) of \(\rho_{\Lambda_l}\) for every
    \(l\in\mathbb N_+\), with a common range \(r'\) for every \(L_l\) and
    a common range \(r''\) for every \(E_l\).

    Weak and strong clustering. For \(\rho\in S_\Lambda^+\), \(\rho\)
    satisfies weak clustering if there exist \(c,\xi\gt0\) such that, for
    every \(f,g\in A_\Lambda\),
    \[
      \mathrm{Cov}_\rho(f,g)\ \le\ c\,\|f\|_{2,\rho}\|g\|_{2,\rho}\,
      e^{-d(\Sigma_f,\Sigma_g)/\xi}
    \]
    (Kastoryano-Brandão (2016), Definition 14, Eq. (37)). For an
    \(r''\)-local family \((E_A)_{A\subseteq\Lambda}\) of conditional
    expectations of \(\rho\) and \(A,B\subseteq\Lambda\), the conditional
    covariance is \(\mathrm{Cov}_A(f,g):=\bigl|\langle f-E_A(f),g-E_A(g)
    \rangle_\rho\bigr|\) (Eq. (36)); \(\rho\) satisfies strong clustering with
    respect to \((E_A)_A\) if there exist \(c,\xi\gt0\) such that, for
    every \(A,B\subseteq\Lambda\) with \(A\cap B\ne\emptyset\) and every
    \(f\in A_\Lambda\),
    \[
      \mathrm{Cov}_{A\cup B}\bigl(E_A(f),E_B(f)\bigr)\ \le\ c\,
      \|f\|_{2,\rho}^2\,e^{-d(B\setminus A,\,A\setminus B)/\xi}
    \]
    (Kastoryano-Brandão (2016), Definition 15, Eq. (38)). Strong
    clustering implies weak clustering by combining Theorem 23 (strong
    clustering implies \(L^D_\Lambda\) gapped) with Corollary 27 (gapped,
    under the hypotheses of Theorem 26, implies weak clustering);
    Kastoryano-Brandão (2016) prove the
    converse implication only for \(d=1\) and only for commuting \(\Phi\)
    (Theorem 28), and only conjecture it, without proof, for \(d=2\)
    commuting \(\Phi\) (their Outlook, numbered Conjecture 1, quoted in
    the Known section). Consequently "correlation decay," in the informal
    sense the survey lead behind this entry uses, is weak clustering -- a
    condition formally different from, and outside \(d=1\) commuting
    \(\Phi\) not known to be equivalent to, the strong clustering that
    appears in the equivalence of the claim above.
    \((\rho_{\Lambda_l})_{l\in\mathbb N_+}\) satisfies strong clustering
    with respect to \((E_l)_{l\in\mathbb N_+}\) uniformly in \(l\) if the
    constants \(c,\xi\) above can be chosen independently of \(l\), for
    every \(l\in\mathbb N_+\).
  </def>

  <known>
    Theorem 1 (informal), formally Theorems 23 and 26 (the commuting case,
    proved for \((L^D,E^L)\); proved only in part for \((L^H,E^\rho)\), see
    below). Kastoryano and Brandão (2016) prove: fixing \(\beta\in
    (0,\infty)\) and a commuting \(r\)-local bounded potential \(\Phi\)
    (\([\Phi(i),\Phi(j)]=0\) for every \(i,j\)), there is \(l_0\gt0\) such
    that, for the Davies generator \(L^D_{\Lambda_l}\) (their Sec. IV.A
    construction) on every cube \(\Lambda_l\) with \(l\ge l_0\),
    \(\lambda_{\Lambda_l}(L^D_{\Lambda_l})\) is bounded below
    independently of \(l\) if and only if \(\rho_{\Lambda_l}\) satisfies
    strong clustering with respect to \(E^L\), uniformly in \(l\)
    (Theorem 23: strong clustering \(\Rightarrow\) \(L^D_{\Lambda_l}\)
    gapped; Theorem 26, stated for a general local Gibbs sampler
    \((L_\Lambda,E)\) of \(\rho_\Lambda\) with \(E\) a projective
    conditional expectation: gapped \(\Rightarrow\) strong clustering;
    \(E^L\) is a projector by construction [Sec. III.A, Eq. (12)], so
    Theorem 26 applies to \((L^D,E^L)\) directly). For the heat-bath
    generator \(L^H_{\Lambda_l}\) (their Sec. IV.B construction),
    \(E^\rho\) is not a projective conditional expectation [Sec. III.B,
    remark following Eq. (13): "\(E^\rho_A\) is not a projector"; Sec. VI,
    discussion following Definition 21: "\(\mathbb E^\rho\) is not a
    projective conditional expectation"], so Theorem 26 does not apply to
    \((L^H,E^\rho)\) directly. Remark iv after Corollary 27 only partially
    circumvents this, proving an asymmetric pair of implications rather
    than a clean iff with \(E^\rho\) on both sides: strong clustering with
    respect to \(E^\rho\), uniformly in \(l\), implies
    \(\lambda_{\Lambda_l}(L^H_{\Lambda_l})\) bounded below independently
    of \(l\); conversely, \(\lambda_{\Lambda_l}(L^H_{\Lambda_l})\) bounded
    below independently of \(l\) implies strong clustering only with
    respect to \(\lim_{n\to\infty}(E^\rho)^n\) -- a different, generally
    non-identical idempotent limit of \(E^\rho\) [Sec. III.B, remark
    following Eq. (13)] -- not with respect to \(E^\rho\) itself. This
    resolves the claim above completely for the Davies sampler
    \((L^D,E^L)\) whenever \(\Phi\) is a commuting potential; for the
    heat-bath sampler \((L^H,E^\rho)\), the direction gapped
    \(\Rightarrow\) strong clustering with respect to \(E^\rho\) itself is
    not established by Kastoryano-Brandão (2016), even when \(\Phi\) is
    commuting. The open content of the claim is therefore the extension
    to \(\Phi\) with \([\Phi(i),\Phi(j)]\ne0\) permitted, together with
    this residual heat-bath direction that Kastoryano-Brandão (2016)
    leave open even for commuting \(\Phi\).

    The definitional obstruction (Outlook, Sec. IX, verbatim).
    Immediately after summarizing their results, Kastoryano and Brandão
    (2016) write: "One major drawback of our framework is that it is not
    very well suited for Hamiltonians with non-commuting local terms.
    Indeed, it is easy to see that in general \(L_D\), \(L_H\),
    \(E_\rho\), and \(E_L\) all become non-local when \(H\) is
    non-commuting, and very little of the framework can be recovered. It
    would be very interesting to explore extensions of our results to
    non-commuting Hamiltonians, as it would incorporate many of the more
    interesting models in quantum statistical mechanics." This is the
    direct source of the claim recorded above and of the ambiguity
    flagged there: the sentence records a research direction in prose,
    not one of Kastoryano-Brandão (2016)'s three explicitly numbered Outlook conjectures
    (on, respectively, weak/strong clustering equivalence for \(d=2\)
    commuting \(\Phi\); transfer of ground-state clustering to low
    non-zero temperature for commuting \(\Phi\); and absence of 2D
    self-correction), none of which is the non-commuting extension.
    Kastoryano and Brandão (2016) do not resolve the non-commuting case,
    state a conjectural form of it, or supply a general existence
    argument for a local Gibbs sampler when \(\Phi\) is not commuting,
    anywhere else in the paper.

    The source of the obstruction, more precisely (Sec. III.B and IV).
    Kastoryano-Brandão (2016) prove that the minimal conditional
    expectation \(E^\rho_A\) satisfies the four conditional-expectation
    axioms (their Definition 8) for any full-rank \(\rho\) (their
    Proposition 10) -- so \(E^\rho\) remains well defined outside the
    commuting case -- but state, immediately after introducing it, that it
    "can potentially not exhibit any locality properties" for a general
    \(\rho\), and prove \(r''\)-locality of \(E^\rho\) only "in the
    special case when \(\rho\) is the Gibbs state of a commuting
    Hamiltonian" (Sec. III.B, remark following Eq. (13)). Similarly, the
    Davies generator \(L^D\) is proved \(r'\)-local (their Lemma 11) only
    under the hypothesis that \(\Phi\) is an "\(r\)-local bounded and
    commuting potential." Consequently, for a general (non-commuting)
    \(\Phi\), the claim above is not merely unproved but existentially
    nontrivial already at the level of exhibiting a single coherent family
    of local Gibbs samplers \((L_l,E_l)_{l}\) of \((\rho_{\Lambda_l})_l\)
    with a common range as \(l\) grows; this entry's claim is conditional
    on such a family being given and does not itself assert its
    existence.

    Relation to companion entries in this collection (scope, not a source
    for this claim). Two other entries in this problem collection, drawn
    from the same survey, concern related but logically distinct
    questions about non-commuting quantum Gibbs samplers and do not
    resolve the claim above. `quantum-gibbs-sampler-rapid-mixing-2d`
    (QTD025) asks only whether some Davies generator has a
    size-independent gap throughout an independently-operationalized
    clustering-threshold regime, for \(d\ge2\), without addressing whether
    gap and Kastoryano-Brandão (2016) Definition-15-style strong
    clustering are equivalent (QTD025's own "strong clustering" is defined
    by a uniform-over-boundary-conditions ordinary covariance bound, a
    different formalization from Definition 15 above).
    `uniform-mlsi-for-one-dimensional-gibbs-samplers` (QTD026) asks
    whether a proven \(d=1\) spectral gap for a specific non-commuting
    -capable generator upgrades to a uniform log-Sobolev constant, not
    whether the gap is equivalent to clustering. Neither entry's Known
    section reports a resolution of the general strong-clustering
    -equivalent-to-gap statement for non-commuting \(\Phi\).

    Verification scope. This entry verifies Kastoryano-Brandão (2016)
    itself (fetched as arXiv:1409.3435 and cross-checked against the
    published Communications in Mathematical Physics pagination) and the
    two companion entries already present in this repository. It does not
    search the broader 2016-2026 literature on non-commuting quantum
    Gibbs samplers and Davies generators (e.g. further work building on
    the Chen-Kastoryano-Gilyén construction cited by QTD025 and QTD026)
    for a paper that states and proves or disproves the general
    strong-clustering-gap equivalence itself, as distinct from the
    one-directional gap or MLSI questions QTD025 and QTD026 already
    record. Residual doubt on whether such a paper exists is recorded
    here rather than investigated further, per this entry's verification
    scope.
  </known>

  <refs>
    <ref>Michael J. Kastoryano and Fernando G. S. L. Brandão, Quantum Gibbs Samplers: The Commuting Case, Communications in Mathematical Physics 344, 915-957 (2016), DOI: 10.1007/s00220-016-2641-8, arXiv:1409.3435, used throughout: Sec. II.A-II.C (lattice, algebra, \(L^p\) inner product, and covariance conventions, Eqs. (1)-(9)), Sec. III and Definitions 5, 8 with Propositions 9-10 (locally primitive/reversible Liouvillians, conditional expectations, the local Liouvillian projector \(E^L\) of Eq. (12), and the remark following Eq. (13) stating \(E^\rho_A\) is not a projector and has no guaranteed locality outside the commuting case), Sec. IV and Eqs. (25), (27) (\(r\)-local bounded potential and Gibbs state) and Lemma 11 (locality of the Davies generator only for commuting \(\Phi\)), Definition 7 (spectral gap), Definitions 14-15 and Eqs. (36)-(38) (weak and strong clustering), Definition 21 and the discussion following it (\(E^\rho\) not a projective conditional expectation), the informal Theorem 1 restated formally as Theorem 23 and Theorem 26 (which requires \(E\) projective) with Remark iv following Corollary 27 (the proved commuting-case equivalence for the Davies local Gibbs sampler \((L^D,E^L)\), and, for the heat-bath local Gibbs sampler \((L^H,E^\rho)\), only the asymmetric pair of implications Remark iv establishes), Theorem 23 together with Corollary 27 (strong clustering implies weak clustering), Theorem 28 (their equivalence for \(d=1\), commuting \(\Phi\)), and Sec. IX "Outlook" (the non-commuting extension left as a prose research direction, quoted verbatim in the Known section above, distinct from the three separately numbered Outlook conjectures).</ref>
  </refs>

  <ask>
    Settle, for every integer \(d\ge1\), every \(\beta\in(0,\infty)\),
    every \(r\)-local bounded potential \(\Phi\) on \(\mathbb Z^d\)
    (commuting or not), and every coherent family of local Gibbs samplers
    \((L_l,E_l)_{l\in\mathbb N_+}\) of \((\rho_{\Lambda_l})_{l\in
    \mathbb N_+}\), whether \(\inf_l\lambda_{\Lambda_l}(L_l)\gt0\) if and
    only if \((\rho_{\Lambda_l})_l\) satisfies strong clustering with
    respect to \((E_l)_l\) uniformly in \(l\).

    A proof must establish both implications for every \(d\), \(\beta\),
    \(\Phi\) (not assumed commuting), and every coherent family
    \((L_l,E_l)_l\) of local Gibbs samplers of \((\rho_{\Lambda_l})_l\):
    (a) if \(\inf_l\lambda_{\Lambda_l}(L_l)\gt0\) then
    \((\rho_{\Lambda_l})_l\) satisfies strong clustering with respect to
    \((E_l)_l\) uniformly in \(l\), and conversely (b) if
    \((\rho_{\Lambda_l})_l\) satisfies strong clustering with respect to
    \((E_l)_l\) uniformly in \(l\) then \(\inf_l\lambda_{\Lambda_l}
    (L_l)\gt0\). Because the claim is conditional on a coherent family of
    local Gibbs samplers being given, a proof may either (i) establish
    both implications for every family that exists, for every \(\Phi\)
    for which at least one such family exists (leaving \(\Phi\) for which
    none exists outside its scope, since the claim is vacuous there), or
    (ii) separately establish existence of a coherent family for every
    \(r\)-local bounded \(\Phi\) and then prove the equivalence for it;
    either suffices.

    A disproof must exhibit one specific tuple \((d,\beta,\Phi,
    (L_l,E_l)_l)\) as above, with \(\Phi\) an \(r\)-local bounded
    potential for some \(r\), together with a proof that exactly one of
    (i) \(\inf_l\lambda_{\Lambda_l}(L_l)\gt0\) and (ii)
    \((\rho_{\Lambda_l})_l\) satisfies strong clustering with respect to
    \((E_l)_l\) uniformly in \(l\) holds. A single such tuple disproves
    the claim, since the claim is universally quantified over
    \((d,\beta,\Phi,(L_l,E_l)_l)\); no argument about other tuples is
    needed. A tuple with \(\Phi\) commuting and \((L_l,E_l)_l=
    (L^D_{\Lambda_l},E^L)_l\) does not disprove the claim, since that case
    is Theorem 23 and Theorem 26 of Kastoryano-Brandão (2016), already
    proved (see the Known section above). A tuple with \(\Phi\) commuting
    and \((L_l,E_l)_l=(L^H_{\Lambda_l},E^\rho)_l\) cannot disprove the
    claim by exhibiting (ii) holding with (i) failing, by the same
    theorem; but such a tuple with (i) holding and (ii) failing is not
    excluded by Kastoryano-Brandão (2016) (see the Known section above)
    and would disprove the claim if established.

    The following count as progress, not as resolving the claim: (i) a
    proof of implication (a) or (b) alone, without the converse, for some
    non-commuting \(\Phi\); (ii) a proof restricted to one fixed \(d\)
    (including \(d=1\)) or to \(\Phi\) in a proper subclass of \(r\)-local
    bounded potentials (e.g. finite-range perturbations of a commuting or
    free-fermion \(\Phi\), or \(\Phi\) with an explicit small
    non-commuting part), without covering every \(r\)-local bounded
    \(\Phi\) on every \(\mathbb Z^d\); (iii) a proof for a specific
    non-commuting-capable construction of \((L_l,E_l)_l\) (for instance
    one built from Davies' 1974 general weak-coupling-limit construction,
    or from the Chen-Kastoryano-Gilyén noncommutative quantum Gibbs
    sampler), without a proof that this construction is a local Gibbs
    sampler in the sense of the Def section above (a common range
    \(r',r''\) independent of \(l\)) for every \(\Phi\) considered, since
    the equivalence proved for a non-local family does not resolve the
    claim as stated; (iv) a proof of a general existence result for
    coherent families of local Gibbs samplers, without also proving the
    gap-clustering equivalence for the family constructed; (v) numerical
    evidence, for finitely many \(l\), that a size-dependent gap estimate
    and a size-dependent clustering estimate track each other, without a
    proof of the \(l\to\infty\) equivalence; (vi) a proof of the analogous
    equivalence with strong clustering (Definition 15) replaced by weak
    clustering (Definition 14) throughout, without also showing the two
    clustering notions coincide for the \(\Phi\), \(E_l\) in question,
    since Kastoryano-Brandão (2016) show these notions coincide
    unconditionally only for \(d=1\) commuting \(\Phi\) (Theorem 28), so a
    weak-clustering version is a priori a different statement from the
    claim above outside that case.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
