# 48 Equal-Entropy Exact Catalysis with an Infinite-Dimensional Catalyst

```xml
<problem id="48">
  <status>open</status>
  <kind>construction</kind>
  <field>QTD</field>
  <name>Equal-Entropy Exact Catalysis with an Infinite-Dimensional Catalyst</name>
  <keys>catalytic entropy conjecture CEC exact catalytic entropy conjecture equal entropy catalysis infinite-dimensional catalyst infinite dimensional catalyst countably infinite catalyst quantum catalysis catalytic transition catalytic transformation catalyst returned exactly trumping catalytic majorization von Neumann entropy entropy monotonicity subadditivity of entropy isospectral states unitarily equivalent states unitary equivalence spectrum matching single-shot quantum thermodynamics single-shot quantum information noisy operations resource theory of entropy Boes Eisert Gallego Muller Wilming von Neumann entropy from unitarity Wilming entropy and reversible catalysis Wilming correlations in typicality IQOQI Vienna open quantum problems OQP problem 49 catalytic entropy conjecture residue</keys>
  <related>45 60</related>

  <claim>
    Fix a finite-dimensional Hilbert space \(S\) and let \(\rho_S,\rho'_S\) be
    density operators on \(S\) satisfying
    \[
      H(\rho_S)=H(\rho'_S),
      \qquad
      \rho_S\not\simeq\rho'_S,
    \]
    where \(H\) is the von Neumann entropy and \(\simeq\) is the
    unitary-equivalence relation on states of \(S\), both defined below; such
    pairs exist exactly once \(\dim S\geq3\) (Def section, Remark on scope).

    For every finite-dimensional \(S\) and every pair \(\rho_S,\rho'_S\) on
    \(S\) meeting both conditions above, there exist an infinite-dimensional
    Hilbert space \(C\) with a countable orthonormal basis, a density
    operator \(\sigma_C\) on \(C\), and a unitary \(U\) on \(S\otimes C\)
    such that
    \[
      \operatorname{tr}_C\!\left[U(\rho_S\otimes\sigma_C)U^\dagger\right]=\rho'_S,
      \qquad
      \operatorname{tr}_S\!\left[U(\rho_S\otimes\sigma_C)U^\dagger\right]=\sigma_C,
    \]
    both exactly, with no error anywhere. No bound on the von Neumann
    entropy of \(\sigma_C\) (finite or infinite) and no restriction to a
    \(\sigma_C\) chosen independently of \((\rho_S,\rho'_S)\) is imposed;
    \(C\), \(\sigma_C\), and \(U\) may depend on \(S\), \(\rho_S\), and
    \(\rho'_S\). The claim imposes no rank hypothesis analogous to
    \(\mathrm{rank}(\rho'_S)\geq\mathrm{rank}(\rho_S)\) from the
    strict-inequality exact catalytic entropy conjecture (CEC, Known section
    below): no source found for this entry states a rank hypothesis for the
    equal-entropy case, so the claim is posed over every pair satisfying only
    the two displayed conditions, and the absence of a rank hypothesis here
    is a scope choice, not a verified necessary condition.

    This is the residual case left open once the CEC itself -- the same
    statement with \(H(\rho'_S)\gt H(\rho_S)\) (strict) in place of equality,
    a finite-dimensional \(C\) in place of an infinite-dimensional one, and
    an additional rank hypothesis -- was resolved (Known section below). The
    originating survey lead's phrase "convert between" is ambiguous between
    the one-directional reading fixed above, for an ordered pair
    \((\rho_S,\rho'_S)\), and a reading additionally requiring the same
    catalyst to realize both \(\rho_S\to\rho'_S\) and \(\rho'_S\to\rho_S\);
    the Def section's Remark on isospectral products shows the two readings
    coincide whenever \(\sigma_C\) has finite von Neumann entropy, so the
    one-directional statement above is used as the canonical claim without
    loss on that domain, and the two readings' possible divergence for
    infinite-entropy \(\sigma_C\) is left to the Ask section.
  </claim>

  <def>
    Density operator, spectrum. For a finite-dimensional Hilbert space
    \(S\) with \(d=\dim S\), a density operator on \(S\) is a positive
    semi-definite \(\rho_S\) with \(\operatorname{tr}\rho_S=1\). Write
    \(\mathrm{spec}(\rho_S)=(p_1\geq p_2\geq\cdots\geq p_d\geq0)\) for its
    eigenvalues counted with multiplicity in non-increasing order
    (\(\sum_ip_i=1\)).

    Von Neumann entropy. For \(\rho_S\) as above,
    \(H(\rho_S):=-\operatorname{tr}[\rho_S\ln\rho_S]=-\sum_{i=1}^dp_i\ln p_i\),
    with \(0\ln0:=0\). For a density operator \(\sigma_C\) on a Hilbert
    space \(C\) with a countable orthonormal basis and eigenvalues
    \((q_j)_{j\in J}\) (\(J\) countable, \(q_j\geq0\), \(\sum_jq_j=1\)),
    \(H(\sigma_C):=-\sum_jq_j\ln q_j\in[0,\infty]\); a sum of non-negative
    terms is always well-defined in the extended reals, so \(H(\sigma_C)\)
    need not be finite.

    Unitary equivalence. For density operators \(\rho_S,\rho'_S\) on the
    same \(S\), write \(\rho_S\simeq\rho'_S\) iff there is a unitary \(V\) on
    \(S\) with \(V\rho_SV^\dagger=\rho'_S\), equivalently iff
    \(\mathrm{spec}(\rho_S)=\mathrm{spec}(\rho'_S)\) entrywise.
    \(\rho_S\simeq\rho'_S\) implies \(H(\rho_S)=H(\rho'_S)\), since entropy
    is a function of the spectrum alone, but not conversely.

    Remark (scope, elementary). For \(\dim S\leq2\), every pair with
    \(H(\rho_S)=H(\rho'_S)\) already satisfies \(\rho_S\simeq\rho'_S\): with
    \(\mathrm{spec}(\rho_S)=(p,1-p)\), \(p\in[0,1/2]\), the binary entropy
    \(p\mapsto-p\ln p-(1-p)\ln(1-p)\) is strictly monotonic on \([0,1/2]\),
    so \(H(\rho_S)=H(\rho'_S)\) forces \(p=p'\). Pairs with
    \(H(\rho_S)=H(\rho'_S)\) and \(\rho_S\not\simeq\rho'_S\) exist for every
    \(\dim S\geq3\): restrict to \(\dim S=3\) without loss (pad with zero
    eigenvalues for \(\dim S\gt3\)); the interior points \((0.9,0.05,0.05)\)
    and \((\frac13,\frac13,\frac13)\) of the probability simplex have
    entropies \(\approx0.394\) and \(\ln3\approx1.099\) nats respectively,
    both different from \(\ln2\approx0.693\); the straight segment joining
    them stays in the interior of the simplex (every point on it has three
    strictly positive entries), so by continuity of entropy some point on
    that segment has entropy exactly \(\ln2\) -- the entropy of
    \((\frac12,\frac12,0)\) -- while having no zero entry, hence not a
    permutation of \((\frac12,\frac12,0)\). So the claim above is non-vacuous
    exactly from \(\dim S=3\) on.

    Catalyst, exact catalytic transition. A catalyst is a pair
    \((C,\sigma_C)\) of a Hilbert space \(C\) and a density operator
    \(\sigma_C\) on \(C\); \(C\) is finite-dimensional if \(\dim C\lt\infty\)
    and infinite-dimensional if \(C\) has a countably infinite orthonormal
    basis. Given \(\rho_S,\rho'_S\) on \(S\) and a catalyst \((C,\sigma_C)\),
    a unitary \(U\) on \(S\otimes C\) realizes an exact catalytic transition
    \(\rho_S\to\rho'_S\) via \((C,\sigma_C,U)\) iff
    \(\operatorname{tr}_C[U(\rho_S\otimes\sigma_C)U^\dagger]=\rho'_S\) and
    \(\operatorname{tr}_S[U(\rho_S\otimes\sigma_C)U^\dagger]=\sigma_C\)
    exactly; the joint state \(U(\rho_S\otimes\sigma_C)U^\dagger\) is not
    required to equal \(\rho'_S\otimes\sigma_C\) and may remain correlated
    across the \(S\!:\!C\) cut. This is the operational relation used by
    Boes, Eisert, Gallego, Müller, and Wilming and by Wilming for the CEC
    (Known section below), generalized here from a finite-dimensional to an
    arbitrary \(C\); it carries no Hamiltonian, energy-conservation, or
    Gibbs-state structure of any kind, unlike the thermal-operation-based
    catalysis of the companion entries 45 and 60 in this same
    directory.

    Remark (isospectral reduction; elementary, not attributed to a cited
    source). If \(H(\rho_S)=H(\rho'_S)\), every exact catalytic transition
    \(\rho_S\to\rho'_S\) via \((C,\sigma_C,U)\) with \(H(\sigma_C)\lt\infty\)
    has \(U(\rho_S\otimes\sigma_C)U^\dagger=\rho'_S\otimes\sigma_C\) exactly,
    i.e. the joint output cannot remain correlated: unitary invariance and
    additivity of \(H\) on product states give
    \(H\!\left(U(\rho_S\otimes\sigma_C)U^\dagger\right)=
    H(\rho_S)+H(\sigma_C)=H(\rho'_S)+H(\sigma_C)\), which is exactly the
    equality case of the subadditivity inequality
    \(H(\tau_{SC})\leq H(\tau_S)+H(\tau_C)\) for a state \(\tau_{SC}\) on
    \(S\otimes C\) with marginals \(\tau_S,\tau_C\) (equality holds iff
    \(\tau_{SC}=\tau_S\otimes\tau_C\)). Consequently
    \(\rho_S\otimes\sigma_C\simeq\rho'_S\otimes\sigma_C\) (as bipartite
    states on \(S\otimes C\), \(U\) conjugating one into the other), so
    realizing an exact catalytic transition \(\rho_S\to\rho'_S\) with a
    finite-entropy catalyst is, at equal entropy, equivalent to finding a
    \(\sigma_C\) with \(\mathrm{spec}(\rho_S)\otimes\mathrm{spec}(\sigma_C)=
    \mathrm{spec}(\rho'_S)\otimes\mathrm{spec}(\sigma_C)\) as multisets,
    writing \(\mathrm{spec}(\rho_S)\otimes\mathrm{spec}(\sigma_C)\) for the
    multiset of pairwise products \(\{p_iq_j\}\). Since multiset equality is
    symmetric in \(\rho_S,\rho'_S\), the same \((C,\sigma_C)\) then also
    realizes \(\rho'_S\to\rho_S\) (via a second unitary matching eigenbases
    the other way), which is the equivalence used in the Claim section to
    fix the one-directional reading without loss on the finite-entropy
    domain. This reduction is stated only for \(H(\sigma_C)\lt\infty\);
    whether it, or the claim itself, extends to catalysts of infinite von
    Neumann entropy is not addressed by any source found for this entry.
  </def>

  <known>
    Boes, Eisert, Gallego, Müller, and Wilming, Von Neumann Entropy from
    Unitarity, Physical Review Letters 122, 210402 (2019), introduce the
    exact catalytic entropy conjecture (CEC) in the form generalized above:
    for finite-dimensional \(\rho_S,\rho'_S\) on a common \(S\) with
    \(H(\rho'_S)\gt H(\rho_S)\) and \(\mathrm{rank}(\rho'_S)\geq
    \mathrm{rank}(\rho_S)\), they conjecture that a finite-dimensional
    catalyst realizing the exact transition \(\rho_S\to\rho'_S\) of the Def
    section above always exists, and prove a restricted version of the same
    statement in which an additional dephasing environment is available.

    Wilming, Entropy and Reversible Catalysis, Physical Review Letters 127,
    260402 (2021), proves, for finite-dimensional quantum states (von
    Neumann entropy) and separately for finite discrete probability
    distributions (Shannon entropy, with permutations in place of
    unitaries), that non-decreasing entropy is necessary and sufficient for
    a transition realized by a finite-dimensional catalyst returned
    exactly, with the transformed system's state matched only within
    arbitrary trace distance \(\epsilon\gt0\) of the target for the
    general, non-strict statement -- not the Def section's zero-error exact
    catalytic transition. Its own abstract accordingly describes this
    general result as resolving an "(approximate)" form of the CEC. The
    IQOQI Vienna Open Quantum Problems page (Known section below) confirms
    that "approximate" qualifies the target state reached, not the
    catalyst's exact return, and reports the zero-error, exact-target
    extension as applying to the strict-inequality sub-case matching Boes,
    Eisert, Gallego, Müller, and Wilming's hypotheses above
    (\(H(\rho'_S)\gt H(\rho_S)\), \(\mathrm{rank}(\rho'_S)\geq
    \mathrm{rank}(\rho_S)\)), not to the general non-strict statement.
    Since equal entropy \(H(\rho'_S)=H(\rho_S)\) violates that strict
    inequality by hypothesis, this sufficiency direction's zero-error form
    does not, by itself, reach the equal-entropy case either way; this
    paragraph was prepared from the abstract and the OQP page's account of
    it, the full text not having been retrieved.

    Wilming, Correlations in Typicality and an Affirmative Solution to the
    Exact Catalytic Entropy Conjecture, Quantum 6, 858 (2022), proves the
    CEC's own exact statement recalled above: for finite-dimensional
    \(\rho_S,\rho'_S\) of the same dimension with \(H(\rho'_S)\gt H(\rho_S)\)
    (strict) and \(\mathrm{rank}(\rho'_S)\geq\mathrm{rank}(\rho_S)\), a
    finite-dimensional catalyst realizing the exact transition
    \(\rho_S\to\rho'_S\) always exists; its abstract states the underlying
    technical lemma as a majorization statement about finitely many
    tensor-copies of \(\rho_S\). This entry was prepared from this paper's
    abstract and its Quantum-journal listing page only: the full-text PDF
    could not be parsed and no HTML version was found, so the exact theorem
    number is not confirmed, and no discussion of the equal-entropy case or
    of infinite-dimensional catalysts was located in the material that could
    be fetched.

    IQOQI Vienna, Open Quantum Problems database, Problem 49, "Catalytic
    entropy conjecture," page fetched 2026-07-18, records the CEC as
    originally posed by Boes, Eisert, Gallego, Müller, and Wilming (2019)
    and as affirmatively resolved; its Solution-section prose inline-cites
    only Wilming's 2021 paper above, for both an approximate-variant step
    and its extension to the exact conjecture, and lists Wilming's 2022
    paper above only in its bibliography, never pointing to it from that
    prose. The same page separately records, as a remaining open point,
    exactly the case fixed by the claim above: equal von Neumann entropy,
    \(H(\rho'_S)=H(\rho_S)\). It reports this case as unresolved,
    states that a finite-dimensional catalyst cannot realize it, and raises
    an infinite-dimensional catalyst as a possible but unproven route; two
    independent automated fetches of the page (one requesting a verbatim
    reproduction) agree on this substance. This entry's claim is a direct
    formalization of that reported residual question, and is the direct
    source of the survey lead this entry records. This entry could not
    independently verify, from a primary theorem statement in either
    Wilming paper above, the page's assertion that finite-dimensional
    catalysts provably fail at equal entropy; that assertion is recorded
    here as this database's own reported position, not as independently
    confirmed by this entry.
  </known>

  <refs>
    <ref>P. Boes, J. Eisert, R. Gallego, M. P. Müller, and H. Wilming, Von Neumann Entropy from Unitarity, Physical Review Letters 122, 210402 (2019), DOI: 10.1103/PhysRevLett.122.210402, arXiv:1807.08773, used for the original statement of the exact catalytic entropy conjecture (CEC), including its strict-entropy-inequality and rank hypotheses, and for the restricted, dephasing-assisted case it proves.</ref>
    <ref>H. Wilming, Entropy and Reversible Catalysis, Physical Review Letters 127, 260402 (2021), DOI: 10.1103/PhysRevLett.127.260402, arXiv:2012.05573, used for the necessary-and-sufficient, non-decreasing-entropy criterion for catalytic transitions with a finite-dimensional, exactly-returned catalyst and the target state matched exactly only for the strict-inequality sub-case (matched within arbitrary trace distance otherwise), and for the paper's own description, in its abstract, of its result as resolving an "(approximate)" form of the CEC.</ref>
    <ref>H. Wilming, Correlations in Typicality and an Affirmative Solution to the Exact Catalytic Entropy Conjecture, Quantum 6, 858 (2022), DOI: 10.22331/q-2022-11-10-858, arXiv:2205.08915, used for the exact resolution of the CEC for finite-dimensional states of strictly smaller von Neumann entropy and matching rank hypothesis, and for the finite-dimensional, marginal-only definition of exact catalytic transition generalized in the Def section above; consulted only via its abstract and its Quantum-journal listing page, not the full text.</ref>
    <ref>IQOQI Vienna, Open Quantum Problems database, Problem 49, "Catalytic entropy conjecture," https://oqp.iqoqi.oeaw.ac.at/catalytic-entropy-conjecture, page content as fetched 2026-07-18, used for the identification of the CEC's proposers, the page's own inline crediting of its resolution to Wilming's 2021 paper above (its 2022 paper above appears only in its bibliography, not inline-cited), and -- the direct source of this entry's claim -- the statement that the equal-entropy case remains unresolved, that finite-dimensional catalysts cannot realize it, and that an infinite-dimensional catalyst is a proposed but unproven route. This entry relies on the page's own automatically-rendered text rather than a manually verified byte-exact quotation.</ref>
  </refs>

  <ask>
    A proof gives, for every finite-dimensional \(S\) and every pair
    \(\rho_S,\rho'_S\) on \(S\) with \(H(\rho_S)=H(\rho'_S)\) and
    \(\rho_S\not\simeq\rho'_S\), an infinite-dimensional \(C\) (countable
    orthonormal basis), a density operator \(\sigma_C\) on \(C\), and a
    unitary \(U\) on \(S\otimes C\) realizing the exact catalytic transition
    \(\rho_S\to\rho'_S\) of the Def section, simultaneously for every such
    pair (a single uniform construction, or a proof of existence for every
    pair taken separately, both qualify; a construction exhibited only for
    specific \(\dim S\) or specific pairs does not).

    A disproof exhibits one finite-dimensional \(S\) and one pair
    \(\rho_S,\rho'_S\) on \(S\) with \(H(\rho_S)=H(\rho'_S)\) and
    \(\rho_S\not\simeq\rho'_S\), together with a proof that no
    infinite-dimensional \(C\), no density operator \(\sigma_C\) on \(C\),
    and no unitary \(U\) on \(S\otimes C\) realize the exact catalytic
    transition \(\rho_S\to\rho'_S\). Because the claim is universally
    quantified over every admissible pair, a single such impossibility proof
    disproves it, even though it leaves every other admissible pair
    undecided.

    The following count as progress, not as a solution to the claim, unless
    combined with a proof removing the stated restriction: (i) a realizing
    \((C,\sigma_C,U)\) constructed only for a proper subfamily of admissible
    pairs (a fixed \(\dim S\), or pairs related by additional structure),
    not shown to extend to every admissible pair; (ii) an approximate
    construction, i.e. \(\operatorname{tr}_C[U(\rho_S\otimes\sigma_C)
    U^\dagger]\) within trace distance \(\epsilon\) of \(\rho'_S\) for every
    \(\epsilon\gt0\) but not exactly equal to \(\rho'_S\) for any single
    \(U\); (iii) a realizing catalyst restricted to structure not required
    by the claim above, such as finite von Neumann entropy
    \(H(\sigma_C)\), a fixed \(\sigma_C\) chosen independently of
    \((\rho_S,\rho'_S)\), or a sequence of finite-dimensional catalysts not
    shown to have an infinite-dimensional exact limit; (iv) a resolution of
    the equivalent isospectral-product formulation in the Def section's
    Remark restricted to catalysts of finite von Neumann entropy, without
    addressing whether an infinite-entropy \(\sigma_C\) could realize
    additional pairs or change which of the two "convert between" readings
    in the Claim section applies.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
