# QTD009 Markovian Thermal-Operation Generators from Finite-Bath Coupling

```xml
<problem id="QTD009">
  <status>open</status>
  <name>Markovian Thermal-Operation Generators from Finite-Bath Coupling</name>
  <keys>markovian thermal operations thermal operations TO(H0,T) Lindblad generator GKSL generator gksl-generator gksl-form Lie wedge Lie semigroup tangent cone edge of a Lie wedge energy-conserving coupling finite thermal bath finite-dimensional bath Stinespring dilation thermal bath coupling Conjecture 1 vom Ende Malvetti Dirr Schulte-Herbruggen Schulte-Herbrüggen Gibbs-preserving maps enhanced thermal operations EnTO MTO Markovian thermal operations semigroup majorisation semigroup majorization thermomajorisation Kossakowski GKS-Lindblad equation quantum-dynamical semigroup bilinear control systems reachable sets resource theory of quantum thermodynamics energy-conserving unitary system-bath coupling dissipator jump operators Lindblad master equation generator classification which Lindblad generators are thermal thermal generator classification converse of Theorem 2 closure of generators bath Hamiltonian total Hamiltonian energy conservation commutator infinitesimal generator</keys>

  <claim>
    Fix an integer \(n\ge1\). For \(H_0\in iu(n)\) (defined below) and
    \(T\in(0,\infty]\), let \(L(\overline{TO(H_0,T)})\subseteq L(\mathbb
    C^{n\times n})\) be the Lie wedge of \(\overline{TO(H_0,T)}\), the
    topological closure of the thermal-operations semigroup \(TO(H_0,T)\),
    and let \(G(H_0,T)\subseteq L(\mathbb C^{n\times n})\) be the set of all
    generators obtained by the finite-bath-coupling construction of Theorem
    2 (all objects defined below). Then:
    \[
      L(\overline{TO(H_0,T)}) \subseteq \overline{G(H_0,T)}
    \]
    holds for every integer \(n\ge1\), every \(H_0\in iu(n)\), and every
    \(T\in(0,\infty]\), where \(\overline{(\cdot)}\) denotes topological
    closure in \(L(\mathbb C^{n\times n})\).
  </claim>

  <def>
    Ambient spaces. \(\mathbb C^{n\times n}\) is the space of \(n\times n\)
    complex matrices. \(iu(n):=\{X\in\mathbb C^{n\times n}:X^\dagger=X\}\) is
    the (real) vector space of \(n\times n\) Hermitian matrices; likewise
    \(iu(m)\), \(iu(mn)\) for \(m\times m\), \(mn\times mn\) Hermitian
    matrices. For a positive integer \(k\),
    \(U(k):=\{U\in\mathbb C^{k\times k}:U^\dagger U=1_k\}\) is the group of
    \(k\times k\) unitary matrices, where \(1_k\) denotes the \(k\times k\)
    identity matrix. For \(H\in\mathbb C^{k\times k}\), \(\mathrm{ad}_H:\mathbb
    C^{k\times k}\to\mathbb C^{k\times k}\) is \(\mathrm{ad}_H(X):=HX-XH\).
    \(L(\mathbb C^{n\times n})\) is the space of \(\mathbb C\)-linear maps
    \(\mathbb C^{n\times n}\to\mathbb C^{n\times n}\) ("superoperators");
    being finite-dimensional (\(\dim_{\mathbb C}L(\mathbb
    C^{n\times n})=n^4\)), all norms on it induce the same topology, so
    "closure" below is unambiguous. \(CPTP(n)\subset L(\mathbb
    C^{n\times n})\) is the set of completely positive, trace-preserving
    linear maps (quantum channels); \(\mathrm{id}\in CPTP(n)\) denotes the
    identity channel.

    Partial trace. For \(X\in\mathbb C^{m\times m}\), \(\mathrm{tr}_X:\mathbb
    C^{n\times n}\otimes\mathbb C^{m\times m}\to\mathbb C^{n\times n}\) is the
    unique linear map with \(\mathrm{tr}(A\,\mathrm{tr}_X(B))=\mathrm{tr}((A
    \otimes X)B)\) for every \(A\in\mathbb C^{n\times n}\), \(B\in\mathbb
    C^{n\times n}\otimes\mathbb C^{m\times m}\); \(\mathrm{tr}_X\) reduces to
    the ordinary partial trace over the second factor when \(X=1_m\), and is
    then written \(\mathrm{tr}_B\).

    Thermal operations. For \(H_0\in iu(n)\) (system Hamiltonian) and
    \(T\in(0,\infty]\) (inverse-temperature parameter; \(T=\infty\) is the
    infinite-temperature limit \(e^{-H_B/T}\to1_m\)),
    \[
      TO(H_0,T):=\Bigl\{\Phi\in CPTP(n):\ \exists\,m\in\mathbb N,\
      H_B\in iu(m),\ U\in U(mn)\ \text{with}\
      U(H_0\otimes1_m+1_n\otimes H_B)U^\dagger=H_0\otimes1_m+1_n\otimes
      H_B,\\
      \text{such that}\ \Phi(\rho)=\mathrm{tr}_B\bigl[U(\rho\otimes
      \rho_B^{(T)})U^\dagger\bigr]\ \forall\rho\in\mathbb C^{n\times n}
      \Bigr\},
    \]
    where \(\rho_B^{(T)}:=e^{-H_B/T}/\mathrm{tr}(e^{-H_B/T})\) for
    \(T\in(0,\infty)\) and \(\rho_B^{(\infty)}:=1_m/m\). This is Eq. (10) of
    vom Ende, Malvetti, Dirr, and Schulte-Herbrüggen, Open Systems and
    Information Dynamics 30(1), 2350005 (2023). \(TO(H_0,T)\) is a bounded,
    path-connected sub-semigroup of \(CPTP(n)\) containing \(\mathrm{id}\)
    (vom Ende-Malvetti-Dirr-Schulte-Herbrüggen (2023), Proposition 3(i));
    its closure \(\overline{TO(H_0,T)}\) is a compact, convex sub-semigroup
    of \(CPTP(n)\) containing \(\mathrm{id}\) (Proposition 3(ii)).

    Lie wedge. For a closed sub-semigroup \(S\subseteq CPTP(n)\) with
    \(\mathrm{id}\in S\),
    \[
      L(S):=\{A\in L(\mathbb C^{n\times n}):e^{tA}\in S\ \text{for all}\
      t\in[0,\infty)\}
    \]
    (vom Ende-Malvetti-Dirr-Schulte-Herbrüggen (2023), Eq. (6), specialized
    to \(Z=iu(n)\); there, \(B(Z)\) denotes \(\mathbb R\)-linear self-maps
    of the real vector space \(iu(n)\), naturally identified with the
    real subspace of Hermiticity-preserving elements of \(L(\mathbb
    C^{n\times n})\), not with all of \(L(\mathbb C^{n\times n})\) -- this
    does not affect any set or closure below, since \(\overline{TO(H_0,T)}\)
    -type sets and \(G(H_0,T)\) are Hermiticity-preserving regardless of
    the ambient superoperator space). Because
    \(\overline{TO(H_0,T)}\) is closed (Proposition 3(ii)),
    \(L(\overline{TO(H_0,T)})\) is
    well defined by this formula; it is exactly the set of infinitesimal
    generators \(A\) such that the one-parameter semigroup
    \((e^{tA})_{t\ge0}\) stays inside \(\overline{TO(H_0,T)}\) for every
    \(t\ge0\) -- i.e. the "Markovian generators inside the closure of the
    thermal-operations set" of the classification question this entry
    records.

    Finite-bath-coupling generators (Theorem 2 construction). Fix
    \(m\in\mathbb N\), \(H_B\in iu(m)\) with a spectral decomposition
    \(H_B=\sum_{j=1}^mE_j^0|g_j\rangle\langle g_j|\) (\(E_j^0\in\mathbb R\),
    \(\{|g_j\rangle\}_{j=1}^m\) an orthonormal basis of \(\mathbb C^m\)), and
    \(H_{tot}\in iu(mn)\) with
    \[
      [H_{tot},\,H_0\otimes1_m+1_n\otimes H_B]=0
    \]
    (\(H_{tot}\) generates a joint system-bath unitary that conserves the
    bare system-plus-bath energy \(H_0\otimes1_m+1_n\otimes H_B\); together
    \((m,H_B,H_{tot})\) model an energy-conserving coupling of the system to
    a finite-dimensional thermal bath). For \(j,k\in\{1,\dots,m\}\) set
    \[
      V_{jk}:=e^{-E_k^0/(2T)}\,\mathrm{tr}_{|g_k\rangle\langle g_j|}(H_{tot})
      \ \in\mathbb C^{n\times n},
    \]
    and define \(\Gamma_{B,tot}\in L(\mathbb C^{n\times n})\) by
    \[
      \Gamma_{B,tot}(\rho):=\sum_{j,k=1}^m\Bigl[\tfrac12\bigl(V_{jk}^\dagger
      V_{jk}\rho+\rho V_{jk}^\dagger V_{jk}\bigr)-V_{jk}\rho V_{jk}^\dagger
      \Bigr].
    \]
    For \(H\in iu(n)\) with \([H,H_0]=0\), set
    \[
      A(m,H_B,H_{tot},H):=-i\,\mathrm{ad}_H-\Gamma_{B,tot}\ \in L(\mathbb
      C^{n\times n}).
    \]
    Then
    \[
      G(H_0,T):=\bigl\{A(m,H_B,H_{tot},H):\ m\in\mathbb N,\ H_B\in
      iu(m),\ H_{tot}\in iu(mn)\ \text{with}\ [H_{tot},H_0\otimes1_m+1_n
      \otimes H_B]=0,\ H\in iu(n)\ \text{with}\ [H,H_0]=0\bigr\}
    \]
    is the set of generators "arising from an energy-conserving coupling to
    a finite thermal bath" referred to in the claim.
  </def>

  <known>
    Theorem 2 (the proven inclusion). vom Ende-Malvetti-Dirr-Schulte
    -Herbrüggen (2023), Theorem 2, proves: for every
    \((m,H_B,H_{tot},H)\) as in the construction above, the solution
    \(\Phi(t)\) of \(\dot\Phi(t)=A(m,H_B,H_{tot},H)\,\Phi(t)\),
    \(\Phi(0)=\mathrm{id}\), satisfies \(\Phi(t)\in\overline{TO(H_0,T)}\)
    for every \(t\ge0\); consequently \(A(m,H_B,H_{tot},H)\in
    L(\overline{TO(H_0,T)})\) (their Eq. (15), end of the proof of Theorem
    2). Hence \(G(H_0,T)\subseteq L(\overline{TO(H_0,T)})\) for every
    \(n\), \(H_0\), \(T\) as in the claim -- this is the "one inclusion
    proven" of the issue that seeded this entry. Combining this with their
    Proposition 2(i) ("\(L(S)\) is a closed convex cone") and Proposition
    3(ii) (\(\overline{TO(H_0,T)}\) compact, hence closed),
    \(L(\overline{TO(H_0,T)})\) is itself a closed set, so
    \(\overline{G(H_0,T)}\subseteq L(\overline{TO(H_0,T)})\) already
    follows from Theorem 2 without any further hypothesis. The claim above
    is therefore exactly the reverse containment, and is equivalent to the
    set equality \(L(\overline{TO(H_0,T)})=\overline{G(H_0,T)}\).

    Conjecture 1 (the open direction). Immediately after proving Theorem 2,
    vom Ende-Malvetti-Dirr-Schulte-Herbrüggen (2023) state: "At this point,
    Andrzej Kossakowski would have insisted that Thm. 2 does not provide a
    complete characterisation yet -- and rightly so: we have to leave the
    converse open for now, but do so at the level of a well supported
    conjecture. Conjecture 1. The converse of Thm. 2 holds true, up to
    taking the closure of the collection of all these generators." This is
    the direct source of the claim recorded above:
    \(L(\overline{TO(H_0,T)})\subseteq\overline{G(H_0,T)}\).

    Supporting evidence offered by vom Ende-Malvetti-Dirr-Schulte
    -Herbrüggen (2023) for Conjecture 1 (motivation, not proof). (i) They
    note that the unconstrained analogue of Conjecture 1 -- dropping the
    commutator condition \([H_{tot},H_0\otimes1_m+1_n\otimes H_B]=0\) and
    replacing \(TO(H_0,T)\) with the full channel semigroup \(CPTP(n)\) --
    is already a theorem, not a conjecture, citing "[86, Thm. 2]", i.e.
    F. vom Ende, Quantum-Dynamical Semigroups and the Church of the Larger
    Hilbert Space, Open Systems and Information Dynamics 30(1), 2350003
    (2023), Theorem 2, up to the same closure operation and up to a
    full-rank-versus-arbitrary-ancilla difference that vom
    Ende-Malvetti-Dirr-Schulte-Herbrüggen (2023) state "vanishes in the
    closure." This general-\(CPTP(n)\) result does not by itself establish
    Conjecture 1, because \(TO(H_0,T)\) is a proper, energy-conservation
    -constrained sub-semigroup of \(CPTP(n)\) whenever \(H_0\) is not a
    multiple of the identity. (ii) They verify that
    \(\{A(m,H_B,H_{tot},H)\}\) is invariant under conjugation by arbitrary
    edge elements of \(L(\overline{TO(H_0,T)})\), matching the necessary
    condition of their Proposition 2(ii) -- a consistency check, not a
    proof of the inclusion.

    vom Ende-Malvetti-Dirr-Schulte-Herbrüggen (2023) do not resolve
    Conjecture 1 anywhere in the paper. Their concluding Section 5 states:
    "Thereby we could determine (up to Conj. 1) the structure of the
    respective semigroups, their Lie wedges as well as their edges,"
    maintaining the same open qualification after presenting all of the
    paper's results, including the worked qubit example described next.

    The qubit worked example (Section 3.2) and a residual doubt. For the
    single-qubit case (\(n=2\), \(H_0=\mathrm{diag}(-1,1)/2\)), vom
    Ende-Malvetti-Dirr-Schulte-Herbrüggen (2023) use their Lemma 2 (an
    unconditional, non-conjectural characterization of \(L(EnTO(H_0,T))\),
    the Lie wedge of the "enhanced thermal operations"
    \(EnTO(H_0,T)\supseteq TO(H_0,T)\)) to parametrize every element of
    \(L(EnTO(H_0,T))\) by \((u,x,\omega)\) with \(u\ge0\),
    \(2x\ge u(1+\varepsilon)\), \(\omega\in\mathbb R\)
    (\(\varepsilon:=e^{-1/T}\)), and then exhibit, for the general point of
    this family, explicit \(V_{jk}\) operators together with a matching
    \(H_{tot}\) that they describe as being "in the language of Thm. 2"
    (Eqs. 16-17 and the surrounding text). They separately state that for a
    single qubit "the thermal operations and the enhanced thermal
    operations approximately coincide," citing F. vom Ende, Which Bath
    Hamiltonians Matter for Thermal Operations?, Journal of Mathematical
    Physics 63, 112202 (2022), and their own Theorem 10. Taken together,
    these statements are suggestive that the qubit case of Conjecture 1 may
    already follow from material inside this same paper; however, vom
    Ende-Malvetti-Dirr-Schulte-Herbrüggen (2023) never state this as a
    corollary, never assert \(TO(H_0,T)=EnTO(H_0,T)\) exactly (only
    "approximately"), and -- as quoted above -- continue to qualify their
    own results "up to Conj. 1" after presenting the qubit example. This
    entry does not independently verify whether the qubit case is a hidden
    corollary of Section 3.2; it is recorded here as a residual doubt
    rather than resolved, per the verification scope of this entry.

    A different open question in the same section, not the subject of this
    entry. Directly after Eq. (17), vom Ende-Malvetti-Dirr-Schulte
    -Herbrüggen (2023) separately note that, for a single qubit,
    "Markovian thermal operations even exhaust the entire set of thermal
    operations" as \(T\to0^+\) (i.e. \(MTO(H_0,T)\to TO(H_0,T)\), a
    statement about the Markovian Lie semigroup, not the Lie wedge), and
    state "[w]hether this holds for qutrits and higher dimensions is an
    open question." This zero-temperature \(MTO\)-versus-\(TO\) question is
    a distinct open problem from Conjecture 1 and is not recorded by this
    entry.
  </known>

  <refs>
    <ref>F. vom Ende, E. Malvetti, G. Dirr, and T. Schulte-Herbrüggen, Exploring the Limits of Controlled Markovian Quantum Dynamics with Thermal Resources, Open Systems and Information Dynamics 30(1), 2350005 (2023), DOI: 10.1142/S1230161223500051, arXiv:2303.01891, used for the definition of thermal operations \(TO(H_0,T)\) (Eq. 10), the Lie-wedge and Lie-semigroup framework (Eqs. 6-9, Propositions 1-2, Theorem 1), Proposition 3 and Corollary 3 (topological and covariance properties of \(TO(H_0,T)\)), Lemma 1, Lemma 2 (Lie wedges of \(Gibbs(H_0,T)\) and \(EnTO(H_0,T)\)), Theorem 2 and its proof (Eqs. 14-15, the finite-bath-coupling construction), Conjecture 1 and its two supporting observations (Section 3.1, immediately following Theorem 2), the worked qubit example (Section 3.2, Eqs. 16-17), and Section 5 ("Conclusions"), the source of the open-problem statement recorded here.</ref>
    <ref>F. vom Ende, Quantum-Dynamical Semigroups and the Church of the Larger Hilbert Space, Open Systems and Information Dynamics 30(1), 2350003 (2023), DOI: 10.1142/S1230161223500038, arXiv:2211.08351, Theorem 2, used in vom Ende-Malvetti-Dirr-Schulte-Herbrüggen (2023)'s first supporting observation for Conjecture 1 (the unconstrained, non-energy-conserving analogue on general \(CPTP(n)\) is a proven theorem), cited there as "[86, Thm. 2]".</ref>
    <ref>F. vom Ende, Which Bath Hamiltonians Matter for Thermal Operations?, Journal of Mathematical Physics 63, 112202 (2022), arXiv:2207.11189, Theorem 10, used for vom Ende-Malvetti-Dirr-Schulte-Herbrüggen (2023)'s qubit-case statement that thermal operations and enhanced thermal operations "approximately coincide," cited there as "[85, Thm. 10]" in Section 3.2.</ref>
  </refs>

  <ask>
    Settle, for every integer \(n\ge1\), every \(H_0\in iu(n)\), and every
    \(T\in(0,\infty]\), whether
    \[
      L(\overline{TO(H_0,T)})\subseteq\overline{G(H_0,T)}.
    \]

    A proof must establish this inclusion for every triple \((n,H_0,T)\):
    for every \(A\in L(\overline{TO(H_0,T)})\) and every \(\epsilon\gt0\), exhibit (or
    prove the existence of) a tuple \((m,H_B,H_{tot},H)\) as in the
    finite-bath-coupling construction with
    \(\lVert A-A(m,H_B,H_{tot},H)\rVert\lt\epsilon\) for some (equivalently,
    by finite-dimensionality, any) norm on \(L(\mathbb C^{n\times n})\).

    A disproof must exhibit one specific triple \((n,H_0,T)\) with
    \(n\ge1\), \(H_0\in iu(n)\), \(T\in(0,\infty]\), and one specific
    \(A\in L(\overline{TO(H_0,T)})\) together with a proof that \(A\notin
    \overline{G(H_0,T)}\) for that triple (equivalently, that
    \(A\) has strictly positive distance from \(G(H_0,T)\)). A single such
    counterexample triple disproves the claim, since the claim is a
    universal statement over \((n,H_0,T)\); no argument about the
    admissible dimensions being finite or infinite is needed.

    The following count as progress, not as resolving the claim: (i) a
    proof of the inclusion for one fixed \(n\) (including \(n=2\)) or for
    finitely many values of \(n\), without a construction covering every
    \(n\ge1\) -- this includes any argument that formalizes the qubit
    worked example of Section 3.2 of vom Ende-Malvetti-Dirr-Schulte
    -Herbrüggen (2023) into a complete proof for \(n=2\) alone; (ii) a
    proof restricted to \(H_0\) with non-degenerate spectrum, or to some
    other proper subclass of \(H_0\in iu(n)\), without covering every
    \(H_0\); (iii) for \(X\in\{EnTO(H_0,T),\,Gibbs(H_0,T),\,CPTP(n)\}\), a proof of
    \(L(X)\subseteq\overline{G(H_0,T)}\) when \(G(H_0,T)\) is replaced by
    the same finite-bath-coupling construction with the commutator
    constraint \([H_{tot},H_0\otimes1_m+1_n\otimes H_B]=0\) dropped,
    without also showing that the closure of this enlarged, unconstrained
    construction equals \(\overline{G(H_0,T)}\) for the \(n\), \(H_0\),
    \(T\) in question. This exemption applies only to the dropped
    -constraint target: a proof of \(L(X)\subseteq\overline{G(H_0,T)}\)
    itself, with the commutator constraint retained as in the claim
    above, is not merely progress but resolves the claim outright,
    because \(\overline{TO(H_0,T)}\subseteq EnTO(H_0,T)\subseteq
    Gibbs(H_0,T)\subseteq CPTP(n)\) for every \(n\ge1\), \(H_0\in iu(n)\),
    \(T\in(0,\infty]\) (vom Ende-Malvetti-Dirr-Schulte-Herbrüggen (2023),
    Proposition 3(iii)-(iv) and the definition of \(EnTO(H_0,T)\)), so
    that \(L(\cdot)\) -- monotone under set inclusion directly from the
    definition of \(L(S)\) above -- gives
    \(L(\overline{TO(H_0,T)})\subseteq L(X)\subseteq\overline{G(H_0,T)}\);
    combined with \(\overline{G(H_0,T)}\subseteq L(\overline{TO(H_0,T)})\)
    (already established above, following from Theorem 2 alone), such a
    proof in fact establishes
    \(L(\overline{TO(H_0,T)})=L(X)=\overline{G(H_0,T)}\) as a corollary; (iv) necessary conditions on
    \(A\in L(\overline{TO(H_0,T)})\) for membership in
    \(\overline{G(H_0,T)}\) that are not also shown sufficient, or
    sufficient conditions not also shown necessary; (v) numerical evidence
    that sampled elements of \(L(\overline{TO(H_0,T)})\) lie within some
    fixed tolerance of \(G(H_0,T)\), without an analytic proof of the exact
    (\(\epsilon\to0\)) inclusion; (vi) a proof of the inclusion
    \(L(TO(H_0,T))\subseteq\overline{G(H_0,T)}\) for the unclosed semigroup
    \(TO(H_0,T)\) in place of its closure, without also showing
    \(L(TO(H_0,T))=L(\overline{TO(H_0,T)})\) for the \(n\), \(H_0\), \(T\)
    in question -- since \(L(TO(H_0,T))\subseteq L(\overline{TO(H_0,T)})\)
    trivially, this is a priori a weaker statement than the claim.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
