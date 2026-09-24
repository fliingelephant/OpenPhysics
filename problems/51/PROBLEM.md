# 51 Thermal Recall Protocol Conjectures

```xml
<problem id="51">
  <status>open</status>
  <kind>proof</kind>
  <field>QTD</field>
  <name>Thermal Recall Protocol Conjectures</name>
  <keys>thermal recall memory-assisted Markovian thermal process memory-assisted Markovian thermal processes MeMTP memory-assisted thermal process beta-swap beta swap beta-cycle beta cycle beta-order beta order thermomajorization thermomajorisation future thermal cone extreme points extreme point convergence Czartowski de Oliveira Junior Korzekwa Conjecture 1 Conjecture 2 truncated protocol truncated MeMTP full protocol non-Markovian memory effects two-level thermalisation two-level thermalization elementary thermalisation neighbour thermalisation neighbor thermalization Gibbs-preserving thermal operations Markovian thermal process MTP total variation distance permutation decomposition neighbour transposition PRX Quantum finite temperature convergence infinite temperature limit memory size scaling exponential convergence rate polynomial convergence rate ancillary memory system quantum thermodynamics resource theory elementary thermal operations Lostaglio Korzekwa geometric structure of thermal cones weak Bruhat order reduced word</keys>

  <claim>
    Fix an integer \(d\ge2\), a Hamiltonian \(H=\sum_{i=1}^dE_i\Pi_i\) on a
    \(d\)-dimensional Hilbert space (\(E_i\in\mathbb R\), \(\Pi_i\) the
    rank-one projector onto \(|E_i\rangle\) of an orthonormal energy
    eigenbasis), and an inverse temperature \(\beta\ge0\), fixing the Gibbs
    vector \(\bm\gamma\) (defined below). Let \(\bm p\) range over
    probability vectors on \(\{1,\dots,d\}\) with \(p_i/\gamma_i\) pairwise
    distinct, so the \(\beta\)-order \(\pi_{\bm p}\) (defined below) is
    unique. For \(N\ge1\), let \(\bm\eta_M:=(1/N,\dots,1/N)\) be the Gibbs
    state of an \(N\)-dimensional memory with trivial (fully degenerate)
    Hamiltonian, and let \(\mathcal P^\Pi,\widetilde{\mathcal P}^\Pi\)
    denote the memory-assisted Markovian thermal process (MeMTP) protocols
    defined below, built from a chosen neighbour-transposition decomposition
    of a permutation matrix \(\Pi\) with respect to \(\bm p\) (defined
    below).

    Two logically independent conjectures of Czartowski, de Oliveira
    Junior, and Korzekwa, "Thermal recall: Memory-assisted Markovian
    thermal processes," PRX Quantum 4, 040304 (2023):

    Claim (a) [their Conjecture 1, restricted to \(\beta=0\)]. For
    \(\beta=0\): for every \(d\ge2\), every \(\bm p\), and every
    permutation matrix \(\Pi\) on \(d\) elements, there is a
    neighbour-transposition decomposition of \(\Pi\) with respect to
    \(\bm p\) such that, for every memory dimension \(N\ge1\), writing
    \(\widetilde{\mathcal P}^\Pi(\bm p\otimes\bm\eta_M)=\tilde{\bm
    q}_N\otimes\bm\eta_M\) and \(\mathcal P^\Pi(\bm p\otimes\bm\eta_M)=\bm
    q_N\otimes\bm\eta_M\) for the protocols built from that decomposition
    with memory size \(N\),
    \[
      \delta(\Pi\bm p,\tilde{\bm q}_N)\ \le\ \delta(\Pi\bm p,\bm q_N).
    \]

    Claim (b) [their Conjecture 2, restricted to \(\beta\in(0,\infty)\)].
    For every \(\beta\in(0,\infty)\): for every \(d\ge2\), every \(\bm p\),
    and every extreme point \(\bm p^{\pi'}\) of the future thermal cone
    \(C_+^{\mathrm{TO}}(\bm p)\) (defined below) whose \(\beta\)-order
    \(\pi'\) has matrix representation \(\Pi'=\Pi\Pi_{\bm p}\) for some
    permutation matrix \(\Pi\), there is a neighbour-transposition
    decomposition of \(\Pi\) with respect to \(\bm p\) such that, writing
    \(\widetilde{\mathcal P}^\Pi(\bm p\otimes\bm\eta_M)=\bm q_N\otimes
    \bm\eta_M\) for the truncated protocol of memory size \(N\) built from
    that decomposition,
    \[
      \delta(\bm q_N,\bm p^{\pi'})\ \xrightarrow{N\to\infty}\ 0.
    \]

    Neither claim is proved or disproved by the source; each is stated
    there only as a conjecture, backed by numerical evidence and by proved
    special cases (see Known).

    Ambiguity on decomposition choice. The source introduces
    \(\mathcal P^\Pi,\widetilde{\mathcal P}^\Pi\) via "let us decompose
    \(\Pi\) into neighbour transpositions," without asserting that this
    decomposition is unique or that either conjecture's truth is
    independent of which valid decomposition is used. Claims (a) and (b)
    above accordingly record the weaker, existential-over-decomposition
    reading in both cases; a reading universally quantified over every
    valid decomposition is also consistent with the source text, is
    strictly stronger, and is not the one recorded here (see Known for the
    reasoning).
  </claim>

  <def>
    All objects live in the energy-incoherent (classical) sector: a state
    is a probability vector \(\bm p=(p_1,\dots,p_d)\) on \(\{1,\dots,d\}\),
    representing the populations, in the eigenbasis \(\{|E_i\rangle\}\) of
    \(H=\sum_iE_i|E_i\rangle\langle E_i|\), of a density operator
    commuting with \(H\). \(\bm\gamma:=(\gamma_1,\dots,\gamma_d)\),
    \(\gamma_i:=e^{-\beta E_i}/Z\), \(Z:=\sum_ie^{-\beta E_i}\), is the
    Gibbs vector at inverse temperature \(\beta\ge0\) (at \(\beta=0\),
    \(\bm\gamma\) is uniform, \(\gamma_i=1/d\), for every \(H\)).

    Thermal operations (TO) and the future thermal cone. A quantum channel
    is a thermal operation if \(\mathcal E(\rho)=\operatorname{Tr}_E[U(\rho
    \otimes\gamma_E)U^\dagger]\) for a bath in its own Gibbs state
    \(\gamma_E\) and a joint unitary \(U\) commuting with the total
    Hamiltonian. Restricted to energy-incoherent states, existence of a TO
    from \(\bm p\) to \(\bm q\) is equivalent to existence of a
    Gibbs-preserving stochastic matrix \(\Lambda\) (\(\Lambda\bm\gamma=
    \bm\gamma\)) with \(\Lambda\bm p=\bm q\). Write
    \(C_+^{\mathrm{TO}}(\bm p):=\{\bm q:\text{such a }\Lambda\text{
    exists}\}\), the future thermal cone; it is a convex polytope with at
    most \(d!\) extreme points.

    \(\beta\)-order. For \(\bm p\) with \(p_i/\gamma_i\) pairwise distinct,
    \(\pi_{\bm p}\in S_d\) is the permutation with
    \(p_{\pi_{\bm p}^{-1}(1)}/\gamma_{\pi_{\bm p}^{-1}(1)}\ge\cdots\ge
    p_{\pi_{\bm p}^{-1}(d)}/\gamma_{\pi_{\bm p}^{-1}(d)}\); \(\Pi_{\bm p}\)
    is its \(d\times d\) permutation matrix, \(\Pi_{\bm p}\bm p=\bm
    p^\beta\) (the \(\beta\)-ordered rearrangement of \(\bm p\)). Indices
    \(i,j\) are neighbours in the \(\beta\)-order of \(\bm p\) if
    \(\pi_{\bm p}(i)=\pi_{\bm p}(j)\pm1\).

    Extreme points of \(C_+^{\mathrm{TO}}(\bm p)\). Every extreme point of
    \(C_+^{\mathrm{TO}}(\bm p)\) has the form \(\bm p^\pi\) for some
    \(\pi\in S_d\) (so \(\bm p^{\pi_{\bm p}}=\bm p\)), constructed as
    follows. Let \(f_{\bm p}^\beta:[0,1]\to[0,1]\) be the thermomajorization
    curve of \(\bm p\): the piecewise-linear function through \((0,0)\) and
    the points \(\bigl(\sum_{k\le\ell}\gamma_{\pi_{\bm p}^{-1}(k)},
    \sum_{k\le\ell}p_{\pi_{\bm p}^{-1}(k)}\bigr)\), \(\ell=1,\dots,d\). For
    \(\pi\in S_d\), set \(x_0^\pi:=0\),
    \(x_i^\pi:=\sum_{k=1}^i\gamma_{\pi^{-1}(k)}\) for \(i=1,\dots,d\) (so
    \(x_d^\pi=1\)), and \(y_i^\pi:=f_{\bm p}^\beta(x_i^\pi)\); then
    \(p_i^\pi:=y^\pi_{\pi(i)}-y^\pi_{\pi(i)-1}\). (The source's Appendix
    A.2, Lemma 6, states \(x_i^\pi\) with the unnormalized weight
    \(e^{-\beta E_{\pi^{-1}(i)}}\) in place of \(\gamma_{\pi^{-1}(i)}\);
    since \(f_{\bm p}^\beta\) is only defined on \([0,1]\), the normalized
    form is recorded here.)

    Total variation distance. \(\delta(\bm p,\bm
    q):=\tfrac12\sum_{i=1}^d|p_i-q_i|\).

    \(\beta\)-swap. For \(i\ne j\) with \(E_i\le E_j\), \(\Pi_{ij}^\beta\)
    is the \(d\times d\) stochastic matrix equal to the identity outside
    rows and columns \(i,j\), with entries on those two rows and columns
    given by \((\Pi_{ij}^\beta)_{ii}=1-e^{-\beta(E_j-E_i)}\),
    \((\Pi_{ij}^\beta)_{ji}=e^{-\beta(E_j-E_i)}\),
    \((\Pi_{ij}^\beta)_{ij}=1\), \((\Pi_{ij}^\beta)_{jj}=0\); at
    \(\beta=0\), \(\Pi_{ij}^0=\Pi_{ij}\), the transposition of
    \(i,j\). A \(\beta\)-swap sends \(\bm p\) to the extreme point
    \(\bm p^\pi\) with \(\pi\) equal to \(\pi_{\bm p}\) composed with the
    transposition of \(i,j\), exactly when \(i,j\) are neighbours in the
    \(\beta\)-order of \(\bm p\).

    Two-level thermalisation and MTP. For \(\lambda\in[0,1]\), the
    two-level partial thermalisation of levels \(i,j\) sends
    \(p_i\mapsto(1-\lambda)p_i+\lambda\frac{p_i+p_j}{\gamma_i+\gamma_j}
    \gamma_i\) and symmetrically for \(p_j\), fixing every other
    coordinate. A Markovian thermal process (MTP) is any channel obtained
    by composing such maps (equivalently, by integrating a Lindbladian with
    \(\mathcal L_t(\bm\gamma)=0\) and \([\mathcal L_t,\mathrm{ad}_{-iH}]=0\)
    for all \(t\)). \(C_+^{\mathrm{MTP}}(\bm p)\subseteq
    C_+^{\mathrm{TO}}(\bm p)\) is the set reachable from \(\bm p\) by MTPs;
    \(T_{ij}\) denotes the \(\lambda=1\) full two-level thermalisation, and
    it is a neighbour thermalisation exactly when \(i,j\) are neighbours in
    the current \(\beta\)-order.

    Memory-assisted MTP (MeMTP). For an \(N\)-dimensional memory \(M\)
    with trivial (fully degenerate) Hamiltonian, its Gibbs state at every
    \(\beta\) is \(\bm\eta_M:=(1/N,\dots,1/N)\). A channel on the
    \(d\)-dimensional system is an MeMTP with memory size \(N\) if it has
    the form \(\mathcal E(\rho)=\operatorname{Tr}_M[\mathcal
    E_{\mathrm{MTP}}(\rho\otimes\eta_M)]\) for an MTP \(\mathcal
    E_{\mathrm{MTP}}\) on system \(\otimes\) memory.

    \(\beta\)-swap protocol. For \(k=1,\dots,N\), define the \(k\)-th
    round \(\mathcal R_k^{(ij)}(\bm p\otimes\bm\eta_M):=\bigl(\prod_{\ell=
    1}^NT_{(i-1)N+k,\,(j-1)N+\ell}\bigr)(\bm p\otimes\bm\eta_M)\)
    (sequential neighbour thermalisations of the \(k\)-th copy of level
    \(i\) against every copy of level \(j\); well defined whenever \(i,j\)
    are neighbours in the current \(\beta\)-order). The truncated
    \(\beta\)-swap protocol is
    \(\widetilde{\mathcal P}^{(ij)}:=\mathcal R_N^{(ij)}\circ\cdots\circ
    \mathcal R_1^{(ij)}\). Final thermalisation \(\mathcal T\) maps a joint
    system-memory state \(\bm Q\) to \(\bm q\otimes\bm\eta_M\),
    \(q_i:=\sum_{\ell=1}^NQ_{N(i-1)+\ell}\). The full \(\beta\)-swap
    protocol is \(\mathcal P^{(ij)}:=\mathcal T\circ\widetilde{\mathcal
    P}^{(ij)}\).

    Protocols for a general permutation. A neighbour-transposition
    decomposition of a permutation matrix \(\Pi\) with respect to \(\bm p\)
    is a sequence \((i_1,j_1),\dots,(i_m,j_m)\) such that, setting
    \(\bm p^{(0)}:=\bm p\), \(\bm p^{(k)}:=\Pi_{i_kj_k}\bm p^{(k-1)}\), each
    \(\Pi_{i_kj_k}\) transposes levels that are neighbours in the
    \(\beta\)-order of \(\bm p^{(k-1)}\), and \(\Pi_{i_mj_m}\cdots
    \Pi_{i_1j_1}=\Pi\). Given such a decomposition, define
    \[
      \mathcal P^\Pi:=\mathcal P^{(i_mj_m)}\circ\cdots\circ
      \mathcal P^{(i_1j_1)},\qquad
      \widetilde{\mathcal P}^\Pi:=\mathcal T\circ\widetilde{\mathcal
      P}^{(i_mj_m)}\circ\cdots\circ\widetilde{\mathcal P}^{(i_1j_1)}.
    \]
    (\(\mathcal P^\Pi\) re-thermalises the memory after every constituent
    swap; \(\widetilde{\mathcal P}^\Pi\) re-thermalises only once, at the
    very end.)
  </def>

  <known>
    Czartowski, de Oliveira Junior, and Korzekwa (2023) prove, at
    \(\beta=0\), for every \(d\), \(\bm p\), permutation \(\Pi\), and
    neighbour-transposition decomposition of \(\Pi\) with respect to
    \(\bm p\): their Lemma 1 gives \(\mathcal P^{(ij)}(\bm p\otimes\bm
    \eta_M)=\bm q\otimes\bm\eta_M\) with \(\bm q=(\Pi_{ij}+\epsilon(
    \mathbb 1-\Pi_{ij}))\bm p\), \(\epsilon=(\pi N)^{-1/2}+o(N^{-1/2})\)
    (their Eqs. 20-22); their Theorem 2 extends this to \(\mathcal
    P^\Pi(\bm p\otimes\bm\eta_M)=\bm q\otimes\bm\eta_M\) with
    \(\bm q=(\Pi+\epsilon\bm\Delta+o(N^{-1/2}))\bm p\to\Pi\bm p\) (Eqs.
    23-26, \(\bm\Delta\) an explicit decomposition-dependent operator);
    their Corollary 3 gives the bound \(\delta(\bm q',\bm q)\le
    d(d-1)/(2\sqrt{\pi N})+o(N^{-1/2})\) for reaching any \(\bm q\in
    C_+^{\mathrm{TO}}(\bm p)\) by composing \(\mathcal P^\Pi\) with an
    ordinary memory-free MTP within the target's \(\beta\)-order (Eqs.
    27-31), using \(\bm q\in C_+^{\mathrm{TO}}(\bm p)\Rightarrow\bm q\in
    C_+^{\mathrm{TO}}(\bm p^{\pi_{\bm q}})\) and "\([\bm q\in
    C_+^{\mathrm{TO}}(\bm p)\text{ and }\pi_{\bm q}=\pi_{\bm p}]
    \Rightarrow\bm q\in C_+^{\mathrm{MTP}}(\bm p)\)" (their Eqs. 9-10).
    None of Lemma 1, Theorem 2, or Corollary 3 proves Claim (a): all three
    concern only the full protocol \(\mathcal P^\Pi\), never the truncated
    \(\widetilde{\mathcal P}^\Pi\) that Claim (a) compares it against.

    At \(\beta\ne0\), their Theorem 4 proves, for the full protocol on a
    single \(\beta\)-swap, \(\mathcal P^{(ij)}(\bm p\otimes\bm\eta_M)=\bm
    q\otimes\bm\eta_M\) with
    \(\delta(\bm q,\Pi_{ij}^\beta\bm p)=\frac{(4\Gamma_i\Gamma_j)^N}
    {(\Gamma_i-\Gamma_j)^2}\bigl[\frac{|p_i\Gamma_j-p_j\Gamma_i|}
    {(N+1)\sqrt{\pi N}}+o(N^{-3/2})\bigr]\), \(\Gamma_i:=\gamma_i/
    (\gamma_i+\gamma_j)\) (Eqs. 34-35; this formula presupposes \(E_i\ne
    E_j\), which makes \(4\Gamma_i\Gamma_j\lt1\) and hence the
    convergence exponential, not merely polynomial, in \(N\)). Their
    Theorem 5 (proved in their Appendix E) proves convergence of the
    truncated protocol on \(\beta\)-3-cycles: for \(i_1,i_2,i_3\)
    neighbours in \(\pi_{\bm p}\) and \(\Pi=\Pi_{i_1i_3}\Pi_{i_2i_3}\),
    \(\widetilde{\mathcal P}^\Pi(\bm p\otimes\bm\eta_M)=\bm q\otimes\bm
    \eta_M\) with \(\delta(\bm q,\bm p^{\pi'})\to0\) as \(N\to\infty\)
    (Eqs. 38-39) -- i.e. Claim (b) restricted to this three-level family.
    Since only \(F(d+1)\) extreme points (\(F\) the Fibonacci sequence,
    \(F(d+1)\le d!\) with strict inequality for \(d\ge3\)) are reachable
    by composing non-overlapping \(\beta\)-swaps alone, Theorems 4-5 leave
    every extreme point outside the \(\beta\)-swap / \(\beta\)-3-cycle
    families unproved; Claim (b) is exactly the assertion that the
    truncated protocol closes this gap for every remaining extreme point.

    The source frames both conjectures explicitly as unproved. Conjecture
    1 is, verbatim, "solidified by strong numerical evidence (see Fig.
    6)," with no proof offered; the surrounding discussion records that
    equality (not strict advantage) holds for permutations equal to a
    single \(\beta\)-\(k\)-cycle, and that a strict advantage appears once
    at least two \(\beta\)-cycles are composed, but this is reported as an
    empirical observation on tested cases, not a proof for every \(\Pi\).
    Conjecture 2 is "solidified by the following two pieces of evidence"
    -- Theorem 5 plus "extensive numerical simulations" (Figs. 7 and 10)
    -- again with no proof of the general statement. Section V
    ("Conclusions and outlook") lists as the first item of future work:
    "one can try proving that the future thermal cone for memory-assisted
    Markovian thermal processes agrees with that of thermal operations in
    the limit of infinite memory, \(\lim_{N\to\infty}C^+_{\mathrm{MeMTP}}
    =C^+_{\mathrm{TO}}\), as suggested by Conjecture 2. This can be built
    upon the proofs for \(\beta\)-swaps (Theorem 4) and \(\beta\)-3-cycles
    (Theorem 5) presented in this work." No later part of the paper
    (checked in full, including every appendix, A through F) proves,
    disproves, or reports a proof of either conjecture; the
    acknowledgements report only an overlap with Son and Ng, arXiv:
    2303.13020 (2023), noticed during preparation, unrelated to
    Conjectures 1-2.

    A further, strictly stronger form is floated immediately after
    Conjecture 2 but not folded into it: "based on Theorem 4, the proof of
    Theorem 5, and numerical evidence, one can reasonably strengthen
    Conjecture 2" to the specific rate
    \(\delta(\bm q,\bm p^{\pi'})=O\bigl(e^{-A(\Pi)N}/N^{3/2}\bigr)\) (Eq.
    41, \(A(\Pi)=O(1)\) permutation-dependent). This rate refinement is
    not part of Claim (b) as recorded above, which asks only for the bare
    limit; a proof of Eq. (41) for every \(\Pi\) would imply Claim (b),
    but Claim (b) does not require it.

    Ambiguity on decomposition choice, elaborated. Neither conjecture's
    statement, nor its surrounding text, addresses whether a permutation
    \(\Pi\) can admit more than one neighbour-transposition decomposition
    with respect to \(\bm p\), or whether either conjecture's truth could
    depend on which decomposition is used to build \(\mathcal P^\Pi,
    \widetilde{\mathcal P}^\Pi\). A neighbour-transposition decomposition
    is a walk from \(\pi_{\bm p}\) to \(\Pi\pi_{\bm p}\) by adjacent
    transpositions, i.e. a reduced word in the weak order on \(S_d\);
    reduced words are generally non-unique once \(\Pi\) requires three or
    more transpositions (braid-relation rewrites), so this is a genuine,
    unaddressed gap in the source. The numerical evidence for both
    conjectures (Figs. 6, 7, 10) exercises one decomposition per tested
    permutation, not every possible decomposition of it, which supports
    only the weaker, existential-over-decomposition reading recorded in
    Claim (a)-(b); it does not by itself support the stronger reading
    quantified over every decomposition.
  </known>

  <refs>
    <ref>J. Czartowski, A. de Oliveira Junior, and K. Korzekwa, Thermal recall: Memory-assisted Markovian thermal processes, PRX Quantum 4, 040304 (2023), DOI: 10.1103/PRXQuantum.4.040304, arXiv:2303.12840, used for Definition 1 (MeMTP, Eq. 7), Definition 2 (beta-ordering, Eq. 8), the total variation distance (Eq. 11), the beta-swap (Eq. 13), Definition 3 and Eqs. (14)-(19b) (two-level and neighbour thermalisations and the protocols P^(ij), tilde-P^(ij), P^Pi, tilde-P^Pi), Lemma 1 (Eqs. 20-22), Theorem 2 (Eqs. 23-26), Corollary 3 (Eqs. 27-31), Conjecture 1 (Eqs. 32-33, Section III.4), Theorem 4 (Eqs. 34-35), Conjecture 2 (Eqs. 36-37, Section III.5), Theorem 5 and Appendix E (Eqs. 38-39), Eq. (41) (the strengthened rate conjecture), Section V ("Conclusions and outlook"), and Appendix A.2 (Lemma 6, the extreme-point construction), the source of the open-problem statement.</ref>
    <ref>M. Lostaglio, A. M. Alhambra, and C. Perry, Elementary Thermal Operations, Quantum 2, 52 (2018), DOI: 10.22331/q-2018-02-08-52, arXiv:1607.00394, used for the extreme-point construction of C_+^TO(p) (its Lemma 12, reproduced as Lemma 6 of Czartowski-de Oliveira Junior-Korzekwa, Appendix A.2) and for the beta-swap sourcing (their Eq. 9 and the remark on Eq. 13), cited as Ref. [27] by Czartowski-de Oliveira Junior-Korzekwa.</ref>
    <ref>M. Horodecki and J. Oppenheim, Fundamental limitations for quantum and nanoscale thermodynamics, Nature Communications 4, 2059 (2013), DOI: 10.1038/ncomms3059, used for the thermomajorization characterization of thermal-operation convertibility on energy-incoherent states (Section II of Czartowski-de Oliveira Junior-Korzekwa), cited as Ref. [22] there.</ref>
    <ref>M. Lostaglio and K. Korzekwa, Continuous thermomajorization and a complete set of laws for Markovian thermal processes, Physical Review A 106, 012426 (2022), DOI: 10.1103/PhysRevA.106.012426, arXiv:2111.12130, used for the fact that C_+^TO(p) and C_+^MTP(p) coincide within a fixed beta-order (Eq. 10 of Czartowski-de Oliveira Junior-Korzekwa, underlying their Corollary 3), cited as Ref. [25] there.</ref>
    <ref>A. de Oliveira Junior, J. Czartowski, K. Zyczkowski, and K. Korzekwa, Geometric structure of thermal cones, Physical Review E 106, 064109 (2022), DOI: 10.1103/PhysRevE.106.064109, used jointly with Lostaglio-Alhambra-Perry (2018) for the beta-swap and extreme-point results underlying Sections III.2-III.3 of Czartowski-de Oliveira Junior-Korzekwa, cited as Ref. [38] there.</ref>
  </refs>

  <ask>
    Resolve Claim (a) and Claim (b) above. They are logically independent
    and may be worked on separately, but a full "proof" or "disproof" of
    this entry requires settling both; settling only one (see (iv) below)
    is reported as "progress," with the settled claim's status stated
    explicitly and precisely (e.g. "Claim (a) proved; Claim (b) open").

    A proof of Claim (a) shows that, for every \(d\ge2\), every admissible
    \(\bm p\), and every permutation matrix \(\Pi\) on \(d\) elements,
    some neighbour-transposition decomposition of \(\Pi\) with respect to
    \(\bm p\) makes \(\delta(\Pi\bm p,\tilde{\bm q}_N)\le\delta(\Pi\bm
    p,\bm q_N)\) hold for every \(N\ge1\), proved in general rather than
    verified on finitely many \((d,\bm p,\Pi)\).

    A disproof of Claim (a) exhibits a specific \(d\ge2\), \(\bm p\), and
    \(\Pi\), and proves that for every neighbour-transposition
    decomposition of \(\Pi\) with respect to \(\bm p\) there is some
    \(N\ge1\) with \(\delta(\Pi\bm p,\tilde{\bm q}_N)\gt\delta(\Pi\bm
    p,\bm q_N)\). One decomposition failing at some \(N\) is not by itself
    a disproof unless every other decomposition of the same \(\Pi\) with
    respect to the same \(\bm p\) is also shown to fail at some \(N\).

    A proof of Claim (b) shows that, for every \(\beta\in(0,\infty)\),
    every \(d\ge2\), every admissible \(\bm p\), and every permutation
    \(\Pi\), some neighbour-transposition decomposition of \(\Pi\) with
    respect to \(\bm p\) makes \(\delta(\bm q_N,\bm p^{\pi'})\to0\) as
    \(N\to\infty\) (\(\pi'\) the permutation with \(\Pi'=\Pi\Pi_{\bm p}\)),
    proved in general. A proof of the strengthened rate of Eq. (41) for
    every \(\Pi\) also constitutes a proof of Claim (b), since that rate
    implies the bare limit; report it as resolving Claim (b) if obtained.
    Likewise, a proof of either claim under the stronger reading -- the
    stated inequality or limit holding for every neighbour-transposition
    decomposition of \(\Pi\), not merely some decomposition -- also proves
    the claim as recorded here (the recorded claims are implied by the
    stronger, universally-quantified ones); report this explicitly if it
    is what was proved.

    A disproof of Claim (b) exhibits a specific \(\beta\in(0,\infty)\),
    \(d\ge2\), \(\bm p\), and extreme point \(\bm p^{\pi'}\), and proves
    that for every neighbour-transposition decomposition of the associated
    \(\Pi\) with respect to \(\bm p\), \(\delta(\bm q_N,\bm p^{\pi'})\) does
    not tend to \(0\) as \(N\to\infty\) (for instance
    \(\limsup_N\delta(\bm q_N,\bm p^{\pi'})\gt0\), proved for every such
    decomposition).

    The following count as progress on the entry as a whole, not as a
    "proof" or "disproof" of it, even though the last of them is a
    complete resolution of one half of it: (i) extending Theorem 4 or
    Theorem 5 to further explicit families of permutations beyond
    \(\beta\)-swaps and \(\beta\)-3-cycles, without covering every
    permutation of every \(d\); (ii) proving either claim for one or
    finitely many fixed \(d\), or under a restriction on \(\bm p\) (e.g. a
    dense but proper subset), rather than for every \(d\ge2\) and every
    admissible \(\bm p\); (iii) numerical evidence alone, at any finite
    set of \((d,\bm p,\Pi,N)\), however extensive; (iv) a full,
    unqualified proof or disproof of only one of Claim (a), Claim (b),
    with the other left open.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
