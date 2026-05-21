# ET015 Separability from Spectrum

```xml
<problem id="ET015">
  <status>open</status>
  <name>Separability from Spectrum</name>
  <keys>separability from spectrum absolute separability absolutely separable separable from spectrum global spectrum ordered spectrum eigenvalues unitary orbit global unitary fixed tensor factorization local dimensions bipartite dimensions absolute ppt appt ppt from spectrum positive partial transpose separable ball purity gurvits barnum hildebrand johnston qubit-qudit two-qubit open quantum problems oqp problem 15 entanglement theory</keys>

  <claim>
    For every pair of finite local dimensions \(N,M\ge 2\), characterize the
    set
    \[
      \mathrm{ASEP}_{N,M}
      =
      \left\{
        \lambda\in\Delta_{NM}^{\downarrow}:
        \text{every density operator on }
        \mathbb{C}^N\otimes\mathbb{C}^M
        \text{ with ordered spectrum } \lambda
        \text{ is separable}
      \right\}.
    \]
    Equivalently, determine exactly which ordered spectra \(\lambda\) have
    the property that, for one and hence every density operator \(\rho\) with
    \(\operatorname{spec}^{\downarrow}(\rho)=\lambda\),
    \(U\rho U^\dagger\) is separable with respect to the fixed
    \((N,M)\) tensor factorization for every global unitary
    \(U\in\mathcal{U}(NM)\).
  </claim>

  <def>
    Let \(D=NM\), and let
    \[
      \Delta_D^{\downarrow}
      =
      \{\lambda\in\mathbb{R}^D:
        \lambda_1\ge\lambda_2\ge\cdots\ge\lambda_D\ge 0,\,
        \sum_{i=1}^D\lambda_i=1\}.
    \]
    For a density operator \(\rho\succeq 0\) with
    \(\operatorname{tr}\rho=1\), \(\operatorname{spec}^{\downarrow}(\rho)\)
    denotes its eigenvalues sorted in nonincreasing order.

    \(\mathrm{Sep}_{N,M}\) is the convex hull of product density operators
    \(\alpha\otimes\beta\), where \(\alpha\) is a density operator on
    \(\mathbb{C}^N\) and \(\beta\) is a density operator on
    \(\mathbb{C}^M\). Equivalently, one may take the convex hull of pure
    product projectors
    \(|a\rangle\langle a|\otimes |b\rangle\langle b|\).

    A state \(\rho\) on \(\mathbb{C}^N\otimes\mathbb{C}^M\) is absolutely
    separable, or separable from spectrum, if
    \(U\rho U^\dagger\in\mathrm{Sep}_{N,M}\) for every global unitary
    \(U\in\mathcal{U}(NM)\), where \(\mathcal U(D)\) denotes the group of
    \(D\times D\) unitary matrices. A spectrum \(\lambda\) is absolutely
    separable in dimensions \((N,M)\) if any, equivalently all, states with
    ordered spectrum \(\lambda\) are absolutely separable.

    With matrix entries in a fixed product basis, partial transpose on the
    second tensor factor is
    \[
      \langle i j|\rho^\Gamma|k\ell\rangle
      =
      \langle i\ell|\rho|k j\rangle .
    \]
    A state, or its ordered spectrum, is absolutely PPT if
    \((U\rho U^\dagger)^\Gamma\succeq 0\) for every global unitary \(U\).
    Since separability implies PPT, \(\mathrm{ASEP}_{N,M}\subseteq
    \mathrm{APPT}_{N,M}\) for all finite \(N,M\).

    The problem is about the ordered global spectrum and the fixed local
    dimensions \((N,M)\). It is not a criterion involving the reduced spectra
    of \(\operatorname{tr}_B\rho\) or \(\operatorname{tr}_A\rho\), and a
    characterization for one factorization of total dimension \(D\) need not
    characterize another factorization of the same \(D\).
  </def>

  <known>
    Open Quantum Problems records Problem 15, "Separability from spectrum",
    in Entanglement theory and asks for the spectra for which every state
    with that spectrum is separable; the Open Quantum Problems list records
    it as Problem 15 with last progress dated 2013/09/08.

    Kus and Zyczkowski, "Geometry of entangled states", Phys. Rev. A 63,
    032307 (2001), define absolutely separable states as states whose whole
    global unitary orbit is separable, giving the terminology used in this
    entry.

    Gurvits and Barnum, "Largest separable balls around the maximally mixed
    bipartite quantum state", Phys. Rev. A 66, 062311 (2002), prove a largest
    separable ball around the identity in spectral norms. In the normalized
    density-operator form recorded by Open Quantum Problems, if
    \(\operatorname{tr}\rho^2\le 1/(NM-1)\), then \(\rho\) is separable.
    Since \(\operatorname{tr}\rho^2=\sum_i\lambda_i^2\) is invariant under
    global unitaries, this gives the sufficient spectral condition
    \(\sum_i\lambda_i^2\le 1/(NM-1)\) for absolute separability. Open Quantum
    Problems records this purity threshold as tight with respect to purity,
    but not as a complete spectral characterization.

    Verstraete, Audenaert, De Bie, and De Moor, "Maximally entangled mixed
    states of two qubits", arXiv:quant-ph/0011110, with the journal version
    by Verstraete, Audenaert, and De Moor in Phys. Rev. A 64, 012316 (2001),
    solve the \(2\otimes 2\) case. For
    \(\lambda_1\ge\lambda_2\ge\lambda_3\ge\lambda_4\ge 0\),
    \[
      \lambda\in\mathrm{ASEP}_{2,2}
      \quad\Longleftrightarrow\quad
      \lambda_1-\lambda_3-2\sqrt{\lambda_2\lambda_4}\le 0 .
    \]

    Hildebrand, "Positive partial transpose from spectra", Phys. Rev. A 76,
    052325 (2007), completely classifies absolute PPT spectra for finite
    bipartite dimensions by necessary and sufficient linear matrix
    inequalities in the ordered eigenvalues.

    Johnston, "Separability from spectrum for qubit-qudit states", Phys.
    Rev. A 88, 062330 (2013), solves all \(2\otimes d\), \(d\ge 2\), cases and proves
    that absolute separability coincides with absolute PPT there. For
    \(\lambda_1\ge\lambda_2\ge\cdots\ge\lambda_{2d}\ge 0\),
    \[
      \lambda\in\mathrm{ASEP}_{2,d}
      =
      \mathrm{APPT}_{2,d}
      \quad\Longleftrightarrow\quad
      \lambda_1-\lambda_{2d-1}
      -2\sqrt{\lambda_{2d-2}\lambda_{2d}}\le 0 .
    \]
    The Open Quantum Problems Problem 15 page displays the same qubit-qudit
    statement without the coefficient \(2\) in front of the square-root term;
    Johnston, Phys. Rev. A 88, 062330 (2013), and later citations include the
    coefficient \(2\), and the \(d=2\) specialization otherwise would not
    match the two-qubit formula above.

    Arunachalam, Johnston, and Russo, "Is absolute separability determined by
    the partial transpose?", Quantum Inf. Comput. 15, 694-720 (2015),
    investigate whether
    \(\mathrm{ASEP}_{N,M}=\mathrm{APPT}_{N,M}\) for all finite bipartite
    dimensions. They provide evidence by showing that several standard
    separability criteria and positive maps do not detect entanglement inside
    the absolutely PPT set, and prove equality for additional structured
    families, but they do not prove equality in all dimensions.

    Abellanet-Vidal, Muller-Rigat, Rajchel-Mieldzioc, and Sanpera,
    "Sufficient criteria for absolute separability in arbitrary dimensions via
    linear map inverses", arXiv:2410.22415v2 (2025), state that the complete
    characterization of the absolutely separable set remains largely unknown
    and give new sufficient analytic criteria. Louvet, Serrano Ensastiga,
    Bastin, and Martin, "Nonequivalence between absolute separability and
    positive partial transposition in the symmetric subspace", Phys. Rev. A
    111, 042418 (2025), state that the general equivalence between absolute
    separability and absolute PPT remains open; their counterexamples concern
    an analogous symmetric-subspace problem, not the fixed bipartite problem
    in this entry. These 2025 sufficient-criterion, extreme-point, and
    symmetric-subspace results do not settle the fixed bipartite
    \(\mathrm{ASEP}_{N,M}\) characterization beyond the qubit-qudit cases.
    Wang, Chen, and Song, "Extreme points of absolutely PPT states with exactly
    three distinct eigenvalues", arXiv:2603.20717 (2026), likewise describe the
    two-qutrit equality question for absolutely separable and absolutely PPT
    spectra as open while studying extreme points of the absolutely PPT set.
  </known>

  <refs>
    <ref>Open Quantum Problems, Problem 15, Separability from spectrum, https://oqp.iqoqi.oeaw.ac.at/separability-from-spectrum; collection provenance and problem statement for this entry.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems; lists Problem 15 in Entanglement theory.</ref>
    <ref>M. Kus and K. Zyczkowski, Geometry of entangled states, Phys. Rev. A 63, 032307 (2001), DOI 10.1103/PhysRevA.63.032307, arXiv:quant-ph/0006068.</ref>
    <ref>L. Gurvits and H. Barnum, Largest separable balls around the maximally mixed bipartite quantum state, Phys. Rev. A 66, 062311 (2002), DOI 10.1103/PhysRevA.66.062311, arXiv:quant-ph/0204159.</ref>
    <ref>F. Verstraete, K. Audenaert, T. De Bie, and B. De Moor, Maximally entangled mixed states of two qubits, arXiv:quant-ph/0011110; journal version F. Verstraete, K. Audenaert, and B. De Moor, Phys. Rev. A 64, 012316 (2001), DOI 10.1103/PhysRevA.64.012316.</ref>
    <ref>R. Hildebrand, Positive partial transpose from spectra, Phys. Rev. A 76, 052325 (2007), DOI 10.1103/PhysRevA.76.052325, arXiv:quant-ph/0502170.</ref>
    <ref>N. Johnston, Separability from spectrum for qubit-qudit states, Phys. Rev. A 88, 062330 (2013), DOI 10.1103/PhysRevA.88.062330, arXiv:1309.2006.</ref>
    <ref>S. Arunachalam, N. Johnston, and V. Russo, Is absolute separability determined by the partial transpose?, Quantum Inf. Comput. 15, 694-720 (2015), DOI 10.26421/QIC15.7-8-10, arXiv:1405.5853.</ref>
    <ref>J. Abellanet-Vidal, G. Muller-Rigat, G. Rajchel-Mieldzioc, and A. Sanpera, Sufficient criteria for absolute separability in arbitrary dimensions via linear map inverses, arXiv:2410.22415v2 (2025), DOI 10.1088/1361-6633/ae0cfa and DOI 10.48550/arXiv.2410.22415.</ref>
    <ref>J. Louvet, E. Serrano Ensastiga, T. Bastin, and J. Martin, Nonequivalence between absolute separability and positive partial transposition in the symmetric subspace, Phys. Rev. A 111, 042418 (2025), DOI 10.1103/PhysRevA.111.042418, arXiv:2411.16461.</ref>
    <ref>N. Wang, L. Chen, and Z. Song, Extreme points of absolutely PPT states with exactly three distinct eigenvalues, arXiv:2603.20717 (2026), DOI 10.48550/arXiv.2603.20717.</ref>
  </refs>

  <ask>
    Give a complete necessary-and-sufficient characterization of
    \(\mathrm{ASEP}_{N,M}\) for every finite pair \(N,M\ge 2\), in terms of
    the ordered global spectrum \(\lambda\) and the fixed local dimensions
    \((N,M)\). A proof must show that the proposed criterion is invariant
    under degeneracies and sorting conventions, accepts exactly those spectra
    whose entire global unitary orbit is contained in \(\mathrm{Sep}_{N,M}\),
    and rejects every spectrum for which some global unitary produces an
    entangled state.

    One possible proof route is to prove
    \(\mathrm{ASEP}_{N,M}=\mathrm{APPT}_{N,M}\) for all finite \(N,M\) and
    then combine that equality with Hildebrand's absolute-PPT LMIs. A
    counterexample to this equality in dimensions \(N,M\) would be progress on
    the Arunachalam-Johnston-Russo direction, but it would not by itself solve
    this entry unless it is accompanied by a complete replacement
    characterization of \(\mathrm{ASEP}_{N,M}\).

    A proposed disproof of a submitted characterization must provide fixed
    local dimensions and an ordered spectrum for which the characterization
    gives the wrong membership answer, together with a proof of the correct
    answer. A single entangled state with spectrum \(\lambda\) disproves
    absolute separability of that \(\lambda\), not the existence of a general
    characterization.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
