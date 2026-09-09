# 52 Infinite-Round LTOCC Closure Equals Semilocal Thermal Operations

```xml
<problem id="52">
  <status>open</status>
  <kind>proof</kind>
  <field>QTD</field>
  <name>Infinite-Round LTOCC Closure Equals Semilocal Thermal Operations</name>
  <keys>ltocc local thermal operations and classical communication slto semilocal thermal operations semi-local thermal operations distant labs paradigm distant laboratories paradigm thermal tensor thermal tensors bithermal tensor bithermal tensors energy-incoherent states population vector stochastic tensor n-round protocol infinite-round limit closure of ltocc shared randomness ltocc+r ltoccn ltocc_n ltocc_1 conjecture 1 bistron czartowski bistroń czartowski rafal bistron jakub czartowski quantum thermodynamics distributed thermodynamics locc thermodynamics classical communication and thermal operations gibbs-preserving operations semilocal thermal operation carnot efficiency finite-size heat engines bera lewenstein bera thermo-majorization gibbs state ancilla bath</keys>

  <claim>
    Fix finite dimensions \(d_A,d_B\ge1\), Hamiltonians
    \(H^{(A)}=\sum_{k=1}^{d_A}E_k^{(A)}|k\rangle\langle k|\) on Alice's system
    \(S^{(A)}\) and \(H^{(B)}=\sum_{l=1}^{d_B}E_l^{(B)}|l\rangle\langle l|\) on
    Bob's system \(S^{(B)}\) (real energy levels, not necessarily distinct),
    and inverse temperatures \(\beta^{(A)},\beta^{(B)}\in(0,\infty)\). Let
    \(\mathrm{LTOCC}_n{+}R\) (\(n=1,2,\ldots\)) and \(\mathrm{SLTO}_{\mathrm{cl}}\)
    be the sets of \(d_Ad_B\times d_Ad_B\) energy-incoherent stochastic
    tensors defined below (both depending on
    \(H^{(A)},H^{(B)},\beta^{(A)},\beta^{(B)}\)), and let \(\overline{(\cdot)}\)
    denote topological closure in \(\mathbb R^{d_A^2d_B^2}\) (defined below).

    Then, for every such \(d_A,d_B,H^{(A)},H^{(B)},\beta^{(A)},\beta^{(B)}\),
    \[
      \overline{\bigcup_{n=1}^\infty\mathrm{LTOCC}_n{+}R}=\mathrm{SLTO}_{\mathrm{cl}}.
    \]

    This is Bistroń and Czartowski, "Local thermal operations and classical
    communication," Physical Review E 113, 034114 (2026), Conjecture 1
    (Eq. (26)), restricted throughout to energy-incoherent (population)
    processing exactly as the conjecture itself specifies.
  </claim>

  <def>
    Setup. Fix finite \(d_A,d_B\ge1\). Alice's system \(S^{(A)}\) has
    orthonormal energy eigenbasis \(\{|k\rangle\}_{k=1}^{d_A}\) and
    Hamiltonian \(H^{(A)}=\sum_{k=1}^{d_A}E_k^{(A)}|k\rangle\langle k|\),
    \(E_k^{(A)}\in\mathbb R\) (not necessarily distinct); Bob's system
    \(S^{(B)}\) has \(\{|l\rangle\}_{l=1}^{d_B}\) and
    \(H^{(B)}=\sum_{l=1}^{d_B}E_l^{(B)}|l\rangle\langle l|\) similarly. Write
    \(|kl\rangle:=|k\rangle\otimes|l\rangle\) for the product basis of
    \(S^{(A)}\otimes S^{(B)}\). Fix inverse temperatures
    \(\beta^{(A)},\beta^{(B)}\in(0,\infty)\).

    A state \(\rho\) on \(S^{(A)}\otimes S^{(B)}\) is energy-incoherent iff
    \(\rho=\sum_{k,l}r_{kl}|kl\rangle\langle kl|\) for a probability array
    \(r_{kl}\ge0\), \(\sum_{kl}r_{kl}=1\) (jointly diagonal in the product
    energy eigenbasis); call \(r=(r_{kl})\) its population.

    Thermal operation and thermal tensor (single system). For a system \(X\)
    with Hamiltonian \(H_X=\sum_mF_m|m\rangle\langle m|\) (\(F_m\in\mathbb R\))
    at inverse temperature \(\beta\in(0,\infty)\), a thermal operation is a
    CPTP map
    \[
      \Phi(\rho)=\operatorname{tr}_{\mathcal B}\bigl[U(\rho\otimes\gamma_{\mathcal B})U^\dagger\bigr],
      \qquad
      [U,\,H_X\otimes I_{\mathcal B}+I_X\otimes H_{\mathcal B}]=0,
    \]
    for some finite-dimensional bath \((\mathcal B,H_{\mathcal B})\) with
    Gibbs state \(\gamma_{\mathcal B}=e^{-\beta H_{\mathcal B}}/\operatorname{tr}e^{-\beta H_{\mathcal B}}\)
    and unitary \(U\) on \(X\otimes\mathcal B\); \(\mathcal B,H_{\mathcal B},U\)
    are otherwise unconstrained and freely chosen for each \(\Phi\).
    Restricted to acting on populations, \(\Phi\) induces the stochastic
    matrix (thermal tensor)
    \[
      \Lambda_{im}:=\langle i|\Phi(|m\rangle\langle m|)|i\rangle,
      \qquad
      \Lambda_{im}\ge0,\quad\textstyle\sum_i\Lambda_{im}=1\ \text{for every }m.
    \]
    Write \(\mathrm{Th}(X,\beta)\) for the set of all thermal tensors on
    \((X,H_X)\) at inverse temperature \(\beta\), i.e. all \(\Lambda\)
    arising this way from some thermal operation \(\Phi\) on \(X\).

    One-round LTOCC map. A one-round LTOCC map with respect to
    \((H^{(A)},H^{(B)},\beta^{(A)},\beta^{(B)})\) is a
    \(d_Ad_B\times d_Ad_B\) stochastic tensor \(M_{ij,kl}\)
    (\(i,k\in\{1,\ldots,d_A\}\), \(j,l\in\{1,\ldots,d_B\}\);
    \(M_{ij,kl}\ge0\), \(\sum_{i,j}M_{ij,kl}=1\) for every \(k,l\)), of one
    of the two forms
    \[
      M_{ij,kl}=\Lambda_{ik}\,T_{jkl}
      \qquad\text{or}\qquad
      M_{ij,kl}=T_{ikl}\,\Lambda_{jl},
    \]
    where in the first form \(\Lambda\in\mathrm{Th}(S^{(A)},\beta^{(A)})\)
    and \(T_{jkl}:=\Lambda^{(k)}_{jl}\) for some choice of thermal tensors
    \(\Lambda^{(1)},\ldots,\Lambda^{(d_A)}\in\mathrm{Th}(S^{(B)},\beta^{(B)})\)
    (Alice applies \(\Lambda\) to her population, \(k\mapsto i\), and
    separately reports her own input level \(k\) to Bob, who applies the
    \(k\)-dependent thermal tensor \(\Lambda^{(k)}\) to his population,
    \(l\mapsto j\)); the second form is the mirror image, with
    \(\Lambda\in\mathrm{Th}(S^{(B)},\beta^{(B)})\),
    \(T_{ikl}:=\Lambda^{(l)}_{ik}\) for some
    \(\Lambda^{(1)},\ldots,\Lambda^{(d_B)}\in\mathrm{Th}(S^{(A)},\beta^{(A)})\),
    and the roles of Alice and Bob exchanged. Write \(\mathrm{LTOCC}_1\) for
    the set of all such \(M\) (this reproduces Bistroń and Czartowski's
    Eqs. (15)-(17); \(T\) is their thermal tensor, satisfying their
    Eq. (18) -- not the source's strictly more restrictive, symmetric
    bithermal tensor of their Eq. (25), which builds \(\mathcal
    S\mathrm{LTOCC}\) rather than \(\mathrm{LTOCC}_1\)).

    \(n\)-round LTOCC, without memory. For \(n\ge1\), \(\mathrm{LTOCC}_n\) is
    the set of all \(M=M^{(n)}M^{(n-1)}\cdots M^{(1)}\) (composition of
    linear maps on \(\mathbb R^{d_A}\otimes\mathbb R^{d_B}\)), where each
    \(M^{(\alpha)}\in\mathrm{LTOCC}_1\), \(\alpha=1,\ldots,n\), independently
    chosen (Eq. (19)).

    Shared randomness. \(\mathrm{LTOCC}_1{+}R\) is the set of all finite
    convex combinations \(M=\sum_a\lambda_aM^{(a)}\)
    (\(\lambda_a\ge0\), \(\sum_a\lambda_a=1\), each
    \(M^{(a)}\in\mathrm{LTOCC}_1\)) (Eq. (23)). For \(n\ge1\),
    \(\mathrm{LTOCC}_n{+}R:=\mathrm{conv}(\mathrm{LTOCC}_n)\) is the set of
    all finite convex combinations \(M=\sum_a\lambda_aM_a^{(n)}\cdots
    M_a^{(1)}\) (\(\lambda_a\ge0\), \(\sum_a\lambda_a=1\)) of a single
    shared random draw \(a\) of an entire \(n\)-round strategy
    \((M_a^{(1)},\ldots,M_a^{(n)})\in(\mathrm{LTOCC}_1)^n\) jointly -- one
    random variable selecting all \(n\) rounds together, not \(n\)
    independently randomized rounds -- agreeing, at \(n=1\), with the
    definition just above, and matching Bistroń and Czartowski's statement,
    immediately after the proof of Theorem 7 in Appendix D, that
    \(\mathrm{LTOCC}_n\) is an \(n\)-fold composition of \(\mathrm{LTOCC}_1\)
    and \(\mathrm{LTOCC}_n{+}R\) is the convex hull of \(\mathrm{LTOCC}_n\).

    Semilocal thermal operations (SLTO). Following Bera, Lewenstein, and
    Bera (Definition 1), restated as Bistroń and Czartowski's Definition 4:
    a channel \(\Lambda^{(AB)}\) on \(S^{(A)}\otimes S^{(B)}\) is a
    semilocal thermal operation with respect to \(\beta^{(A)},\beta^{(B)}\)
    if there exist finite-dimensional baths \(\mathcal B^{(A)},\mathcal
    B^{(B)}\) with Hamiltonians \(H^{\mathcal B^{(A)}},H^{\mathcal
    B^{(B)}}\), Gibbs states \(\gamma^{\mathcal B^{(A)}},\gamma^{\mathcal
    B^{(B)}}\) at \(\beta^{(A)},\beta^{(B)}\) respectively, and a unitary
    \(U\) on \(S^{(A)}\otimes S^{(B)}\otimes\mathcal B^{(A)}\otimes\mathcal
    B^{(B)}\) (otherwise unconstrained) such that
    \[
      \Lambda^{(AB)}(\rho^{(AB)})=\operatorname{tr}_{\mathcal B^{(A)},\mathcal B^{(B)}}\bigl[U(\gamma^{\mathcal B^{(A)}}\otimes\gamma^{\mathcal B^{(B)}}\otimes\rho^{(AB)})U^\dagger\bigr],
    \]
    \[
      [U,\,H^{(A)}+H^{(B)}+H^{\mathcal B^{(A)}}+H^{\mathcal B^{(B)}}]=0,
      \qquad
      [U,\,\beta^{(A)}(H^{(A)}+H^{\mathcal B^{(A)}})+\beta^{(B)}(H^{(B)}+H^{\mathcal B^{(B)}})]=0
    \]
    (each Hamiltonian acts as itself tensored with the identity on the other
    three factors of \(S^{(A)}\otimes S^{(B)}\otimes\mathcal B^{(A)}\otimes
    \mathcal B^{(B)}\); the two commutators are, respectively, total
    system-plus-bath energy conservation and \(\beta\)-weighted heat-flow
    (entropy) conservation across the two locally-thermal baths). Write
    \(\mathrm{SLTO}(\beta^{(A)},\beta^{(B)})\) for the set of all such
    \(\Lambda^{(AB)}\).

    Energy-incoherent restriction.
    \[
      \mathrm{SLTO}_{\mathrm{cl}}:=\bigl\{M\in\mathbb R^{d_A^2d_B^2}:
      M_{ij,kl}=\langle ij|\Lambda^{(AB)}(|kl\rangle\langle kl|)|ij\rangle
      \text{ for every }i,j,k,l,\text{ for some }
      \Lambda^{(AB)}\in\mathrm{SLTO}(\beta^{(A)},\beta^{(B)})\bigr\},
    \]
    each such \(M\), extended by linearity to general populations \(r_{kl}\),
    being the stochastic tensor by which the corresponding \(\Lambda^{(AB)}\)
    acts on populations.

    Closure. For \(X\subseteq\mathbb R^{d_A^2d_B^2}\), \(\overline X\) is the
    topological closure of \(X\) with respect to any norm on \(\mathbb
    R^{d_A^2d_B^2}\) (e.g. \(\|M\|_\infty:=\max_{i,j,k,l}|M_{ij,kl}|\));
    since \(\mathbb R^{d_A^2d_B^2}\) is finite-dimensional, all norms induce
    the same topology and hence the same closure.
  </def>

  <known>
    Bistroń and Czartowski, Theorem 7 (stated in Section IV.3, proved in
    Appendix D, "LTOCC without memory as a subset of semilocal thermal
    Operations"): every quantum channel on \(S^{(A)}\otimes S^{(B)}\)
    realizable by an \(n\)-round, shared-randomness-assisted LTOCC protocol
    (for any \(n\ge1\)) -- i.e. the full, not-necessarily-energy-incoherent-input
    quantum channel underlying the construction of \(\mathrm{LTOCC}_n{+}R\)
    in def above -- is an element of \(\mathrm{SLTO}(\beta^{(A)},\beta^{(B)})\),
    and this holds even when the protocol acts on non-energy-incoherent
    (coherent) input states, not only on populations. Restricting such a
    channel's action to energy-incoherent inputs and outputs recovers
    exactly an element of \(\mathrm{LTOCC}_n{+}R\) as defined above, so
    Theorem 7 gives
    \(\bigcup_{n\ge1}\mathrm{LTOCC}_n{+}R\subseteq\mathrm{SLTO}_{\mathrm{cl}}\)
    directly; this entry does not treat the further step to
    \(\overline{\bigcup_n\mathrm{LTOCC}_n{+}R}\subseteq\mathrm{SLTO}_{\mathrm{cl}}\)
    as already settled by Theorem 7 alone (see the residual-doubt remark
    below).

    Bistroń and Czartowski, Corollary 2 ("Set-closure of SLTO," Section III,
    stated immediately after Corollary 1 and before Theorems 4-5, derived
    from Corollary 1's Gibbs-preserving-stochastic-matrix characterization
    of single transitions -- Corollary 1 itself attributed there to [34],
    Supplementary Information, Corollary 4): for finite-dimensional
    energy-incoherent states, the set of transitions
    \(\rho^{(AB)}\to\sigma^{(AB)}\) achievable under
    \(\mathrm{SLTO}(\beta^{(A)},\beta^{(B)})\) is closed, i.e. for each
    fixed energy-incoherent input \(\rho^{(AB)}\) the set of outputs
    \(\sigma^{(AB)}\) reachable from it is topologically closed.

    Bistroń and Czartowski, opening paragraphs of Appendix D, separately
    state that composition-closure (Theorem 4) and convexity (Theorem 5) of
    \(\mathrm{SLTO}(\beta^{(A)},\beta^{(B)})\) are established there as
    auxiliary lemmas toward Theorem 7's proof; "closedness" there names
    closure of \(\mathrm{SLTO}(\beta^{(A)},\beta^{(B)})\) under composition
    of channels, a different property from the topological set-closure of
    Corollary 2 above.

    Residual doubt, recorded rather than resolved here: Corollary 2 gives
    topological closedness only per fixed input, i.e. of each set
    \(\{\sigma^{(AB)}:\rho^{(AB)}\to\sigma^{(AB)}\text{ achievable under }
    \mathrm{SLTO}\}\) separately, whereas \(\mathrm{SLTO}_{\mathrm{cl}}
    \subset\mathbb R^{d_A^2d_B^2}\) as defined above is the set of full
    tensors, each the simultaneous restriction to every one of the
    \(d_Ad_B\) basis-population inputs of one single channel
    \(\Lambda^{(AB)}\in\mathrm{SLTO}(\beta^{(A)},\beta^{(B)})\). This entry
    has not independently verified whether Corollary 2, applied
    input-by-input along a convergent sequence in
    \(\bigcup_n\mathrm{LTOCC}_n{+}R\), assembles into closedness of the
    joint tensor set \(\mathrm{SLTO}_{\mathrm{cl}}\) itself -- in particular
    whether the (a priori input-dependent) limiting channels that Corollary 2
    furnishes for each of the \(d_Ad_B\) basis inputs can be taken to
    coincide -- nor whether the bath-dimension bound underlying Theorem 7's
    proof is uniform enough to control the \(n\to\infty\) limit. Both are
    open questions from this entry's point of view.

    Bistroń and Czartowski, Section IV.3, the paragraph immediately
    following Eq. (26): note, without conjecturing it, a strictly stronger
    nearby variant obtained either by dropping the shared-randomness
    extension (asking whether \(\overline{\bigcup_n\mathrm{LTOCC}_n}=\mathrm{SLTO}_{\mathrm{cl}}\)
    already) or by extending the equality beyond energy-incoherent states to
    fully general (coherent) states.

    Bistroń and Czartowski, Appendix D, paragraphs following the proof of
    Theorem 7: state a second, separately open conjecture (their
    Conjecture 2) that \(\mathrm{LTOCC}_n{+}R\) extended by arbitrary
    energy-incoherent POVM measurements (in place of the projective
    energy-incoherent measurements used to build \(T\) above) forms a
    subset of \(\mathrm{SLTO}(\beta^{(A)},\beta^{(B)})\). This is a
    different claim (an inclusion, not an equality-in-closure) about a
    different, POVM-extended protocol class; it is recorded here only for
    provenance, because the issue that seeded this entry bundled it together
    with Conjecture 1 as "two 2026 conjectures." The canonical claim of this
    entry is Conjecture 1 alone; this entry does not ask about Conjecture 2.

    No proof, disproof, or partial resolution of Conjecture 1, in the exact
    form above or in the stronger variants Bistroń and Czartowski note, is
    given anywhere else in that source's main text, appendices, or
    discussion.

    Bera, Lewenstein, and Bera, "Attaining Carnot efficiency with quantum
    and nanoscale heat engines," npj Quantum Information 7 (2021),
    Definition 1: origin of the semilocal-thermal-operation definition
    restated as Definition 4 in Bistroń and Czartowski.
  </known>

  <refs>
    <ref>R. Bistroń and J. Czartowski, "Local thermal operations and classical communication," Physical Review E 113, 034114 (2026), DOI: 10.1103/s965-zwzm, arXiv:2410.14550, used for: Definition 4 (Eqs. (13)-(14b), there attributed to [34]) for semilocal thermal operations (SLTO); Section III, Corollary 1 and Corollary 2 ("Set-closure of SLTO"), for the Gibbs-preserving-matrix and per-input topological-closedness characterizations of SLTO transitions; Section IV.3 "Framework outline," Eqs. (15)-(19) and (23), for the one-round, \(n\)-round, and shared-randomness LTOCC maps; Section IV.3, Eq. (26) and the paragraph immediately following it, for Conjecture 1 and the stronger nearby variant it notes; Theorem 7 (stated Section IV.3, proved Appendix D, "LTOCC without memory as a subset of semilocal thermal Operations") for the proved containment \(\mathrm{LTOCC}_n{+}R\subseteq\mathrm{SLTO}\); the sentence immediately following the proof of Theorem 7 in Appendix D for the characterization of \(\mathrm{LTOCC}_n\) as an \(n\)-fold composition of \(\mathrm{LTOCC}_1\) and \(\mathrm{LTOCC}_n{+}R\) as its convex hull; the opening paragraphs of Appendix D for the composition-closure/convexity lemmas on SLTO; and the closing paragraphs of Appendix D for Conjecture 2 (the POVM-extension conjecture).</ref>
    <ref>M. L. Bera, M. Lewenstein, and M. N. Bera, "Attaining Carnot efficiency with quantum and nanoscale heat engines," npj Quantum Information 7, 31 (2021), DOI: 10.1038/s41534-021-00366-6, Definition 1, used for: the original definition of semilocal thermal operations, restated as Definition 4 in Bistroń and Czartowski.</ref>
  </refs>

  <ask>
    A proof must establish, for every finite \(d_A,d_B\ge1\), every
    \(H^{(A)},H^{(B)}\) with real (possibly degenerate) spectra, and every
    \(\beta^{(A)},\beta^{(B)}\in(0,\infty)\), both inclusions
    \[
      \overline{\bigcup_{n=1}^\infty\mathrm{LTOCC}_n{+}R}\subseteq\mathrm{SLTO}_{\mathrm{cl}}
      \qquad\text{and}\qquad
      \mathrm{SLTO}_{\mathrm{cl}}\subseteq\overline{\bigcup_{n=1}^\infty\mathrm{LTOCC}_n{+}R}
    \]
    as defined above. A single argument covering every
    \(d_A,d_B,H^{(A)},H^{(B)},\beta^{(A)},\beta^{(B)}\) is required; a proof
    restricted to particular dimensions (e.g. \(d_A=d_B=2\)), to
    non-degenerate spectra only, or to equal temperatures
    \(\beta^{(A)}=\beta^{(B)}\) only, is progress, not a proof of the claim
    as quantified above. Establishing only one of the two inclusions, for
    instance invoking Theorem 7 (known above) for a form of the first
    inclusion without also closing the gap to the closure, or exhibiting
    finite-round approximations for a proper subset of
    \(\mathrm{SLTO}_{\mathrm{cl}}\) for the second, is likewise progress, not
    a full proof.

    A disproof must exhibit specific finite \(d_A,d_B\ge1\),
    \(H^{(A)},H^{(B)}\), and \(\beta^{(A)},\beta^{(B)}\in(0,\infty)\),
    together with either:
    (a) an explicit \(M^\star\in\mathrm{SLTO}_{\mathrm{cl}}\) and a proof
    that \(M^\star\) lies at strictly positive distance from
    \(\bigcup_{n\ge1}\mathrm{LTOCC}_n{+}R\) in every (equivalently, some
    fixed) norm on \(\mathbb R^{d_A^2d_B^2}\); or
    (b) an explicit sequence in \(\bigcup_{n\ge1}\mathrm{LTOCC}_n{+}R\)
    converging to some \(M^\star\), together with a proof that
    \(M^\star\notin\mathrm{SLTO}_{\mathrm{cl}}\).
    Because the claim is quantified over every
    \(d_A,d_B,H^{(A)},H^{(B)},\beta^{(A)},\beta^{(B)}\), exhibiting one such
    instance under (a) or (b) fully disproves it. A finite numerical search
    (a sampled grid, or a linear- or semidefinite-programming feasibility
    check at fixed small \(d_A,d_B\)) showing an apparent gap or an apparent
    reachable point is progress, not a disproof, unless paired with an exact
    certificate that the gap in (a) is strictly positive or that \(M^\star\)
    in (b) genuinely lies outside \(\mathrm{SLTO}_{\mathrm{cl}}\).
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
