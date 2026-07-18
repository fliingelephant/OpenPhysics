# QTD006 Markovian Thermal Process Convertibility with Coherence

```xml
<problem id="QTD006">
  <status>open</status>
  <name>Markovian Thermal Process Convertibility with Coherence</name>
  <keys>markovian thermal process markovian thermal processes MTP continuous thermomajorization thermomajorization gamma-ordering thermomajorization curve Lorenz curve Markovian master equation covariant Lindbladian Lindblad master equation quantum coherence energy coherence coherence matrix off-diagonal density matrix elements population dynamics population vector energy distribution complete set of laws second laws of thermodynamics elementary thermalizations universal thermodynamic controls finite verifiability constructibility desiderata D1 D2 entropy production alpha-Renyi entropy production generalized entropy production relative entropy of asymmetry Wigner-Yanase-Dyson skew information Fisher information coherence monotone symmetry monotone asymmetry monotone thermal process TP resource theory of quantum thermodynamics dynamical approach to quantum thermodynamics Lostaglio Korzekwa qubit minimal decoherence theory symmetric dynamics Styliaris Zanardi Gour Jennings Buscemi Duan Marvian quantum majorization entropic conditions embeddability problem block-diagonal energy-incoherent state coherent state extension memoryless thermalization Davies map non-degenerate Hamiltonian Bohr frequency</keys>

  <claim>
    Fix an integer \(d\geq2\), a Hamiltonian \(H=\sum_{i=1}^dE_i\Pi_i\) on a
    \(d\)-dimensional Hilbert space with non-degenerate spectrum (\(E_i\in
    \mathbb R\) pairwise distinct, \(\Pi_i=|E_i\rangle\langle E_i|\)), and an
    inverse temperature \(\beta\in(0,\infty)\), fixing the Gibbs state
    \(\gamma\) (defined below). Let \(\rho(0),\rho(t_f)\) range over
    arbitrary density operators on this Hilbert space, not assumed diagonal
    in \(\{|E_i\rangle\}\), and write
    \(\rho(0)\stackrel{\mathrm{MTP}}{\longmapsto}\rho(t_f)\) when some
    Markovian thermal process for \((H,\beta)\) (defined below) maps
    \(\rho(0)\) to exactly \(\rho(t_f)\).

    Give a relation \(R(\rho(0),\rho(t_f))\), stated purely in terms of
    \(\rho(0)\), \(\rho(t_f)\), \(H\), \(\beta\), such that, for every such
    \(d\), \(H\), and \(\beta\), and for every pair \(\rho(0),\rho(t_f)\) of
    density operators on the \(d\)-dimensional space:

    1. \(R(\rho(0),\rho(t_f))\) holds if and only if
       \(\rho(0)\stackrel{\mathrm{MTP}}{\longmapsto}\rho(t_f)\), and whether
       \(R\) holds can be decided by a procedure that halts after finitely
       many steps on every input pair (D1, finite verifiability, defined
       below);
    2. whenever \(R\) holds, a Markovian thermal process realizing
       \(\rho(0)\stackrel{\mathrm{MTP}}{\longmapsto}\rho(t_f)\) can be
       exhibited by a procedure that also halts after finitely many steps
       (D2, constructibility, defined below);
    3. whenever \(\rho(0),\rho(t_f)\) are both diagonal in \(\{|E_i\rangle\}\)
       (block-diagonal, i.e. energy-incoherent), \(R\) reduces to continuous
       thermomajorization of the population vectors,
       \(p(0)\gg_\gamma p(t_f)\), proved necessary and sufficient by
       Lostaglio and Korzekwa, Physical Review A 106, 012426 (2022),
       Theorem 1, and realized there by a finite sequence of elementary
       thermalizations, their Theorem 3;
    4. whenever \(d=2\), \(R\) reduces to the criterion given by the
       minimal-decoherence-theory solution of Lostaglio, Korzekwa, and
       Milne, Physical Review A 96, 032109 (2017), Section E3.

    No relation \(R\) meeting requirements 1-4 for every \(d\geq2\) is
    currently proved to exist once \(\rho(0)\) is not diagonal in
    \(\{|E_i\rangle\}\) (nonzero coherence); the gap is at \(d\geq3\),
    since requirement 4 already resolves \(d=2\) (see Known).
  </claim>

  <def>
    Setting. \(H=\sum_{i=1}^dE_i\Pi_i\) is a Hamiltonian on a
    \(d\)-dimensional Hilbert space, \(E_i\in\mathbb R\) pairwise distinct
    (non-degenerate spectrum), \(\Pi_i=|E_i\rangle\langle E_i|\) the
    rank-one projector onto the eigenspace of \(E_i\), \(\{|E_i\rangle\}\)
    an orthonormal eigenbasis. \(\beta\in(0,\infty)\) is the inverse
    temperature, \(Z:=\operatorname{tr}(e^{-\beta H})=\sum_ie^{-\beta E_i}\),
    and \(\gamma:=e^{-\beta H}/Z=\sum_i\gamma_i\Pi_i\),
    \(\gamma_i:=e^{-\beta E_i}/Z\), is the Gibbs thermal state.

    For a time-dependent state \(\rho(t)\), the population vector is
    \(p(t)=(p_1(t),\dots,p_d(t))\), \(p_i(t):=\langle E_i|\rho(t)|E_i\rangle\),
    and the coherence matrix has entries
    \(C_{ij}(t):=\langle E_i|\rho(t)|E_j\rangle\) for \(i\neq j\). Because
    \(H\) is non-degenerate, \(\rho(t)\) is diagonal in \(\{|E_i\rangle\}\)
    (equivalently block-diagonal, equivalently energy-incoherent) if and
    only if \(C_{ij}(t)=0\) for every \(i\neq j\).

    Markovian thermal process (MTP). A channel \(T\) is a Markovian thermal
    process for \((H,\beta)\) if it results from integrating, between
    \(t=0\) and some \(t_f\in[0,+\infty]\), a master equation
    \[
      \frac{d\rho(t)}{dt}=\mathcal H(\rho(t))+\mathcal L_t(\rho(t)),
      \qquad
      \mathcal H(\rho):=-i[H,\rho],
    \]
    \[
      \mathcal L_t(\rho)=\sum_kr_k(t)\left(L_k(t)\rho L_k(t)^\dagger-
      \tfrac12\{L_k(t)^\dagger L_k(t),\rho\}\right),
    \]
    with time-dependent jump operators \(L_k(t)\) and rates \(r_k(t)\geq0\),
    such that for every \(t\): (P1) \(\mathcal L_t(\gamma)=0\) (stationarity),
    and (P2) \(\mathcal L_t(\mathcal H(\rho))=\mathcal H(\mathcal L_t(\rho))\)
    for every \(\rho\) (covariance). Write
    \(\rho(0)\stackrel{\mathrm{MTP}}{\longmapsto}\rho(t_f)\) when some such
    \(T\) has \(T(\rho(0))=\rho(t_f)\) exactly (no error tolerance).

    Thermal process (TP, contrast, used only in Known). A channel
    \(\mathcal E\) is a thermal process for \((H,\beta)\) if
    \(\mathcal E(\gamma)=\gamma\) and
    \(\mathcal E(e^{-iHt}\rho e^{iHt})=e^{-iHt}\mathcal E(\rho)e^{iHt}\) for
    every \(\rho\) and every \(t\in\mathbb R\). Every MTP is a TP; a TP need
    not arise from integrating any single master equation of the form
    above, so it may encode an arbitrary, possibly history-dependent
    realization. Write \(\rho(0)\stackrel{\mathrm{TP}}{\longmapsto}\rho(t_f)\)
    for the corresponding, strictly weaker relation.

    Thermomajorization. For a probability vector \(p\) on \(\{1,\dots,d\}\),
    a \(\gamma\)-ordering \(\pi=\pi(p)\) is a permutation of
    \(\{1,\dots,d\}\) with \(p_{\pi_i}/\gamma_{\pi_i}\geq
    p_{\pi_{i+1}}/\gamma_{\pi_{i+1}}\) for \(i=1,\dots,d-1\). The
    thermomajorization curve of \(p\) is the piecewise-linear concave curve
    through the \(d+1\) points
    \(l^{(j)}=\bigl(\sum_{k\le j}\gamma_{\pi_k(p)},\sum_{k\le j}
    p_{\pi_k(p)}\bigr)\), \(j=0,\dots,d\), \(l^{(0)}:=(0,0)\). Write
    \(p\succ_\gamma q\) (\(p\) thermomajorizes \(q\)) when the
    thermomajorization curve of \(p\) is nowhere below that of \(q\).

    Continuous thermomajorization. \(p\gg_\gamma q\) if there is a
    continuous path \(r(t)\), \(t\in[0,t_f]\), of probability vectors with
    \(r(0)=p\), \(r(t_f)=q\), and \(r(t_1)\succ_\gamma r(t_2)\) for every
    \(0\le t_1\le t_2\le t_f\).

    Elementary thermalization. For \(i\neq j\) in \(\{1,\dots,d\}\) and
    \(\lambda\in[0,1]\), \(T^{i,j}(\lambda)\) is the stochastic map on
    population vectors acting as the identity on every coordinate outside
    \(\{i,j\}\) and, on the pair \((p_i,p_j)\), as
    \[
      p_i\mapsto(1-\lambda)p_i+\lambda\,\frac{\gamma_i}{\gamma_i+\gamma_j}
      (p_i+p_j),
      \qquad
      p_j\mapsto(1-\lambda)p_j+\lambda\,\frac{\gamma_j}{\gamma_i+\gamma_j}
      (p_i+p_j).
    \]
    \(T^{i,j}(0)\) is the identity and \(T^{i,j}(1)\) fully equilibrates
    levels \(i,j\) to the ratio \(\gamma_i:\gamma_j\); every
    \(T^{i,j}(\lambda)\) is realized by an explicit two-level reset master
    equation satisfying (P1)-(P2).

    Desiderata (D1)-(D2). A proposed characterization of
    \(\stackrel{\mathrm{MTP}}{\longmapsto}\) satisfies (D1), finite
    verifiability, if whether \(\rho(0)\stackrel{\mathrm{MTP}}{\longmapsto}
    \rho(t_f)\) holds can be decided, for any given \(\rho(0),\rho(t_f)\), by
    a procedure that halts after finitely many steps. It satisfies (D2),
    constructibility, if, whenever the relation holds, a Markovian thermal
    process realizing it can be explicitly exhibited.
  </def>

  <known>
    Lostaglio and Korzekwa, Continuous thermomajorization and a complete set
    of laws for Markovian thermal processes, Physical Review A 106, 012426
    (2022) -- the source cited by the issue for this entry -- Theorem 1
    (Eq. 30) proves: whenever \(\rho(0),\rho(t_f)\) are both diagonal in
    \(\{|E_i\rangle\}\), \(\rho(0)\stackrel{\mathrm{MTP}}{\longmapsto}
    \rho(t_f)\) holds if and only if \(p(0)\gg_\gamma p(t_f)\). Their
    Theorem 3 (Eq. 42) proves, on the same domain, that this holds if and
    only if there is a finite sequence of elementary thermalizations with
    \(T^{i_f,j_f}(\lambda_f)\cdots T^{i_1,j_1}(\lambda_1)p(0)=p(t_f)\).
    Theorems 1 and 3 are proved jointly, as the three-way equivalence
    Theorem 10, in Appendix A ("Proof of Theorem 1 and Theorem 3"). Their
    Sections VII-VIII supply an algorithm deciding continuous
    thermomajorization and returning the realizing sequence in finitely
    many steps (a Mathematica implementation solving \(d\le7\) on a
    laptop), so Theorems 1 and 3 together satisfy both (D1) and (D2) on the
    block-diagonal domain: this is the "complete...laws...solved for
    populations in 2022" of the issue. A footnote to their Section
    II.B.1 records a further elementary bound: because property (P2)
    forces \(C(t)=0\) for every \(t>0\) whenever \(C(0)=0\), no
    Markovian thermal process can carry a diagonal \(\rho(0)\) to a
    non-diagonal \(\rho(t_f)\); that sub-case is therefore already
    decided, trivially and correctly, by taking \(R\) to be identically
    false there.

    The same paper's Section IX ("Conclusions and Outlook") states,
    directly after summarizing these results: "Another direction that
    should be further explored concerns the role of quantum coherence in
    these settings. We provide some initial remarks in Appendix C, while a
    solution to this problem satisfying both desiderata (D1)-(D2) is still
    out of reach." Appendix C ("Remarks on fundamental constraints on
    coherence") poses exactly the claim's general problem -- "What about
    the characterization of the evolution of coherences, i.e., the
    off-diagonal elements of the density matrix in the energy basis?" -- and
    closes: "The general problem of Eq. (24) can be explicitly solved for a
    single qubit system using the minimal decoherence theory of Ref. [74]
    (see Sec. E3 therein). However, for higher dimensional systems
    currently there are no tools to obtain a solution to this problem
    (perhaps up to some approximation) that satisfies both our fundamental
    desiderata. We leave this extremely challenging question to future
    work." This is the direct source of the claim above. Ambiguity: this
    parenthetical shows the authors leave open whether even an approximate
    (not exact) characterization would count as resolving their question;
    the claim recorded here fixes the exact relation
    \(\stackrel{\mathrm{MTP}}{\longmapsto}\) of Eq. (24), i.e.
    \(T(\rho(0))=\rho(t_f)\) with no error tolerance, which is the weaker
    and unambiguous of the two readings, and does not assert that an
    approximate characterization is also unknown.

    Appendix C also records the necessary, not sufficient, conditions known
    for \(d\geq3\) with coherence: a family of \(\alpha\)-Rényi entropy
    production inequalities extended to general (not necessarily
    energy-incoherent) \(\rho(t)\) (Eq. C1); its \(\alpha=1\) decomposition
    into a diagonal-entropy term and the \(\alpha\)-relative entropy of
    asymmetry \(-dA_\alpha/dt\geq0\) (Eqs. C3-C4), which operationally
    quantifies work lost to dephasing; a Fisher-information monotone on the
    unitary orbit of \(\rho(t)\) under \(H\) (Eqs. C5-C6); the
    Wigner-Yanase-Dyson skew-information family \(-dI_s/dt\geq0\),
    \(s\in(0,1)\) (Eqs. C7-C8); and, built from the symmetry-monotone
    framework of Styliaris and Zanardi, Symmetries and monotones in
    Markovian quantum dynamics, Quantum 4, 261 (2020), a one-parameter
    coherence-monotone family \(d\mathcal C_\lambda/dt\geq0\) (Eq. C9)
    weighting each coherence-matrix element by a Gibbs factor and the
    squared energy gap. Appendix C further notes that the general
    symmetry-monotone framework of Gour, Jennings, Buscemi, Duan, and
    Marvian, Quantum majorization and a complete set of entropic conditions
    for quantum thermodynamics, Nature Communications 9, 5352 (2018), can
    formally define a complete set of such monotones, but that these "form
    an infinite number of extremely involved conditions, and it is not yet
    clear how these can be simplified" -- i.e. not known to satisfy (D1). No
    source cited in the paper proves any of these families, singly or
    jointly, sufficient for \(\rho(0)\stackrel{\mathrm{MTP}}{\longmapsto}
    \rho(t_f)\) once either state carries coherence and \(d\geq3\).

    The Gour et al. (2018) result above solves the analogous problem for
    TP, not MTP: the source paper's Section II.A.2 records that, for
    general (not necessarily energy-incoherent) \(\rho(0),\rho(t_f)\), "the
    problem for general states was formally solved by the remarkable work
    in Ref. [22]," referring there to \(\stackrel{\mathrm{TP}}{\longmapsto}\)
    of Eq. (13), not \(\stackrel{\mathrm{MTP}}{\longmapsto}\) of Eq. (24).
    Since every MTP is a TP but not conversely (Def section above), a
    characterization of \(\stackrel{\mathrm{TP}}{\longmapsto}\) does not by
    itself yield one of \(\stackrel{\mathrm{MTP}}{\longmapsto}\); the source
    paper's Section II.B.2 ("Insufficiency of the resource-theoretic
    approach"), Eqs. (19)-(20), illustrates the gap even in the
    energy-incoherent case, exhibiting a two-level population pair that
    satisfies the complete family of generalized free energy (second-law)
    constraints of Brandão, Horodecki, Ng, Oppenheim, and Wehner,
    Proceedings of the National Academy of Sciences 112, 3275 (2015) --
    constraints implied by, though not there proved equivalent to,
    thermomajorization -- while still being unreachable from one another
    by any Markovian thermal process.
    Hence the Gour et al. completion of the TP-level solution to general
    (coherent) states is not, by itself, a solution to the claim above.

    Lostaglio, Korzekwa, and Milne, Markovian evolution of quantum
    coherence under symmetric dynamics, Physical Review A 96, 032109
    (2017), develop a minimal decoherence theory identifying, for a given
    achievable population trajectory, the least decoherence of the
    coherence matrix compatible with a valid generator satisfying (P1)-(P2);
    its Section E3 specializes this to \(d=2\), which the 2022 source paper
    cites (as its Ref. [74], "Sec. E3 therein") as fully solving the
    claim's general problem for \(d=2\).
  </known>

  <refs>
    <ref>M. Lostaglio and K. Korzekwa, Continuous thermomajorization and a complete set of laws for Markovian thermal processes, Physical Review A 106, 012426 (2022), DOI: 10.1103/PhysRevA.106.012426, arXiv:2111.12130, used for Definition 1 (Markovian thermal process, Eqs. 1-6), Definition 2 (thermal process, Eqs. 12-13), Eqs. (24)-(25) (the MTP relation and its population restriction), the (D1)-(D2) desiderata (Section III), Definition 3 and Eqs. (27)-(28) (thermomajorization and continuous thermomajorization), Theorem 1 (Eq. 30), Theorem 3 (Eqs. 39-42, elementary thermalizations), Theorem 10 and its proof (Appendix A), Sections VII-VIII (the finite verification and construction algorithm), Sections II.A.2 and II.B.2 (the TP-versus-MTP distinction, Eqs. 13, 19-20), Section IX ("Conclusions and Outlook"), and Appendix C ("Remarks on fundamental constraints on coherence", Eqs. C1-C9), the source of the open-problem statement.</ref>
    <ref>M. Lostaglio, K. Korzekwa, and A. Milne, Markovian evolution of quantum coherence under symmetric dynamics, Physical Review A 96, 032109 (2017), DOI: 10.1103/PhysRevA.96.032109, used for the minimal decoherence theory and its Section E3 solution of the claim's general problem for \(d=2\), cited as Ref. [74] and "Sec. E3 therein" by Lostaglio-Korzekwa (2022), Appendix C.</ref>
    <ref>G. Gour, D. Jennings, F. Buscemi, R. Duan, and I. Marvian, Quantum majorization and a complete set of entropic conditions for quantum thermodynamics, Nature Communications 9, 5352 (2018), DOI: 10.1038/s41467-018-06261-7, arXiv:1708.04302, used for the complete entropic solution of the thermal-process (TP), as opposed to Markovian-thermal-process (MTP), state-conversion relation for general states, cited as Ref. [22] by Lostaglio-Korzekwa (2022), Sections II.A.2 and Appendix C.</ref>
    <ref>G. Styliaris and P. Zanardi, Symmetries and monotones in Markovian quantum dynamics, Quantum 4, 261 (2020), DOI: 10.22331/q-2020-04-30-261, arXiv:1912.04939, used for the symmetry-monotone framework underlying the one-parameter coherence-monotone family of Eq. (C9), cited as Ref. [73] by Lostaglio-Korzekwa (2022), Appendix C.</ref>
    <ref>F. G. S. L. Brandão, M. Horodecki, N. H. Y. Ng, J. Oppenheim, and S. Wehner, The second laws of quantum thermodynamics, Proceedings of the National Academy of Sciences 112, 3275 (2015), DOI: 10.1073/pnas.1411728112, arXiv:1305.5278, used for the generalized free energy (second-law) family of Eq. (20), cited as Ref. [4] by Lostaglio-Korzekwa (2022), Section II.B.2, in the two-level example (Eqs. 19-20) illustrating that satisfying these constraints does not imply Markovian-thermal-process achievability.</ref>
  </refs>

  <ask>
    Give the relation \(R\) described in the claim above, for every integer
    \(d\geq2\), every non-degenerate \(H\) on a \(d\)-dimensional space, and
    every \(\beta\in(0,\infty)\), and prove:

    (a) \(R(\rho(0),\rho(t_f))\) holds if and only if
        \(\rho(0)\stackrel{\mathrm{MTP}}{\longmapsto}\rho(t_f)\), for every
        pair of density operators \(\rho(0),\rho(t_f)\) on the
        \(d\)-dimensional space;
    (b) \(R\) can be decided by a procedure halting after finitely many
        steps on every input pair (D1);
    (c) whenever \(R\) holds, a procedure halting after finitely many steps
        exhibits an explicit Markovian thermal process realizing
        \(\rho(0)\stackrel{\mathrm{MTP}}{\longmapsto}\rho(t_f)\) (D2);
    (d) \(R\) reduces to \(p(0)\gg_\gamma p(t_f)\) (Theorem 1 of
        Lostaglio-Korzekwa 2022), realized by elementary thermalizations
        (their Theorem 3), whenever \(\rho(0),\rho(t_f)\) are both diagonal
        in \(\{|E_i\rangle\}\);
    (e) \(R\) reduces to the minimal-decoherence-theory criterion of
        Lostaglio, Korzekwa, and Milne (2017), Section E3, whenever
        \(d=2\).

    A disproof must show that no relation \(R\) meeting (a)-(e) exists for
    some specific \(d\geq3\). For example: a proof that, for some
    \(d\geq3\), no procedure halts on every pair of density operators on
    the \(d\)-dimensional space and correctly decides
    \(\rho(0)\stackrel{\mathrm{MTP}}{\longmapsto}\rho(t_f)\) once \(\rho(0)\)
    or \(\rho(t_f)\) carries nonzero coherence would rule out (b) and so
    constitute a disproof; likewise a proof that, for some \(d\geq3\),
    every family of conditions on \((\rho(0),\rho(t_f))\) that is both
    necessary and sufficient for \(\stackrel{\mathrm{MTP}}{\longmapsto}\)
    and extends the monotones of Appendix C (Eqs. C1, C3, C5, C7, C9) of
    Lostaglio-Korzekwa (2022) must be infinite, with no finite subfamily
    also sufficient, would rule out (b) and so also constitute a disproof.
    A single numerical example on which one specific proposed \(R\) fails
    disproves only that \(R\), not the claim.

    The following count as progress, not as resolving the claim, unless
    combined with a proof removing the stated restriction: (i) necessary
    conditions on \((\rho(0),\rho(t_f))\) not also shown sufficient, for
    any \(d\geq3\) with nonzero coherence, including monotones beyond Eqs.
    C1, C3, C5, C7, C9 of Lostaglio-Korzekwa (2022); (ii) sufficient
    conditions not also shown necessary; (iii) an \(R\) meeting (a)-(e) for
    one or finitely many fixed values of \(d\geq3\) (e.g. \(d=3\) alone)
    rather than for every \(d\geq2\); (iv) results restricted to a proper
    subclass of coherence patterns, such as a single nonzero coherence
    entry \(C_{ij}\), coherence confined to a single Bohr frequency
    \(E_i-E_j\), or a bound on the rank or support of
    \(\rho(0),\rho(t_f)\); (v) an \(R\) satisfying (a) only up to an error
    tolerance (e.g. \(\lVert T(\rho(0))-\rho(t_f)\rVert\le\epsilon\) for
    some norm and \(\epsilon\gt0\)) rather than the exact relation
    \(\stackrel{\mathrm{MTP}}{\longmapsto}\) of Eq. (24), unless accompanied
    by a proof that the exact and approximate relations coincide on the
    relevant domain.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
