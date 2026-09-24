# 60 Strict Catalytic Convertibility under Thermal Operations

```xml
<problem id="60">
  <status>open</status>
  <kind>construction</kind>
  <field>QTD</field>
  <name>Strict Catalytic Convertibility under Thermal Operations</name>
  <keys>thermal operations catalytic thermal operations strict catalysis strict catalytic thermal operations exact catalysis exact catalytic thermal operations uncorrelated catalyst catalyst returned exactly catalyst returned unperturbed quantum coherence energy coherence coherent state convertibility state conversion state interconversion generalized free energies generalized second laws of quantum thermodynamics second laws of quantum thermodynamics free coherence coherence monotone asymmetry monotone time-translation covariance time-translation symmetry Bohr frequency mode operator resource theory of quantum thermodynamics resource theory of athermality single-shot quantum thermodynamics one-shot quantum thermodynamics Renyi divergence Renyi relative entropy correlated catalysis catalytic majorization trumping relation quantum catalysis Gibbs state Brandao Horodecki Ng Oppenheim Wehner second laws Lostaglio Jennings Rudolph beyond free energy</keys>
  <related>45</related>

  <claim>
    Fix a finite-dimensional quantum system \(S\) with Hamiltonian
    \(H_S=\sum_a E_a\Pi_a\) (spectral decomposition, with the \(E_a\)
    distinct real numbers and the \(\Pi_a\) orthogonal projectors on \(S\)
    summing to the identity) and an inverse temperature
    \(\beta\in(0,\infty)\). Let \(\rho_S,\sigma_S\) be arbitrary density
    operators on \(S\), not assumed to commute with \(H_S\).

    For every finite \(\dim S\), every Hamiltonian \(H_S\) on \(S\), every
    \(\beta\in(0,\infty)\), and every pair \(\rho_S,\sigma_S\) of density
    operators on \(S\) -- including pairs with \([\rho_S,H_S]\neq0\) or
    \([\sigma_S,H_S]\neq0\) -- give a relation
    \(\mathcal C(\rho_S,\sigma_S,H_S,\beta)\), stated purely in terms of
    \(\rho_S\), \(\sigma_S\), \(H_S\), and \(\beta\) and not by reference to
    the existence of an unspecified catalyst, bath, joint unitary, or
    auxiliary system, that is both necessary and sufficient for the
    existence of a finite-dimensional catalyst system \(C\) with Hamiltonian
    \(H_C\) and state \(\omega_C\) on \(C\), together with a thermal
    operation \(\Phi_{SC}\) on \(S\otimes C\) (defined below), such that
    \[
      \Phi_{SC}(\rho_S\otimes\omega_C)=\sigma_S\otimes\omega_C
    \]
    exactly: the catalyst is returned unperturbed and left completely
    uncorrelated with \(S\), the system output is exactly \(\sigma_S\), and
    there is no error anywhere in the protocol. A relation \(\mathcal C\)
    satisfies this requirement only if it can be evaluated without an
    existential or infimum quantifier ranging over a catalyst, bath, joint
    unitary, or auxiliary system of dimension unbounded in \(\dim S\); in
    particular \(\mathcal C:=[\exists\text{ finite-dim }C,H_C,\omega_C,
    \Phi_{SC}\text{ with }\Phi_{SC}(\rho_S\otimes\omega_C)=
    \sigma_S\otimes\omega_C]\) itself does not satisfy it.

    A correct \(\mathcal C\) must reduce, whenever
    \([\rho_S,H_S]=[\sigma_S,H_S]=0\), to a relation logically equivalent
    to: there exists a unitary \(U\) on \(S\) with \([U,H_S]=0\) and
    \(\sigma_S=U\rho_SU^\dagger\), or \(F_\alpha(\rho_S,H_S)\gt
    F_\alpha(\sigma_S,H_S)\) for every \(\alpha\in\mathbb R\setminus\{0\}\),
    where \(F_\alpha\) is the generalized free energy defined below -- the
    criterion reported by Lipka-Bartosik, Wilming, and Ng as necessary and
    sufficient for exact, uncorrelated catalytic thermal operations between
    energy-incoherent state pairs, though not independently confirmed in
    this exact, pointwise-strict form by this entry's reading of the
    sources cited for it, as detailed in the Known section below. No
    relation \(\mathcal C\) meeting the requirement above for every finite
    \(\dim S\) is currently proved to be both necessary and sufficient once
    \(\rho_S\) or \(\sigma_S\) is permitted to carry coherence between
    eigenspaces of \(H_S\).
  </claim>

  <def>
    Thermal operation. For a finite-dimensional system \(X\) with
    Hamiltonian \(H_X\), a thermal operation on \(X\) at inverse temperature
    \(\beta\) is a CPTP map
    \[
      \Phi_X(\rho)=
      \operatorname{tr}_B\!\left[
        U_{XB}(\rho\otimes\gamma_B)U_{XB}^\dagger
      \right],
      \qquad
      [U_{XB},\,H_X\otimes I_B+I_X\otimes H_B]=0,
    \]
    where \(B\) is a finite-dimensional bath with Hamiltonian \(H_B\) and
    Gibbs state \(\gamma_B=e^{-\beta H_B}/\operatorname{tr}e^{-\beta H_B}\);
    \(B\), \(H_B\), and \(U_{XB}\) are otherwise unconstrained and may be
    chosen freely for each \(\Phi_X\).

    Catalyst and strict catalytic thermal operation. A catalyst for the pair
    \((\rho_S,\sigma_S)\) at inverse temperature \(\beta\) is a
    finite-dimensional system \(C\) with Hamiltonian \(H_C\) and a state
    \(\omega_C\) on \(C\) (not assumed to commute with \(H_C\)). Taking
    \(X=S\otimes C\) with \(H_X=H_S\otimes I_C+I_S\otimes H_C\) in the
    definition above, a strict catalytic thermal operation realizing
    \(\rho_S\to\sigma_S\) via \((C,H_C,\omega_C)\) is a thermal operation
    \(\Phi_{SC}\) on \(S\otimes C\) at inverse temperature \(\beta\) with
    \(\Phi_{SC}(\rho_S\otimes\omega_C)=\sigma_S\otimes\omega_C\) exactly.
    Write \(\rho_S\to_{\mathrm{sTO}}\sigma_S\) for "there exists a catalyst
    \((C,H_C,\omega_C)\) and a strict catalytic thermal operation
    \(\Phi_{SC}\) realizing \(\rho_S\to\sigma_S\)"; this is the relation
    fixed by the claim above. (Brandão, Horodecki, Ng, Oppenheim, and Wehner
    below call this simply "catalytic thermal operations"; Lipka-Bartosik,
    Wilming, and Ng below call it "strict catalysis", Definition III.1. The
    label \(\mathrm{sTO}\) is used here only to keep this relation
    typographically distinct from the plain, catalyst-free
    \(\to_{\mathrm{TO}}\) of the companion entry 45.)

    Correlated catalytic thermal operation (contrast, used only in the
    Known section below). A correlated catalytic thermal
    operation realizing \(\rho_S\to\sigma_S\) via \((C,H_C,\omega_C)\) is a
    thermal operation \(\Phi_{SC}\) on \(S\otimes C\) with only
    \[
      \operatorname{tr}_C[\Phi_{SC}(\rho_S\otimes\omega_C)]=\sigma_S,
      \qquad
      \operatorname{tr}_S[\Phi_{SC}(\rho_S\otimes\omega_C)]=\omega_C,
    \]
    i.e. only the marginals are required to match; the joint output
    \(\Phi_{SC}(\rho_S\otimes\omega_C)\) need not equal
    \(\sigma_S\otimes\omega_C\) and may remain correlated across the
    \(S\!:\!C\) cut. Write \(\rho_S\to_{\mathrm{cTO}}\sigma_S\) for this
    strictly weaker relation. Taking the same \(\Phi_{SC}\) shows
    \(\rho_S\to_{\mathrm{sTO}}\sigma_S\) implies \(\rho_S\to_{\mathrm{cTO}}
    \sigma_S\); the converse fails in general (see the Known section
    below). This entry's claim concerns only \(\to_{\mathrm{sTO}}\).

    Bohr frequency, mode operator, energy-incoherent. For \(\omega\in
    \mathbb R\), the mode, or Bohr-frequency, component at frequency
    \(\omega\) of an operator \(X\) on \(S\) is
    \(X^{(\omega)}=\sum_{a,b:\,E_a-E_b=\omega}\Pi_aX\Pi_b\), so that
    \(X=\sum_\omega X^{(\omega)}\). A state \(\rho_S\) is energy-incoherent
    iff \(\rho_S=\rho_S^{(0)}\), equivalently \([\rho_S,H_S]=0\),
    equivalently \(\Pi_a\rho_S\Pi_b=0\) for every \(a\neq b\); otherwise
    \(\rho_S\) carries coherence between every pair of eigenspaces
    \(a\neq b\) with \(\Pi_a\rho_S\Pi_b\neq0\). Write
    \(D_{H_S}(\rho):=\rho^{(0)}=\sum_a\Pi_a\rho\Pi_a\) for complete dephasing
    of \(\rho\) onto the eigenbasis of \(H_S\); \(D_{H_S}(\rho)\) is always
    energy-incoherent.

    Generalized free energy. Fix the Gibbs state
    \(\gamma_S=e^{-\beta H_S}/Z\), \(Z=\operatorname{tr}e^{-\beta H_S}\),
    which is energy-incoherent by construction. For a state \(\rho_S\) that
    is itself energy-incoherent (so \(\rho_S\) and \(\gamma_S\) commute and
    are simultaneously diagonalizable, with eigenvalues \(p_i\) and
    \(g_i=e^{-\beta E_i}/Z\) respectively on a common eigenbasis
    \(\{|i\rangle\}\)) and \(\alpha\in\mathbb R\setminus\{0,1\}\), the
    (classical) Rényi relative entropy is
    \(D_\alpha(\rho_S\|\gamma_S)=\frac{\operatorname{sgn}(\alpha)}{\alpha-1}
    \log\sum_ip_i^\alpha g_i^{1-\alpha}\), where \(\operatorname{sgn}(\alpha)
    =1\) for \(\alpha\geq0\) and \(\operatorname{sgn}(\alpha)=-1\) for
    \(\alpha\lt0\); \(D_1(\rho_S\|\gamma_S)=\sum_ip_i(\log p_i-\log g_i)\)
    is the \(\alpha\to1\) limit and \(D_0,D_{\pm\infty}\) the standard
    limiting values (\(D_0(\rho_S\|\gamma_S)=-\log\sum_{i:p_i\gt0}g_i\)).
    \(D_\alpha\) is monotonically non-decreasing in \(\alpha\) on
    \(\alpha\in[0,\infty]\) (not asserted, and not needed here, across the
    \(\alpha=0\) boundary); this
    formula is unambiguous precisely because \(\rho_S\) and \(\gamma_S\)
    commute, which is the only case in which \(D_\alpha\) is used in this
    entry's claim. The generalized free energy of order \(\alpha\) of an
    energy-incoherent \(\rho_S\) is
    \[
      F_\alpha(\rho_S,H_S):=\beta^{-1}D_\alpha(\rho_S\|\gamma_S)-
      \beta^{-1}\log Z,
    \]
    so that \(F_1(\rho_S,H_S)=\operatorname{tr}[\rho_SH_S]-\beta^{-1}
    S(\rho_S)\) is the ordinary non-equilibrium Helmholtz free energy
    (\(S\) the von Neumann entropy) and \(F_\alpha(\gamma_S,H_S)=
    -\beta^{-1}\log Z\) for every \(\alpha\).

    Free coherence (used only in the Known section below, to state a known
    necessary condition; not needed to state the claim). For a
    state \(\rho_S\) on \(S\), possibly energy-coherent, the free coherence
    of order \(\alpha\geq0\) is
    \(A_\alpha(\rho_S,H_S):=S_\alpha(\rho_S\|D_{H_S}(\rho_S))\), where
    \(S_\alpha\) is the Rényi relative entropy in the convention of
    Lostaglio, Jennings, and Rudolph below (Section II and Methods of that
    paper give the precise definition for every \(\alpha\geq0\), including
    the non-commuting case since \(\rho_S\) need not commute with
    \(D_{H_S}(\rho_S)\)); at \(\alpha=1\) this is the ordinary relative
    entropy \(A_1(\rho_S,H_S)=S(\rho_S\|D_{H_S}(\rho_S))=
    \operatorname{tr}[\rho_S\log\rho_S]-\operatorname{tr}[\rho_S\log
    D_{H_S}(\rho_S)]\), unambiguous regardless of Rényi convention.
    \(A_\alpha(\rho_S,H_S)=0\) for every \(\alpha\) iff \(\rho_S\) is
    energy-incoherent.
  </def>

  <known>
    Datta, Kondra, Miller, and Streltsov, Catalysis of entanglement and
    other quantum resources, Reports on Progress in Physics 86, 116002
    (2023), Eq. (82), define catalytic thermal operations exactly as
    \(\to_{\mathrm{sTO}}\) above: a transformation from \(\rho_S\) to
    \(\mu_S\) is possible via catalytic thermal operations iff there exists
    a thermal operation \(\Lambda\) and a catalyst state \(\sigma_C\) with
    \(\Lambda(\rho_S\otimes\sigma_C)=\operatorname{tr}_B[U_{SCB}(\rho_S
    \otimes\sigma_C\otimes\gamma_B)U_{SCB}^\dagger]=\mu_S\otimes\sigma_C\),
    an exact, uncorrelated return, with no restriction on \(\sigma_C\)
    carrying coherence. Their Theorem 3, attributed to Brandão et al. below,
    states this relation is completely solved by
    \(F_\alpha(\rho,\gamma_\beta)\geq F_\alpha(\rho',\gamma_\beta)\) for
    \(\alpha\geq0\) only when \(\rho,\rho'\) are diagonal in the energy
    eigenbasis, with no accuracy qualifier attached to the transformation.
    As stated, this does not by itself match Brandão et al.'s own results
    recalled below: their Theorem 18 proves the analogous iff only for
    transformations to arbitrary accuracy (not exact) over the full range
    \(\alpha\in(-\infty,\infty)\), and reaching \(\alpha\geq0\) alone
    additionally requires borrowing a qubit ancilla that is itself returned
    only to arbitrary accuracy, not exactly. Theorem 3 should therefore not
    be read as an independent confirmation that \(\alpha\geq0\)
    characterizes the exact \(\to_{\mathrm{sTO}}\) relation fixed by the
    claim above. Their Section VIII ("Perspectives and open problems") states
    explicitly: "In quantum thermodynamics, most results on catalytic state
    transformations are limited to specific families of states, e.g.,
    states which are block diagonal in the energy eigenbasis. A complete
    characterization of catalytic transformations via thermal operations
    between general quantum states has so far remained open." This is the
    direct source of the claim above.

    Lipka-Bartosik, Wilming, and Ng, Catalysis in quantum information
    theory, Reviews of Modern Physics 96, 025005 (2024), Definition III.1,
    name the \(\to_{\mathrm{sTO}}\) relation of this entry "strict
    catalysis". Their Section V.C.3, Eq. (148), report that for
    energy-incoherent \(\rho_S,\sigma_S\) the necessary and sufficient
    condition for a strictly catalytic transition \(\rho_S
    \to_{\mathrm{sTO}}\sigma_S\) is \(F_\alpha^\beta(\rho_S,\hat H_S)\gt
    F_\alpha^\beta(\sigma_S,\hat H_S)\) for \(\alpha\in\mathbb
    R\setminus\{0\}\) -- the pointwise-exact, strict-inequality form used
    in the claim above -- attributing this directly to Brandão et al.
    below ("they identified a family of monotones ... the necessary and
    sufficient conditions ... are then given by" Eq. (148)). Their very
    next sentence, not Eq. (148) itself, is what they attribute to Gour
    and Tomamichel (2021) and Rethinasamy and Wilde (2020): relaxing
    strict catalysis to the weaker, \(\varepsilon\)-approximate-catalyst-
    return notion they call "arbitrarily strict catalysis" (their
    Definition III.3) lets the \(\alpha\lt0\) conditions be dropped, and
    it is this dropping result, not Eq. (148), that the two papers are
    cited as refining. Independent reading of Brandão et al.'s own
    Theorem 18 below shows it proves the analogous iff only for
    transformation to arbitrary accuracy, not exactly, over \(\alpha\in
    (-\infty,\infty)\) -- so Eq. (148)'s exact, strict-inequality
    statement is not independently confirmed by Brandão et al.'s own
    theorem either, for the same reason already noted above for Datta et
    al.'s Theorem 3. Immediately after presenting Eq. (148) they state:
    "The fully-quantum case where \(\rho_S\) or \(\sigma_S\) are
    energy-coherent is still open, even in the case when no catalyst is
    used." Their
    Section III.B further records that correlated catalysis is a strictly
    more permissive resource than strict catalysis in general: e.g. their
    Eq. (52), for any resource theory whose free operations allow permuting
    subsystems, \(\rho_{S_1S_2}\to_{\mathrm{cTO}}\rho_{S_1}\otimes
    \rho_{S_2}\) always holds (take the catalyst to be a copy of \(S_2\)
    and swap it in), a decorrelating transition with no strict-catalytic or
    catalyst-free analogue in general.

    Brandão, Horodecki, Ng, Oppenheim, and Wehner, The second laws of
    quantum thermodynamics, Proceedings of the National Academy of Sciences
    112 (11), 3275-3279 (2015), introduce catalytic thermal operations
    exactly as \(\to_{\mathrm{sTO}}\) above (Section "Catalysis"): "\((\rho,
    H)\) can be catalytically transformed into \((\rho',H')\) if there
    exists catalyst \((\rho_C,H_C)\) such that \((\rho\otimes\rho_C,H+H_C)\)
    can be transformed into \((\rho'\otimes\rho_C,H'+H_C)\)", explicitly
    "demanding that the catalyst be returned uncorrelated from the system",
    with no restriction on \(\rho_C\) carrying coherence in the definition
    itself. Their Theorem 18 proves that for \(\rho,\rho'\) block-diagonal
    in the energy eigenbasis, \(\rho\) can be transformed into \(\rho'\)
    with arbitrary accuracy (i.e. for every \(\epsilon\gt0\) there is an
    exactly-returned, uncorrelated catalyst and a thermal operation on
    system-plus-catalyst whose system-marginal output is within trace
    distance \(\epsilon\) of \(\rho'\)) under catalytic thermal operations
    if and only if \(F_\alpha(\rho,H)\geq F_\alpha(\rho',H)\) for every
    \(\alpha\in(-\infty,\infty)\); their Remark 19 extends this to
    \(\alpha=\pm\infty\) by continuity. Their discussion after Theorem 18
    ("Getting rid of second laws with negative \(\alpha\)") shows that the
    \(\alpha\lt0\) conditions can be dropped only by additionally
    borrowing a separate, fixed-size ancilla -- a qubit with trivial
    Hamiltonian, distinct from the catalyst \(C\) itself, which remains
    exactly returned -- and returning that ancilla only to arbitrary,
    non-size-dependent accuracy (their Theorem 20, restricted to
    \(\alpha\geq0\)). A different, later part of the same paper (Appendix
    G) instead considers the catalyst \(C\) itself returned with a
    genuinely size-dependent error \(\epsilon/\log N\) (\(N\) the
    catalyst's dimension); there the surviving monotone collapses to the
    ordinary free energy \(F_1\) alone, not to any wider \(\alpha\)-family.
    Neither mechanism drops only the negative-\(\alpha\) conditions while
    keeping the catalyst \(C\) itself exactly and unconditionally
    returned, so this entry's strict, zero-error catalysis genuinely
    requires the full \(\alpha\in\mathbb R\setminus\{0\}\) family.

    Lostaglio, Jennings, and Rudolph, Description of quantum coherence in
    thermodynamic processes requires constraints beyond free energy, Nature
    Communications 6, 6383 (2015), prove (Theorem 2, and Theorems 4-5 in
    the Methods) that for catalytic thermal operations whose catalyst is
    itself energy-incoherent (\([\omega_C,H_C]=0\)), the free coherence
    \(A_\alpha\) defined above is necessarily non-increasing:
    \(A_\alpha(\rho_S,H_S)\geq A_\alpha(\sigma_S,H_S)\) for every
    \(\alpha\geq0\) whenever \(\rho_S\to_{\mathrm{sTO}}\sigma_S\) via such a
    catalyst. Their Theorem 2 exhibits a qubit pair \(\rho_S,\sigma_S\) with
    \(F_\alpha(\rho_S,H_S)\geq F_\alpha(\sigma_S,H_S)\) for every
    \(\alpha\geq0\) (so the energy-incoherent second laws impose no
    obstruction) yet \(A_\alpha(\sigma_S,H_S)\gt0=A_\alpha(\rho_S,H_S)\),
    which rules out the transition; this shows the \(A_\alpha\) constraint
    is logically independent of the \(F_\alpha\) family and that no finite
    extension of the energy-incoherent second laws by free-energy-type
    quantities alone can be expected to complete the characterization. This
    paper does not claim, and no source found for this entry claims, that
    \(\{F_\alpha\}\cup\{A_\alpha\}\), or any other specific family, is
    sufficient for \(\to_{\mathrm{sTO}}\) once coherence is present; only
    necessity is established, and only for a catalyst additionally
    restricted to be itself energy-incoherent.

    Elkouss, Maity, Nema, and Strelchuk, A finite sufficient set of
    conditions for catalytic majorization, Communications Physics 9, 164
    (2026) (posted as arXiv:2502.20588 in February 2025), Appendix B, state:
    "At first, it may be noted that no necessary and sufficient set of
    conditions are currently known for catalytic state transformations
    involving states having coherence under the corresponding free
    operations, rather only necessary conditions have been established",
    citing the \(A_\alpha\) result above (their Fact 8-9) as the state of
    the art, and note that even restricted to an energy-incoherent catalyst
    only necessary conditions are known for coherent system states. Their
    own new contribution, a finite sufficient set of inequalities implying
    catalytic thermal-operation transformations, is explicitly restricted
    to energy-incoherent system states throughout; they separately flag "a
    finite set of sufficient conditions for general (mixed) states and
    mixed catalysis" as "an open challenge" even in a related,
    energy-incoherent pure-state setting. As the most recent source located
    for this entry that directly addresses the claim above, this confirms
    the claim remains unresolved through at least February 2025.

    Shiraishi, Quantum Thermodynamics with Coherence: Covariant
    Gibbs-Preserving Operation Is Characterized by the Free Energy, Physical
    Review Letters 134, 160402 (2025), and Shiraishi and Takagi, Recovery of
    the second law in fully quantum thermodynamics, arXiv:2510.05642 (2025),
    completely characterize state convertibility of energy-coherent states
    for, respectively, covariant Gibbs-preserving operations and plain
    thermal operations, in both cases assisted by a correlated catalyst,
    i.e. \(\to_{\mathrm{cTO}}\) above, not \(\to_{\mathrm{sTO}}\):
    Shiraishi and Takagi's own abstract states the result determines
    "whether a quantum state with quantum coherence is convertible to
    another by a thermal operation with a correlated catalyst." Since
    \(\to_{\mathrm{sTO}}\) implies \(\to_{\mathrm{cTO}}\) but not
    conversely (see the Def section above), and since correlated
    catalysis is shown above (Lipka-Bartosik, Wilming, and Ng, Eq. (52)) to
    be strictly more permissive in general, these 2025 results settle a
    different, more permissive problem and leave the claim above, which
    requires the catalyst to return uncorrelated, untouched. This is the
    same distinction recorded in the companion entry 45's Known
    section, and is why the corresponding correlated-catalysis survey lead
    was separately rejected as already solved by these same two papers.
  </known>

  <refs>
    <ref>C. Datta, T. V. Kondra, M. Miller, and A. Streltsov, Catalysis of entanglement and other quantum resources, Reports on Progress in Physics 86, 116002 (2023), DOI: 10.1088/1361-6633/acfbec, arXiv:2207.05694, used for the Eq. (82) definition of catalytic thermal operations (matching the strict, uncorrelated notion of this entry's claim), the Theorem 3 statement of the block-diagonal solution, and Section VIII's explicit statement that a complete characterization of catalytic thermal-operation transformations between general (possibly energy-coherent) quantum states has so far remained open.</ref>
    <ref>P. Lipka-Bartosik, H. Wilming, and N. H. Y. Ng, Catalysis in quantum information theory, Reviews of Modern Physics 96, 025005 (2024), DOI: 10.1103/RevModPhys.96.025005, arXiv:2306.00798, used for Definition III.1 (strict catalysis, matching this entry's claim), Section V.C.3 Eq. (148)'s pointwise-exact necessary-and-sufficient criterion for energy-incoherent states, attributed there directly to Brandão et al. (2015), and for the immediately following sentence's separate attribution to Gour and Tomamichel (2021) and Rethinasamy and Wilde (2020) of a refined argument for dropping the \(\alpha\lt0\) conditions under the weaker "arbitrarily strict catalysis" (Definition III.3), the explicit statement that the energy-coherent case is still open even without a catalyst, and Section III.B Eq. (52) on correlated catalysis strictly enlarging the reachable set relative to strict catalysis.</ref>
    <ref>F. G. S. L. Brandão, M. Horodecki, N. Ng, J. Oppenheim, and S. Wehner, The second laws of quantum thermodynamics, Proceedings of the National Academy of Sciences 112 (11), 3275-3279 (2015), DOI: 10.1073/pnas.1411728112, arXiv:1305.5278, used for the original definition of catalytic thermal operations with an exactly-returned, uncorrelated catalyst, and for Theorem 18 (with Remark 19) proving the generalized free energies \(F_\alpha\), \(\alpha\in(-\infty,\infty)\), necessary and sufficient for converting energy-incoherent states to within arbitrary accuracy under such catalysis, together with the discussion of why negative \(\alpha\) cannot be dropped in the zero-error case.</ref>
    <ref>M. Lostaglio, D. Jennings, and T. Rudolph, Description of quantum coherence in thermodynamic processes requires constraints beyond free energy, Nature Communications 6, 6383 (2015), DOI: 10.1038/ncomms7383, arXiv:1405.2188, used for Theorem 2 and Theorems 4-5 (Methods), proving that the free coherence \(A_\alpha\) is a necessary, non-increasing quantity under strict catalytic thermal operations with an energy-incoherent catalyst, and that this constraint is logically independent of the \(F_\alpha\) family.</ref>
    <ref>D. Elkouss, A. G. Maity, A. Nema, and S. Strelchuk, A finite sufficient set of conditions for catalytic majorization, Communications Physics 9, 164 (2026), DOI: 10.1038/s42005-026-02583-x, arXiv:2502.20588, used for Appendix B's explicit (February 2025) restatement that no necessary-and-sufficient conditions are known for catalytic thermal-operation transformations of energy-coherent states, and for confirming that their own new finite-sufficient-condition results remain restricted to energy-incoherent states.</ref>
    <ref>N. Shiraishi, Quantum Thermodynamics with Coherence: Covariant Gibbs-Preserving Operation Is Characterized by the Free Energy, Physical Review Letters 134, 160402 (2025), DOI: 10.1103/PhysRevLett.134.160402, arXiv:2406.06234, used for the correlated-catalytic covariant Gibbs-preserving solution, a strictly more permissive setting than the strict, uncorrelated catalysis fixed by this entry's claim.</ref>
    <ref>N. Shiraishi and R. Takagi, Recovery of the second law in fully quantum thermodynamics, arXiv:2510.05642 (2025), used for the correlated-catalytic plain-thermal-operation solution and its own abstract's explicit restriction to a correlated catalyst rather than the strict, uncorrelated catalyst of this entry's claim.</ref>
    <ref>G. Gour and M. Tomamichel, Entropy and relative entropy from information-theoretic principles, IEEE Transactions on Information Theory 67 (10), 6313-6327 (2021), DOI: 10.1109/TIT.2021.3078337, arXiv:2006.11164, Theorem 20 (catalytic relative majorization, a limit-of-sequences pre-order distinct from exact trumping, Definition 10) and Theorem 21, and Definitions 4-5 (Rényi relative entropy defined only for \(\alpha\geq0\)); cited by Lipka-Bartosik, Wilming, and Ng (2024), Section V.C.3, not for Eq. (148) itself but for the following sentence on dropping \(\alpha\lt0\) under arbitrarily strict catalysis. Independent reading of this paper confirms its Theorem 20/21 give only a non-strict inequality for \(\alpha\geq1/2\), with no negative-\(\alpha\) case treated at all.</ref>
    <ref>S. Rethinasamy and M. M. Wilde, Relative entropy and catalytic relative majorization, Physical Review Research 2, 033455 (2020), DOI: 10.1103/PhysRevResearch.2.033455, arXiv:1912.04254, Theorem 1 (One-Shot Characterization of Exact Pair Transformations) and Table I; cited by Lipka-Bartosik, Wilming, and Ng (2024), Section V.C.3, alongside Gour and Tomamichel (2021) above for the same dropping-\(\alpha\lt0\) sentence, not for Eq. (148) itself. Independent reading of this paper confirms its Theorem 1 characterizes only the ordinary relative entropy \(D\) and the min-relative entropy \(D_0\), not a family over general \(\alpha\) (Table I attributes the \(\alpha\)-family condition to Brandão et al., their ref. [18]), and its equivalent condition 2 allows the catalyst to retain a residual correlation \(D(t'\|p'\otimes r)\lt\gamma\) for arbitrary \(\gamma\gt0\), not exactly zero.</ref>
  </refs>

  <ask>
    Give the relation \(\mathcal C\) described in the claim above and prove
    it is both necessary and sufficient for \(\rho_S\to_{\mathrm{sTO}}
    \sigma_S\) for every finite-dimensional \(S\), every Hamiltonian
    \(H_S\) on \(S\), every \(\beta\in(0,\infty)\), and every pair of
    states \(\rho_S,\sigma_S\) on \(S\), including pairs where
    \([\rho_S,H_S]\neq0\) or \([\sigma_S,H_S]\neq0\). The proof must
    establish the reduction, whenever \([\rho_S,H_S]=[\sigma_S,H_S]=0\), to
    the pointwise-exact energy-incoherent criterion recalled above
    (there exists a unitary \(U\) on \(S\) with \([U,H_S]=0\) and
    \(\sigma_S=U\rho_SU^\dagger\), or \(F_\alpha(\rho_S,H_S)\gt
    F_\alpha(\sigma_S,H_S)\) for every \(\alpha\in\mathbb
    R\setminus\{0\}\)).

    A disproof must show that no relation \(\mathcal C\) meeting the
    requirement stated in the claim above can be both necessary and
    sufficient for \(\to_{\mathrm{sTO}}\) on the full domain fixed above.
    For example, a proof that deciding \(\rho_S\to_{\mathrm{sTO}}\sigma_S\)
    is undecidable in \(\dim S\) in a way that rules out any
    characterization of the sought form, or a proof that the preorder
    \(\to_{\mathrm{sTO}}\) restricted to energy-coherent states admits no
    finite or countable generating set of monotones for some finite
    \(\dim S\), would each constitute a disproof. A single additional
    numerical counterexample to one specific proposed criterion, without
    such an impossibility argument, disproves only that criterion, not the
    claim.

    The following count as progress, not as a solution to the claim,
    unless combined with a proof removing the stated restriction: (i)
    necessary conditions on \((\rho_S,\sigma_S,H_S,\beta)\) not also proved
    sufficient for some \(\dim S\geq2\) with an energy-coherent pair,
    including further extensions of the \(F_\alpha\) or \(A_\alpha\)
    families or other new coherence monotones; (ii) sufficient conditions
    not also proved necessary; (iii) results restricted to a proper
    subclass of states, Hamiltonians, or catalysts, such as a catalyst
    required to be energy-incoherent (as in the necessary conditions
    recalled above), a single nonzero Bohr frequency, a bounded catalyst
    dimension, or specific low-dimensional Hamiltonians; (iv) results for a
    relation that strictly contains \(\to_{\mathrm{sTO}}\), such as
    \(\to_{\mathrm{cTO}}\) (correlated catalysis, defined above), arbitrarily
    strict or approximate catalysis (a nonzero error on the catalyst, the
    system output, or both), or thermal operations enlarged to
    Gibbs-preserving or generalized-thermal-process free operations, unless
    accompanied by a proof that the enlargement is never needed for the
    transitions in question; (v) asymptotic, many-copy, or approximate
    (nonzero-error) conversion criteria, unless accompanied by a proof that
    they coincide with the exact single-shot \(\to_{\mathrm{sTO}}\) answer
    above on the relevant domain.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
