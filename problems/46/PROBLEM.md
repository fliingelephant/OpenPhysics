# 46 Cooling Maps versus Low-Temperature Thermal Operations

```xml
<problem id="46">
  <status>open</status>
  <kind>proof</kind>
  <field>QTD</field>
  <name>Cooling Maps versus Low-Temperature Thermal Operations</name>
  <keys>cooling maps cooling map thermal operations low-temperature thermal operations zero-bath-temperature thermal operations zero temperature bath ground-state bath ground eigenspace energy-conserving unitary gibbs state ancilla bath low temperature quantum thermodynamics quantum coherence energy coherence coherence transfer optimally coherent cooling maps qubit thermal operations two-level thermal operations kraus operators kraus decomposition ut-majorization upper-triangular majorization thermo-majorization state transformation conditions narasimhachar gour narasimhachar-gour main open question unattainability third law of thermodynamics low-temperature thermodynamics with quantum coherence cooling map thermal operation equivalence cooling map realizability transition feasibility state transition feasibility conjecture 1 channel set equality reachability equivalence</keys>

  <claim>
    Fix a finite-dimensional quantum system \(S\), \(d:=\dim S\lt\infty\), and a
    Hamiltonian \(H_S=\sum_{i=1}^dE_i\,|E_i\rangle\langle E_i|\) on \(S\) with
    non-degenerate spectrum \(E_1\lt E_2\lt\cdots\lt E_d\) and non-degenerate
    energy gaps \(E_i-E_j\ne E_k-E_l\) for any index pairs \((i,j)\ne(k,l)\)
    except when \(i=j,k=l\) or \(i=k,j=l\) (the def below fixes this energy
    ordering and uses it throughout). Let \(\mathrm{CM}(S)\) be the set of
    cooling maps on \((S,H_S)\) and \(\mathrm{TO}_{\mathrm{low}}(S)\) the set
    of low-temperature thermal operations on \((S,H_S)\), and let
    \(\to_{\mathrm{CM}}\) and \(\to_{\mathrm{TO}}\) be the transition-feasibility
    relations they induce on states of \(S\), all defined below.

    For every finite \(d\ge1\), every non-degenerate \(H_S\) as above, and
    every pair of states \(\rho,\sigma\) on \(S\),
    \[
      \rho\to_{\mathrm{CM}}\sigma \iff \rho\to_{\mathrm{TO}}\sigma.
    \]
    Since \(\mathrm{TO}_{\mathrm{low}}(S)\subseteq\mathrm{CM}(S)\) already holds
    for every such \(S,H_S\) (stated in known below), the implication
    \(\rho\to_{\mathrm{TO}}\sigma\implies\rho\to_{\mathrm{CM}}\sigma\) holds
    automatically for every \(\rho,\sigma\) (the map witnessing
    \(\rho\to_{\mathrm{TO}}\sigma\) is itself already an element of
    \(\mathrm{CM}(S)\)), and the open content of the claim is exactly the
    converse: for every \(\rho,\sigma\),
    \(\rho\to_{\mathrm{CM}}\sigma\implies\rho\to_{\mathrm{TO}}\sigma\), i.e.
    every transition achievable by some cooling map is achievable by some
    low-temperature thermal operation, not necessarily the same map.

    This transition-feasibility form is the reading fixed by Narasimhachar
    and Gour's own "Conjecture 1" (Supplementary Information Section S2, part
    F, quoted in full in known below): "Cooling maps are equivalent to
    low-temperature thermal operations, with regard to the feasibility of
    state transitions," together with their accompanying caveat that this
    could hold "even if the set of cooling maps is strictly larger than that
    of thermal operations." This claim is therefore deliberately weaker than
    the superficially similar statement of literal channel-set equality,
    \(\mathrm{CM}(S)=\mathrm{TO}_{\mathrm{low}}(S)\) as sets of CPTP maps on
    \(S\): channel-set equality would imply the transition-feasibility
    statement above (a single map on each side would witness every
    transition simultaneously) but is not implied by it, since a priori
    \(\mathrm{CM}(S)\) could contain channels absent from
    \(\mathrm{TO}_{\mathrm{low}}(S)\) while every individual transition any
    such channel realizes is separately reproduced by some other, possibly
    transition-dependent, element of \(\mathrm{TO}_{\mathrm{low}}(S)\).
    Narasimhachar and Gour explicitly leave this distinction open and
    conjecture only the transition-feasibility statement above; this entry
    follows that choice as the weaker, precisely source-attested reading.
  </claim>

  <def>
    Fix a finite-dimensional system \(S\), \(d:=\dim S\), and a Hamiltonian
    \(H_S=\sum_{i=1}^dE_i\,|E_i\rangle\langle E_i|\) with non-degenerate
    spectrum \(E_1\lt E_2\lt\cdots\lt E_d\), non-degenerate energy gaps (i.e.
    \(E_i-E_j\ne E_k-E_l\) for any index pairs \((i,j)\ne(k,l)\) except when
    \(i=j,k=l\) or \(i=k,j=l\)), and orthonormal energy eigenbasis
    \(\{|E_i\rangle\}_{i=1}^d\). For an operator \(X\) on \(S\) write
    \(X_{ij}:=\langle E_i|X|E_j\rangle\).

    A cooling map on \((S,H_S)\) is a CPTP map \(\mathcal C\) on \(S\)
    admitting a Kraus decomposition consisting of, for some integer
    \(n\le d\) and each \(i=1,\ldots,n\), a diagonal Kraus operator
    \(K_i=\sum_{j=1}^d\lambda_j^{(i)}\,|E_j\rangle\langle E_j|\) with
    \(\lambda_j^{(i)}\in\mathbb C\), together with, for each pair
    \(1\le j\lt k\le d\), a population-lowering Kraus operator
    \(J_{jk}=\mu_{jk}\,|E_j\rangle\langle E_k|\) with \(\mu_{jk}\in\mathbb C\)
    (so \(J_{jk}\) maps the higher level \(E_k\) to the lower level \(E_j\)
    and annihilates every other \(|E_l\rangle\)). For \(j=1,\ldots,d\) write
    \(\boldsymbol\lambda_j:=(\lambda_j^{(1)},\ldots,\lambda_j^{(n)})\in\mathbb
    C^n\), with \(\langle\cdot,\cdot\rangle\) the standard inner product on
    \(\mathbb C^n\); the coefficients satisfy, for every \(k=1,\ldots,d\),
    \[
      \langle\boldsymbol\lambda_k,\boldsymbol\lambda_k\rangle+\sum_{1\le j\lt k}|\mu_{jk}|^2=1
    \]
    (trace preservation; the \(k=1\) case forces
    \(\langle\boldsymbol\lambda_1,\boldsymbol\lambda_1\rangle=1\), so the
    ground level is never depleted). Write \(\mathrm{CM}(S)\) for the set of
    all cooling maps on \((S,H_S)\), i.e. all \(\mathcal C\) arising this way
    for some \(n\le d\) and some choice of
    \(\{\boldsymbol\lambda_j\}_{j=1}^d\subset\mathbb C^n\),
    \(\{\mu_{jk}\}_{1\le j\lt k\le d}\) satisfying the relation above.

    An ancilla for \(S\) is a finite-dimensional system \(A\) with a
    Hamiltonian \(H_A\); write \(F_1:=\min\operatorname{spec}(H_A)\) for its
    ground energy and \(\Pi_A\) for the orthogonal projector onto the
    (possibly degenerate) \(F_1\)-eigenspace of \(H_A\), of dimension
    \(g:=\operatorname{tr}\Pi_A\ge1\). The low-temperature state of \(A\) is
    \(\gamma_A:=\Pi_A/g\), the maximally mixed state on the ground eigenspace
    of \(H_A\); for fixed \(H_A\) this is the \(\beta\to\infty\) limit of the
    Gibbs state \(e^{-\beta H_A}/\operatorname{tr}e^{-\beta H_A}\).

    A low-temperature thermal operation on \((S,H_S)\) is a CPTP map
    \(\mathcal E\) on \(S\) of the form
    \[
      \mathcal E(\rho)=\operatorname{tr}_A\!\bigl[U(\rho\otimes\gamma_A)U^\dagger\bigr],
      \qquad
      \bigl[U,\,H_S\otimes\mathbb 1_A+\mathbb 1_S\otimes H_A\bigr]=0,
    \]
    for some finite-dimensional ancilla \((A,H_A)\), otherwise unconstrained
    and freely chosen for each \(\mathcal E\), its low-temperature state
    \(\gamma_A\) as above, and some unitary \(U\) on \(S\otimes A\) commuting
    with the total Hamiltonian. Write \(\mathrm{TO}_{\mathrm{low}}(S)\) for
    the set of all such \(\mathcal E\).

    Reading note (this fixes an ambiguity in the source, flagged here rather
    than resolved by it). Narasimhachar and Gour work throughout with a
    genuine finite-temperature bath in Gibbs state
    \(\gamma_A(\beta)=e^{-\beta H_A}/\operatorname{tr}e^{-\beta H_A}\) and
    introduce \(\Pi_A/g\) only as an approximation to \(\gamma_A(\beta)\),
    valid once \(\beta\) is large enough that the ground level dominates the
    Gibbs weight; they do not name or fix a symbol for a resulting limiting
    set of channels. \(\mathrm{TO}_{\mathrm{low}}(S)\) above is read as the
    exact \(\beta\to\infty\) idealization of that approximation, fixing
    \(\gamma_A\) to equal \(\Pi_A/g\) exactly rather than tracking a residual
    finite-\(\beta\) correction. This is the reading under which the results
    recorded in known below hold as exact statements about a fixed,
    temperature-independent class of maps, and it is the reading needed for
    "\(\mathrm{CM}(S)\) is identical with thermal operations" to be a
    well-posed exact claim for \(d=2\); a formulation that instead keeps
    \(\beta\lt\infty\) throughout and asks only for approximate or asymptotic
    equivalence between \(\mathrm{CM}(S)\) and finite-\(\beta\) thermal
    operations is a strictly weaker, still-meaningful nearby variant that
    this entry's claim does not cover.

    For states \(\rho,\sigma\) on \(S\), write \(\rho\to_{\mathrm{CM}}\sigma\)
    iff there exists \(\mathcal C\in\mathrm{CM}(S)\) with
    \(\mathcal C(\rho)=\sigma\), and write \(\rho\to_{\mathrm{TO}}\sigma\) iff
    there exists \(\mathcal E\in\mathrm{TO}_{\mathrm{low}}(S)\) with
    \(\mathcal E(\rho)=\sigma\).

    UT-majorization. For \(u=(u_1,\ldots,u_d)^T,v=(v_1,\ldots,v_d)^T\in\mathbb
    R^d\), write \(u\succ^{\mathrm{UT}}v\) iff
    \(\sum_{i=k}^du_i\ge\sum_{i=k}^dv_i\) for every \(k=2,\ldots,d\) (\(d-1\)
    inequalities, read from the top energy level down).

    For states \(\rho,\sigma\) on \(S\) with \(\rho_{ii}\gt0\) for every \(i\)
    and \(\rho_{jk}\ne0\) for every \(j\ne k\), define the \(d\times d\)
    Hermitian matrix \(Q=Q(\rho,\sigma)\) by \(Q_{ii}:=\min(\sigma_{ii}/\rho_{ii},1)\)
    and \(Q_{jk}:=\sigma_{jk}/\rho_{jk}\) for \(j\ne k\).
  </def>

  <known>
    Narasimhachar and Gour, Low-temperature thermodynamics with quantum
    coherence, Nature Communications 6, 7689 (2015), Supplementary
    Information Section S2.1, Observation S2.1, prove that every
    low-temperature thermal operation on \((S,H_S)\) is a cooling map on
    \((S,H_S)\): \(\mathrm{TO}_{\mathrm{low}}(S)\subseteq\mathrm{CM}(S)\) for
    every finite-dimensional \(S\) and every non-degenerate \(H_S\) as in def
    above. This containment is a proved fact, not part of the open claim.

    Narasimhachar and Gour, Results section "State transformation
    conditions", Theorem 1, together with Supplementary Information Section
    S2.3 (definition of UT-majorization), prove that for states
    \(\rho,\sigma\) on \(S\) with \(\rho_{ii}\gt0\) for every \(i\) and
    \(\rho_{jk}\ne0\) for every \(j\ne k\), there exists
    \(\mathcal C\in\mathrm{CM}(S)\) with \(\mathcal C(\rho)=\sigma\) if and
    only if (i) \(u\succ^{\mathrm{UT}}v\), where
    \(u:=(\rho_{11},\ldots,\rho_{dd})^T\), \(v:=(\sigma_{11},\ldots,\sigma_{dd})^T\),
    and (ii) \(Q(\rho,\sigma)\succeq0\). This is the closed-form,
    necessary-and-sufficient transition condition for \(\mathrm{CM}(S)\)
    alone; it is a fully solved result and does not by itself bear on whether
    \(\mathrm{CM}(S)=\mathrm{TO}_{\mathrm{low}}(S)\), since it says nothing
    about which cooling maps arise from an ancilla, Gibbs-type bath state,
    and energy-conserving unitary.

    Narasimhachar and Gour, Results section "Optimally coherent cooling maps
    are thermal", together with Supplementary Information Section S2
    (Corollary S2.7 in that section's numbering), prove
    \(\mathrm{CM}(S)=\mathrm{TO}_{\mathrm{low}}(S)\) exactly when \(d=2\), by
    an explicit ancilla-and-unitary construction realizing every qubit
    cooling map. The same Results section, together with Section S2, also
    proves, for general \(d\), that every individual cooling map saturating a
    coherence-transfer bound established elsewhere in the paper (an
    "optimally coherent process") lies exactly in \(\mathrm{TO}_{\mathrm{low}}(S)\)
    (Corollary S2.8), and that every rational convex combination of
    optimally coherent processes also lies exactly in
    \(\mathrm{TO}_{\mathrm{low}}(S)\), with a general (e.g. irrational-weight)
    mixture of optimally coherent processes only approximated arbitrarily
    well by an element of \(\mathrm{TO}_{\mathrm{low}}(S)\), via density of
    the rationals (Corollary S2.10); this is partial, motivating evidence for
    \(d\gt2\), not a proof that \(\mathrm{CM}(S)\subseteq\mathrm{TO}_{\mathrm{low}}(S)\)
    for every cooling map at every \(d\gt2\). Because channel-set equality
    implies the transition-feasibility relation of the claim above (a single
    map witnesses every transition on each side at once), the \(d=2\) result
    and the \(\mathrm{TO}_{\mathrm{low}}(S)\)-membership results for
    optimally coherent processes and their rational mixtures each establish
    the claim above on their respective restricted domains, in this strictly
    stronger channel-set form.

    Narasimhachar and Gour, Supplementary Information Section S2, part F
    ("Cooling maps and thermal operations"), state (verbatim): "Conjecture 1.
    Cooling maps are equivalent to low-temperature thermal operations, with
    regard to the feasibility of state transitions." They immediately add
    (verbatim): "Note that this could be true even if the set of cooling
    maps is strictly larger than that of thermal operations—there could
    still be a thermal operation achieving every state transition that is
    possible through cooling maps." This passage is the source of the
    transition-feasibility formulation used in the claim above, and of its
    explicit distinction from literal channel-set equality
    \(\mathrm{CM}(S)=\mathrm{TO}_{\mathrm{low}}(S)\).

    Narasimhachar and Gour, Discussion, restate the same conjecture in prose
    (verbatim): "The main open question emerging from this work is whether
    the mathematically characterized cooling maps are equivalent to the
    physically motivated thermal operations, or merely a close approximation
    thereof. Their equivalence for the cases of two-level systems and
    mixtures of optimally coherent processes motivates us to conjecture
    equivalence in general. The study of cooling maps aided by catalysts,
    and possible generalizations to higher temperatures, are other open
    problems that would provide insight into thermodynamics." No proof,
    disproof, or further resolution of the \(d\gt2\) direction of this
    question is given anywhere else in the main text or Supplementary
    Information of the same paper.
  </known>

  <refs>
    <ref>V. Narasimhachar and G. Gour, Low-temperature thermodynamics with quantum coherence, Nature Communications 6, 7689 (2015), DOI: 10.1038/ncomms8689, arXiv:1409.7740, used for: the Results section "The emergence of 'cooling maps'" (definition of cooling maps and of the low-temperature, ground-eigenspace-projector bath state); Supplementary Information Section S2.1, Observation S2.1 (proof that every low-temperature thermal operation is a cooling map); the Results section "State transformation conditions", Theorem 1, together with Supplementary Information Section S2.3 (definition of UT-majorization) (the closed-form necessary-and-sufficient transition condition on cooling maps); the Results section "Optimally coherent cooling maps are thermal" together with Supplementary Information Section S2, Corollaries S2.7, S2.8, and S2.10 (proof that cooling maps and low-temperature thermal operations coincide when \(\dim S=2\) [Corollary S2.7], and the \(\dim S\gt2\) partial evidence that optimally coherent cooling maps [Corollary S2.8] and rational-weight mixtures thereof [Corollary S2.10] lie exactly in low-temperature thermal operations, with general mixtures only approximated arbitrarily well); Supplementary Information Section S2, part F, Conjecture 1 and its accompanying caveat (the exact source of the transition-feasibility formulation used in this entry's claim, as distinct from literal channel-set equality); and the Discussion (the same open question restated in prose, as recorded by this entry's claim).</ref>
  </refs>

  <ask>
    A proof must show, for every finite \(d\ge1\), every non-degenerate
    \(H_S\) on a \(d\)-dimensional \(S\), every cooling map
    \(\mathcal C\in\mathrm{CM}(S)\) (every \(n\le d\) and every choice of
    \(\{\boldsymbol\lambda_j\}_{j=1}^d\subset\mathbb C^n\),
    \(\{\mu_{jk}\}_{1\le j\lt k\le d}\) satisfying the trace-preservation
    relation in def), and every state \(\rho\) on \(S\), writing
    \(\sigma:=\mathcal C(\rho)\), that \(\rho\to_{\mathrm{TO}}\sigma\): it
    must establish, for every such \(\mathcal C\) and \(\rho\), the existence
    of a finite-dimensional ancilla \((A,H_A)\) (which may depend on both
    \(\mathcal C\) and \(\rho\)) and a unitary \(U\) on \(S\otimes A\)
    commuting with \(H_S\otimes\mathbb 1_A+\mathbb 1_S\otimes H_A\) such that
    \(\operatorname{tr}_A[U(\rho\otimes\gamma_A)U^\dagger]=\mathcal C(\rho)\),
    with \(\gamma_A=\Pi_A/g\) as in def. A single argument covering every
    \(d\), every admissible \(\{\boldsymbol\lambda_j\},\{\mu_{jk}\}\), and
    every \(\rho\) is required; verifying finitely many \(d\), a proper
    subfamily of cooling maps, or a proper subset of initial states \(\rho\)
    is not a proof of the claim as quantified above. A construction that,
    for a given \(\mathcal C\), additionally produces a single
    \(\mathcal E\in\mathrm{TO}_{\mathrm{low}}(S)\) equal to \(\mathcal C\) as
    a map on every \(\rho\) at once (i.e. that proves the strictly stronger
    \(\mathrm{CM}(S)\subseteq\mathrm{TO}_{\mathrm{low}}(S)\), as Narasimhachar
    and Gour's own constructions do on the restricted domains recorded in
    known) suffices for the claim but is not required by it.

    A disproof must exhibit some finite \(d\ge3\), some non-degenerate
    \(H_S\) on a \(d\)-dimensional \(S\), and some specific states
    \(\rho,\sigma\) on \(S\) with \(\rho\to_{\mathrm{CM}}\sigma\) (an explicit
    witnessing \(\mathcal C\in\mathrm{CM}(S)\), i.e. explicit \(n\le d\) and
    \(\{\boldsymbol\lambda_j\},\{\mu_{jk}\}\), with \(\mathcal C(\rho)=\sigma\)),
    and prove that \(\rho\to_{\mathrm{TO}}\sigma\) fails: that no
    finite-dimensional ancilla \((A,H_A)\), state \(\gamma_A=\Pi_A/g\), and
    energy-conserving unitary \(U\) satisfies
    \(\operatorname{tr}_A[U(\rho\otimes\gamma_A)U^\dagger]=\sigma\), ruling
    out every such \((A,H_A,U)\), i.e. every element of
    \(\mathrm{TO}_{\mathrm{low}}(S)\) whatsoever, not merely showing that the
    particular witnessing \(\mathcal C\) fails to be an element of
    \(\mathrm{TO}_{\mathrm{low}}(S)\). Because the claim is universally
    quantified over \(d\), \(H_S\), and \((\rho,\sigma)\), exhibiting one such
    instance fully disproves it; \(d=2\) is excluded from this search because
    the claim is already proved there (known above).

    The following count as progress, not as a solution to the claim, unless
    combined with a proof removing the stated restriction: (i) realizing
    \(\rho\to_{\mathrm{TO}}\sigma\) only up to nonzero diamond-norm or
    trace-norm error on \(\sigma\), or only in an asymptotic many-copy sense;
    (ii) realizations using a catalyst system required to return unchanged
    or correlated with \(S\) at the end of the protocol, beyond the plain
    ancilla-and-unitary form of \(\mathrm{TO}_{\mathrm{low}}(S)\) fixed in
    def (Narasimhachar and Gour's own listed open problem of cooling maps
    aided by catalysts); (iii) results restricted to a proper subclass of
    \(\mathrm{CM}(S)\), such as the optimally coherent cooling maps and their
    mixtures already handled by Narasimhachar and Gour, to a proper subclass
    of non-degenerate Hamiltonians, or to a proper subset of the pairs
    \((\rho,\sigma)\) rather than every pair with \(\rho\to_{\mathrm{CM}}\sigma\);
    (iv) extending \(\mathrm{TO}_{\mathrm{low}}(S)\) or \(\mathrm{CM}(S)\) to
    degenerate \(H_S\) or to genuine finite-\(\beta\) baths, unless
    accompanied by a proof that this extension leaves the claim exactly as
    fixed above unchanged.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
