# 25 Reversible dynamics on composite systems

```xml
<problem id="25">
  <status>open</status>
  <kind>proof</kind>
  <field>QIT</field>
  <name>Reversible dynamics on composite systems</name>
  <keys>oqp problem 42 reversible dynamics composite systems generalized probabilistic theories gpt locally tomographic composite local tomography tomographic locality transitive pure states homogeneous pure-state orbit reversible transformations correlation-creating reversible transformation product-state-correlating reversible dynamics non-product reversible transformation quantum embeddable quantum-realizable subtheory boxworld bloch ball gbit euclidean jordan algebra stabilizer states clifford group quantum foundations</keys>

  <claim>
    Let \(A\) and \(B\) be finite-dimensional compact-convex GPT systems whose
    chosen reversible transformation groups \(G_A\) and \(G_B\) act transitively
    on their respective pure normalized states. Let \(AB\) be a locally
    tomographic composite of \(A\) and \(B\), containing the product states,
    product effects, and product reversible transformations defined below. If
    the reversible transformation group \(G_{AB}\) contains a
    correlation-creating reversible transformation, then the state-space and
    reversible-dynamics data of \(A\), \(B\), and \(AB\) are quantum-embeddable
    in the sense defined below.
  </claim>

  <def>
    A finite-dimensional compact-convex GPT system \(X\) consists of a real
    vector space \(V_X\), a nonzero unit effect \(u_X\in V_X^*\), a compact
    convex set of normalized states
    \[
      \Omega_X\subset \{\omega\in V_X: u_X(\omega)=1\},
    \]
    a closed convex set of allowed effects
    \[
      \mathcal E_X\subset \{e\in V_X^*:0\le e(\omega)\le 1
      \ \mathrm{for\ all}\ \omega\in\Omega_X\},
    \]
    with \(0,u_X\in\mathcal E_X\), and a compact group \(G_X\) of reversible
    transformations. The representation is operationally reduced: the
    normalized states linearly span \(V_X\), and
    \(\operatorname{span}\mathcal E_X=V_X^*\), so allowed effects separate
    states. A reversible transformation is an invertible linear map
    \(T:V_X\to V_X\) such that \(u_X\circ T=u_X\),
    \(T\Omega_X=\Omega_X\), \(T^{-1}\Omega_X=\Omega_X\), and the chosen
    operational theory regards both \(T\) and \(T^{-1}\) as allowed. The
    allowed effects are covariant under the chosen reversible transformations:
    \(e\circ T\in\mathcal E_X\) for all \(e\in\mathcal E_X\) and \(T\in G_X\).
    A pure state is an extreme point of \(\Omega_X\). Transitivity on pure
    states means that for every pair of pure states
    \(\alpha,\beta\in\Omega_X\) there is \(T\in G_X\) with \(T\alpha=\beta\).

    A locally tomographic composite \(AB\) is represented on
    \(V_{AB}=V_A\otimes V_B\), with unit \(u_{AB}=u_A\otimes u_B\), normalized
    state space \(\Omega_{AB}\subset\{\omega:u_{AB}(\omega)=1\}\), allowed
    effect set \(\mathcal E_{AB}\), and reversible group \(G_{AB}\). Local
    tomography means that product effects separate joint states: if
    \[
      (e_A\otimes e_B)(\omega)=(e_A\otimes e_B)(\varphi)
    \]
    for all \(e_A\in\operatorname{span}\mathcal E_A\) and
    \(e_B\in\operatorname{span}\mathcal E_B\), then \(\omega=\varphi\).
    Equivalently, the real vector space of unnormalized joint states is the
    tensor product \(V_A\otimes V_B\) used above.

    Product states are the tensors
    \(\alpha\otimes\beta\in\Omega_{AB}\) with
    \(\alpha\in\Omega_A\) and \(\beta\in\Omega_B\). Product effects are the
    tensors \(e_A\otimes e_B\in\mathcal E_{AB}\), evaluated by
    \[
      (e_A\otimes e_B)(\alpha\otimes\beta)=e_A(\alpha)e_B(\beta),
    \]
    for \(e_A\in\mathcal E_A\) and \(e_B\in\mathcal E_B\). Product reversible
    transformations are maps \(T_A\otimes T_B\in G_{AB}\), with
    \(T_A\in G_A\) and \(T_B\in G_B\), acting linearly on
    \(V_A\otimes V_B\).

    A joint reversible transformation \(T\in G_{AB}\) is
    correlation-creating if there exist \(\omega_A\in\Omega_A\) and
    \(\omega_B\in\Omega_B\), not necessarily pure, such that
    \(T(\omega_A\otimes\omega_B)\) is not a product state. This is the precise
    version used here of the Open Quantum Problems phrase "not of the form
    \(T_A\otimes T_B\)", because the same Open Quantum Problems sentence
    identifies such a transformation with one that reversibly creates
    correlations, and because classical CNOT is one of the embeddable examples
    on the Open Quantum Problems page. A local product transformation and a
    mere relabeling or swap of isomorphic tensor factors map every product
    state to a product state and are not counted as correlation-creating for
    this entry.

    The state-space and reversible-dynamics data of \(A,B,AB\) are
    quantum-embeddable if there are finite-dimensional complex Hilbert spaces
    \(H_A,H_B\), affine injective maps
    \[
      \iota_A:\Omega_A\to\mathcal D(H_A),\qquad
      \iota_B:\Omega_B\to\mathcal D(H_B),\qquad
      \iota_{AB}:\Omega_{AB}\to\mathcal D(H_A\otimes H_B),
    \]
    where \(\mathcal D(H)\) is the density-operator state space on \(H\), such
    that pure GPT states are mapped to pure quantum states and
    \(\iota_{AB}(\alpha\otimes\beta)=
    \iota_A(\alpha)\otimes\iota_B(\beta)\). Every reversible transformation
    \(T\in G_X\) is represented on the image by
    unitary conjugation, meaning that there is a unitary \(U_T\) on the
    corresponding Hilbert space with
    \[
      \iota_X(T\omega)=U_T\iota_X(\omega)U_T^\dagger
    \]
    for all \(\omega\in\Omega_X\). Product reversible transformations are
    represented tensorially:
    \(U_{T_A\otimes T_B}=U_{T_A}\otimes U_{T_B}\) for all
    \(T_A\in G_A\) and \(T_B\in G_B\), up to an irrelevant global phase. The
    embedding need not be onto the full
    quantum state or effect space; classical systems, stabilizer subtheories,
    and other quantum subtheories count as quantum-embeddable when the above
    structure is preserved. No condition is imposed here that every allowed GPT
    effect be represented by a quantum effect; the allowed effect sets enter
    this entry only to define local tomography, product measurements, and the
    operational reduction of the stated GPT systems.
  </def>

  <known>
    Open Quantum Problems, Reversible dynamics on composite systems, Problem 42,
    asks whether the presence of a reversible transformation that is not a
    product transformation, for a locally tomographic composite of two local
    state spaces whose reversible groups are transitive on pure states, forces
    the state space to be embeddable into standard quantum theory. The Open
    Quantum Problems index lists this as Problem 42 in Quantum foundations, with
    Borivoje Dakic and Markus P. Muller as contacts and date 2017/05/20.

    The Open Quantum Problems formulation does not fully specify the allowed
    effect sets, whether disconnected reversible symmetries count in the same
    way as continuous dynamics, or what exact operational content is required by
    "embeddable into standard quantum theory". This entry records one precise
    state-space and reversible-dynamics version of the Open Quantum Problems
    question: an implication for finite-dimensional locally tomographic
    compact-convex GPTs, with embedding as a possibly proper complex-quantum
    subtheory preserving product structure and the specified reversible groups.
    Failure to represent an extra allowed effect quantum mechanically is not,
    by itself, a disproof of this entry's claim.

    Barrett, "Information processing in generalized probabilistic theories",
    Physical Review A 75, 032304 (2007), gives the GPT framework used by the
    Open Quantum Problems page for compact convex state spaces, effects,
    transformations, and composites. Hardy, "Quantum Theory From Five
    Reasonable Axioms", arXiv:quant-ph/0101012 (2001), is cited by the Open
    Quantum Problems page for tomographic locality, the principle that joint
    states are determined by local measurement statistics.

    Gross, Muller, Colbeck, and Dahlsten, "All reversible dynamics in maximally
    non-local theories are trivial", Physical Review Letters 104, 080402
    (2010), prove that reversible transformations in the boxworld-like
    maximally nonlocal theories considered there are generated only by local
    operations and subsystem permutations, so such theories do not supply the
    correlation-creating reversible transformation required in the claim.
    Al-Safi and
    Short, "Reversible Dynamics in Strongly Non-Local Boxworld Systems",
    Journal of Physics A 47, 325303 (2014), Al-Safi and Richens,
    "Reversibility and the structure of the local state space", New Journal of
    Physics 17, 123001 (2015), and Richens, Selby, and Al-Safi, "Entanglement
    is an inevitable feature of any non-classical theory", arXiv:1610.00682
    (2016), give related restrictions linking local state-space structure,
    nonclassicality, entanglement, and reversible interactions. Richens, Selby,
    and Al-Safi's later "Entanglement is Necessary for Emergent Classicality in
    All Physical Theories", Physical Review Letters 119, 080503 (2017), gives
    adjacent classical-limit context. These boxworld
    and local-state-space results are substantial evidence, but they do not
    prove the general implication in the claim for arbitrary compact convex
    local state spaces.

    Masanes, Muller, Perez-Garcia, and Augusiak, "Entanglement and the
    three-dimensionality of the Bloch ball", Journal of Mathematical Physics
    55, 122203 (2014), classify bipartite composites of equal-dimensional
    binary \(d\)-ball local state spaces with \(d\ge2\), under local
    tomography and connected continuous global reversible dynamics: except for
    the \(d=3\) quantum two-qubit case, the theories in that family have no
    entangled states and no interacting dynamics. Krumm and Muller, "Quantum
    computation is the unique reversible
    circuit model for which bits are balls", npj Quantum Information 5, 7
    (2019), extend the ball-state-space no-go direction to arbitrary finite
    numbers of gbits with local transformation group \(\mathrm{SO}(d)\): for
    \(d\ge2\) and \(d\neq3\), no-signaling, tomographic locality, and a closed
    connected global transformation group force all reversible gates to be
    single-gbit gates. These theorems settle the important Euclidean-ball
    family, not OQP42 for arbitrary local compact convex state spaces or for
    all possible disconnected reversible groups.

    Hardy and Wootters, "Limited Holism and Real-Vector-Space Quantum Theory",
    Foundations of Physics 42, 454-473 (2012), and Barnum, Graydon, and Wilce,
    "Some Nearly Quantum Theories", Electronic Proceedings in Theoretical
    Computer Science 195, 59-70 (2015), describe non-complex or nearly quantum
    examples relevant to the Open Quantum Problems background. Barnum,
    Graydon, and Wilce, "Composites and Categories of Euclidean Jordan
    Algebras", Quantum 4, 359 (2020), develop Jordan-algebraic composites.
    These examples do not give a locally tomographic non-quantum
    counterexample to the claim as stated here; real-vector-space quantum
    theory and the Jordan-algebraic alternatives are either not locally
    tomographic in the standard composite sense, are covered by special no-go
    results such as the ball-state-space theorems, or require assumptions
    outside the Open Quantum Problems formulation.

    Dakic and Brukner, "The Classical Limit of a Physical Theory and the
    Dimensionality of Space", in Quantum Theory: Informational Foundations and
    Foils, Springer (2016), discuss invariant reversible dynamics for
    generalized bits and spatial dimension. Kueng, Zhu, and Gross, "Low rank
    matrix recovery from Clifford orbits", arXiv:1610.08070 (2016), is cited by
    the Open Quantum Problems page for Clifford-orbit facts supporting the
    stabilizer example: stabilizer states have transitive Clifford dynamics and
    non-product Clifford gates such as CNOT, but the stabilizer theory is a
    subtheory of standard complex quantum theory and hence is not a
    counterexample.

    Later reconstruction results impose additional hypotheses that are stronger
    than OQP42. Chiribella, D'Ariano, and Perinotti, "Probabilistic theories
    with purification", Physical Review A 81, 062348 (2010), and
    "Informational derivation of quantum theory", Physical Review A 84, 012311
    (2011), use purification and other operational axioms to reconstruct
    quantum theory. Barnum, Ududec, and van de Wetering, "Self-duality and
    Jordan structure of quantum theory follow from homogeneity and pure
    transitivity", arXiv:2306.00362 (2023), derive Jordan structure from
    homogeneity plus pure transitivity and discuss additional local-tomography
    routes to complex quantum theory. These do not settle the OQP42 conjecture,
    because OQP42 assumes only local pure-state transitivity and existence of a
    correlation-creating reversible composite transformation, not purification,
    homogeneity, spectrality, or strong symmetry.
  </known>

  <refs>
    <ref>Open Quantum Problems, Reversible dynamics on composite systems, Problem 42, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/reversible-dynamics-on-composite-systems, used for the conjecture statement, GPT setup, transitive-pure-state assumption, local-tomography assumption, non-product reversible-transformation premise, examples, and listed partial results.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is derived from Problem 42, Reversible dynamics on composite systems, listed in Quantum foundations.</ref>
    <ref>J. Barrett, Information processing in generalized probabilistic theories, Physical Review A 75, 032304 (2007), DOI: 10.1103/PhysRevA.75.032304, used for the finite-dimensional GPT framework of states, effects, transformations, and composites.</ref>
    <ref>L. Hardy, Quantum Theory From Five Reasonable Axioms, arXiv:quant-ph/0101012 (2001), used for tomographic locality as cited by the Open Quantum Problems page.</ref>
    <ref>D. Gross, M. Muller, R. Colbeck, and O. C. O. Dahlsten, All reversible dynamics in maximally non-local theories are trivial, Physical Review Letters 104, 080402 (2010), DOI: 10.1103/PhysRevLett.104.080402, used for the maximally nonlocal boxworld no-go result.</ref>
    <ref>S. W. Al-Safi and A. J. Short, Reversible Dynamics in Strongly Non-Local Boxworld Systems, Journal of Physics A: Mathematical and Theoretical 47, 325303 (2014), DOI: 10.1088/1751-8113/47/32/325303, used for boxworld reversible-dynamics restrictions beyond the Gross-Muller-Colbeck-Dahlsten setting.</ref>
    <ref>S. W. Al-Safi and J. Richens, Reversibility and the structure of the local state space, New Journal of Physics 17, 123001 (2015), DOI: 10.1088/1367-2630/17/12/123001, used for local-state-space restrictions from reversibility.</ref>
    <ref>J. G. Richens, J. H. Selby, and S. W. Al-Safi, Entanglement is an inevitable feature of any non-classical theory, arXiv:1610.00682 (2016), used for links between nonclassical local structure, local transitivity or no-broadcasting assumptions, and entanglement in theories with reversible interactions.</ref>
    <ref>J. G. Richens, J. H. Selby, and S. W. Al-Safi, Entanglement is Necessary for Emergent Classicality in All Physical Theories, Physical Review Letters 119, 080503 (2017), DOI: 10.1103/PhysRevLett.119.080503, arXiv:1705.08028, used for adjacent emergent-classicality context.</ref>
    <ref>Ll. Masanes, M. P. Muller, D. Perez-Garcia, and R. Augusiak, Entanglement and the three-dimensionality of the Bloch ball, Journal of Mathematical Physics 55, 122203 (2014), DOI: 10.1063/1.4903510, used for the bipartite Euclidean-ball classification and the \(d=3\) Bloch-ball exception.</ref>
    <ref>M. Krumm and M. P. Muller, Quantum computation is the unique reversible circuit model for which bits are balls, npj Quantum Information 5, 7 (2019), DOI: 10.1038/s41534-018-0123-x, used for the finite-multipartite gbit extension of the Bloch-ball no-go result.</ref>
    <ref>L. Hardy and W. K. Wootters, Limited Holism and Real-Vector-Space Quantum Theory, Foundations of Physics 42, 454-473 (2012), DOI: 10.1007/s10701-011-9616-6, used for the real-vector-space quantum-theory background and failure of standard local tomography.</ref>
    <ref>H. Barnum, M. A. Graydon, and A. Wilce, Some Nearly Quantum Theories, Electronic Proceedings in Theoretical Computer Science 195, 59-70 (2015), DOI: 10.4204/EPTCS.195.5, used for nearly quantum examples from the Open Quantum Problems background.</ref>
    <ref>H. Barnum, M. A. Graydon, and A. Wilce, Composites and Categories of Euclidean Jordan Algebras, Quantum 4, 359 (2020), DOI: 10.22331/q-2020-11-08-359, arXiv:1606.09331, used for Euclidean-Jordan-algebra composite background and local-tomography limitations.</ref>
    <ref>B. Dakic and C. Brukner, The Classical Limit of a Physical Theory and the Dimensionality of Space, in Quantum Theory: Informational Foundations and Foils, edited by G. Chiribella and R. W. Spekkens, Springer, Dordrecht, 2016, used for invariant reversible-dynamics and generalized-bit background cited by the Open Quantum Problems page.</ref>
    <ref>R. Kueng, H. Zhu, and D. Gross, Low rank matrix recovery from Clifford orbits, arXiv:1610.08070 (2016), used for the Clifford-orbit and stabilizer-state background cited by the Open Quantum Problems page.</ref>
    <ref>G. Chiribella, G. M. D'Ariano, and P. Perinotti, Probabilistic theories with purification, Physical Review A 81, 062348 (2010), DOI: 10.1103/PhysRevA.81.062348, used for comparison with stronger reconstruction assumptions involving purification and reversible interactions.</ref>
    <ref>G. Chiribella, G. M. D'Ariano, and P. Perinotti, Informational derivation of quantum theory, Physical Review A 84, 012311 (2011), DOI: 10.1103/PhysRevA.84.012311, used for comparison with stronger operational reconstructions of finite-dimensional quantum theory.</ref>
    <ref>H. Barnum, C. Ududec, and J. van de Wetering, Self-duality and Jordan structure of quantum theory follow from homogeneity and pure transitivity, arXiv:2306.00362 (2023), used as later literature showing quantum or Jordan reconstruction under assumptions stronger than OQP42.</ref>
  </refs>

  <ask>
    Prove or disprove the implication in the claim with the definitions above.
    A proof must show that every finite-dimensional locally tomographic
    compact-convex GPT composite satisfying the stated local pure-state
    transitivity and correlation-creating reversible-transformation premises
    admits a complex quantum embedding preserving product states and the
    specified reversible transformation groups.

    A disproof must give a concrete finite-dimensional compact-convex GPT
    counterexample with specified \(V_A,V_B,\Omega_A,\Omega_B,\Omega_{AB}\),
    allowed effect sets, and reversible groups \(G_A,G_B,G_{AB}\), prove local
    tomography, prove transitivity of \(G_A\) and \(G_B\) on pure local states,
    exhibit a correlation-creating reversible \(T\in G_{AB}\), and prove that no
    quantum state-space and reversible-dynamics embedding in the sense of this
    entry exists. A theory that lacks local tomography, has only product
    reversible transformations and subsystem permutations, differs from quantum
    theory only by adding non-quantum-representable effects, embeds as a proper
    quantum subtheory, satisfies stronger reconstruction assumptions such as
    purification or homogeneity, or covers only Euclidean-ball local state
    spaces is progress unless it implies the full implication or such a
    counterexample.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
