# 59 Star Optimality for the Uniform Overlap

```xml
<problem id="59">
  <status>open</status>
  <kind>proof</kind>
  <field>QTD</field>
  <name>Star Optimality for the Uniform Overlap</name>
  <keys>star graph star topology star optimality conjecture quantum battery quantum batteries fermionic battery graph-structured battery charging power average charging power early-time charging power maximal average power uniform overlap minimal energy uniform overlap w_min fundamental weights of a graph adjacency matrix adjacency spectrum spectral graph theory spectral graph inequality least eigenvalue minimum eigenvalue ground eigenspace hub-and-spoke topology complete bipartite graph K_{1,N-1} Conjecture 1 Sarkis Prosniak Nigro Tkatchenko battery architecture design regular graph almost-regular graph spectral expander Ramanujan graph pineapple graph Collatz-Sinogowitz irregularity quantum thermodynamics of batteries</keys>

  <claim>
    Let \(G=(V,E)\) be a simple, undirected, unweighted graph with \(E\ne\emptyset\),
    let \(N:=|V|\), and let \(S=S_N\) denote the star graph on \(N\) vertices (def
    below). Then
    \[
      w_{\mathrm{min}}(G)\;\le\;w_{\mathrm{min}}(S)\;=\;\frac{N}{2}-\sqrt{N-1},
    \]
    where \(w_{\mathrm{min}}(\cdot)\) is the minimal-eigenvalue uniform overlap of
    a graph's adjacency matrix (def below).

    This is Sarkis, Prośniak, Nigro, and Tkatchenko, "Star Topology Optimizes the
    Charging Power of Quantum Batteries," arXiv:2603.10853 (2026) [full citation in
    refs], Conjecture 1 ("Star optimality for the uniform overlap"), their Eq. (24),
    stated verbatim up to notation fixed locally below. The equality
    \(w_{\mathrm{min}}(S)=N/2-\sqrt{N-1}\) on the right-hand side is an already
    established computation (known below, "Established value for the star"), not
    itself open; only the inequality \(w_{\mathrm{min}}(G)\le w_{\mathrm{min}}(S)\)
    for every non-empty \(G\) is open. No uniqueness of the maximizer is asserted:
    the claim is the bound alone, not that \(S\) is the unique graph attaining it
    (the weaker, precisely source-verified reading; the source states no separate
    uniqueness clause for Conjecture 1 itself).
  </claim>

  <def>
    Graph and adjacency matrix. \(G=(V,E)\) is simple (no loops or multi-edges),
    undirected, and unweighted; \(N:=|V|\), \(M:=|E|\). The adjacency matrix
    \(A\in\{0,1\}^{N\times N}\) is symmetric with zero diagonal, \(A_{ij}=1\) iff
    \(\{i,j\}\in E\). "Non-empty" means \(E\ne\emptyset\) (\(M\ge1\)); since \(G\)
    is simple this forces \(N\ge2\).

    Adjacency spectrum and uniform overlaps. Since \(A\) is real symmetric,
    \(\mathbb R^N\) admits an orthonormal eigenbasis \(u_1,\dots,u_N\),
    \(Au_k=\varepsilon_ku_k\), \(\varepsilon_k\in\mathbb R\). Write
    \(\mathbf 1:=(1,\dots,1)^\top\in\mathbb R^N\) and define, for each \(k\), the
    uniform overlap \(v_k:=\mathbf 1^\top u_k\) and its square
    \(w_k:=v_k^2\ge0\); \(w_k\) does not depend on the sign convention chosen for
    \(u_k\). By Parseval, \(\sum_{k=1}^Nw_k=\|\mathbf 1\|_2^2=N\).

    Minimal-eigenvalue uniform overlap. Let
    \(\varepsilon_{\mathrm{min}}(G):=\min_{1\le k\le N}\varepsilon_k\) (the least
    eigenvalue of \(A\)) and \(K_{\mathrm{min}}(G):=\{k:\varepsilon_k=\varepsilon_{\mathrm{min}}(G)\}\)
    (possibly more than one index, if \(\varepsilon_{\mathrm{min}}(G)\) is
    degenerate). Define
    \[
      w_{\mathrm{min}}(G):=\sum_{k\in K_{\mathrm{min}}(G)}w_k
      \;=\;\mathbf 1^\top P_{\mathrm{min}}\,\mathbf 1,
    \]
    where \(P_{\mathrm{min}}\) is the orthogonal projector of \(\mathbb R^N\) onto
    the eigenspace \(\{x\in\mathbb R^N:Ax=\varepsilon_{\mathrm{min}}(G)x\}\); the
    second expression shows \(w_{\mathrm{min}}(G)\) is independent of which
    orthonormal eigenbasis is chosen inside a degenerate eigenspace (matching the
    source's own remark that \(w_{\mathrm{min}}\) "is invariant under the unitary
    symmetry of the eigenspace").

    Scope check (non-emptiness is load-bearing). If \(E=\emptyset\) then \(A=0\),
    so \(\varepsilon_{\mathrm{min}}(G)=0\) with \(K_{\mathrm{min}}(G)=\{1,\dots,N\}\)
    and \(w_{\mathrm{min}}(G)=\sum_kw_k=N\) (Parseval above), which exceeds
    \(N/2-\sqrt{N-1}\) for every \(N\ge3\). The hypothesis \(E\ne\emptyset\) in the
    claim is therefore necessary, not a stylistic restriction.

    Star graph. For \(N\ge2\), \(S=S_N\) is the graph on vertex set
    \(\{0,1,\dots,N-1\}\) with edge set \(\{\{0,j\}:1\le j\le N-1\}\) (hub \(0\)
    adjacent to every other vertex; no edges among \(1,\dots,N-1\)); \(M=N-1\).
    Equivalently \(S_N=K_{1,N-1}\), the complete bipartite graph with parts of
    size \(1\) and \(N-1\).
  </def>

  <known>
    Physical origin and naming provenance (context, not part of the claim). Sarkis
    et al. (2026), Sec. II.2, attach \(A\) to a fermionic "battery" Hamiltonian
    \(H_{\textsc b}^0=h\sum_{i\in V}c_i^\dagger c_i+\frac{N}{M}\sum_{\{i,j\}\in
    E}A_{ij}(c_i^\dagger c_j+\mathrm{h.c.})\) with \(h\ge0\), charged through a
    fermionic device by an interaction \(H_{\mathrm{int}}\) uniform across sites
    (their Eq. (7)), and study the average charging power \(P(t)\) of Eq. (8)-(9).
    Diagonalizing \(A\) as above and setting \(E_k(h):=\frac{N}{M}\varepsilon_k+h\),
    their Eq. (15)-(16) reduce the early-time power to
    \(R_G(h):=\frac1{E_{\min}}\sum_{k\in K_-(h)}w_kE_k(h)\), a weighted average of
    the \(w_k\) over the "active" negative-energy modes
    \(K_-(h):=\{k:E_k(h)\lt0\}\), where \(E_{\min}:=\sum_{k\in K_-(h)}E_k(h)\) is a
    many-body sum over the whole active sector — not the same object as the
    single-mode \(\varepsilon_{\mathrm{min}}(G)\) fixed in def above, despite the
    similar name; the two are not conflated anywhere in def or claim.
    Their Observation 1 isolates \(w_{\mathrm{min}}:=\sum_{k'}w_{k'}\) over
    \(k'\in\arg\min_kE_{k'}(h)\) as the mode receiving the largest weight in that
    average, motivating its name "minimal energy uniform overlap." Because
    \(E_k(h)-E_j(h)=\frac{N}{M}(\varepsilon_k-\varepsilon_j)\) does not depend on
    \(h\) and \(N/M\gt0\), the ordering of the \(E_k(h)\) across \(k\) equals the
    ordering of the \(\varepsilon_k\) for every \(h\); hence
    \(\arg\min_kE_k(h)=\arg\min_k\varepsilon_k\) for every \(h\), which is exactly
    \(K_{\mathrm{min}}(G)\) as fixed in def above, independently of \(h\). This is
    why \(w_{\mathrm{min}}\) is written as a function of \(G\) alone in Conjecture
    1, with no \(h\)-argument, even though it is called an "energy" quantity.

    Established value for the star. Sarkis et al. (2026), Observation 2, compute
    the adjacency spectrum of \(S\) exactly as \(\{\sqrt{N-1},-\sqrt{N-1},0,\dots,0\}\)
    (simple extremal eigenvalues, \(N-2\) zero modes), identify \(K_{\mathrm{min}}(S)\)
    as the single mode at \(-\sqrt{N-1}\), and evaluate their Eq. (23),
    \(R_S(h)\equiv w_{\mathrm{min}}(S)=\frac N2-\sqrt{N-1}\), for \(h\) in a window
    containing \(0\). This value, which is exactly the right-hand side of the
    claim above, is a closed-form computation, independently re-derivable from def
    above (the eigenvector at \(-\sqrt{N-1}\) is
    \(u=(-1/\sqrt2,\,b,\dots,b)\) with \(b=1/\sqrt{2(N-1)}\), giving
    \(v=\mathbf 1^\top u=(\sqrt{N-1}-1)/\sqrt2\) and \(v^2=N/2-\sqrt{N-1}\)); it is
    not itself in question.

    A related but logically separate proven result at \(h=0\) (not a direct proof
    of this claim). Sarkis et al. (2026), Lemma 1 and Theorem 1, prove that for
    \(h=0\) the star maximizes \(R_G(0)=\frac1{\varepsilon_+}\sum_{k\in
    K_-}w_k|\varepsilon_k|\) (\(\varepsilon_+:=\sum_{\ell\in K_+}\varepsilon_\ell\),
    their Eq. (17)-(18)) among all graphs on \(N\) vertices, via a degree-norm
    bound (Eq. (20)) that is monotonically worst-cased at \(M=N-1\) and saturated
    by the star; disconnected graphs are separately shown strictly worse "(algebra
    omitted)" in their proof of Theorem 1. \(R_G(0)\) is a weighted average of
    \(w_k\) over the entire negative sector \(K_-\), not only over
    \(K_{\mathrm{min}}(G)\); it equals \(w_{\mathrm{min}}(G)\) only when
    \(K_-(0)=K_{\mathrm{min}}(G)\), which the source verifies holds for the star
    (a single negative eigenvalue) but does not assert for general \(G\). The
    source itself treats Theorem 1 and Conjecture 1 as separate results: Sec.
    III.3 introduces \(w_{\mathrm{min}}\) as a new quantity for the \(h\gt0\)
    generalization precisely because "we cannot prove that \(R_G(h)\) is governed
    by \(w_{\mathrm{min}}\)." Theorem 1 is therefore evidence and motivation for
    the claim above, not a proof of an \(h=0\) special case of it.

    Partial results that do establish the claim on restricted graph families.
    (a) Proposition 1: if \(G\) is connected and \(d\)-regular, every eigenvector
    with \(\varepsilon_k\ne d\) is orthogonal to \(\mathbf 1\), so
    \(w_{\mathrm{min}}(G)=0\le w_{\mathrm{min}}(S)\) trivially (since
    \(\varepsilon_{\mathrm{min}}(G)\ne d\) whenever \(G\) is not edgeless, which
    holds here as \(E\ne\emptyset\)). (b) Lemma 2: for any simple graph \(G\)
    with average degree \(\bar d\gt0\) (i.e. any non-empty \(G\), unconditionally,
    for every finite \(N\)), \(w_-(G):=\max_{k\in K_-}w_k\le
    N\delta_{\mathrm{rms}}^2\) (Eq. (28)), with \(\delta_{\mathrm{rms}}\) the
    relative root-mean-square degree fluctuation (Eq. (27)); since
    \(w_{\mathrm{min}}(G)\le w_-(G)\) always (\(K_{\mathrm{min}}(G)\subseteq K_-\)
    once \(\varepsilon_{\mathrm{min}}(G)\lt0\), true for non-empty \(G\)), this
    gives the explicit, finite-\(N\), unconditional bound
    \(w_{\mathrm{min}}(G)\le N\delta_{\mathrm{rms}}(G)^2\), which already proves
    the claim for any specific non-empty \(G\) whose \(N\) and
    \(\delta_{\mathrm{rms}}(G)\) happen to satisfy
    \(N\delta_{\mathrm{rms}}(G)^2\le N/2-\sqrt{N-1}\), without needing \(G\) to
    belong to a named family. Corollary 1 sharpens this along families: if
    \(\delta_{\mathrm{rms}}\le\delta_0\lt1/\sqrt2\) uniformly along a family
    \(\{G_N\}_N\) (in particular for any fixed-degree almost-regular expander
    family), then for all sufficiently large \(N\),
    \(w_-(G_N)/w_-(S_N)\le2\delta_0^2(1+o(1))\lt1\) (their Eq. (32)); combined
    with \(w_{\mathrm{min}}\le w_-\) and \(w_-(S)=w_{\mathrm{min}}(S)\) (single
    negative mode), this establishes the claim along such a family for all
    sufficiently large \(N\) in that family — an asymptotic, family-uniform
    statement, not a bound covering every non-empty graph at every finite
    \(N\). (c)
    Proposition 2 and Corollary 2: for the complete bipartite family \(K_{p,q}\)
    (\(p+q=N\)), the unique negative eigenvalue is \(-\sqrt{pq}\) with uniform
    overlap \((\sqrt p-\sqrt q)^2/2\) (Eq. (34)-(35)), so
    \(w_{\mathrm{min}}(K_{p,q})=w_-(K_{p,q})=(\sqrt p-\sqrt q)^2/2\) exactly, and
    this is uniquely maximized over \(p\in\{1,\dots,N-1\}\) at \(p\in\{1,N-1\}\),
    i.e. at \(S_N=K_{1,N-1}\) itself — the claim holds, with the star as the
    unique maximizer, restricted to this one-parameter family.

    A documented failed attempt at a general proof. Appendix F, Lemma 4, bounds
    \(w_k\le N\bigl(\varepsilon_{\max}-\tfrac{2M}N\bigr)/(\varepsilon_{\max}-\varepsilon_k)\)
    for \(\varepsilon_k\ne\varepsilon_{\max}\) (Eq. (48)). The source states
    explicitly, directly after proving Lemma 4, that "Lemma 4 does not provide a
    strong enough bound on \(w_{\mathrm{min}}\) for proving the conjecture 1, with
    pineapple graph \(P(N,\lfloor N/2\rfloor+1)\) being a counterexample" — i.e. a
    counterexample to this bounding technique being tight enough, not a
    counterexample to Conjecture 1 itself, which the source never claims is
    violated by any graph.

    Numerical support and its stated scope. Immediately before stating Conjecture
    1, Sec. III.3 reports "exhaustive numerical check of graphs with \(N\le7\) and
    sample check of larger random graphs," after which "we arrived with the
    following claim, which however we were unable to prove." Separately, Sec.
    IV.1 numerically probes the related, logically stronger envelope inequality
    \(w_-(G)/w_-(S)\le1\) (stronger because \(w_{\mathrm{min}}\le w_-\) pointwise
    above, so \(w_-(G)\le w_-(S)\) is sufficient, though not necessary, for
    \(w_{\mathrm{min}}(G)\le w_{\mathrm{min}}(S)\); it is not itself named as a
    conjecture in the source) by \(500\)-sample sweeps over
    Erdős–Rényi, uniform random tree, Barabási–Albert, and unbalanced
    stochastic-block-model graphs at \(N\in\{10,15,20,30,40,50,60\}\), reporting
    "no exceedances" (their footnote: "not an exhaustive search over all
    \(N\)-vertex graphs"). Sec. IV.2.1 separately reports direct simulation
    confirming star-maximal \(P_{\max}\) over "all connected graphs with up to
    \(N=7\) vertices."

    Residual doubt (recorded, not resolved by further search per this entry's
    instructions). (i) It is not stated in the fetched text, and not independently
    resolved here, whether the "exhaustive numerical check of graphs with
    \(N\le7\)" that directly precedes Conjecture 1 (Sec. III.3) ranged over all
    non-empty (possibly disconnected) graphs on up to \(7\) vertices, or only
    connected ones, since the paper's other stated \(N=7\) sweeps (Sec. IV.2.1,
    IV.2.3) are explicitly restricted to connected graphs. The claim above as
    stated by Conjecture 1 covers disconnected non-empty graphs too, and no
    argument in the fetched text (Theorem 1's disconnected-graph remark concerns
    \(R_G(0)\), not \(w_{\mathrm{min}}\), and does not transfer by the argument
    given in "related but logically separate" above) was found that separately
    rules out disconnected counterexamples. (ii) The proofs of Lemma 1, Theorem
    1, Proposition 1, Lemma 2, Proposition 2, and Lemma 4 were read from the
    arXiv HTML rendering of the source and checked for internal consistency (the
    star's closed-form value was independently re-derived above and matches
    exactly) but their algebraic steps were not independently re-verified line by
    line beyond that cross-check.
  </known>

  <refs>
    <ref>M. Sarkis, O. A. Prośniak, S. Nigro, and A. Tkatchenko, Star Topology Optimizes the Charging Power of Quantum Batteries, arXiv:2603.10853 [quant-ph, cond-mat.stat-mech], submitted 11 Mar 2026 (v1, the only version as of 2026-07-18), no journal reference as of 2026-07-18, arXiv-issued DOI: 10.48550/arXiv.2603.10853, used for: Sec. II.1 (graph and adjacency conventions); Sec. II.2 Eqs. (1)-(9) (battery/charger/interaction Hamiltonians, average and maximal charging power); Sec. II.3 Claim 1 (the broader, only partially proven, charging-power optimality claim that this entry's Conjecture 1 is one ingredient toward); Sec. III.1 Eqs. (10)-(17) (early-time spectral reduction to \(R_G(h)\)); Sec. III.2 Lemma 1 and Theorem 1 (the proven \(h=0\) result for \(R_G(0)\)); Sec. III.3 Observation 1 (definition of \(w_{\mathrm{min}}\)), Observation 2 and Eq. (23) (closed form \(w_{\mathrm{min}}(S)\)), Conjecture 1 and Eq. (24) (the claim of this entry), Observation 3 and Eq. (25) (the \(w_{\mathrm{min}}\le w_-\) bound); Sec. III.3.1 Definition 1, Proposition 1, Lemma 2, Remark 3, Corollary 1 (regular/almost-regular partial results); Sec. III.3.2 Definition 2, Proposition 2, Corollary 2 (complete-bipartite partial result); Sec. IV.1 (random-ensemble envelope-ratio numerics) and its footnote (non-exhaustiveness of that sweep); Sec. IV.2.1 (connected-graph \(N\le7\) direct-simulation sweep); Sec. V (conclusions, listing open extensions but not claiming Conjecture 1 resolved); Appendix F Lemma 4 and the sentence immediately following its proof (documented failed general-proof attempt and the pineapple-graph counterexample to that technique).</ref>
  </refs>

  <ask>
    A proof must show that for every simple, undirected, unweighted graph
    \(G=(V,E)\) with \(E\ne\emptyset\), writing \(N:=|V|\),
    \[
      w_{\mathrm{min}}(G)\le\frac N2-\sqrt{N-1}
    \]
    holds, with \(w_{\mathrm{min}}\) exactly as defined above (no restriction to
    connected, regular, almost-regular, complete-bipartite, or any other proper
    subfamily of non-empty graphs, and no restriction on \(N\) beyond
    \(N\ge2\), which is automatic).

    A disproof must exhibit one non-empty graph \(G_0\) on some \(N_0\ge2\)
    vertices with \(w_{\mathrm{min}}(G_0)\gt N_0/2-\sqrt{N_0-1}\); because the
    claim is a single inequality holding for every non-empty \(G\), one such
    \(G_0\) already disproves it and no further argument about other graphs is
    needed.

    The following count as progress, not as a solution to the claim, unless
    combined with a proof removing the stated restriction: (i) a proof restricted
    to a subfamily of non-empty graphs (e.g. an extension of the regular,
    almost-regular, or complete-bipartite families already handled in known
    above, or a new family such as trees, planar graphs, or bounded-degree
    graphs) without covering every non-empty \(G\); (ii) a bound of the form
    \(w_{\mathrm{min}}(G)\le f(N)\) for some \(f\) with \(f(N)\ge N/2-\sqrt{N-1}\)
    for all or infinitely many \(N\) (not matching the exact source right-hand
    side), or a bound proved only asymptotically as \(N\to\infty\) rather than
    for every finite \(N\ge2\); (iii) numerical verification extending the
    exhaustive check beyond \(N=7\), or clarifying whether the existing \(N\le7\)
    check (residual doubt above) covered disconnected graphs, without a general
    proof; (iv) a proof of the logically stronger envelope bound
    \(w_-(G)\le w_-(S)\) (Observation 3's quantity, known above; stronger because
    \(w_{\mathrm{min}}\le w_-\) pointwise, so this bound is sufficient but not
    necessary for the claim) restricted to a family of non-empty graphs narrower
    than all of them — a proof of \(w_-(G)\le w_-(S)\) covering every non-empty
    \(G\) would instead already establish the claim above in full, not merely
    progress; (v) a proof or disproof of the broader charging-power
    optimality (Claim 1 in known above) for \(h\gt0\) that does not go through
    \(w_{\mathrm{min}}(G)\le w_{\mathrm{min}}(S)\) at all, since that would bear
    on the source's motivating question without settling the specific inequality
    asked for here.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
