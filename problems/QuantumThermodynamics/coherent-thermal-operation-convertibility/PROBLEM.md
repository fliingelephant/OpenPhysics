# QTD001 Coherent State Convertibility under Thermal Operations

```xml
<problem id="QTD001">
  <status>open</status>
  <name>Coherent State Convertibility under Thermal Operations</name>
  <keys>thermal operations coherent thermal operations quantum coherence energy coherence state convertibility state conversion state interconversion thermomajorization thermo-majorization second laws of thermodynamics many second laws single-shot quantum thermodynamics one-shot quantum thermodynamics resource theory of quantum thermodynamics resource theory of athermality damping matrix positivity dmp condition qubit thermal operations energy-incoherent states block-diagonal states bohr frequency mode operator time-translation covariance time-translation symmetry asymmetry monotone free energy gibbs state coherent state conversion problem founding question of single-shot quantum thermodynamics</keys>

  <claim>
    Fix a finite-dimensional quantum system \(S\) with Hamiltonian
    \(H_S=\sum_a E_a\Pi_a\) (spectral decomposition, with the \(E_a\)
    distinct real numbers and the \(\Pi_a\) orthogonal projectors on \(S\)
    summing to the identity) and an inverse temperature
    \(\beta\in(0,\infty)\). Let
    \(\rho_S,\sigma_S\) be arbitrary density operators on \(S\), not assumed
    to commute with \(H_S\).

    For every finite \(\dim S\), every Hamiltonian \(H_S\) on \(S\), every
    \(\beta\in(0,\infty)\), and every pair \(\rho_S,\sigma_S\) of density
    operators on \(S\) -- including pairs with \([\rho_S,H_S]\neq0\) or
    \([\sigma_S,H_S]\neq0\) -- give a relation
    \(\mathcal C(\rho_S,\sigma_S,H_S,\beta)\), stated purely in terms of
    \(\rho_S\), \(\sigma_S\), \(H_S\), and \(\beta\) and not by reference to
    the existence of an unspecified bath, joint unitary, or auxiliary
    system, that is both necessary and sufficient for the existence of a
    thermal operation \(\Phi_S\) (defined below) with
    \(\Phi_S(\rho_S)=\sigma_S\). A relation \(\mathcal C\) satisfies this
    requirement only if it can be evaluated without an existential or
    infimum quantifier ranging over a bath, joint unitary, or auxiliary
    system of dimension unbounded in \(\dim S\); in particular
    \(\mathcal C:=[\exists\,\Phi_S\text{ with }\Phi_S(\rho_S)=\sigma_S]\)
    itself does not satisfy it.

    A correct \(\mathcal C\) must reduce to the thermo-majorization order
    defined below whenever \([\rho_S,H_S]=[\sigma_S,H_S]=0\), and must
    reduce, whenever \(\dim S=2\), to a criterion logically equivalent to
    the damping-matrix-positivity criterion proved necessary and sufficient
    for qubits by Ćwikliński, Studziński, Horodecki, and Oppenheim, Physical
    Review Letters 115, 210403 (2015). No relation \(\mathcal C\) meeting
    both requirements for every finite \(\dim S\) is currently proved to be
    both necessary and sufficient.
  </claim>

  <def>
    A thermal operation on \(S\) with Hamiltonian \(H_S\) at inverse
    temperature \(\beta\) is a CPTP map
    \[
      \Phi_S(\rho)=
      \operatorname{tr}_B\!\left[
        U_{SB}(\rho\otimes\gamma_B)U_{SB}^\dagger
      \right],
      \qquad
      [U_{SB},\,H_S\otimes I_B+I_S\otimes H_B]=0,
    \]
    where \(B\) is a finite-dimensional bath with Hamiltonian \(H_B\) and
    Gibbs state \(\gamma_B=e^{-\beta H_B}/\operatorname{tr}e^{-\beta H_B}\);
    \(B\), \(H_B\), and \(U_{SB}\) are otherwise unconstrained and may be
    chosen freely for each \(\Phi_S\). Write \(\rho_S\to_{\mathrm{TO}}\sigma_S\)
    for "there exists a thermal operation \(\Phi_S\) with
    \(\Phi_S(\rho_S)=\sigma_S\)". No catalyst, work-storage system, or clock
    is attached: the only systems in play are \(S\) and \(B\), and \(B\)
    does not need to return to \(\gamma_B\) or to any fixed state.

    For \(\omega\in\mathbb R\), the mode, or Bohr-frequency, component at
    frequency \(\omega\) of an operator \(X\) on \(S\) is
    \(X^{(\omega)}=\sum_{a,b:\,E_a-E_b=\omega}\Pi_aX\Pi_b\), so that
    \(X=\sum_\omega X^{(\omega)}\) and
    \(e^{-iH_St}Xe^{iH_St}=\sum_\omega e^{-i\omega t}X^{(\omega)}\). A state
    \(\rho_S\) is energy-incoherent, or carries no coherence relative to
    \(H_S\), iff \(\rho_S=\rho_S^{(0)}\), equivalently \([\rho_S,H_S]=0\),
    equivalently \(\Pi_a\rho_S\Pi_b=0\) for every \(a\neq b\). Otherwise
    \(\rho_S\) carries coherence between the eigenspaces of every pair
    \(a\neq b\) with \(\Pi_a\rho_S\Pi_b\neq0\), i.e. between eigenspaces
    separated by every Bohr frequency \(\omega\neq0\) with
    \(\rho_S^{(\omega)}\neq0\). Every thermal operation is
    time-translation covariant,
    \(\Phi_S(e^{-iH_St}\rho e^{iH_St})=e^{-iH_St}\Phi_S(\rho)e^{iH_St}\)
    for all real \(t\), so a thermal operation cannot map an
    energy-incoherent \(\rho_S\) to a \(\sigma_S\) with
    \(\sigma_S\neq\sigma_S^{(0)}\).

    Thermo-majorization. Suppose \([\rho_S,H_S]=0\) and let \(d=\dim S\).
    Because \(\rho_S\) is block-diagonal on the eigenspaces of \(H_S\) and
    Hermitian on each block, there is an eigenbasis \(\{|i\rangle\}_{i=1}^d\)
    of \(H_S\) (one vector per eigenvalue, counted with multiplicity) that
    also diagonalizes \(\rho_S\) within each eigenspace; let
    \(p_i=\langle i|\rho_S|i\rangle\), let \(E_i\) be the eigenvalue of
    \(|i\rangle\), and let \(g_i=e^{-\beta E_i}/Z\),
    \(Z=\operatorname{tr}e^{-\beta H_S}\). Order the indices so that
    \(p_{i_1}/g_{i_1}\ge p_{i_2}/g_{i_2}\ge\cdots\ge p_{i_d}/g_{i_d}\), and
    define the thermo-majorization curve of \((\rho_S,H_S)\) as the
    piecewise-linear, concave function through the \(d+1\) points
    \(\bigl(\sum_{k\le j}g_{i_k},\ \sum_{k\le j}p_{i_k}\bigr)\) for
    \(j=0,1,\dots,d\) (the \(j=0\) point is the origin). For two
    energy-incoherent pairs \((\rho_S,H_S)\), \((\sigma_S,H_S)\), say
    \((\rho_S,H_S)\) thermo-majorizes \((\sigma_S,H_S)\) when the
    thermo-majorization curve of \((\rho_S,H_S)\) lies nowhere below that
    of \((\sigma_S,H_S)\).

    Qubit specialization. If \(\dim S=2\), \(H_S=E_0\Pi_0+E_1\Pi_1\) with
    \(E_1>E_0\) and eigenvectors \(|0\rangle,|1\rangle\), every state on
    \(S\) is determined by its matrix elements
    \(p=\langle1|\rho_S|1\rangle\in[0,1]\), the excited-state population,
    and \(c=\langle1|\rho_S|0\rangle\in\mathbb C\) with \(|c|^2\le p(1-p)\),
    the unique coherence between the two eigenspaces, at the unique nonzero
    Bohr frequency \(\omega=E_1-E_0\); explicitly
    \(\rho_S=p\,\Pi_1+(1-p)\,\Pi_0+c\,|1\rangle\langle0|+\bar c\,|0\rangle\langle1|\).
  </def>

  <known>
    Horodecki and Oppenheim, Fundamental limitations for quantum and
    nanoscale thermodynamics, Nature Communications 4, 2059 (2013), Theorem
    2 (Supplementary Note 2), prove that for state-Hamiltonian pairs
    \((\rho_S,H_S)\), \((\sigma_S,H_S)\) with
    \([\rho_S,H_S]=[\sigma_S,H_S]=0\), \(\rho_S\to_{\mathrm{TO}}\sigma_S\)
    holds if and only if \((\rho_S,H_S)\) thermo-majorizes
    \((\sigma_S,H_S)\); this is the "solved for states without coherence,
    2013" half of the survey lead. Their main text extends this
    immediately after Theorem 2 to every \(\rho_S\), coherent or not,
    whenever \(\sigma_S\) alone is energy-incoherent: \(\rho_S\to_{\mathrm{TO}}\sigma_S\)
    holds if and only if \((\rho_S^{(0)},H_S)\) thermo-majorizes
    \((\sigma_S,H_S)\), because dephasing in the energy eigenbasis commutes
    with thermal operations, so this "coherent input, incoherent output"
    region is solved by the same theorem. Their main text also remarks that
    once the final state is not diagonal in the energy eigenbasis, the
    criterion for which transitions are possible depends on more than this
    population data, on the details of the system-bath coupling, flagging
    the energy-coherent-output case as unresolved by their own theorem.

    Ćwikliński, Studziński, Horodecki, and Oppenheim, Limitations on the
    Evolution of Quantum Coherences: Towards Fully Quantum Second Laws of
    Thermodynamics, Physical Review Letters 115, 210403 (2015), derive, for
    every finite \(\dim S\) with non-degenerate Bohr spectrum (no two
    distinct pairs of \(H_S\)-eigenvalues yield the same nonzero gap), a
    family of necessary conditions on \((\rho_S,\sigma_S,H_S,\beta)\) --
    damping-matrix positivity -- bounding how much coherence between two
    energy eigenspaces can survive a thermal operation in terms of the
    induced population-transition probabilities between those eigenspaces;
    their main text notes that Lostaglio, Korzekwa, Jennings, and Rudolph
    (below) subsequently generalize this family of conditions to
    Hamiltonians with arbitrary, possibly degenerate, Bohr spectra. Their
    Supplementary Note 4 proves that
    damping-matrix positivity is also sufficient when \(\dim S=2\), giving a
    complete necessary-and-sufficient solution for qubit thermal operations
    with coherence; this is the "solved for qubits" half of the survey
    lead. Their Supplementary Note 6 (Corollary 11) proves that no thermal
    operation can realize one explicit three-level transition exactly while
    also saturating its damping-matrix-positivity bound; their own main
    text calls this "not a conclusive result" toward damping-matrix-positivity
    insufficiency beyond qubits, since a thermal operation might still
    approximate that transition arbitrarily closely, and leaves the
    robust, perturbed version of the example open as their Conjecture 13.
    Ding, Ding, and Hu, Exploring the Gap between Thermal Operations and
    Enhanced Thermal Operations, Physical Review A 103, 052214 (2021),
    resolve the open question behind Conjecture 13, proving analytically,
    for a three-level transition of their own construction and a
    restricted low-temperature regime (\(0\ll e^{-\beta\omega}\ll
    (\sqrt{5}-1)/2\) for that construction's elementary Bohr gap \(\omega\)),
    that no thermal operation can approximately realize it either; since
    Ćwikliński et al.'s Enhanced Thermal Operations realize exactly the
    transitions satisfying damping-matrix positivity, this single-regime
    result already robustly establishes that damping-matrix positivity
    alone is not sufficient once \(\dim S\ge3\). The correct
    necessary-and-sufficient
    strengthening of damping-matrix positivity for general \(\dim S\)
    remains open.

    Lostaglio, Korzekwa, Jennings, and Rudolph, Quantum Coherence,
    Time-Translation Symmetry, and Thermodynamics, Physical Review X 5,
    021001 (2015), independently derive general upper and lower bounds on
    the evolution of the mode operators \(\rho_S^{(\omega)}\) under thermal
    operations at any temperature, from the time-translation covariance
    recalled above. As an application they specialize these bounds to
    \(\dim S=2\) and, citing Ćwikliński et al., record that the bound is
    achieved, reproducing the complete qubit solution from an independent
    formalism.

    Lostaglio, Jennings, and Rudolph, Description of quantum coherence in
    thermodynamic processes requires constraints beyond free energy, Nature
    Communications 6, 6383 (2015), prove that the coherence constraints on
    thermal-operation transitions are not implied by any finite family of
    generalized free energies (Rényi divergences from the Gibbs state) of
    the kind that is complete for the energy-incoherent case; this gives a
    structural reason a routine finite extension of thermo-majorization
    cannot be expected to settle the claim.

    Ng and Woods, Resource Theory of Quantum Thermodynamics: Thermal
    Operations and Second Laws, in Thermodynamics in the Quantum Regime:
    Fundamental Aspects and New Directions, F. Binder, L. A. Correa, C.
    Gogolin, J. Anders, and G. Adesso, eds. (Springer, Cham, 2018), pp.
    625-650, Section II.2, after proving the energy-incoherent
    thermo-majorization theorem, state explicitly that it remains an open
    question what the necessary and sufficient conditions are for
    thermal-operation transitions between arbitrary energy-coherent states,
    and note that dephasing a coherent state to its \(\rho_S^{(0)}\) part
    and applying thermo-majorization to that part gives only a necessary
    condition on the coherent transition, not a sufficient one. This is one
    of the two source reviews cited by the survey lead for this entry.

    Lostaglio, An introductory review of the resource theory approach to
    thermodynamics, Reports on Progress in Physics 82, 114001 (2019),
    Section III.2 ("Thermodynamic constraints on the evolution of quantum
    coherence"), assembles the damping-matrix-type necessary conditions
    (Section III.2.1, "General theorem connecting population and coherence
    constraints") and the qubit case (Section III.2.2, "Application: qubit
    Thermal Operations"); its abstract states that the review works out the
    complete solution for a single qubit specifically, consistent with the
    general \(\dim S\ge3\) case being left open. This is the second of the
    two source reviews cited by the survey lead for this entry.

    Gour, Jennings, Buscemi, Duan, and Marvian, Quantum majorization and a
    complete set of entropic conditions for quantum thermodynamics, Nature
    Communications 9, 5352 (2018), prove a complete set of necessary and
    sufficient entropic conditions for state conversion under what they
    define as generalized thermal processes, a class of free operations
    that they define, from physical assumptions strictly weaker than the
    definition of thermal operation above, to extend thermal operations:
    every thermal operation is a generalized thermal process, and their
    introduction states, without proof at that point, that this
    containment is proper. Their Lemma 1 proves that the set of processes
    satisfying their three defining assumptions coincides with the set of
    Gibbs-preserving processes covariant under the group generated by the
    conserved thermodynamic observables; taking \(H_S\) as the only such
    observable -- the setting of this entry's claim -- this is exactly the
    set of Gibbs-preserving, time-translation covariant maps, i.e. exactly
    Ćwikliński et al.'s Enhanced Thermal Operations above. Lostaglio's
    review above (Remark 4) makes the same identification, naming this
    class Thermal Processes and citing this Gour et al. paper, alongside
    Ćwikliński et al., as its two defining sources. Whether thermal
    operations and this class nonetheless induce the same set of achievable
    state conversions is exactly Lostaglio's Remark 16 conjecture in that
    review; Ding, Ding, and Hu's construction above refutes it (within the
    temperature regime given there): the state pair it exhibits is
    achievable by Enhanced Thermal Operations but lies outside the closure
    of what thermal operations can achieve, so the two classes do not, in
    general, induce the same conversion relation. Gour et al.'s theorem
    therefore answers the analogous question for a free-operation set that
    is at least as large as the one fixed in this entry's claim; restricted
    to \(H_S\) alone, that set is now known to be a strict enlargement of
    thermal operations at the level of achievable conversions, not just of
    the operations themselves, so their theorem does not by itself supply a
    relation \(\mathcal C\) that is sufficient for \(\to_{\mathrm{TO}}\).
    This is consistent with Ng-Woods and with Lostaglio's review above,
    both of which continue to record this entry's own
    thermal-operation-specific claim -- the necessary-and-sufficient
    relation for plain \(\to_{\mathrm{TO}}\), not the containment question
    just resolved -- as open.

    Shiraishi, Quantum Thermodynamics with Coherence: Covariant
    Gibbs-Preserving Operation Is Characterized by the Free Energy,
    Physical Review Letters 134, 160402 (2025), proves a complete
    characterization of state convertibility for covariant Gibbs-preserving
    operations assisted by a correlated catalyst, and Shiraishi and Takagi,
    Recovery of the second law in fully quantum thermodynamics, arXiv:
    2510.05642 (2025), prove the analogous complete characterization for
    plain thermal operations assisted by a correlated catalyst. In both
    papers the catalyst is a system that need only return marginally
    unchanged, possibly correlated with \(S\) at the end of the protocol.
    A correlated catalyst strictly enlarges what is reachable relative to
    the catalyst-free thermal operation of this entry's claim (and
    covariant Gibbs-preserving operations enlarge it further still), so
    these 2025 results settle a different, catalyst-assisted problem and
    leave this claim, which fixes plain thermal operations with no
    catalyst, untouched.

    Hsieh and Gessner, Complete characterisation of state conversions by
    work extraction, arXiv:2510.20366 (2025), prove a general
    majorization-type necessary-and-sufficient condition for state
    conversion in an abstract resource-theoretic work-extraction framework,
    and instantiate it for unital channels and for LOCC in entanglement
    theory (recovering Nielsen's theorem), but state in their own
    discussion that recovering thermo-majorization, and by extension its
    coherent generalization, within their framework is a conjecture left
    for future work; this is evidence the claim was still unresolved as of
    that date, less than a year before this entry's survey date.

    Lie, Son, Boes, Ng, and Wilming, Thermal operations from informational
    equilibrium, Physical Review Letters 137, 030403 (2026), characterize
    which quantum channels count as thermal operations by an
    information-theoretic dilation property, rather than giving conditions
    for converting a fixed pair of states; this is a characterization of
    the free-operation set itself, a different question from the claim of
    this entry, which fixes the thermal-operation set and asks for the
    state-conversion relation it induces.
  </known>

  <refs>
    <ref>M. Horodecki and J. Oppenheim, Fundamental limitations for quantum and nanoscale thermodynamics, Nature Communications 4, 2059 (2013), DOI: 10.1038/ncomms3059, arXiv:1111.3834, used for Theorem 2 (Supplementary Note 2), the thermo-majorization criterion solving the energy-incoherent case, the main-text extension solving every energy-coherent-input, energy-incoherent-output case by a dephasing argument, and the main-text remark that non-diagonal final states require more than this criterion.</ref>
    <ref>P. Ćwikliński, M. Studziński, M. Horodecki, and J. Oppenheim, Limitations on the Evolution of Quantum Coherences: Towards Fully Quantum Second Laws of Thermodynamics, Physical Review Letters 115, 210403 (2015), DOI: 10.1103/PhysRevLett.115.210403, arXiv:1405.5029, used for the damping-matrix-positivity necessary conditions for non-degenerate Bohr spectra, the definition of Enhanced Thermal Operations, the proof of sufficiency for qubits in Supplementary Note 4, and the qutrit boundary-case impossibility (Corollary 11) and its open robust version (Conjecture 13) in Supplementary Note 6.</ref>
    <ref>Y. Ding, F. Ding, and X. Hu, Exploring the Gap between Thermal Operations and Enhanced Thermal Operations, Physical Review A 103, 052214 (2021), DOI: 10.1103/PhysRevA.103.052214, arXiv:2101.05690, used for the analytic proof, for a three-level transition of their own construction, that no thermal operation can approximately realize a transition achievable by Enhanced Thermal Operations, resolving the open question behind Ćwikliński et al.'s Conjecture 13 and establishing damping-matrix-positivity insufficiency beyond qubits.</ref>
    <ref>M. Lostaglio, K. Korzekwa, D. Jennings, and T. Rudolph, Quantum Coherence, Time-Translation Symmetry, and Thermodynamics, Physical Review X 5, 021001 (2015), DOI: 10.1103/PhysRevX.5.021001, arXiv:1410.4572, used for the mode-operator formalism, the general coherence-evolution bounds under thermal operations, and the independently derived qubit-case bound with its cited saturation.</ref>
    <ref>M. Lostaglio, D. Jennings, and T. Rudolph, Description of quantum coherence in thermodynamic processes requires constraints beyond free energy, Nature Communications 6, 6383 (2015), DOI: 10.1038/ncomms7383, arXiv:1405.2188, used for the proof that finitely many generalized free energies cannot capture the coherence constraints on thermal-operation transitions.</ref>
    <ref>N. H. Y. Ng and M. P. Woods, Resource Theory of Quantum Thermodynamics: Thermal Operations and Second Laws, in Thermodynamics in the Quantum Regime: Fundamental Aspects and New Directions, F. Binder, L. A. Correa, C. Gogolin, J. Anders, and G. Adesso, eds. (Springer, Cham, 2018), pp. 625-650, DOI: 10.1007/978-3-319-99046-0_26, arXiv:1805.09564, used for Section II.2's statement that the necessary and sufficient conditions for energy-coherent thermal-operation transitions are an open question, and for the remark that dephased thermo-majorization is only necessary in the coherent case.</ref>
    <ref>M. Lostaglio, An introductory review of the resource theory approach to thermodynamics, Reports on Progress in Physics 82, 114001 (2019), DOI: 10.1088/1361-6633/ab46e5, arXiv:1807.11549, used for Section III.2's assembly of the general coherence constraints (III.2.1) and the qubit solution (III.2.2, "Application: qubit Thermal Operations"), for the abstract's framing of the single-qubit case as the complete solution, for Remark 4's identification of Thermal Processes/Enhanced Thermal Operations with the generalized thermal processes of Gour et al. below, and for Remark 16's open conjecture on whether Thermal Operations and Thermal Processes coincide.</ref>
    <ref>G. Gour, D. Jennings, F. Buscemi, R. Duan, and I. Marvian, Quantum majorization and a complete set of entropic conditions for quantum thermodynamics, Nature Communications 9, 5352 (2018), DOI: 10.1038/s41467-018-06261-7, arXiv:1708.04302, used for the complete entropic solution of the generalized-thermal-process free-operation class, its explicit definition as a physically weaker-assumption extension of thermal operations, its introduction's own unproved statement that this extension is a proper containment, and Lemma 1's identification of generalized thermal processes, when the Hamiltonian is the only conserved observable, with Gibbs-preserving, time-translation covariant maps.</ref>
    <ref>N. Shiraishi, Quantum Thermodynamics with Coherence: Covariant Gibbs-Preserving Operation Is Characterized by the Free Energy, Physical Review Letters 134, 160402 (2025), DOI: 10.1103/PhysRevLett.134.160402, arXiv:2406.06234, used for the correlated-catalytic covariant Gibbs-preserving solution, a free-operation setting strictly larger than plain thermal operations.</ref>
    <ref>N. Shiraishi and R. Takagi, Recovery of the second law in fully quantum thermodynamics, arXiv:2510.05642 (2025), used for the correlated-catalytic thermal-operation solution and its explicit restriction to the catalyst-assisted setting, as distinct from the catalyst-free claim of this entry.</ref>
    <ref>C.-Y. Hsieh and M. Gessner, Complete characterisation of state conversions by work extraction, arXiv:2510.20366 (2025), used for the general work-extraction majorization framework and its authors' own statement that recovering (coherent) thermo-majorization within it is left as a conjecture for future work.</ref>
    <ref>S. H. Lie, J. Son, P. Boes, N. H. Y. Ng, and H. Wilming, Thermal operations from informational equilibrium, Physical Review Letters 137, 030403 (2026), arXiv:2507.16637, used for the information-theoretic characterization of the thermal-operation channel set itself, distinguished from the state-conversion question of this entry's claim.</ref>
  </refs>

  <ask>
    Give the relation \(\mathcal C\) described in the claim above and prove it is
    both necessary and sufficient for \(\rho_S\to_{\mathrm{TO}}\sigma_S\)
    for every finite-dimensional \(S\), every Hamiltonian \(H_S\) on \(S\),
    every \(\beta\in(0,\infty)\), and every pair of states \(\rho_S,\sigma_S\)
    on \(S\), including pairs where \([\rho_S,H_S]\neq0\) or
    \([\sigma_S,H_S]\neq0\). The proof must establish the reduction to
    thermo-majorization in the energy-incoherent case and to a criterion
    equivalent to damping-matrix positivity in the qubit case, exactly as
    required above.

    A disproof must show that no relation \(\mathcal C\) meeting the
    requirement stated in the claim above can be both necessary and
    sufficient for \(\to_{\mathrm{TO}}\) on the full domain fixed above.
    For example, a proof that deciding \(\rho_S\to_{\mathrm{TO}}\sigma_S\)
    is undecidable in \(\dim S\) in a way that rules out any
    characterization of the form sought, or a proof that the preorder
    \(\to_{\mathrm{TO}}\) restricted to energy-coherent states admits no
    finite generating set of monotones for some finite \(\dim S\), would
    each constitute a disproof. A single additional numerical counterexample
    to one specific proposed criterion, without such an impossibility
    argument, disproves only that criterion, not the claim.

    The following count as progress, not as a solution to the claim, unless
    combined with a proof removing the stated restriction: (i) necessary
    conditions on \((\rho_S,\sigma_S,H_S,\beta)\) not also proved sufficient
    for some \(\dim S\ge3\), including further damping-matrix-type or
    Rényi-divergence-type families; (ii) sufficient conditions not also
    proved necessary; (iii) results restricted to a proper subclass of
    states or Hamiltonians, such as a single nonzero Bohr frequency,
    two-level coherence blocks inside a larger system, or specific
    low-dimensional Hamiltonians; (iv) results for a free-operation set that
    strictly contains thermal operations, such as generalized thermal
    processes, Gibbs-preserving maps, enhanced thermal operations, or
    thermal operations assisted by any catalyst (correlated or
    uncorrelated), unless accompanied by a proof that the extra resource is
    never needed for the transitions in question; (v) asymptotic,
    many-copy, or approximate (nonzero-error) conversion criteria, unless
    accompanied by a proof that they coincide with the exact single-shot
    answer above on the relevant domain.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
