# 2 SIC-POVMs

```xml
<problem id="2">
  <status>open</status>
  <kind>construction</kind>
  <field>QIT</field>
  <name>SIC-POVMs</name>
  <keys>sic sics povm sic-povm symmetric informationally complete positive operator valued measure zauner zauner conjecture rank-one not-necessarily-covariant equiangular lines equiangular tight frame etf complex projective 2-design weyl-heisenberg heisenberg-weyl wh-sic hw clifford fiducial fiducial vector quantum information quantum tomography</keys>

  <claim>
    There exists a strictly increasing infinite sequence
    \(N_1 \lt N_2 \lt \cdots\) of integers \(N_r \ge 2\) such that, for every \(r\),
    there are unit vectors
    \[
      |\psi^{(r)}_1\rangle,\ldots,|\psi^{(r)}_{N_r^2}\rangle
      \in \mathbb{C}^{N_r}
    \]
    satisfying
    \[
      \left|\langle \psi^{(r)}_j|\psi^{(r)}_k\rangle\right|^2
      =
      \frac{N_r\delta_{jk}+1}{N_r+1},
      \qquad 1 \le j,k \le N_r^2 .
    \]
  </claim>

  <def>
    For \(N \ge 2\), a rank-one symmetric informationally complete
    configuration, or SIC, in \(\mathbb{C}^N\) is a set of projectors
    \(\Pi_j=|\psi_j\rangle\langle\psi_j|\), \(1 \le j \le N^2\), with
    \(\|\psi_j\|=1\), satisfying
    \[
      \operatorname{tr}(\Pi_j\Pi_k)
      =
      |\langle \psi_j|\psi_k\rangle|^2
      =
      \frac{N\delta_{jk}+1}{N+1},
      \qquad 1 \le j,k \le N^2 .
    \]
    The corresponding SIC-POVM, or symmetric informationally complete positive
    operator-valued measure, is the set of effects
    \[
      E_j=\frac{1}{N}\Pi_j .
    \]
    The SIC equations imply
    \[
      \sum_{j=1}^{N^2}E_j=I_N .
    \]
    Here \(I_N\) is the identity operator on \(\mathbb{C}^N\). The SIC
    equations also imply that the projectors \(\Pi_j\) form a basis for the
    real vector space of Hermitian operators on \(\mathbb{C}^N\), so the POVM
    is informationally complete.
  </def>

  <known>
    Horodecki, Rudnicki, and Zyczkowski, "Five Open Problems in Quantum
    Information Theory", Problem 1, state this as constructing SIC-POVMs in an
    infinite sequence of dimensions \(N_1,N_2,N_3,\ldots\), using the overlap
    relation in the claim. They recall Zauner's 1999 conjecture in the
    Weyl-Heisenberg form: for every dimension \(N\), there should exist a
    fiducial vector whose Weyl-Heisenberg orbit gives the \(N^2\) SIC vectors.
    As external context, Open Quantum Problems separates stronger all-dimensions
    variants: arbitrary SIC-POVMs in every dimension, group-covariant
    Weyl-Heisenberg SIC-POVMs in every dimension, and a Zauner
    Clifford-eigenvector formulation. The claim in this entry is only the weaker
    infinite-sequence claim from Horodecki, Rudnicki, and Zyczkowski; it imposes
    neither all dimensions nor Weyl-Heisenberg covariance.

    Appleby, Flammia, and Kopp, "A Constructive Approach to Zauner's Conjecture
    via the Stark Conjectures", arXiv:2501.03970v2 (2025), give a conditional
    construction of Weyl-Heisenberg SICs in all dimensions \(d\gt 3\), assuming
    an order-one abelian Stark conjecture for real quadratic fields and a
    special-value identity for the Shintani-Faddeev modular cocycle. Joka,
    "Symmetric Informationally Complete Positive Operator Valued Measure and
    Zauner conjecture", arXiv:2601.13475v4 (submitted 2026-01-20, revised
    2026-03-07), is a recent preprint whose abstract claims arbitrary SIC-POVMs
    in every finite dimension. If accepted and verified, that preprint would
    imply the weaker infinite-sequence claim here; this entry does not mark the
    problem solved from that preprint abstract.
  </known>

  <refs>
    <ref>P. Horodecki, L. Rudnicki, and K. Zyczkowski, Five Open Problems in Quantum Information Theory, PRX Quantum 3, 010101 (2022), DOI: 10.1103/PRXQuantum.3.010101, arXiv:2002.03233, Problem 1, used for the infinite-sequence SIC-POVM claim.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is related to the Problem 23 listing, SIC POVMs and Zauner's Conjecture, in Quantum communication.</ref>
    <ref>Open Quantum Problems, SIC POVMs and Zauner's Conjecture, Problem 23, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/sic-povms-and-zauners-conjecture, used for stronger all-dimensions, Weyl-Heisenberg covariant, and Zauner Clifford-eigenvector variants.</ref>
    <ref>M. Appleby, S. T. Flammia, and G. S. Kopp, A Constructive Approach to Zauner's Conjecture via the Stark Conjectures, arXiv:2501.03970v2 (2025), used for conditional all-dimensions progress under Stark and Shintani-Faddeev special-value conjectures.</ref>
    <ref>S. Joka, Symmetric Informationally Complete Positive Operator Valued Measure and Zauner conjecture, arXiv:2601.13475v4 (2026), used only as a recent preprint claim of arbitrary SIC-POVM existence in every finite dimension, not as accepted solved status.</ref>
  </refs>

  <ask>
    Construct a strictly increasing infinite sequence \(N_1,N_2,N_3,\ldots\)
    and vectors satisfying the equations in the claim. To disprove the claim,
    prove that the set
    \[
      \{N \ge 2 : \text{a rank-one SIC exists in } \mathbb{C}^N\}
    \]
    is finite. A proof may use Weyl-Heisenberg covariance, but the claim only
    requires rank-one SIC-POVMs, not necessarily covariant ones.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
