# 44 Catalysable Future Region Exactness

```xml
<problem id="44">
  <status>open</status>
  <kind>proof</kind>
  <field>QTD</field>
  <name>Catalysable Future Region Exactness</name>
  <keys>catalytic thermal operations strict catalysis strict catalyst uncorrelated catalyst catalyst returned exactly catalytic future thermal cone catalysable future region catalysable set thermomajorization thermo-majorization beta-order beta-ordering thermomajorization curve tangent vectors thermal tangent vectors trumping catalytic majorization second laws of quantum thermodynamics generalized free energies Renyi divergence future thermal cone past thermal cone incomparable thermal region incomparability geometric thermal cones Birkhoff theorem catalysis extreme points populations energy-incoherent states Czartowski de Oliveira Junior Grabowecky Gour entanglement catalysts dimensionality bounds quantum thermodynamics resource theory Gibbs-preserving maps</keys>

  <claim>
    Fix a positive integer \(d\), a Hamiltonian \(H=\sum_{i=1}^dE_i\Pi_i\)
    on a \(d\)-dimensional system with the \(E_i\) pairwise distinct real
    numbers and the \(\Pi_i\) rank-one orthogonal projectors summing to the
    identity, and an inverse temperature \(\beta\geq0\). Let \(\v
    p\in\Delta_d\) be an energy-incoherent state of this system (a
    population vector, defined below).

    Then, with \(\mathcal T_{\mathcal C+}(\v p)\) the catalytic future
    thermal cone and \(\mathcal C_+(\v p)\) the catalysable future region,
    both defined below,
    \[
      \mathcal C_+(\v p)=\mathcal T_{\mathcal C+}(\v p).
    \]
    Equivalently, unpacking both sides: for every \(\v q\in\Delta_d\),
    \(\v q\) belongs to the geometric region \(\mathcal C_+(\v p)\)
    constructed in Czartowski and de Oliveira Junior's Theorem 2 (below) if
    and only if \(\v q\perp_\beta\v p\) (defined below) and there exist an
    integer \(k\geq2\), a Hamiltonian \(H_C\) on a \(k\)-dimensional space,
    a catalyst population vector \(\v c\in\Delta_k\) (energy-incoherent
    state of \(H_C\)), and a thermal operation \(\mathcal E\) on the joint
    system \(H\otimes I_k+I_d\otimes H_C\) at inverse temperature \(\beta\)
    (defined below) with
    \[
      \mathcal E(\v p\otimes\v c)=\v q\otimes\v c
    \]
    exactly: the catalyst is returned unperturbed and uncorrelated from the
    main system. (The incomparability conjunct \(\v q\perp_\beta\v p\) is
    part of Definition 5 below and rules out the degenerate case in which
    \(\v q\) would already be reachable from \(\v p\), or vice versa,
    without any catalyst.)

    Of this equality, the inclusion \(\mathcal T_{\mathcal C+}(\v
    p)\subseteq\mathcal C_+(\v p)\) is already proved for every \(d\), every
    such \(H\), every \(\beta\geq0\), and every \(\v p\in\Delta_d\) (Known,
    below); the mathematically live content of the claim, and the target of
    the Ask below, is the reverse inclusion \(\mathcal C_+(\v
    p)\subseteq\mathcal T_{\mathcal C+}(\v p)\), i.e. that every \(\v q\) in
    the explicitly constructed geometric region \(\mathcal C_+(\v p)\) is
    actually achievable by some finite-dimensional strict catalyst.
  </claim>

  <def>
    Population vectors and the simplex. An energy-incoherent state of a
    \(d\)-level system with Hamiltonian \(H=\sum_{i=1}^dE_i\Pi_i\) is
    identified with its vector of eigenvalues (populations) \(\v
    p=(p_1,\ldots,p_d)\) in the \(d\)-dimensional probability simplex
    \[
      \Delta_d=\Big\{\v p=(p_1,\ldots,p_d)\in\mathbb R_{\geq0}^d:
      \textstyle\sum_ip_i=1\Big\}.
    \]

    Gibbs vector and slopes. Let \(Z:=\sum_{i=1}^de^{-\beta E_i}\) and
    \(\v\gamma:=\frac1Z(e^{-\beta E_1},\ldots,e^{-\beta E_d})\in\Delta_d\)
    (the thermal, or Gibbs, population vector). For \(\v p\in\Delta_d\),
    the slope vector \(\v s(\v p):=(p_1/\gamma_1,\ldots,p_d/\gamma_d)^\downarrow\)
    is the non-increasing rearrangement of the ratios \(p_i/\gamma_i\), with
    entries written \(s_1(\v p)\geq\cdots\geq s_d(\v p)\). The \(\beta\)-order
    of \(\v p\) is the permutation \(\pi_{\v p}\in\mathcal S_d\) (symmetric
    group on \(\{1,\ldots,d\}\)) satisfying
    \(s_{\pi_{\v p}(i)}(\v p)=p_i/\gamma_i\) for every \(i\); the
    \(\beta\)-ordered form of \(\v p\) is \(\v p^\beta:=
    (p_{\pi_{\v p}^{-1}(1)},\ldots,p_{\pi_{\v p}^{-1}(d)})\), and
    \(\v\gamma^\beta\) is \(\v\gamma\) reordered by the same \(\pi_{\v
    p}^{-1}\).

    Thermomajorization. The thermomajorization curve of \(\v p\) is the
    piecewise-linear function \(f^\beta_{\v p}:[0,1]\to[0,1]\) through
    \((0,0)\) and the points \(\big(\sum_{i=1}^k\gamma^\beta_i,
    \sum_{i=1}^kp^\beta_i\big)\) for \(k=1,\ldots,d\). For \(\v
    p,\v q\in\Delta_d\), \(\v p\) thermomajorizes \(\v q\), written \(\v
    p\succ_\beta\v q\), iff \(f^\beta_{\v p}(x)\geq f^\beta_{\v q}(x)\) for
    every \(x\in[0,1]\); \(\v p\) and \(\v q\) are incomparable, written
    \(\v p\perp_\beta\v q\), iff neither \(\v p\succ_\beta\v q\) nor \(\v
    q\succ_\beta\v p\). At \(\beta=0\), \(\v\gamma\) is uniform and
    \(\succ_\beta\) reduces to ordinary majorization \(\succ\).

    Thermal operations and thermal cones. For a finite-dimensional system
    \(X\) with Hamiltonian \(H_X\), a thermal operation at inverse
    temperature \(\beta\) is a CPTP map \(\mathcal E(\rho)=
    \operatorname{Tr}_R\big[U(\rho\otimes\gamma_R)U^\dagger\big]\), where
    \(R\) is a finite-dimensional bath with Hamiltonian \(H_R\) and Gibbs
    state \(\gamma_R=e^{-\beta H_R}/\operatorname{Tr}e^{-\beta H_R}\), and
    \(U\) is a unitary on \(X\otimes R\) with \([U,H_X\otimes
    I_R+I_X\otimes H_R]=0\) (\(R\), \(H_R\), \(U\) freely chosen). For
    energy-incoherent \(\rho,\sigma\) on \(X\) with population vectors \(\v
    x,\v y\), the existence of such an \(\mathcal E\) with \(\mathcal
    E(\rho)=\sigma\) is equivalent to \(\v x\succ_\beta\v y\) (Horodecki and
    Oppenheim, below). The future, past, and incomparable thermal cones of
    \(\v p\in\Delta_d\) are
    \(\mathcal T_+(\v p):=\{\v q\in\Delta_d:\v p\succ_\beta\v q\}\),
    \(\mathcal T_-(\v p):=\{\v q\in\Delta_d:\v q\succ_\beta\v p\}\),
    \(\mathcal T_\emptyset(\v p):=\{\v q\in\Delta_d:\v q\perp_\beta\v p\}\)
    (de Oliveira Junior, Czartowski, Życzkowski, and Korzekwa, below).

    Catalytic future thermal cone (Czartowski and de Oliveira Junior,
    Definition 5). For \(\v p\in\Delta_d\),
    \[
      \mathcal T_{\mathcal C+}(\v p):=\Big\{\v q\in\Delta_d:\v
      q\perp_\beta\v p,\ \exists\,k\geq2,\,H_C\text{ on }
      \mathbb C^k,\,\v c\in\Delta_k\text{ with }\v p\otimes\v
      c\succ_\beta\v q\otimes\v c\Big\},
    \]
    where \(\v p\otimes\v c\in\Delta_{dk}\) is the population vector with
    entries \(p_ic_j\) and \(\succ_\beta\) on \(\Delta_{dk}\) is computed
    with respect to the joint Hamiltonian \(H\otimes I_k+I_d\otimes H_C\)
    (so \(H_C\), like \(k\) and \(\v c\), is existentially quantified: the
    source does not fix a catalyst Hamiltonian, e.g. does not assume
    \(H_C\propto I_k\)).

    Thermal tangent vectors (Czartowski and de Oliveira Junior, Definition
    4). For \(\v p\in\Delta_d\), \(n\in\{1,\ldots,d\}\), and
    \(\pi\in\mathcal S_d\), the tangent vector \(\v t^{(n,\pi)}(\v
    p)\in\mathbb R^d\) is defined entrywise by
    \[
      \big(\v t^{(n,\pi)}(\v p)\big)_{\pi(i)}=s_n(\v p)\,\gamma_{\pi(i)},
      \qquad 2\leq i\leq d-1,
    \]
    \[
      \big(\v t^{(n,\pi)}(\v p)\big)_{\pi(1)}=\sum_{j=1}^np_j^\beta-s_n(\v
      p)\Big(\sum_{j=1}^n\gamma_j^\beta-\gamma_{\pi(1)}\Big),
    \]
    \[
      \big(\v t^{(n,\pi)}(\v p)\big)_{\pi(d)}=1-\big(\v t^{(n,\pi)}(\v
      p)\big)_{\pi(1)}-s_n(\v p)\sum_{i=2}^{d-1}\gamma_{\pi(i)}.
    \]
    (\(\v t^{(n,\pi)}(\v p)\) may have a negative \(\pi(1)\)-th or
    \(\pi(d)\)-th entry; this "quasi-probability" possibility does not
    affect its use below, where it enters only through a convex hull.)

    Catalysable future region (Czartowski and de Oliveira Junior, Theorem
    2). For \(i\in\{1,d\}\), let \(T_i(\v p):=\operatorname{conv}\big(\{\v
    t^{(i,\pi)}(\v p):\pi\in\mathcal S_d\}\big)\subset\mathbb R^d\). The
    catalysable future region of \(\v p\) is
    \[
      \mathcal C_+(\v p):=\big(T_1(\v p)\cap T_d(\v p)\big)\setminus
      \mathcal T_+(\v p).
    \]
  </def>

  <known>
    Czartowski and de Oliveira Junior, Catalytic transformations for
    thermal operations, Physical Review Research 6, 033203 (2024), prove
    the inclusion used in the claim above. Their Lemma 1 (Catalytic
    condition) shows that \(\v q\perp_\beta\v p\) and \(\v q\in\mathcal
    T_{\mathcal C+}(\v p)\) together imply \(s_1(\v p)\gt s_1(\v q)\) and
    \(s_d(\v p)\lt s_d(\v q)\); their Theorem 2 (Catalysable regions) then
    shows this necessary slope condition, combined with the tangent-vector
    construction above, forces \(\mathcal T_{\mathcal C+}(\v
    p)\subseteq\mathcal C_+(\v p)\). Immediately after Theorem 2 (Section
    III.A) they restate this in words: "there is no thermodynamic process
    aided by a strict catalyst that can bring the initial state out of the
    catalysable future \(\mathcal C_+\)", and then flag the reverse
    direction as unresolved: "given a state \(\v q\) belonging to such a
    region, whether there exists a catalyst \(\v c\) and a thermal
    operation \(\mathcal E\), such that \(\mathcal E(\v p\otimes\v c)=\v q\)
    still remains an open problem." Their Section V (Summary and
    Discussion) restates the same gap: "While membership in this region
    does not guarantee the existence of a catalyst, non-membership
    guarantees its non-existence." No later section, corollary, or
    application in this paper closes the gap for the general case; the
    paper's own examples (their Observation, Thermal catalysability in
    \(d=3\)) exhibit specific \(\v p,\v q\) with \(\v q\in\mathcal C_+(\v
    p)\) and an explicit witnessing catalyst, but do not address
    achievability of every point of \(\mathcal C_+(\v p)\) in general.

    Note on the theorem's own inline shorthand: immediately after stating
    the two defining equations for \(\mathcal C_+,\mathcal C_-\), Theorem 2
    adds the sentence "That is, \(\mathcal C_{-/+}(\v p)\subset\mathcal
    T_{\mathcal C-/+}\)", with the containment symbol printed in the
    opposite direction from the one used in the claim above. This entry
    follows the substantive argument given in the theorem's proof and in
    the two restatements quoted above (both explicitly say membership in
    \(\mathcal C_+\) is necessary but not known to be sufficient for
    catalytic reachability), which is internally consistent only with
    \(\mathcal T_{\mathcal C+}(\v p)\subseteq\mathcal C_+(\v p)\); a
    reviewer should re-check this inline shorthand against the published
    (not just arXiv) typesetting if the direction matters to a proposed
    solution.

    This entry is distinct from 44's neighbor entry
    60, which asks
    for an explicit necessary-and-sufficient monotone criterion (a family
    of generalized free energies \(F_\alpha\)) for strict catalytic
    convertibility between arbitrary, possibly energy-coherent states, and
    records that even the energy-incoherent case of that criterion is not
    independently confirmed as exact by the sources reviewed there. The
    claim here is narrower and purely geometric: it asks only whether one
    specific, explicitly constructed polytope-intersection region (with
    known extreme points, the closest analogue to Birkhoff's theorem for
    this problem per Czartowski and de Oliveira Junior's own framing)
    coincides with the true
    catalytic future cone, for energy-incoherent \(\v p\) only.

    The issue tracker lead for this entry additionally asserts, without a
    citation, that "a 2026 result certifies part of the interior" of the
    open direction. No source for this claim was supplied, and this entry
    does not rely on it; see this entry's returned notes for the residual
    doubt this leaves.

    Horodecki and Oppenheim, Fundamental limitations for quantum and
    nanoscale thermodynamics, Nature Communications 4, 2059 (2013), is the
    source (cited by Czartowski and de Oliveira Junior, Section II.A) for
    the equivalence between thermal-operation reachability and
    thermomajorization for energy-incoherent states, used above to connect
    the algebraic condition \(\v p\otimes\v c\succ_\beta\v q\otimes\v c\)
    in Definition 5 to the explicit-operation phrasing \(\mathcal E(\v
    p\otimes\v c)=\v q\otimes\v c\) used in the open-problem sentence and
    in the Ask below.

    de Oliveira Junior, Czartowski, Życzkowski, and Korzekwa, Geometric
    structure of thermal cones, Physical Review E 106, 064109 (2022), is
    the source (cited by Czartowski and de Oliveira Junior, Section II) for
    the future/past/incomparable thermal cone terminology and the
    light-cone analogy used in the Def section above.
  </known>

  <refs>
    <ref>J. Czartowski and A. de Oliveira Junior, Catalytic transformations for thermal operations, Physical Review Research 6, 033203 (2024), DOI: 10.1103/PhysRevResearch.6.033203, arXiv:2403.04845 (v2, 27 August 2024), used for Definition 4 (thermal tangent vectors), Definition 5 (catalytic thermal cones), Lemma 1 (catalytic condition), Theorem 2 (catalysable regions, Section III.A) and its immediately following discussion paragraph stating the open problem, and Section V (Summary and Discussion) restating the same open problem.</ref>
    <ref>M. Horodecki and J. Oppenheim, Fundamental limitations for quantum and nanoscale thermodynamics, Nature Communications 4, 2059 (2013), DOI: 10.1038/ncomms3059, used for the equivalence between existence of a thermal operation and thermomajorization for energy-incoherent states, underlying the two equivalent phrasings of the claim above.</ref>
    <ref>A. de Oliveira Junior, J. Czartowski, K. Życzkowski, and K. Korzekwa, Geometric structure of thermal cones, Physical Review E 106, 064109 (2022), DOI: 10.1103/PhysRevE.106.064109, arXiv:2207.02237, used for the definitions of the future, past, and incomparable thermal cones \(\mathcal T_+,\mathcal T_-,\mathcal T_\emptyset\) used in the Def section above.</ref>
  </refs>

  <ask>
    Settle the reverse inclusion \(\mathcal C_+(\v p)\subseteq\mathcal
    T_{\mathcal C+}(\v p)\) for every positive integer \(d\), every
    Hamiltonian \(H\) on \(\mathbb C^d\) with pairwise distinct
    eigenvalues, every \(\beta\geq0\), and every \(\v p\in\Delta_d\); a
    proof of this reverse inclusion, combined with the already-known
    forward inclusion recorded above, establishes the full claim \(\mathcal
    C_+(\v p)=\mathcal T_{\mathcal C+}(\v p)\). Concretely, a proof must
    show that for every \(\v q\in\mathcal C_+(\v p)\) there exist a finite
    \(k\geq2\), a Hamiltonian \(H_C\) on \(\mathbb C^k\), a state \(\v
    c\in\Delta_k\), and a thermal operation \(\mathcal E\) on \(H\otimes
    I_k+I_d\otimes H_C\) at inverse temperature \(\beta\) with \(\mathcal
    E(\v p\otimes\v c)=\v q\otimes\v c\) exactly.

    A disproof must exhibit a specific \(d\), \(H\), \(\beta\), and \(\v
    p\in\Delta_d\) together with a specific \(\v q\in\mathcal C_+(\v p)\)
    for which no finite \(k\), no \(H_C\), and no \(\v c\in\Delta_k\) admit
    a thermal operation with \(\mathcal E(\v p\otimes\v c)=\v q\otimes\v
    c\); since the forward inclusion is already established, a single such
    example refutes the equality claim without needing any further
    impossibility argument over dimensions.

    The following count as progress, not as a solution to the claim, unless
    combined with a proof removing the stated restriction: (i) achievability
    results for a fixed, bounded catalyst dimension \(k\) or a fixed small
    \(d\), rather than for every finite \(k\) and every \(d\); (ii)
    achievability results that allow the catalyst to be returned only to
    within a nonzero trace-distance error, rather than exactly; (iii)
    achievability results only on the topological interior of \(\mathcal
    C_+(\v p)\) (relative to the affine hull of \(\Delta_d\)) or only on a
    measure-zero subset (e.g. an explicit finite family of extreme points,
    or of boundary points) of \(\mathcal C_+(\v p)\), rather than for every
    \(\v q\) in the region; (iv) results for the catalysable past region
    \(\mathcal C_-(\v p)\) instead of \(\mathcal C_+(\v p)\), unless
    accompanied by a proof that the same argument transfers to the future
    region; (v) a new necessary-and-sufficient monotone criterion for
    strict catalytic convertibility (in the sense of the neighboring entry
    60) that is not also shown to reproduce \(\mathcal C_+(\v p)\)
    exactly as a set.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
