# QCOMP035 Eight-Quhex AME State

```xml
<problem id="QCOMP035">
  <status>open</status>
  <name>Eight-Quhex AME State</name>
  <keys>qcomp035 ame absolutely maximally entangled state perfect tensor eight quhex quhex six-level qudit eight parties local dimension 6 ame(8,6) oqp35 concrete subcase quantum maximum distance separable qmds pure quantum mds code pure ((8,1,5))_6 quantum error correcting code qecc 4-uniform k-uniform multiunitary orthogonal array</keys>

  <claim>
    There exists a unit vector
    \[
      |\psi\rangle \in \left(\mathbb{C}^6\right)^{\otimes 8}
    \]
    such that, for every subset \(S \subseteq \{1,\ldots,8\}\) with
    \(|S| \le 4\),
    \[
      \operatorname{tr}_{S^c} |\psi\rangle\langle\psi|
      =
      \frac{I_{6^{|S|}}}{6^{|S|}} .
    \]
  </claim>

  <def>
    For integers \(n \ge 2\) and \(D \ge 2\), an
    absolutely maximally entangled state, denoted \(\operatorname{AME}(n,D)\),
    is a pure state
    \[
      |\phi\rangle \in \left(\mathbb{C}^D\right)^{\otimes n}
    \]
    such that every reduced density operator on at most
    \(\lfloor n/2\rfloor\) subsystems is maximally mixed:
    \[
      \rho_T=\operatorname{tr}_{T^c} |\phi\rangle\langle\phi|
      =
      \frac{I_{D^{|T|}}}{D^{|T|}},
      \qquad
      T \subseteq \{1,\ldots,n\},\quad |T| \le \lfloor n/2\rfloor .
    \]
    Equivalently, because \(|\phi\rangle\) is pure, it is maximally
    entangled across every bipartition, with maximal mixing on the smaller
    side of the bipartition. A quhex is a six-level quantum system, so
    \(\operatorname{AME}(8,6)\) means an AME state of eight quhex systems.

    In the standard exact, finite-dimensional, homogeneous local-dimension
    setting, the span of an \(\operatorname{AME}(n,D)\) state is a pure
    quantum error-correcting code with parameters
    \[
      ((n,1,\lfloor n/2\rfloor+1))_D .
    \]
    Conversely, a pure one-dimensional code with those parameters gives an
    \(\operatorname{AME}(n,D)\) state. This equivalence assumes a pure
    state, equal local dimension \(D\) on all \(n\) tensor factors, exact
    maximal mixing of all reductions of size at most \(\lfloor n/2\rfloor\),
    and the usual exact QECC distance convention.
  </def>

  <known>
    Open Quantum Problems, "Existence of absolutely maximally entangled pure
    states", Problem 35, asks for which numbers of systems and local
    dimensions \(\operatorname{AME}(n,D)\) states exist. The Open Quantum
    Problems index lists Problem 35 under quantum computation and records it
    as collected from OQP. This entry records only the concrete OQP35 subcase
    \(\operatorname{AME}(8,6)\), not the full classification problem.

    F. Huber and N. Wyderka, "Table of AME states", online table, last updated
    29.04.2026 on the live table page, marks \(\operatorname{AME}(8,6)\) as
    Unknown. In the same parsed table, \(\operatorname{AME}(11,3)\) is the
    other explicit Unknown entry. The table cites D. Goyeneche and K.
    Zyczkowski, "Genuinely multipartite entangled states and orthogonal
    arrays", Phys. Rev. A 90, 022316 (2014), DOI
    10.1103/PhysRevA.90.022316, for the \(\operatorname{AME}(8,6)\) row.

    A broad formulation asking to classify all \((n,D)\) would make a proof
    require an infinite classification and a disproof ambiguous without a
    chosen universal assertion. The finite claim here is proof/disproof
    meaningful: a construction of one state proves the claim, while a theorem
    excluding such a vector disproves it.

    D. Goyeneche, D. Alsina, J. I. Latorre, A. Riera, and K. Zyczkowski,
    "Absolutely maximally entangled states, combinatorial designs, and
    multiunitary matrices", Phys. Rev. A 92, 032316 (2015), DOI
    10.1103/PhysRevA.92.032316, develops the AME, combinatorial-design, and
    multiunitary-matrix viewpoints. A. J. Scott, "Multipartite entanglement,
    quantum-error-correcting codes, and entangling power of quantum
    evolutions", Phys. Rev. A 69, 052330 (2004), DOI
    10.1103/PhysRevA.69.052330, relates multipartite entanglement and QECC
    weight distributions. F. Huber and M. Grassl, "Quantum Codes of Maximal
    Distance and Highly Entangled Subspaces", Quantum 4, 284 (2020), DOI
    10.22331/q-2020-06-18-284, gives general QMDS bounds and propagation
    context; those results do not settle \(\operatorname{AME}(8,6)\).

    For qubits, Scott, Phys. Rev. A 69, 052330 (2004), and F. Huber, O.
    Guhne, and J. Siewert, "Absolutely Maximally Entangled States of Seven
    Qubits Do Not Exist", Phys. Rev. Lett. 118, 200502 (2017), DOI
    10.1103/PhysRevLett.118.200502, leave no qubit analogue of this concrete
    case open. S. A. Rather, A. Burchardt, W. Bruzda, G.
    Rajchel-Mieldzioc, A. Lakshminarayan, and K. Zyczkowski, "Thirty-six
    Entangled Officers of Euler: Quantum Solution to a Classically Impossible
    Problem", Phys. Rev. Lett. 128, 080507 (2022), DOI
    10.1103/PhysRevLett.128.080507, solved the four-quhex case
    \(\operatorname{AME}(4,6)\), but does not decide the eight-quhex case.
  </known>

  <refs>
    <ref>Open Quantum Problems, Existence of absolutely maximally entangled pure states, Problem 35, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/existence-of-absolutely-maximally-entangled-pure-states, collected from OQP and used for the broad AME existence formulation and QECC phrasing.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems index, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, collected from OQP and used for the Problem 35 listing and category.</ref>
    <ref>F. Huber and N. Wyderka, Table of AME states, https://huberfe.github.io/table.html and citation page http://www.tp.nt.uni-siegen.de/+fhuber/ame.html, live table last updated 29.04.2026, used for current Unknown status of \(\operatorname{AME}(8,6)\).</ref>
    <ref>MathBases, Table of AME states, https://mathbases.org/d/ame-states, updated 2026-01-27, used as current metadata for the Huber-Wyderka AME table.</ref>
    <ref>F. Huber and M. Grassl, Quantum Codes of Maximal Distance and Highly Entangled Subspaces, Quantum 4, 284 (2020), DOI 10.22331/q-2020-06-18-284, used for QMDS and pure-code context.</ref>
    <ref>D. Goyeneche, D. Alsina, J. I. Latorre, A. Riera, and K. Zyczkowski, Absolutely maximally entangled states, combinatorial designs, and multiunitary matrices, Phys. Rev. A 92, 032316 (2015), DOI 10.1103/PhysRevA.92.032316, used for AME, combinatorial-design, and multiunitary context.</ref>
    <ref>D. Goyeneche and K. Zyczkowski, Genuinely multipartite entangled states and orthogonal arrays, Phys. Rev. A 90, 022316 (2014), DOI 10.1103/PhysRevA.90.022316, cited by the Huber-Wyderka table for the \(\operatorname{AME}(8,6)\) row.</ref>
    <ref>A. J. Scott, Multipartite entanglement, quantum-error-correcting codes, and entangling power of quantum evolutions, Phys. Rev. A 69, 052330 (2004), DOI 10.1103/PhysRevA.69.052330, used for QECC and multipartite-entanglement context.</ref>
    <ref>F. Huber, O. Guhne, and J. Siewert, Absolutely Maximally Entangled States of Seven Qubits Do Not Exist, Phys. Rev. Lett. 118, 200502 (2017), DOI 10.1103/PhysRevLett.118.200502, used for qubit-status context.</ref>
    <ref>S. A. Rather, A. Burchardt, W. Bruzda, G. Rajchel-Mieldzioc, A. Lakshminarayan, and K. Zyczkowski, Thirty-six Entangled Officers of Euler: Quantum Solution to a Classically Impossible Problem, Phys. Rev. Lett. 128, 080507 (2022), DOI 10.1103/PhysRevLett.128.080507, used for the solved \(\operatorname{AME}(4,6)\) context.</ref>
  </refs>

  <ask>
    Prove the claim by constructing a vector satisfying the reduction equations
    in the claim, or equivalently by constructing a pure
    \(((8,1,5))_6\) quantum error-correcting code under the hypotheses in
    the definition. Disprove the claim by proving that no vector in
    \(\left(\mathbb{C}^6\right)^{\otimes 8}\) satisfies those equations,
    equivalently that no pure \(((8,1,5))_6\) code exists under the same
    hypotheses. Equivalently, an unrestricted \(4\)-unitary tensor of local
    dimension \(6\) proves the claim. Progress may settle restricted classes,
    such as stabilizer, minimal-support, orthogonal-array, or particular
    multiunitary ansatz families, but must state the restriction explicitly.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
