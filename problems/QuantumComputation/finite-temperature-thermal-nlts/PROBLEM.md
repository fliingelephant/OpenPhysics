# QTD036 Thermal NLTS at Constant Temperature

```xml
<problem id="QTD036">
  <status>open</status>
  <name>Thermal NLTS at Constant Temperature</name>
  <keys>thermal nlts finite-temperature nlts finite temperature nlts constant-temperature thermal states no low-energy trivial states nlts conjecture nlts theorem robust circuit depth for topologically ordered systems eldar conjecture 1 gibbs state thermal gibbs state circuit depth lower bound quantum circuit complexity mixed-state circuit complexity self-correcting quantum memory robust topological order topological order at nonzero temperature freedman-hastings conjecture room temperature entanglement shallow quantum circuit lower bound quantum locally testable code qltc quantum ldpc code qldpc polylog-local hamiltonian anshu breuckmann nirkhe lior eldar matthew hastings beni yoshida inverse temperature threshold quantum pcp conjecture gibbs sampling hardness</keys>

  <claim>
    There exists a constant \(\beta_0\in(0,\infty)\) and a \(k(n)\)-local
    Hamiltonian family \(\{H_n\}_{n\in\mathbb N}\) with
    \(k(n)=O(\operatorname{polylog} n)\) (defined below) such that, for
    every constant \(\beta\ge\beta_0\): for
    every sequence \(\{(V_n,S_n)\}_{n\in\mathbb N}\), where \(V_n\) is a
    quantum circuit on \(a_n\ge n\) qubits and \(S_n\subseteq\{1,\dots,a_n\}\)
    is an \(n\)-element subset, satisfying
    \[
      \bigl\|
      \operatorname{tr}_{-S_n}\bigl(V_n|0^{\otimes a_n}\rangle\langle0^{\otimes a_n}|V_n^\dagger\bigr)
      -\rho_n(\beta)
      \bigr\|_1=o(1)
      \qquad(n\to\infty),
    \]
    the depth satisfies \(d(V_n)=\Omega(\log n)\), where
    \(\rho_n(\beta)=e^{-\beta\tilde H_n}/Z_n(\beta)\), with \(\tilde H_n\)
    the energy-density-normalized Hamiltonian defined below, is the thermal
    Gibbs state of \(H_n\) at inverse temperature \(\beta\).

    This entry formalizes Eldar, "Robust Quantum Entanglement at (nearly)
    Room Temperature," arXiv:1911.04461, Conjecture 1 ("Robust Circuit
    Depth for Topologically Ordered Systems"), whose boxed statement
    requires only "a family of topologically-ordered systems (local
    Hamiltonians)" and leaves the locality of \(\{H_n\}_n\) unquantified;
    the bound \(k(n)=O(\operatorname{polylog} n)\) above is this entry's own reading of that
    phrase, chosen to match the scope of Eldar's Theorem 2, not a
    quantifier stated in Conjecture 1 itself. The threshold \(\beta_0\) must be a
    single constant independent of \(n\), valid simultaneously for the
    entire ray \(\beta\ge\beta_0\); a family and threshold sequence
    \(\beta_n\to\infty\) for which the displayed bound holds only at
    \(\beta=\beta_n\) (shrinking temperature) does not satisfy this claim,
    and neither does a result confined to \(\beta=\infty\) (the ground-state
    case alone).
  </claim>

  <def>
    For \(n\in\mathbb N\), a Hamiltonian on \(n\) qubits is
    \(H_n=\sum_{i=1}^{m_n}h_i\), a sum of \(m_n\) Hermitian terms with
    \(0\preceq h_i\preceq I\); \(H_n\) is \(k(n)\)-local when every \(h_i\)
    acts as the identity on all but at most \(k(n)\) of the \(n\) qubits.
    This entry allows \(m_n=\operatorname{poly}(n)\) and requires
    \(H_n\succeq0\) (ground energy normalized to \(0\), as in the
    constructions in the known-results section below); restricting to
    \(h_i\succeq0\) with zero ground energy is this entry's own reading,
    matching those frustration-free constructions, and is not required by
    Conjecture 1 or by Eldar's Definition 2 itself, which bounds only
    \(\|H_i\|\le1\) and permits negative-eigenvalue terms. Following Eldar's
    Definition 2, \(H_n\) has energy density \(\lambda_n:=m_n/n\), and the
    energy-density-normalized Hamiltonian is
    \(\tilde H_n:=(n/m_n)H_n=H_n/\lambda_n\); this entry does not bound
    \(\lambda_n\) as \(n\to\infty\).

    Fix units with Boltzmann constant \(\kappa=1\), so inverse temperature
    \(\beta=1/T\) for temperature \(T\gt0\). Following Eldar's Definition 3,
    the thermal (Gibbs) state of \(H_n\) at inverse temperature
    \(\beta\in(0,\infty)\) is defined via the energy-density-normalized
    Hamiltonian:
    \[
      \rho_n(\beta):=\frac{e^{-\beta\tilde H_n}}{Z_n(\beta)},
      \qquad
      Z_n(\beta):=\operatorname{tr}\bigl(e^{-\beta\tilde H_n}\bigr).
    \]
    As \(\beta\to\infty\), \(\rho_n(\beta)\) approaches the maximally mixed
    state on \(\ker H_n=\ker\tilde H_n\) (a ground state); this entry
    concerns finite \(\beta\) bounded away from \(0\) by an
    \(n\)-independent constant, not this limit.

    A quantum circuit on \(a\) qubits is a unitary
    \(V=L_dL_{d-1}\cdots L_1\), where each layer \(L_j\) is a tensor product
    of one- and two-qubit unitary gates acting on pairwise-disjoint qubits
    among the \(a\) qubits (qubits untouched in a layer carry the identity).
    The depth \(d(V)\) is the least such \(d\) over all such decompositions
    of \(V\).

    For \(S\subseteq\{1,\dots,a\}\) with \(|S|=n\le a\) and a pure state
    \(|\phi\rangle\) on \(a\) qubits, \(\operatorname{tr}_{-S}(|\phi\rangle\langle\phi|)\)
    denotes the reduced density operator on the \(n\) qubits indexed by
    \(S\), obtained by tracing out the complementary \(a-n\) qubits.

    For a trace-class operator \(X\), \(\|X\|_1:=\operatorname{tr}\sqrt{X^\dagger X}\)
    is the trace norm. Following the sources cited below, this entry bounds
    \(\|\rho-\sigma\|_1\) directly (twice the trace distance
    \(\tfrac12\|\rho-\sigma\|_1\)); the constant factor of \(2\) does not
    affect any \(o(1)\) or \(\Omega(\log n)\) statement used here.

    For a sequence \(x_n\), \(x_n=o(1)\) means \(x_n\to0\) as
    \(n\to\infty\), and \(x_n=\Omega(\log n)\) means there is a constant
    \(c\gt0\) with \(x_n\ge c\log n\) for all sufficiently large \(n\).
  </def>

  <known>
    Freedman and Hastings, "Quantum systems on non-k-hyperfinite complexes:
    A generalization of classical statistical mechanics on expander
    graphs," Quantum Information and Computation 14(1-2), 144-180 (2014),
    pose the No Low-Energy Trivial States (NLTS) conjecture: a fixed
    constant \(\epsilon\gt0\) and a family of \(O(1)\)-local Hamiltonians
    \(\{H_n\}\) on \(n\) qubits such that every state of energy below
    \(\epsilon n\) has circuit complexity growing with \(n\). This is the
    \(T=0\) statement; per Eldar's Section 2.1.6 (quoted below), the
    finite-temperature mixed-state claim above is related to, but formally
    incomparable with, this pure-state statement, not a strengthening of it.

    Anshu, Breuckmann, and Nirkhe, "NLTS Hamiltonians from Good Quantum
    Codes," arXiv:2206.13228 (2022; published Proceedings of the 55th
    Annual ACM Symposium on Theory of Computing, STOC '23, Orlando, FL,
    June 20-23, 2023), Theorem 1, prove the Freedman-Hastings NLTS
    conjecture: a fixed \(\epsilon\gt0\) and an explicit family of
    \(O(1)\)-local frustration-free commuting Hamiltonians \(\{H^{(n)}\}\)
    (frustration-free: \(H^{(n)}\succeq0\) with
    \(\ker H^{(n)}=\bigcap_i\ker h_i^{(n)}\ne\{0\}\)), built from
    constant-rate linear-distance quantum LDPC codes with \(m=\Theta(n)\)
    terms, such that every state \(\psi_n\) with
    \(\operatorname{tr}(H^{(n)}\psi_n)\lt\epsilon n\) has circuit complexity
    \(\Omega(\log n)\). This is the "ground-state analogue proven 2022"
    behind this entry's motivation; because Theorem 1 bounds every state in
    the entire \(\epsilon n\)-wide low-energy band while Eldar's Definition 3
    gives the \(\beta\to\infty\) Gibbs state as any \(\rho\in\ker(H_n)\) (the
    exact zero-energy kernel only), Theorem 1 is closely related to, but not
    exactly, the \(\beta=\infty\) endpoint of the family of statements
    indexed by \(\beta\) above. Anshu,
    Breuckmann, and Nirkhe's own "Open questions" (Section 1, page 3) --
    whether Property 1 holds for every good qLDPC code, a connection to
    Grigoriev's sum-of-squares lower bounds, a connection to the Overlap Gap
    Property, implications for the quantum PCP conjecture, and generalizing
    the technique to non-commuting Hamiltonians -- do not mention thermal
    states, Gibbs states, or any finite-temperature generalization; Anshu,
    Breuckmann, and Nirkhe's paper neither claims nor discusses the
    finite-temperature case.

    Eldar, "Robust Quantum Entanglement at (nearly) Room Temperature,"
    arXiv:1911.04461 (v3, 2020), poses the claim above as Conjecture 1,
    describing it (Section 2.1.6) as a mixed-state analogue of NLTS that is,
    as far as is known, "formally incomparable" to it (neither is known to
    imply the other). Eldar's Theorem 2 (Section 9.2) proves only
    a relaxed version: for an explicit polylog-local (\(k(n)=\Theta(\log^3 n)\))
    frustration-free commuting Hamiltonian family built by amplifying a
    quantum locally testable code, the depth bound \(d(V)=\Omega(\log n)\)
    is proved to hold for every inverse temperature
    \(\beta\ge20e\cdot(\log\log n)^2\), i.e. only down to a temperature
    \(T_n=\Theta(1/(\log\log n)^2)\) that shrinks, slowly, with \(n\) -- not
    at any \(n\)-independent constant \(\beta_0\). Eldar's Section 2.1.2
    situates this among the only other results it identifies on this
    exact question: Yoshida, "Feasibility of self-correcting quantum memory
    and thermal stability of topological order," Annals of Physics
    326(10), 2566-2633 (2011), proves a no-go theorem ruling out any such
    robustness (even at shrinking temperature, beyond inverse-polynomial)
    for geometrically local 2-D and 3-D commuting-Hamiltonian models; and
    Hastings, "Topological Order at Nonzero Temperature," Physical Review
    Letters 107, 210501 (2011), gives what Eldar's own summary describes as
    only a heuristic, non-rigorous argument that the 4-D toric code
    specifically might sustain a circuit-depth bound at constant
    (\(\Omega(1)\)) temperature. Neither Yoshida (2011) nor Hastings (2011)
    reports a rigorous proof at constant temperature for any Hamiltonian
    family.

    A stronger nearby variant is not distinguished by the claim above and is
    open a fortiori: the same statement with \(k(n)=O(1)\) rather than
    \(k(n)=O(\operatorname{polylog} n)\), matching the dimension-independent
    locality of the original NLTS conjecture and of
    Anshu-Breuckmann-Nirkhe's theorem. Eldar's Conjecture 1 as stated does not
    fix this restriction; this entry follows the generality of Eldar's own
    Theorem 2 (polylog-local, ancilla-allowed, frustration-free commuting) as
    the weakest reading for which a nontrivial partial result already exists,
    since closing the temperature gap in that same setting is the most
    directly motivated open target.

    Restricting instead to ancilla-free circuits (\(a_n=n\), so \(S_n\) is the
    entire system and nothing is traced out) is not a further stronger
    variant: the claim above already universally quantifies over
    \(a_n\ge n\), so any family and \(\beta_0\) witnessing the claim as stated
    also witnesses this restriction, but not conversely, since a proof
    confined to \(a_n=n\) leaves ancilla-assisted approximations unaddressed.
    This restricted statement is weaker, not stronger, than the claim above,
    so it is not listed as a variant open a fortiori; a proof of it alone is
    a partial result that should be reported as a gap (see below), not as
    resolving or strengthening the claim.

    Whether frustration-free or commuting structure is necessary, as opposed
    to an artifact of the only two constructions available, is likewise not
    resolved by Eldar's paper or by Anshu, Breuckmann, and Nirkhe's paper.
  </known>

  <refs>
    <ref>Lior Eldar, "Robust Quantum Entanglement at (nearly) Room Temperature," arXiv:1911.04461v3 [quant-ph] (2020), DOI 10.48550/arXiv.1911.04461, used for Conjecture 1 (Section 2, "Robust Circuit Depth for Topologically Ordered Systems," the claim of this entry), Theorem 2 (Section 9.2, the proved shrinking-temperature partial result with beta at least 20e times (log log n) squared), Section 2.1.2 (the summary of proved and heuristic temperature thresholds), and Section 2.1.6 (the stated relation to the Freedman-Hastings NLTS conjecture).</ref>
    <ref>Anurag Anshu, Nikolas P. Breuckmann, and Chinmay Nirkhe, "NLTS Hamiltonians from Good Quantum Codes," arXiv:2206.13228v4 [quant-ph] (2022), published in Proceedings of the 55th Annual ACM Symposium on Theory of Computing (STOC '23), Orlando, FL, June 20-23, 2023, DOI 10.48550/arXiv.2206.13228, used for Theorem 1 (Section 1, the T=0 ground-state analogue resolving the Freedman-Hastings NLTS conjecture) and the "Open questions" list (Section 1, page 3, confirming the finite-temperature generalization is neither claimed nor discussed there).</ref>
    <ref>Michael H. Freedman and Matthew B. Hastings, "Quantum systems on non-k-hyperfinite complexes: A generalization of classical statistical mechanics on expander graphs," Quantum Information and Computation 14(1-2), 144-180 (2014), arXiv:1301.1363, used for the original statement of the NLTS conjecture that both partial results above target at beta equal to infinity.</ref>
    <ref>Matthew B. Hastings, "Topological Order at Nonzero Temperature," Physical Review Letters 107, 210501 (2011), DOI 10.1103/PhysRevLett.107.210501, arXiv:1106.6026, used, via Eldar's Section 2.1.2 summary table, as the only claimed evidence -- heuristic rather than rigorous -- toward a constant-temperature circuit-depth bound, specific to the 4-D toric code.</ref>
    <ref>Beni Yoshida, "Feasibility of self-correcting quantum memory and thermal stability of topological order," Annals of Physics 326(10), 2566-2633 (2011), arXiv:1103.1885, used for the no-go theorem ruling out this kind of robustness for geometrically local 2-D and 3-D models, cited in Eldar's Section 2.1.2 as part of the motivation for higher-dimensional or non-geometrically-local constructions.</ref>
  </refs>

  <ask>
    A PROOF must exhibit an explicit \(k(n)=O(\operatorname{polylog} n)\)-local Hamiltonian
    family \(\{H_n\}\) (stating \(m_n\) and \(k(n)\) explicitly) together
    with an explicit constant \(\beta_0\), and prove that for every constant
    \(\beta\ge\beta_0\) and every sequence \(\{(V_n,S_n)\}\) satisfying the
    stated \(o(1)\) approximation, \(d(V_n)=\Omega(\log n)\). The proof must
    cover the entire ray \(\beta\ge\beta_0\) with one family and one
    threshold; a result that only handles a sequence \(\beta_n\to\infty\)
    (as in Eldar's Theorem 2) or only \(\beta=\infty\) (as in
    Anshu-Breuckmann-Nirkhe's Theorem 1) does not prove this claim, since
    neither exhibits an \(n\)-independent \(\beta_0\), even though both are
    progress on weaker variants.

    Because the claim is an existence statement over Hamiltonian families, a
    DISPROOF cannot rest on one family failing to reach constant
    temperature: showing that Eldar's specific amplified
    quantum-locally-testable-code construction cannot be pushed past
    \(\beta=\Theta((\log\log n)^2)\) disproves only that witness, not the
    claim. A disproof must instead give an argument applying to every
    \(k(n)=O(\operatorname{polylog} n)\)-local Hamiltonian family: for every such family and
    every constant \(\beta_0\), there is some \(\beta\ge\beta_0\) and a
    sequence \(\{(V_n,S_n)\}\) achieving the stated \(o(1)\) approximation
    with \(d(V_n)\) not \(\Omega(\log n)\) (for instance
    \(d(V_n)=O(\operatorname{poly}\log\log n)\) along a subsequence). A
    construction-independent argument -- for example from rapid mixing or
    quasi-locality of Gibbs states -- that no \(O(\operatorname{polylog} n)\)-local
    Hamiltonian's thermal state can require \(\Omega(\log n)\)-depth
    circuits once \(\beta\) is bounded above by a fixed \(n\)-independent
    constant would suffice.

    A GAP submission should state exactly which quantifier is left open:
    for instance, a family and \(\beta_0\) with the bound proved only along
    a strict subsequence of \(n\), only for \(\beta\) in a proper sub-ray of
    \([\beta_0,\infty)\), only against ancilla-free circuits (\(a_n=n\), the
    weaker restriction noted above), or only at the stronger locality
    \(k(n)=O(1)\) (which would also settle this claim, as a special case,
    and should be reported as doing so). A PROGRESS submission should state
    which nearby variant it resolves, if any, without resolving the claim as
    stated -- for instance the stronger \(k(n)=O(1)\)-local variant noted
    above, or some other nearby formulation not already covered by the GAP
    category above.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
