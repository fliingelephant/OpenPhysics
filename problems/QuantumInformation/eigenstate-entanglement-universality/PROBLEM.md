# QTD032 Average Eigenstate Entanglement Entropy of the Free-Fermion Ring

```xml
<problem id="QTD032">
  <status>open</status>
  <name>Average Eigenstate Entanglement Entropy of the Free-Fermion Ring</name>
  <keys>eigenstate entanglement entropy average entanglement entropy of eigenstates free fermion chain quadratic fermionic hamiltonian tight-binding ring XX chain translationally invariant free fermions volume-law entanglement volume-law coefficient subsystem fraction page curve page value typical pure gaussian states fermionic gaussian states universal entanglement curve eigenstate thermalization hypothesis ETH quantum chaos diagnostic integrability diagnostic highly excited eigenstates closed form exact analytical form vidmar rigol hackl bianchi kieburg reduced correlation matrix slater determinant entanglement bipartite entanglement entropy</keys>

  <claim>
    There is a closed-form expression \(g:(0,1)\to\mathbb R_{\ge0}\), in the
    sense of the Def section, such that
    \[
      \liminf_{V\to\infty} \frac{\overline S_T(\lfloor fV\rfloor,V)}{V}
      \;=\;
      \limsup_{V\to\infty} \frac{\overline S_T(\lfloor fV\rfloor,V)}{V}
      \;=\;
      g(f)
      \qquad\text{for every } f\in(0,1),
    \]
    where \(\overline S_T(V_A,V)\) is the average, over all \(2^V\) energy
    eigenstates of the translationally invariant free-fermion ring
    \(\hat H_T\) on \(V\) sites (Def section), of the von Neumann
    entanglement entropy of a contiguous \(V_A\)-site subsystem.

    Quantifier note (weaker reading chosen; ambiguity flagged). The survey
    lead this entry is drawn from asks to prove the conjectured universal
    entanglement curves of energy eigenstates, naming two nearby targets: an
    explicit free-fermion curve for quadratic models, and a maximal curve
    for quantum-chaotic interacting models. This entry adopts only the
    first target, and only for the single, concretely pinned quadratic
    Hamiltonian \(\hat H_T\) defined below, because it is the one target for
    which Bianchi, Hackl, Kieburg, Rigol, and Vidmar, PRX Quantum 3, 030201
    (2022), Section V.C, give an unambiguous statement that the exact
    closed form is not yet known. The chaotic-model maximal curve requires
    a separate, not-yet-pinned-down mathematical definition of
    "quantum-chaotic Hamiltonian" and is recorded only as a parallel,
    still-open conjecture in the Known section, not as part of this claim.
    Within the quadratic side, this entry also does not adopt the stronger
    claim that the same function \(g\) governs every translationally
    invariant quadratic fermionic Hamiltonian; that cross-model
    universality (tested, but not proved in general, for the transverse-field
    Ising and XY chains in Hackl, Vidmar, Rigol, and Bianchi, Phys. Rev. B
    99, 075123 (2019)) is logically independent of, and stronger than, the
    single-model existence-of-closed-form claim adopted here. Finally, this
    entry states convergence itself (liminf equals limsup) as part of the
    claim, rather than assuming it, because the cited sources establish
    only finite upper and lower bounds on \(\overline S_T(V_A,V)/V\) at
    fixed \(f\), not a proof that the limit exists.
  </claim>

  <def>
    Fix an integer \(V\ge2\) (number of lattice sites). Spinless-fermion
    operators \(\hat f_1,\ldots,\hat f_V\) satisfy
    \(\{\hat f_j,\hat f_k^\dagger\}=\delta_{jk}\), \(\{\hat f_j,\hat f_k\}=0\),
    acting on the \(2^V\)-dimensional Fock space
    \(\mathcal H=\bigotimes_{j=1}^V\mathbb C^2\). The translationally
    invariant free-fermion ring is
    \[
      \hat H_T
      =
      -\sum_{j=1}^V\big(\hat f_j^\dagger\hat f_{j+1}+\hat f_{j+1}^\dagger\hat f_j\big),
      \qquad \hat f_{V+1}:=\hat f_1 .
    \]
    With \(\hat c_k=V^{-1/2}\sum_{j=1}^V e^{-2\pi ikj/V}\hat f_j\) for
    \(k=0,1,\ldots,V-1\), one has
    \(\hat H_T=\sum_{k=0}^{V-1}\epsilon_k\,\hat c_k^\dagger\hat c_k\) with
    \(\epsilon_k=-2\cos(2\pi k/V)\). For each occupation vector
    \(\vec n=(n_0,\ldots,n_{V-1})\in\{0,1\}^V\), the Slater determinant
    \[
      |\vec n\rangle=\prod_{k:\,n_k=1}\hat c_k^\dagger\,|0\rangle
    \]
    is an exact eigenstate of \(\hat H_T\); the \(2^V\) states \(|\vec
    n\rangle\) are pairwise orthonormal and exhaust the spectrum. Energy
    degeneracies among different \(\vec n\) (which occur whenever
    \(\sum_kn_k\epsilon_k\) coincides for distinct occupation vectors) do
    not affect the definitions below, since every \(|\vec n\rangle\) is a
    fixed, well-defined basis vector regardless of degeneracy, and the
    average over eigenstates below is a uniform average over all \(2^V\)
    vectors \(\vec n\), with no restriction on particle number
    \(N(\vec n)=\sum_kn_k\).

    For \(1\le V_A\le V-1\), fix the contiguous subsystem
    \(A=\{1,\ldots,V_A\}\). The single-particle correlation submatrix of
    \(|\vec n\rangle\) restricted to \(A\) is the \(V_A\times V_A\)
    Hermitian matrix
    \[
      [C_A(\vec n)]_{j\ell}
      =
      \langle\vec n|\hat f_j^\dagger\hat f_\ell|\vec n\rangle
      =
      \frac1V\sum_{k=0}^{V-1} n_k\, e^{2\pi ik(\ell-j)/V},
      \qquad j,\ell\in A ,
    \]
    which satisfies \(0\preceq C_A(\vec n)\preceq\mathbf 1\). Writing its
    eigenvalues as \(\zeta_1,\ldots,\zeta_{V_A}\in[0,1]\), the von Neumann
    entanglement entropy of \(|\vec n\rangle\) across \(A\) is
    \[
      S_A(\vec n)
      =
      -\sum_{\ell=1}^{V_A}
      \Big[\zeta_\ell\ln\zeta_\ell+(1-\zeta_\ell)\ln(1-\zeta_\ell)\Big],
    \]
    with the convention \(0\ln0=0\); this is the standard reduced-entropy
    formula for a fermionic Gaussian (Slater-determinant) state, computed
    from \(A\) alone, without diagonalizing the complementary block or
    resolving energy degeneracies. Define the eigenstate average
    \[
      \overline S_T(V_A,V)
      =
      \frac{1}{2^V}\sum_{\vec n\in\{0,1\}^V} S_A(\vec n) .
    \]
    Because each \(|\vec n\rangle\) is pure, \(S_A(\vec n)\) equals the
    entanglement entropy of the complementary \((V-V_A)\)-site block, so
    \(\overline S_T(V_A,V)=\overline S_T(V-V_A,V)\).

    For \(f\in(0,1)\), set \(V_A(V)=\lfloor fV\rfloor\) and define
    \[
      \underline s_T(f)=\liminf_{V\to\infty}\frac{\overline S_T(V_A(V),V)}{V},
      \qquad
      \overline s_T(f)=\limsup_{V\to\infty}\frac{\overline S_T(V_A(V),V)}{V} ,
    \]
    both finite by the bound in the Known section. \(\underline s_T\) and
    \(\overline s_T\) inherit the symmetry \(f\leftrightarrow1-f\) from
    \(\overline S_T(V_A,V)=\overline S_T(V-V_A,V)\) above.

    Closed form. A function \(g:(0,1)\to\mathbb R\) is given in closed form
    if it is a finite expression in the real variable \(f\), built from:
    the four arithmetic operations and real powers; the exponential,
    logarithm, trigonometric, and hyperbolic functions and their inverses;
    finitely many applications of named special functions whose definition
    is fixed independently of this problem (for instance the Gamma
    function, the polylogarithm \(\mathrm{Li}_s\), the Hurwitz zeta
    function, the complete elliptic integrals, or the Gauss hypergeometric
    function \({}_2F_1\)); finitely many case distinctions on \(f\) (for
    instance a split at \(f=1/2\)); and, optionally, a finite sum or a
    definite integral, over an \(f\)-independent domain, of an expression
    of the preceding kind in which \(f\) appears only as an explicit
    parameter. An expression that presupposes the value of
    \(\underline s_T\) or \(\overline s_T\) (for instance, defining \(g\)
    as "the thermodynamic limit of \(\overline S_T/V\)" itself), an
    asymptotic or perturbative series in \(1/V\) whose resummation in the
    preceding sense is not shown, or a quantity defined only by numerical
    root-finding, optimization, or simulation, is not a closed form in this
    sense.
  </def>

  <known>
    Vidmar, Hackl, Bianchi, and Rigol, Entanglement Entropy of Eigenstates
    of Quadratic Fermionic Hamiltonians, Phys. Rev. Lett. 119, 020601
    (2017), derive, for the most general translationally invariant
    quadratic fermionic Hamiltonian on \(V\) sites with subsystem size
    \(V_A\) and Hilbert-space dimensions \(\mathcal D_A=2^{V_A}\),
    \(\mathcal D=2^V\), the bound
    \[
      \ln\mathcal D_A-\frac{(\ln\mathcal D_A)^2}{\ln\mathcal D}
      \;\le\;
      \langle S\rangle
      \;\le\;
      \ln\mathcal D_A-\frac{1}{2\ln2}\frac{(\ln\mathcal D_A)^2}{\ln\mathcal D} ,
    \]
    where \(\langle S\rangle\) is the average entanglement entropy over all
    eigenstates. \(\hat H_T\) above is a translationally invariant quadratic
    fermionic Hamiltonian, so this bound applies to it; substituting
    \(\mathcal D_A=2^{V_A}\), \(\mathcal D=2^V\), \(f=V_A/V\), it reads
    \(f(1-f)\ln2\le\overline S_T(V_A,V)/V\le f\ln2-\frac{f^2}{2}\), which proves
    \(0\lt\underline s_T(f)\le\overline s_T(f)\lt\infty\) for every
    \(f\in(0,1)\) but pins down neither their common value nor their
    equality. The same Letter separately proves that, in the limit
    \(f\to0\) at fixed \(V_A\), the average entanglement entropy of
    eigenstates of a quadratic fermionic Hamiltonian approaches the maximal
    (Page) value; this small-subsystem-fraction statement is not the
    fixed-\(f\) claim above.

    Hackl, Vidmar, Rigol, and Bianchi, Average eigenstate entanglement
    entropy of the XY chain in a transverse field and its universality for
    translationally invariant quadratic fermionic models, Phys. Rev. B 99,
    075123 (2019), tighten the bound above, to fourth order in an explicit
    systematic expansion, for the spin-1/2 XY chain in a transverse
    magnetic field, which maps by a Jordan-Wigner transformation to a
    translationally invariant quadratic fermionic Hamiltonian, and report that
    the resulting volume-law coefficient function of \(f\) is concave; they
    state that it coincides
    with an analogous coefficient function they had reported earlier
    (Phys. Rev. Lett. 121, 220602 (2018)) for the quantum Ising chain,
    citing this as evidence of cross-model universality within the
    translationally invariant quadratic family. \(\hat H_T\) above is
    itself the zero-transverse-field (XX) member of this same family, so
    the tightened bound and the tested cross-model universality are
    directly relevant background, but neither this paper nor Phys. Rev.
    Lett. 119, 020601 (2017) exhibits a closed-form expression matching the
    tightened bounds, and neither proves convergence (equality of
    \(\underline s_T\) and \(\overline s_T\)) from first principles.

    Bianchi, Hackl, Kieburg, Rigol, and Vidmar, Volume-Law Entanglement
    Entropy of Typical Pure Quantum States, PRX Quantum 3, 030201 (2022),
    Section V.C, study \(\hat H_T\) directly (their Eq. (139), with
    \(\overline S_T\) written as \(\bar S_{\mathrm T}\)) and report, from
    exact diagonalization up to \(V=36\) sites at \(f=1/2\), a volume-law
    coefficient \(s_T^\infty=0.5378(1)\), defined by
    \(\overline S_T(V_A,V)\approx s_T^\infty V_A\ln2\) at \(f=1/2\),
    consistent with the numerics of Phys. Rev. Lett. 119, 020601 (2017).
    They contrast this with the exactly known closed-form volume-law
    coefficient of typical (Haar-random) pure fermionic Gaussian states at
    the same filling, \(s^\infty_{G,w=0}=0.5573\) (their Eq. (118) at
    \(f=1/2\)), and report that the two coefficients are numerically close
    but distinct. Immediately afterward, they state that the exact
    analytical form of the \(\bar S_{\mathrm T}(f)\) curve for
    translationally invariant free fermions is not known, although tight
    bounds are available, citing Phys. Rev. B 99, 075123 (2019) for the
    latter. This is the direct source of the open status recorded in the
    Claim section: as of that review (arXiv:2112.06959v2, 27 July 2022), no
    closed-form expression for \(\overline s_T\) had been exhibited, only
    numerical values and bound sequences, and the coincidence
    \(\underline s_T=\overline s_T\) (existence of the limit) is not
    separately proved either, although the clean, fast finite-size
    convergence reported in their Fig. 16 inset (fit to a power law in
    \(1/V\)) is strong numerical evidence for it. In the \(V\)-normalized
    convention of the Def section, since
    \(\overline S_T(V_A,V)/V=[\overline S_T(V_A,V)/(V_A\ln2)]\cdot(V_A\ln2/V)\),
    this numerical value corresponds to
    \(\overline s_T(1/2)\approx\frac{\ln2}{2}\times0.5378(1)\approx0.1864\),
    inside the bound \([0.1733,0.2216]\) established above, not the raw
    figure \(0.5378(1)\) itself.

    The same review, Section V.A, records a parallel, logically independent
    conjecture for genuinely interacting (not quadratic) quantum-chaotic
    Hamiltonians: numerically, the eigenstate-averaged entanglement entropy
    of a particle-number-conserving hard-core-boson chain (their Eq. (136))
    in its quantum-chaotic regime matches, to within terms that vanish
    algebraically in \(V\), the exactly known closed-form average
    entanglement entropy of general pure states with fixed particle number
    (their Eq. (45)) -- the maximal curve of the survey lead this entry is
    drawn from. The review describes this match as numerical only and
    identifies an explanation for it as an open direction for future work.
    This chaotic-model conjecture is not part of the Claim section above;
    it needs its own mathematical definition of "quantum-chaotic
    Hamiltonian" (the review's own criterion is Wigner-Dyson level
    statistics, an ensemble-level, not per-Hamiltonian, property) before it
    can be stated as a single theorem-shaped claim.

    Vidmar and Rigol, Entanglement Entropy of Eigenstates of Quantum
    Chaotic Hamiltonians, Phys. Rev. Lett. 119, 220603 (2017), is a related
    but separate result on the chaotic-model side: for random pure states
    with a fixed particle number, real Gaussian-distributed coefficients,
    and a system divided into two equal halves away from half filling, they
    prove an upper bound showing that the average entanglement entropy
    deviates from the maximal value by a term growing with the square root
    of the system volume, and give numerical evidence of saturation of that
    bound for one quantum-chaotic model. This result concerns the
    maximal-curve side, not the free-fermion/quadratic side, of the survey
    lead this entry is drawn from; it is recorded here only as chaotic-model
    background and is not used in the Claim section.

    As of the Bianchi-Hackl-Kieburg-Rigol-Vidmar review, no closed-form
    expression, in the sense of the Def section, had been exhibited or
    proved to equal \(\overline s_T(f)\) or \(\underline s_T(f)\) for
    \(\hat H_T\) at any \(f\in(0,1)\).
  </known>

  <refs>
    <ref>L. Vidmar, L. Hackl, E. Bianchi, and M. Rigol, Entanglement Entropy of Eigenstates of Quadratic Fermionic Hamiltonians, Physical Review Letters 119, 020601 (2017), DOI 10.1103/PhysRevLett.119.020601, arXiv:1703.02979, used for the general translationally-invariant quadratic-Hamiltonian bound stated in the Known section and the small-subsystem-fraction maximal-entropy result.</ref>
    <ref>L. Hackl, L. Vidmar, M. Rigol, and E. Bianchi, Average eigenstate entanglement entropy of the XY chain in a transverse field and its universality for translationally invariant quadratic fermionic models, Physical Review B 99, 075123 (2019), DOI 10.1103/PhysRevB.99.075123, arXiv:1812.08757, used for the tightened (fourth-order) bounds and the numerically tested cross-model universality noted in the Known section.</ref>
    <ref>E. Bianchi, L. Hackl, M. Kieburg, M. Rigol, and L. Vidmar, Volume-Law Entanglement Entropy of Typical Pure Quantum States, PRX Quantum 3, 030201 (2022), DOI 10.1103/PRXQuantum.3.030201, arXiv:2112.06959, Section V.C (translationally invariant noninteracting fermions, their Eq. (139), Fig. 16) for the open-status statement, the numerical value 0.5378(1), and the contrast with the closed-form Gaussian-state coefficient 0.5573 of their Eq. (118); Section V.A (their Eq. (136), Eq. (45)) for the parallel quantum-chaotic-interacting conjecture recorded in the Known section.</ref>
    <ref>L. Vidmar and M. Rigol, Entanglement Entropy of Eigenstates of Quantum Chaotic Hamiltonians, Physical Review Letters 119, 220603 (2017), DOI 10.1103/PhysRevLett.119.220603, arXiv:1708.08453, used only for the chaotic-model background noted in the Known section.</ref>
  </refs>

  <ask>
    Prove or disprove the claim that \(\underline s_T(f)=\overline s_T(f)\)
    equals a closed-form expression \(g(f)\), in the sense of the Def
    section, for every \(f\in(0,1)\).

    A proof must exhibit an explicit \(g\) satisfying the Def-section
    closed-form criterion and must rigorously establish
    \(\underline s_T(f)=\overline s_T(f)=g(f)\) for every \(f\in(0,1)\), not
    merely at \(f=1/2\), not merely as an asymptotic or numerical fit, and
    not merely within the bound sandwich of Phys. Rev. Lett. 119, 020601
    (2017) or Phys. Rev. B 99, 075123 (2019). Matching \(g(1/2)\) to
    \(\frac{\ln2}{2}\times0.5378(1)\approx0.1864\) (the \(V\)-normalized
    value corresponding, via the Known-section conversion, to the source's
    \(s_T^\infty=0.5378(1)\)) to a few reported digits is evidence, not a
    proof.

    A disproof must establish, for some particular \(f_0\in(0,1)\), at
    least one of: (i) \(\underline s_T(f_0)\ne\overline s_T(f_0)\), so that
    no limiting curve exists at that filling and hence no single \(g\) can
    equal both; or (ii) that \(\underline s_T(f_0)=\overline s_T(f_0)\)
    exists but cannot be written in the closed form of the Def section, by
    a rigorous non-expressibility argument, not merely by the failure of
    one proposed candidate \(g\) to match numerics or bounds. Tightening
    the known bound sequence, or exhibiting a \(g\) that matches only at
    isolated fillings or only asymptotically as \(f\to0\), is progress
    only. A resolution of the cross-model universality question (whether
    the same \(g\) governs other translationally invariant quadratic
    fermionic Hamiltonians) or of the quantum-chaotic-interacting maximal
    curve conjecture recorded in the Known section is progress on a
    related but logically separate claim, not a proof or disproof of the
    claim above.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
