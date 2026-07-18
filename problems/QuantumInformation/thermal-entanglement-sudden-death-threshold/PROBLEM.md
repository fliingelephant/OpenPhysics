# QTD031 Thermodynamic-Limit Persistence of Symmetry-Protected Entanglement Negativity at High Temperature

```xml
<problem id="QTD031">
  <status>open</status>
  <name>Thermodynamic-Limit Persistence of Symmetry-Protected Entanglement Negativity at High Temperature</name>

  <keys>thermal entanglement sudden death of entanglement entanglement sudden death SDOE Gibbs state canonical ensemble strongly symmetric thermal state weakly symmetric thermal state symmetric separability symmetrically separable symmetrically entangled persistence of entanglement POE entanglement negativity logarithmic negativity partial transpose positive partial transpose PPT NPT on-site symmetry homogeneous on-site symmetry Abelian symmetry strong symmetry weak symmetry superselection rule charge sector irrep character isotypic component entangling condition EC symmetric entangling condition SEC negativity condition NC high-temperature entanglement thermodynamic limit low-intersection Hamiltonian degree local Hamiltonian separable state product-state mixture Bakshi Liu Moitra Tang FOCS 2024 high-temperature Gibbs states unentangled Negari Lessa Sahu symmetry enforces entanglement Zeng thermally activated long-range entanglement non-Abelian conservation law SU(2)-invariant spin chain global singlet sector dimer chain distillable entanglement quantum thermodynamics QTD031</keys>

  <claim>
    For a finite Abelian group \(G\), an integer \(q\ge2\) (local
    dimension), and a unitary representation \(u:G\to\mathcal U(\mathbb
    C^q)\) (Def section), write, for \(n\ge2\), \(U_n(g):=u(g)^{\otimes n}\)
    for the induced on-site action on \(n\) qudits.

    Call a triple \((H_n,\Lambda_n,A_n)\) an EC-instance at \(n\) if: \(H_n\)
    is an \((\mathfrak d,\mathfrak k)\)-low-intersection Hamiltonian on \(n\)
    qudits (Def section: locality \(\mathfrak k\) and degree \(\mathfrak d\)
    both fixed constants, independent of \(n\)) with \([U_n(g),H_n]=0\) for
    every \(g\in G\); \(\Lambda_n\in\widehat G\) is a character of \(G\); and
    \(A_n\sqcup B_n=\{1,\ldots,n\}\) is a bipartition; such that the
    Entangling Condition of Negari, Lessa, and Sahu (Def section, "EC")
    holds: \(\forall g\in G\) with \(u(g)\ne I_q\),
    \([U_{A_n}(g),H_n]\,\Pi_{\Lambda_n}\ne0\).

    Claim: there exist a finite Abelian group \(G\), an integer \(q\ge2\), a
    unitary representation \(u:G\to\mathcal U(\mathbb C^q)\), a sequence
    \((H_n,\Lambda_n,A_n)_{n\ge n_0}\) of EC-instances with
    \(\min(|A_n|,|B_n|)\to\infty\) as \(n\to\infty\), and a real number
    \(\beta\gt0\), such that, writing
    \(\rho_{\beta,n}:=e^{-\beta H_n}\Pi_{\Lambda_n}
    /\operatorname{Tr}[e^{-\beta H_n}\Pi_{\Lambda_n}]\) for the canonical
    ensemble at \(n\) (Def section) and \(\mathcal N(\rho_{\beta,n})\) for its
    logarithmic negativity across \(A_n|B_n\) (Def section),
    \[
      \liminf_{n\to\infty}\ \mathcal N(\rho_{\beta,n})\ \gt\ 0.
    \]

    Quantifier note (weaker reading chosen; ambiguity flagged). This is the
    existential reading: it asks only that some EC-sequence and some
    \(\beta\) survive the limit \(n\to\infty\), not that every EC-instance
    sequence does, and not that this hold at every \(\beta\gt0\)
    simultaneously. This matches how Negari, Lessa, and Sahu phrase the
    residual gap in their own Discussion section (Known section below: "it
    could vanish as \(N\to\infty\)" together with "calculations on stabilizer
    models... suggest that the entanglement negativity does not vanish...
    which will be useful to establish more generally") -- an existence
    question about whether their finite-\(n\) mechanism has any
    thermodynamic-limit-robust instance at all, not a claim that it always
    does. A strictly stronger reading -- every EC-instance sequence has
    \(\liminf_n\mathcal N(\rho_{\beta,n})\gt0\) at every \(\beta\gt0\), or at
    every \(\beta\) in the specific interval \((0,\beta^*)\) on which EC is
    shown (Known section) to give entanglement at fixed \(n\) -- is a
    different, harder statement not adopted here. This entry also does not
    require \((H_n)_n\) to be translation-invariant or the bipartition to be
    an equal split: both are additional structure present in the
    constructions Negari, Lessa, and Sahu sketch (their Theorem 3) and in the
    "stabilizer models" they cite as suggestive, so a construction with that
    extra structure remains a valid proof of the claim as stated; it is not
    required by the claim itself, which is stated for the broadest
    admissible class of local Hamiltonians for which the existence question
    is open.
  </claim>

  <def>
    Local Hamiltonian family. For \(n\ge2\), an \(n\)-qudit Hamiltonian is
    \(H_n=\sum_{a=1}^{m}h_a\in\mathbb C^{q^n\times q^n}\) with each
    \(\|h_a\|_{\mathrm{op}}\le1\). Following Bakshi, Liu, Moitra, and Tang
    (Known section), form the dual interaction graph \(\mathfrak G_n\) with
    one vertex per term and an edge between \(a,b\) iff
    \(\operatorname{supp}(h_a)\cap\operatorname{supp}(h_b)\ne\emptyset\); say
    \(H_n\) has locality \(\mathfrak k\) if \(|\operatorname{supp}(h_a)|\le
    \mathfrak k\) for every \(a\), and degree \(\mathfrak d\) if every vertex
    of \(\mathfrak G_n\) has degree at most \(\mathfrak d\); \(H_n\) is
    \((\mathfrak d,\mathfrak k)\)-low-intersection if both hold, with
    \(\mathfrak d,\mathfrak k\) fixed constants not depending on \(n\).

    On-site Abelian symmetry, characters, charge sectors. \(G\) is a finite
    Abelian group, \(u:G\to\mathcal U(\mathbb C^q)\) a fixed unitary
    representation of a single qudit, and
    \(U_n(g):=u(g)^{\otimes n}:\mathcal U(\mathbb C^{q^n})\) the induced
    homogeneous on-site action on \(n\) qudits (Negari, Lessa, and Sahu's
    setting, Known section). Write \(\widehat G\) for the character group of
    \(G\) (since \(G\) is finite Abelian, every irreducible unitary
    representation of \(G\) is a one-dimensional character
    \(\Lambda:G\to U(1)\)). For \(\Lambda\in\widehat G\), the projector onto
    the \(\Lambda\)-isotypic component of \(U_n\) is the standard
    representation-theoretic average
    \[
      \Pi_\Lambda\ :=\ \frac1{|G|}\sum_{g\in G}\overline{\Lambda(g)}\,U_n(g),
      \qquad
      \Pi_\Lambda\,\bigl|\psi\bigr\rangle=\bigl|\psi\bigr\rangle
      \ \Longleftrightarrow\ U_n(g)\bigl|\psi\bigr\rangle=\Lambda(g)
      \bigl|\psi\bigr\rangle\ \ \forall g\in G.
    \]
    A Hamiltonian \(H_n\) is \(G\)-symmetric if \([U_n(g),H_n]=0\) for every
    \(g\in G\); such an \(H_n\) commutes with every \(\Pi_\Lambda\).

    Canonical ensemble. For a \(G\)-symmetric \(H_n\), inverse temperature
    \(\beta\ge0\), and \(\Lambda\in\widehat G\) with
    \(\operatorname{Tr}[e^{-\beta H_n}\Pi_\Lambda]\gt0\), the canonical
    ensemble in charge sector \(\Lambda\) is
    \[
      \rho_{\beta,\Lambda,n}\ :=\ \frac{e^{-\beta H_n}\Pi_\Lambda}
      {\operatorname{Tr}[e^{-\beta H_n}\Pi_\Lambda]}\ ,
    \]
    the Gibbs state of \(H_n\) restricted to, and renormalized on, the
    \(\Lambda\)-charge subspace. (This is the "strongly symmetric" ensemble
    of Negari, Lessa, and Sahu, distinct from the plain, unprojected Gibbs
    state \(\rho_{\beta,n}^{\mathrm{plain}}:=e^{-\beta H_n}
    /\operatorname{Tr}e^{-\beta H_n}\) that Bakshi, Liu, Moitra, and Tang
    study; the two are different physical objects and this entry does not
    assert any direct inequality between them.)

    Bipartition, partial transpose, logarithmic negativity. For a
    bipartition \(A\sqcup B=\{1,\ldots,n\}\) and \(g\in G\), write
    \(U_A(g):=\bigl(\bigotimes_{i\in A}u(g)\bigr)\otimes\mathbb 1_B\) for the
    symmetry action restricted to \(A\) alone (identity on \(B\)); note
    \(U_A(g)\ne U_n(g)\) in general. For a state \(\rho\) on
    \(\mathcal H_A\otimes\mathcal H_B\), let \(\rho^{T_A}\) be its partial
    transpose on \(A\) (in any fixed local product basis; its spectrum does
    not depend on the choice) and \(\|\cdot\|_1\) the trace norm. The
    logarithmic negativity is
    \[
      \mathcal N(\rho)\ :=\ \log_2\bigl\|\rho^{T_A}\bigr\|_1\ \ge\ 0,
    \]
    with \(\mathcal N(\rho)=0\) iff \(\rho\) has positive partial transpose
    (PPT); \(\mathcal N(\rho)\gt0\) certifies that \(\rho\) is entangled
    across \(A|B\) (a computable, one-sided witness: PPT states may or may
    not be entangled, but every NPT state, \(\mathcal N\gt0\), is).

    Entangling condition (EC). For a \(G\)-symmetric
    \((\mathfrak d,\mathfrak k)\)-low-intersection \(H_n\), a character
    \(\Lambda\in\widehat G\), and a bipartition \(A_n|B_n\), say
    \((H_n,\Lambda,A_n)\) satisfies EC if
    \[
      \forall g\in G\text{ with }u(g)\ne I_q,\qquad
      [U_{A_n}(g),H_n]\,\Pi_\Lambda\ \ne\ 0
      \qquad\text{[Negari-Lessa-Sahu Eq.\ (2), Known section]}.
    \]
    (The restriction to \(u(g)\ne I_q\) is necessary and excludes \(g=e\):
    if \(u(g)=I_q\) then \(U_{A_n}(g)=\mathbb 1\) on the full space for
    every bipartition, so \([U_{A_n}(g),H_n]=0\) identically and the
    condition would be unsatisfiable at that \(g\) for every \(H_n\).
    Negari, Lessa, and Sahu state Eq.\ (2) with an unrestricted \(\forall
    g\in G\); their own worked example (Known section) satisfies EC only
    under this implicit exclusion.)
    Since \(H_n\) commutes with the full \(U_n(g)=U_{A_n}(g)\otimes
    U_{B_n}(g)\) but \(U_{A_n}(g)\) need not commute with \(H_n\) alone, EC
    measures the presence, within charge sector \(\Lambda\), of
    \(G\)-charged terms of \(H_n\) straddling the cut \(A_n|B_n\).

    Symmetric entangling condition (SEC). For a \(G\)-symmetric \(H_n\) and
    a bipartition \(A_n|B_n\), say \(H_n\) satisfies SEC if
    \[
      \forall g\in G\text{ with }u(g)\ne I_q,\qquad
      [U_{A_n}(g),H_n]\ \ne\ 0
      \qquad\text{[Negari-Lessa-Sahu Eq.\ (1), Known section]},
    \]
    the same restriction, for the same reason, as for EC above. SEC is the
    unprojected analogue of EC (no \(\Pi_\Lambda\)); it governs entanglement
    of the plain, unprojected Gibbs state \(\rho_{\beta,n}^{\mathrm{plain}}\)
    for \(G\)-symmetric \(H_n\), not the charge-projected canonical
    ensemble that the Claim above concerns.
  </def>

  <known>
    Ainesh Bakshi, Allen Liu, Ankur Moitra, and Ewin Tang, High-Temperature
    Gibbs States are Unentangled and Efficiently Preparable, 2024 IEEE 65th
    Annual Symposium on Foundations of Computer Science (FOCS), pp.
    1027-1036, DOI: 10.1109/FOCS61266.2024.00068, arXiv:2403.16850 (v2, 24
    Feb 2025), Theorem 1.5 (formal, qubit case) and Theorem 1.1 / Theorem 4.20
    (informal / formal qudit version), prove that for an \((\mathfrak
    d,\mathfrak k)\)-low-intersection Hamiltonian \(H\) on \(n\) qubits with
    Pauli terms (Def section terminology, reproduced from their Def.\ 1.3,
    1.4), the plain Gibbs state \(\rho=e^{-\beta H}/\operatorname{tr}
    (e^{-\beta H})\) is separable -- explicitly, a distribution over
    single-qubit stabilizer product states \(A_1\otimes\cdots\otimes A_n\)
    -- for every \(\beta\lt1/(100\mathfrak{d}\mathfrak{k})\), a threshold
    independent of \(n\), so this "sudden death of thermal entanglement"
    (their coinage) already holds robustly in the thermodynamic limit for
    the plain, unprojected Gibbs state. Their Section 1.1 discussion "On
    temperature" states the threshold "cannot be significantly improved" for
    their companion efficient-preparability result (Theorem 1.7), citing
    Sly and Sun's NP-hardness of classical Gibbs sampling for the
    antiferromagnetic Ising model at the classical uniqueness threshold
    \(\beta=\Theta(1/\Delta)\); this is a computational-hardness argument
    about classical sampling, not a construction of entanglement, and
    Bakshi, Liu, Moitra, and Tang give no discussion, tightness claim, or
    matching entangled/non-separable construction for the separability
    threshold of Theorem 1.5 itself. Their paper is the origin of the
    "sudden death of thermal entanglement" phenomenon that Negari, Lessa, and Sahu (next)
    react to; it does not consider any symmetry constraint or charge-sector
    projection, so it neither states nor resolves the claim above, which is
    specifically about the charge-projected canonical ensemble.

    Amir-Reza Negari, Leonardo A. Lessa, and Subhayan Sahu, Symmetry
    Enforces Entanglement at High Temperatures, arXiv:2508.20166 (v2, 2 Oct
    2025), Theorem 1 (informal) and Theorem 2 (formal), prove that for a
    \(G\)-symmetric Hamiltonian \(H_n\) on \(n\) qudits with on-site Abelian
    symmetry \(G\) (Def section) satisfying EC for a character \(\Lambda\)
    and bipartition \(A_n|B_n\) (Def section, their Eq.\ (2)), the canonical
    ensemble \(\rho_{\beta,\Lambda,n}\) is entangled with strictly positive
    logarithmic negativity at arbitrarily high temperature, i.e.\ for every
    \(\beta\) in some interval \((0,\beta^*)\) with \(\beta^*\gt0\) -- for
    each individual, fixed \(n\). Their Theorem 3 further gives what they
    call a complete classification, for homogeneous on-site \(G\) and a
    connected interaction graph, of which charge sectors \(\Lambda\) admit
    an entangling nearest-neighbor perturbation at all (their
    "semiuniform"/"not semiuniform" dichotomy): this addresses, and by their
    own statement resolves, the classification thread of the survey lead
    this entry is drawn from, for the on-site Abelian case specifically; it
    does not address the thermodynamic limit. Their own Discussions section
    states the residual gap verbatim: "our results only imply that a
    faithful measure of entanglement (and negativity) will remain non-zero
    at arbitrarily high temperatures for any fixed system size \(N\), but it
    could vanish as \(N\to\infty\). Calculations on stabilizer models,
    however, suggest that the entanglement negativity does not vanish at
    non-zero \(\beta\) even in the thermodynamic limit \(N\to\infty\), which
    will be useful to establish more generally." This sentence is the direct
    source of the claim above: existence, for at least one EC-sequence and
    one \(\beta\), of a thermodynamic-limit-robust negativity lower bound is
    exactly what they flag as suggested by non-rigorous ("calculations on")
    evidence but not established in general. Their introduction also notes,
    citing separate prior work (their refs.\ [25-27] for non-Abelian
    symmetries and [28,30-34] for anomalous non-on-site symmetries, not
    independently verified for this entry), that symmetry classes broader
    than on-site Abelian were already known to be capable of guaranteeing
    entanglement at infinite temperature before this paper; their own
    contribution is specifically the on-site Abelian, finite-\(n\) case
    described above.

    Shuai Zeng, Thermally Activated Long-Range Entanglement from Non-Abelian
    Conservation Laws, arXiv:2607.12710v1 (14 Jul 2026), states a Theorem
    (unlabeled in the main text) that for a class of \(\mathrm{SU}(2)\)
    -invariant spin chains restricted to the global-singlet sector, there is
    a \(\beta_0\gt0\) such that, for every \(0\le\beta\lt\beta_0\) and all
    sufficiently large even \(n\) with nonempty singlet sector, an explicit
    protocol certifies distillable entanglement
    \(E_D(\rho_{\beta,n})\ge Y_n=\tfrac12\log_2n+O_\beta(1)\) across a
    macroscopic bipartition -- a bound that diverges as \(n\to\infty\) at
    every fixed \(\beta\) in that range, including \(\beta\to0\). For an
    exactly solvable dimer-chain instance of this class, Zeng reports
    that every fixed \(\beta\gt0\) gives
    \(E_D=\tfrac12\log_2n+O(1)\), consistent with the general theorem. Zeng
    explicitly contrasts this with the Abelian case, citing Negari, Lessa,
    and Sahu as reference [6]: "Our result identifies a qualitative
    distinction between Abelian and non-Abelian constraints. Abelian
    constraints can preserve nonzero entanglement at every fixed size, yet
    the known bounds allow it to vanish in the thermodynamic limit [6]."
    This is a July-2026 (most recent-in-survey) source explicitly treating
    the Negari-Lessa-Sahu thermodynamic-limit gap as still open, and its own
    construction does not close that gap: it is built from a non-Abelian
    (\(\mathrm{SU}(2)\)) conservation law and a global-singlet-sector
    restriction, not from an on-site Abelian \(G\)-symmetric EC-instance
    sequence in the sense of the Def section above, so it is not itself an
    instance of the claim as stated (though the mechanism -- a conserved
    non-Abelian charge forcing cross-cut entanglement in a projected sector
    -- is structurally close, and an adaptation to an Abelian EC-instance
    sequence, if one exists, would resolve the claim). Theorem numbering and
    exact wording for this source were obtained from an automated fetch of
    the arXiv HTML rendering rather than a manual read of the typeset PDF,
    and are reported with that caveat, unlike the two sources above, whose
    theorem numbers and quotations were checked against the typeset PDF
    directly.

    Together, these three sources leave exactly the gap recorded by the
    claim above: Bakshi, Liu, Moitra, and Tang establish that the plain
    (unprojected) Gibbs state undergoes sudden death of entanglement at high
    temperature, robustly as \(n\to\infty\); Negari, Lessa, and Sahu show
    that projecting onto an EC-satisfying charge sector defeats this at
    every fixed \(n\), but explicitly flag, without proving either
    direction, whether this persistence survives \(n\to\infty\); and Zeng
    (2026) reports a thermodynamic-limit-divergent construction for a
    structurally different, non-Abelian symmetry class, explicitly citing
    the Abelian thermodynamic-limit question as still unresolved. This
    entry's search was restricted to these three cited sources, per the
    scope of this survey lead; it did not search for other papers that may
    bear on the on-site Abelian thermodynamic-limit question (for instance,
    later or unrelated work on entanglement in symmetric random or
    stabilizer ensembles), so residual doubt about whether the claim has
    been resolved elsewhere is recorded here rather than closed by further
    search.
  </known>

  <refs>
    <ref>Ainesh Bakshi, Allen Liu, Ankur Moitra, and Ewin Tang, High-Temperature Gibbs States are Unentangled and Efficiently Preparable, 2024 IEEE 65th Annual Symposium on Foundations of Computer Science (FOCS), pp. 1027-1036, DOI: 10.1109/FOCS61266.2024.00068, arXiv:2403.16850 (v2, 24 Feb 2025), Theorem 1.5 and Section 1.1 ("On temperature"), used for the baseline sudden-death-of-entanglement threshold for the plain (unprojected) Gibbs state and for confirming it does not address any symmetry-projected ensemble.</ref>
    <ref>Amir-Reza Negari, Leonardo A. Lessa, and Subhayan Sahu, Symmetry Enforces Entanglement at High Temperatures, arXiv:2508.20166 (v2, 2 Oct 2025), Theorem 1, Theorem 2, Theorem 3, Eq.\ (2) (Entangling Condition), and the Discussions section, used for the Def section's canonical ensemble, EC, and charge-sector formalism, for the finite-\(n\) persistence-of-entanglement result, and for the verbatim statement of the thermodynamic-limit open question that is the direct source of the claim above.</ref>
    <ref>Shuai Zeng, Thermally Activated Long-Range Entanglement from Non-Abelian Conservation Laws, arXiv:2607.12710v1 (14 Jul 2026), main Theorem and the paragraph contrasting Abelian and non-Abelian constraints (citing Negari, Lessa, and Sahu as its reference [6]), used for confirming, as of the most recent source in this survey lead, that the Abelian thermodynamic-limit question remains open, and for the structurally different non-Abelian construction described in the Known section above.</ref>
  </refs>

  <ask>
    A proof exhibits a sequence \((H_n,\Lambda_n,A_n)_{n\ge n_0}\) of
    EC-instances (Def section: \((\mathfrak d,\mathfrak k)\)-low-intersection,
    \(G\)-symmetric \(H_n\), with \(\mathfrak d,\mathfrak k,G,u\) fixed
    independent of \(n\)) with \(\min(|A_n|,|B_n|)\to\infty\), a real
    \(\beta\gt0\), and a proof (not a finite-size numerical calculation)
    that \(\liminf_{n\to\infty}\mathcal N(\rho_{\beta,n})\gt0\). A rigorous
    proof of the "stabilizer model" non-vanishing-negativity behavior that
    Negari, Lessa, and Sahu report as suggestive but unproven (Known
    section), if it meets the Def-section hypotheses above, is itself a
    complete proof of the claim, not merely progress toward one.

    A disproof shows that for every sequence of EC-instances
    \((H_n,\Lambda_n,A_n)_{n\ge n_0}\) satisfying the Def-section hypotheses
    with \(\min(|A_n|,|B_n|)\to\infty\), and every \(\beta\gt0\),
    \(\liminf_{n\to\infty}\mathcal N(\rho_{\beta,n})=0\). Because the claim
    is existential over both the sequence and \(\beta\), a disproof must
    rule out every admissible sequence and temperature simultaneously; no
    single failed construction disproves it.

    The following count as progress, not as a solution, unless combined
    with what is stated: (i) finite-\(n\) results such as Negari, Lessa, and
    Sahu's own Theorem 2 (Known section), which already establish
    \(\mathcal N(\rho_{\beta,n})\gt0\) at each fixed \(n\) but say nothing
    about the limit; (ii) numerical, exact-diagonalization, or
    tensor-network evidence of non-vanishing negativity at large but finite
    \(n\), without a limiting proof; (iii) a construction with
    \(\min(|A_n|,|B_n|)\) bounded rather than diverging, which does not
    address macroscopic (thermodynamic-limit) entanglement; (iv) a proof of
    persistent entanglement for the weakly-symmetric Gibbs ensemble
    (Negari, Lessa, and Sahu's \(\rho_{\beta,n}^{\mathrm{plain}}\) for
    \(G\)-symmetric \(H_n\), Def section) under the Symmetric Entangling
    Condition (SEC) of Negari, Lessa, and Sahu (Def section) rather than
    for the canonical, charge-projected ensemble under EC as defined
    above -- a related but logically distinct
    statement about a different state; (v) a proof of persistent
    entanglement by some measure other than logarithmic negativity (e.g.\
    mutual information, or a non-computable entanglement measure), since the
    claim is specifically about \(\mathcal N\); (vi) Zeng's non-Abelian,
    global-singlet-sector construction (Known section) as it stands, since
    it is not an on-site Abelian EC-instance sequence in the sense of the
    Def section; an explicit adaptation of that mechanism into a genuine
    EC-instance sequence, with a proof that the adaptation still satisfies
    EC and still gives non-vanishing \(\liminf_n\mathcal N\), would however
    count as a full proof; (vii) an upper bound showing
    \(\mathcal N(\rho_{\beta,n})\) can be made to decay arbitrarily slowly
    (e.g.\ sub-polynomially) without excluding it reaching zero in the
    limit, which is progress toward a disproof but not a disproof itself.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
