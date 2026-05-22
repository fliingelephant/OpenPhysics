# QC013 Mutually Unbiased Bases

```xml
<problem id="QC013">
  <status>open</status>
  <name>Mutually Unbiased Bases</name>
  <keys>mutually unbiased bases mub mubs maximal number complete set complete system complete collection prime power dimension non-prime-power dimension composite dimension dimension six d=6 k=7 seven bases mub triplets mub quartets zauner conjecture complex hadamard matrices hadamard cubes inverse orthogonal cubes finite fields galois fields state determination quantum tomography complementary observables quantum cryptography mean king problem open quantum problems oqp quantum communication</keys>

  <claim>
    For every integer \(d\ge 2\), determine the exact value
    \[
      M(d)
      :=
      \max\{m:\text{there exist }m\text{ pairwise mutually unbiased
      orthonormal bases of }\mathbb{C}^d\}.
    \]
  </claim>

  <def>
    An orthonormal basis of \(\mathbb{C}^d\) is written
    \[
      B_a=(e_{a,1},\ldots,e_{a,d}),
      \qquad
      \langle e_{a,j},e_{a,\ell}\rangle=\delta_{j\ell}.
    \]
    Two orthonormal bases \(B_a\) and \(B_b\) are unbiased if
    \[
      |\langle e_{a,j},e_{b,\ell}\rangle|^2=\frac{1}{d}
      \qquad
      \text{for all }1\le j,\ell\le d.
    \]
    A family \(B_1,\ldots,B_m\) is a family of mutually unbiased bases, or
    MUBs, if every two distinct bases in the family are unbiased. Reordering
    bases, reordering vectors within bases, and multiplying basis vectors by
    phase factors do not change the represented MUB family for purposes of
    \(M(d)\).

    A complete set of MUBs in dimension \(d\) means a family of \(d+1\) MUBs.
    This terminology uses the general upper bound \(M(d)\le d+1\), not an
    assertion that such a family exists in every dimension.
  </def>

  <known>
    Open Quantum Problems, Problem 13, "Mutually unbiased bases", asks for the
    maximal number \(k\) of pairwise mutually unbiased orthonormal bases in a
    \(d\)-dimensional Hilbert space. The Open Quantum Problems list records
    Problem 13 in the Quantum communication category.

    Wootters and Fields, "Optimal state-determination by mutually unbiased
    measurements", Annals of Physics 191(2), 363-381 (1989), prove the upper
    bound \(M(d)\le d+1\) and construct complete sets when \(d\) is a prime
    power. Pittenger and Rubin, "Mutually unbiased bases, generalized spin
    matrices and separability", Linear Algebra and its Applications 390,
    255-278 (2004), and Klappenecker and Roetteler, "Constructions of Mutually
    Unbiased Bases", LNCS 2948, 137-144 (2004), give later constructive
    prime-power proofs. Thus \(M(d)=d+1\) is proved for every prime-power
    dimension \(d\).

    Weiner, "A gap for the maximum number of mutually unbiased bases",
    Proceedings of the American Mathematical Society 141, 1963-1969 (2013),
    proves that \(M(d)\ne d\) for \(d\gt 1\). Hence every finite dimension has
    the dichotomy \(M(d)=d+1\) or \(M(d)\le d-1\). In particular, \(M(6)=6\)
    is not a possible final value.

    The OQP page emphasizes the first unsettled complete-set case \(d=6\): it
    asks whether there exist \(k=7\) MUBs in \(\mathbb{C}^6\), and records
    that sets of three MUBs are known. More recent literature is consistent
    with keeping this entry open. Jaming, Matolcsi, Mora, Szollosi, and Weiner,
    "A generalized Pauli problem and an infinite family of MUB-triplets in
    dimension 6", Journal of Physics A 42, 245305 (2009), construct infinite
    families of MUB triplets in dimension \(6\) and prove that the standard
    basis together with any member of the Fourier family \(F(a,b)\) cannot be
    extended to a MUB quartet. Brierley and Weigert, "Maximal Sets of Mutually
    Unbiased Quantum States in Dimension Six", Physical Review A 78, 042312
    (2008), provide numerical evidence against complete sets in dimension
    \(6\). Matolcsi, Matszangosz, Varga, and Weiner, "Triplets of mutually
    unbiased bases", Journal of Algebraic Combinatorics 63, 26 (2026), state
    that \(d=6\) remains the lowest dimension where existence of a complete
    \(d+1\) MUB system is not known, and that Zauner's conjecture is
    \(M(6)=3\). Deciding \(M(6)\), proving or disproving \(M(6)=7\), or
    proving or disproving the stronger \(M(6)=3\) conjecture would be major
    progress on this entry, but would not by itself determine \(M(d)\) for all
    finite dimensions.

    Durt, Englert, Bengtsson, and Zyczkowski, "On mutually unbiased bases",
    International Journal of Quantum Information 8(04), 535-640 (2010), review
    the finite-dimensional MUB problem, the prime-power construction, the
    connection with complex Hadamard matrices, and applications in quantum
    information protocols.
  </known>

  <refs>
    <ref>Open Quantum Problems, Mutually unbiased bases, Problem 13, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/mutually-unbiased-bases, used for the problem statement, the prime-power known case, and the highlighted \(d=6,k=7\) special case.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 13, Mutually unbiased bases, in Quantum communication.</ref>
    <ref>I. D. Ivanovic, Geometrical description of quantal state determination, Journal of Physics A: Mathematical and General 14(12), 3241-3245 (1981), DOI: 10.1088/0305-4470/14/12/019, used for the state-determination provenance and early prime-dimensional MUB construction context.</ref>
    <ref>W. K. Wootters and B. D. Fields, Optimal state-determination by mutually unbiased measurements, Annals of Physics 191(2), 363-381 (1989), DOI: 10.1016/0003-4916(89)90322-9, used for the MUB definition, \(M(d)\le d+1\), and prime-power complete-set construction.</ref>
    <ref>A. O. Pittenger and M. H. Rubin, Mutually unbiased bases, generalized spin matrices and separability, Linear Algebra and its Applications 390, 255-278 (2004), DOI: 10.1016/j.laa.2004.04.025, arXiv:quant-ph/0308142, used for a generalized-spin-matrix prime-power construction.</ref>
    <ref>A. Klappenecker and M. Roetteler, Constructions of Mutually Unbiased Bases, in Finite Fields and Applications: 7th International Conference, Lecture Notes in Computer Science 2948, 137-144 (Springer, 2004), DOI: 10.1007/978-3-540-24633-6_10, arXiv:quant-ph/0309120, used for the upper bound, prime-power construction, and arbitrary-dimension open-problem formulation.</ref>
    <ref>M. Weiner, A gap for the maximum number of mutually unbiased bases, Proceedings of the American Mathematical Society 141(6), 1963-1969 (2013), DOI: 10.1090/S0002-9939-2013-11487-5, arXiv:0902.0635, used for the gap theorem \(M(d)\ne d\).</ref>
    <ref>S. Brierley and S. Weigert, Maximal Sets of Mutually Unbiased Quantum States in Dimension Six, Physical Review A 78, 042312 (2008), DOI: 10.1103/PhysRevA.78.042312, arXiv:0808.1614, used for numerical evidence concerning complete sets in dimension \(6\).</ref>
    <ref>P. Jaming, M. Matolcsi, P. Mora, F. Szollosi, and M. Weiner, A generalized Pauli problem and an infinite family of MUB-triplets in dimension 6, Journal of Physics A: Mathematical and Theoretical 42, 245305 (2009), DOI: 10.1088/1751-8113/42/24/245305, arXiv:0902.0882, used for dimension-\(6\) MUB triplets and a non-extension theorem for the Fourier family.</ref>
    <ref>T. Durt, B.-G. Englert, I. Bengtsson, and K. Zyczkowski, On mutually unbiased bases, International Journal of Quantum Information 8(04), 535-640 (2010), DOI: 10.1142/S0219749910006502, arXiv:1004.3348, used for review context on prime-power constructions, non-prime-power open cases, complex Hadamard matrices, and quantum-information applications.</ref>
    <ref>P. Horodecki, L. Rudnicki, and K. Zyczkowski, Five Open Problems in Quantum Information Theory, PRX Quantum 3, 010101 (2022), Problem 2, DOI: 10.1103/PRXQuantum.3.010101, arXiv:2002.03233, used for the narrower \(d=6\) MUB problem that the repository treats as covered by this broader entry.</ref>
    <ref>M. Matolcsi, A. K. Matszangosz, D. Varga, and M. Weiner, Triplets of mutually unbiased bases, Journal of Algebraic Combinatorics 63, article 26 (2026), DOI: 10.1007/s10801-026-01506-x, arXiv:2503.14752, used for the 2026 status of the \(d=6\) complete-set problem and Zauner's \(M(6)=3\) conjecture.</ref>
  </refs>

  <ask>
    Give a theorem that determines \(M(d)\) exactly for every integer
    \(d\ge 2\), together with proofs of both the lower bound constructions and
    the matching upper bounds in every dimension. An answer that only asserts
    feasibility or infeasibility for each fixed \(d\) without yielding
    constructions, nonexistence certificates, or bound explanations is not a
    determination of \(M(d)\).

    A solution restricted to one dimension, one infinite family of dimensions,
    one class of MUB constructions, one class of complex Hadamard matrices, or
    one side of an upper or lower bound is progress unless it implies exact
    values for all \(d\ge 2\). In particular, constructing four or more MUBs in
    \(\mathbb{C}^6\), constructing seven MUBs in \(\mathbb{C}^6\), proving
    \(M(6)\lt 7\), or proving \(M(6)=3\) would be progress on the canonical
    claim, not a full solution by itself.

    A claimed disproof should identify the exact proposed classification,
    formula, algorithm, or auxiliary conjecture being refuted. A counterexample
    to a proposed value of \(M(d)\) is a disproof of that proposed solution
    and should be reported as gap or progress unless it also leads to a
    complete determination of \(M(d)\) for all finite dimensions.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
