# QTD008 Equivalence of ETO and WETO Resource Theories for High-Outlier Quasi-Uniform Gibbs Weights

```xml
<problem id="QTD008">
  <status>open</status>
  <name>Equivalence of ETO and WETO Resource Theories for High-Outlier Quasi-Uniform Gibbs Weights</name>
  <keys>elementary thermal operations ETO weak elementary thermal operations WETO strong elementary thermal operations thermal operations TO resource theory of thermal operations RT_TO RT_ETO RT_WETO d-swap d-swaps beta-swap beta-swaps T^d-transform T^d transform two-level thermal operation two-level thermal swap quasi-uniform distribution quasi-uniform Gibbs weights quasi-uniform equilibrium distribution high-outlier distribution low-outlier distribution single-outlier Gibbs weight Gibbs weights Gibbs distribution Gibbs state equilibrium distribution thermo-majorization d-majorization majorization Hardy Littlewood Polya Muirhead Birkhoff-von Neumann theorem universality of elementary thermal operations weak universality strong universality random variable conditioning randomized protocol deterministic protocol Hack Mendl Hack-Mendl Pedro Hack Christian Mendl Christian B Mendl Conjecture 1 ETO WETO equivalence resource theory equivalence energy levels finite-dimensional quantum thermodynamics Lostaglio Alhambra Perry elementary thermal operations resource theory of quantum thermodynamics resource theory of athermality quantum thermodynamics polytope of thermal operations ETO polytope WETO polytope TO polytope d-stochastic matrix stochastic matrix Journal of Physics A quasi uniform Gibbs weight equivalence conjecture eto weto quasi uniform equivalence</keys>

  <claim>
    Fix an integer \(n\ge4\), \(\Omega:=\{1,\ldots,n\}\), and a Gibbs weight
    vector \(d\in\mathcal P_\Omega\), \(d\gt0\), whose non-increasing
    rearrangement has the high-outlier quasi-uniform shape
    \[
      d^\downarrow=(d_0,d_1,\ldots,d_1),\qquad d_0\gt d_1\gt0,
    \]
    i.e. one strictly largest weight and \(n-1\) equal, strictly smaller
    weights (def below).

    For every such \(n\) and \(d\),
    \[
      RT_{ETO}(d) = RT_{WETO}(d),
    \]
    where \(RT_{ETO}(d)\) and \(RT_{WETO}(d)\) are the resource theories of
    strong, resp. weak, elementary thermal operations at Gibbs weights \(d\)
    (the reachability preorders \(\preceq_d^{ETO}\), \(\preceq_d^{WETO}\) on
    \(\mathcal P_\Omega\) defined below).

    Since \(RT_{WETO}(d)\subseteq RT_{ETO}(d)\) holds for every \(0\lt
    d\in\mathcal P_\Omega\) by construction (known below), the open content
    of the claim is exactly the reverse containment
    \(RT_{ETO}(d)\subseteq RT_{WETO}(d)\): every \(q\in\mathcal P_\Omega\)
    reachable from \(p\in\mathcal P_\Omega\) by strong elementary thermal
    operations at \(d\) is already reachable from \(p\) by weak elementary
    thermal operations at \(d\).

    This is the unresolved case of Hack and Mendl's Conjecture 1
    ("Equivalence ETO and WETO resource theories"): the complementary
    low-outlier quasi-uniform shape, the necessity direction of the
    conjecture, and the case \(n\le3\) are already theorems of the same
    source (known below).
  </claim>

  <def>
    Let \(n\ge2\) be an integer, \(\Omega:=\{1,\ldots,n\}\), and
    \(\mathcal P_\Omega\) the probability simplex on \(\Omega\). A Gibbs
    weight vector is a \(d\in\mathcal P_\Omega\) with \(d_i\gt0\) for every
    \(i\in\Omega\) (physically \(d_i=e^{-\beta E_i}/Z\) for a Hamiltonian
    with energies \(E_i\) at inverse temperature \(\beta\) and partition
    function \(Z\), but every definition below makes sense for arbitrary
    strictly positive \(d\in\mathcal P_\Omega\)). Write \(d^\downarrow\) for
    the non-increasing rearrangement of \(d\).

    A matrix \(M\in\mathbb R^{n\times n}\) is \(d\)-stochastic if every
    column of \(M\) sums to \(1\) and \(Md=d\). \(RT_{TO}(d)\) denotes the
    reachability relation \(\preceq_d^{TO}\) on \(\mathcal P_\Omega\), where
    \(q\preceq_d^{TO}p\) iff \(q=Mp\) for some \(d\)-stochastic \(M\) (Hack
    and Mendl, Definition 1: resource theory of thermal operations; used
    below only for provenance, not itself part of the claim above).

    By Hack and Mendl's Lemma 1 (equivalence under permutation of \(d\)),
    every containment or equality among the reachability relations defined
    here at \(d\) is equivalent to the same statement at \(d^\downarrow\);
    the claim above and the rest of this def accordingly fix
    \(d=d^\downarrow\), i.e. \(d_1\ge d_2\ge\cdots\ge d_n\gt0\), without loss
    of generality.

    d-swap. For \(1\le i\lt j\le n\), set \(\gamma_{i,j}:=d_j/d_i\in(0,1]\)
    and let \(P^d(i,j)\in\mathbb R^{n\times n}\) be the matrix equal to the
    identity outside rows and columns \(\{i,j\}\) and equal, on rows and
    columns \(\{i,j\}\) (in that order), to
    \[
      \begin{pmatrix}1-\gamma_{i,j} &amp; 1\\ \gamma_{i,j} &amp; 0\end{pmatrix}.
    \]
    \(P^d(i,j)\) is \(d\)-stochastic (Hack and Mendl, Definition 2).

    T^d-transform. For \(1\le i\lt j\le n\) and \(\lambda\in[0,1]\),
    \[
      T_\lambda^d(i,j):=(1-\lambda)\,\mathbb 1_n+\lambda\,P^d(i,j),
    \]
    with \(\mathbb 1_n\) the \(n\times n\) identity. Every
    \(T_\lambda^d(i,j)\) is \(d\)-stochastic and acts non-trivially on at
    most the two levels \(i,j\) (Hack and Mendl, Definition 2).

    Weak elementary thermal operations (WETO). For \(p,q\in\mathcal
    P_\Omega\), write \(q\preceq_d^{WETO}p\) if there is an integer \(m\ge0\)
    and triples \((\lambda_k,i_k,j_k)_{k=1}^m\) with \(\lambda_k\in[0,1]\),
    \(1\le i_k\lt j_k\le n\), such that
    \[
      q = T_{\lambda_m}^d(i_m,j_m)\cdots T_{\lambda_1}^d(i_1,j_1)\,p
    \]
    (\(m=0\) reads \(q=p\)). \(RT_{WETO}(d)\) denotes the relation
    \(\preceq_d^{WETO}\) on \(\mathcal P_\Omega\) (Hack and Mendl, Definition
    3).

    Strong elementary thermal operations (ETO). For \(p,q\in\mathcal
    P_\Omega\), write \(q\preceq_d^{ETO}p\) if there is an integer \(r\ge1\),
    convex weights \(\mu_1,\ldots,\mu_r\ge0\) with \(\sum_{k=1}^r\mu_k=1\),
    and, for each \(k=1,\ldots,r\), an integer \(\ell_k\ge0\) and a
    length-\(\ell_k\) product of \(d\)-swaps
    \(M_k:=P^d(i_{k,\ell_k},j_{k,\ell_k})\cdots P^d(i_{k,1},j_{k,1})\) (the
    empty product, \(\ell_k=0\), being \(\mathbb 1_n\)), such that
    \[
      q = \Bigl(\sum_{k=1}^r\mu_k M_k\Bigr) p.
    \]
    \(RT_{ETO}(d)\) denotes the relation \(\preceq_d^{ETO}\) on
    \(\mathcal P_\Omega\) (Hack and Mendl, Definition 4, shown there to be
    equivalent to the original elementary-thermal-operations resource
    theory of Lostaglio, Alhambra and Perry, Definition 1).

    \(RT_{ETO}(d)=RT_{WETO}(d)\) means \(\preceq_d^{ETO}\) and
    \(\preceq_d^{WETO}\) are the same relation on \(\mathcal P_\Omega\), i.e.
    \(q\preceq_d^{ETO}p\Leftrightarrow q\preceq_d^{WETO}p\) for every
    \(p,q\in\mathcal P_\Omega\). Since every \(T_\lambda^d(i,j)\) is itself
    an \(r\le2\) instance of the convex combination defining
    \(\preceq_d^{ETO}\), \(q\preceq_d^{WETO}p\Rightarrow
    q\preceq_d^{ETO}p\) always, so \(RT_{WETO}(d)\subseteq RT_{ETO}(d)\) for
    every \(0\lt d\in\mathcal P_\Omega\).

    Quasi-uniform distribution (Hack and Mendl, Definition 9). \(0\lt
    d\in\mathcal P_\Omega\) is quasi-uniform if \(d\) takes at most two
    distinct values among its \(n\) entries. Equivalently, \(d^\downarrow\)
    has one of two shapes:
    low-outlier: \(d^\downarrow=(d_0,\ldots,d_0,d_1)\) with \(d_0\ge d_1\)
    (\(n-1\) equal entries and one entry no larger than them), or
    high-outlier: \(d^\downarrow=(d_0,d_1,\ldots,d_1)\) with \(d_0\gt d_1\)
    (one entry strictly larger than the remaining \(n-1\), which are equal).
    These correspond respectively to cases (a) and (b) of Hack and Mendl's
    Figure 14. The claim above concerns the high-outlier shape.
  </def>

  <known>
    Hack and Mendl, "Universality and classification of elementary thermal
    operations," J. Phys. A: Math. Theor. 58, 315302 (2025) [arXiv:2312.11223],
    Section 8.1 (pp. 46-47), pose Conjecture 1 ("Equivalence ETO and WETO
    resource theories"): for every \(0\lt d\in\mathcal P_\Omega\),
    \(RT_{ETO}(d)=RT_{WETO}(d)\) iff \(d\) is quasi-uniform (def above). The
    same paper proves the following pieces of Conjecture 1, all for
    \(RT_{ETO}\), \(RT_{WETO}\) exactly as defined above:

    (1) Necessity, every \(n\). Proposition 4 ("Difference ETO and WETO
    resource theories") proves \(RT_{ETO}(d)=RT_{WETO}(d)\) only if \(d\) is
    quasi-uniform, for every \(n=|\Omega|\) and every \(0\lt d\in\mathcal
    P_\Omega\).

    (2) Sufficiency, low-outlier shape, every \(n\). Corollary 2 proves that
    if \(d^\downarrow=(d_0,\ldots,d_0,d_1)\) with \(d_0\ge d_1\) (low-outlier
    quasi-uniform weights), then \(RT_{ETO}(d)=RT_{WETO}(d)\), for every
    \(n\). Corollary 2 follows because Theorem 5 proves
    \(RT_{TO}(d)=RT_{ETO}(d)\) and Theorem 6 proves
    \(RT_{TO}(d)=RT_{WETO}(d)\) under the same hypothesis
    \(d^\downarrow=(d_0,\ldots,d_0,d_1)\); both then equal \(RT_{TO}(d)\) and
    hence each other.

    (3) Both directions, high-outlier shape, \(n=3\). Proposition 5
    ("Equivalence ETO and WETO resource theories for \(|\Omega|=3\)") proves
    that for \(n=3\), \(RT_{ETO}(d)=RT_{WETO}(d)\) iff \(d\) is
    quasi-uniform, covering the high-outlier shape
    (\(d^\downarrow=(d_0,d_1,d_1)\), \(d_0\gt d_1\)) by an explicit
    case analysis on the ETO polytope (the proof's cases (A)-(C), with
    Figures 11-13) that is not available from Corollary 2, since (2) proves
    only the low-outlier shape.

    (4) Explicit open status. The caption of Figure 14 states (verbatim,
    modulo restoring the inequality symbol from a font-substitution artifact
    in the PDF text extraction; "(b)" there is Hack and Mendl's own label for
    the high-outlier shape, matching the def above): "The diagram looks the same
    for any \(|\Omega|\gt3\) except for the fact we do not know whether weak
    elementary thermal operations are equivalent to strong elementary
    thermal operations in (b) in that scenario." Section 10 (Conclusion)
    states (verbatim): "The most immediate question is whether Conjecture 1
    holds." No proof, disproof, or further partial result for the
    high-outlier shape at \(n\ge4\) appears anywhere else in Hack and Mendl
    (2025); this is the only unresolved piece of Conjecture 1 and is exactly
    the claim above. This is verified against the sole arXiv version of
    Hack and Mendl (2025), v1, submitted 18 December 2023 with no later
    version posted, whose DOI and journal reference match those recorded by
    the publisher for the same title and authors.

    (5) A logically distinct, already fully resolved question in the same
    paper is equality of the ETO and WETO polytopes themselves, \(P_{ETO}(d)
    =P_{WETO}(d)\) (equality of the sets of \(n\times n\) matrices
    \(\{M_k\}\), resp. \(\{T_{\lambda_m}^d(i_m,j_m)\cdots
    T_{\lambda_1}^d(i_1,j_1)\}\), used in the def of \(\preceq_d^{ETO}\),
    resp. \(\preceq_d^{WETO}\), above), as opposed to
    \(RT_{ETO}(d)=RT_{WETO}(d)\), equality of the coarser relations these
    matrix sets induce on \(\mathcal P_\Omega\). Theorem 10 proves
    \(P_{ETO}(d)=P_{WETO}(d)\) iff \(n=2\), for every \(0\lt d\in\mathcal
    P_\Omega\). This does not bear on the claim above: \(RT_{ETO}(d)=
    RT_{WETO}(d)\) is already known to hold strictly more often than
    \(P_{ETO}(d)=P_{WETO}(d)\) (e.g. under the low-outlier shape at every
    \(n\), by (2) above, where the polytopes can still differ), and
    Conjecture 1 is a statement purely about \(RT_{ETO}\), \(RT_{WETO}\).
  </known>

  <refs>
    <ref>Pedro Hack and Christian B. Mendl, "Universality and classification of elementary thermal operations," Journal of Physics A: Mathematical and Theoretical 58, 315302 (2025), DOI: 10.1088/1751-8121/adf26e, arXiv:2312.11223 [quant-ph] (submitted 18 December 2023, single version). Used for: Definition 1 (resource theory of thermal operations \(RT_{TO}(d)\)), Definition 2 (\(d\)-swap \(P^d(i,j)\) and \(T^d\)-transform \(T_\lambda^d(i,j)\)), Definition 3 (WETO resource theory \(RT_{WETO}(d)\)), Definition 4 (ETO resource theory \(RT_{ETO}(d)\), citing Lostaglio, Alhambra and Perry, "Elementary Thermal Operations," Quantum 2, 52 (2018), Definition 1, for equivalence with the original ETO resource theory), Definition 9 (quasi-uniform distribution), Lemma 1 (equivalence under permutation of \(d\), used to fix \(d=d^\downarrow\) without loss of generality), Theorem 5 and Theorem 6 (\(RT_{TO}(d)=RT_{ETO}(d)\), resp. \(RT_{TO}(d)=RT_{WETO}(d)\), iff \(d^\downarrow=(d_0,\ldots,d_0,d_1)\)), Corollary 2 (sufficiency for the low-outlier shape, every \(n\)), Proposition 4 (necessity, every \(n\)), Proposition 5 and its proof cases (A)-(C) with Figures 11-13 (both directions, \(n=3\)), Section 8.1 including Conjecture 1 and Figure 14 with caption (statement of the conjecture and its explicit open case), Theorem 10 (the distinct, fully resolved ETO/WETO polytope-equality question), and Section 10 Conclusion (restating Conjecture 1 as Hack and Mendl's most immediate open question).</ref>
  </refs>

  <ask>
    A proof must show, for every integer \(n\ge4\) and every \(d\in\mathcal
    P_\Omega\) (\(\Omega=\{1,\ldots,n\}\)) with \(d^\downarrow=(d_0,d_1,
    \ldots,d_1)\), \(d_0\gt d_1\gt0\), that \(RT_{ETO}(d)\subseteq
    RT_{WETO}(d)\) (the reverse containment; \(RT_{WETO}(d)\subseteq
    RT_{ETO}(d)\) already holds by def, so this containment alone gives
    \(RT_{ETO}(d)=RT_{WETO}(d)\)): for every \(p,q\in\mathcal P_\Omega\)
    with \(q\preceq_d^{ETO}p\), it must establish the existence of a finite
    sequence of \(T^d\)-transforms realizing \(q\) from \(p\), as in the def
    of \(\preceq_d^{WETO}\) above. A single argument covering every
    \(n\ge4\) and every such \(d\) is required; verifying finitely many
    \(n\), or restating the \(n=3\) case analysis of Proposition 5 without
    extending it to \(n\ge4\), is not a proof of the claim as quantified
    above.

    A disproof must exhibit some \(n\ge4\), some \(d\in\mathcal P_\Omega\)
    with \(d^\downarrow=(d_0,d_1,\ldots,d_1)\), \(d_0\gt d_1\gt0\), and some
    \(p,q\in\mathcal P_\Omega\) with \(q\preceq_d^{ETO}p\) but not
    \(q\preceq_d^{WETO}p\): a \(q\) achievable from \(p\) by some convex
    combination of products of \(d\)-swaps but by no finite product of
    \(T^d\)-transforms. Because the claim is universally quantified over
    \(n\) and \(d\) in the stated family, exhibiting one such instance fully
    disproves it.

    The following count as progress, not as a solution to the claim, unless
    combined with a proof or disproof removing the stated restriction: (i)
    resolving the claim only for a fixed \(n\) or a fixed proper subfamily
    of high-outlier \(d\) (e.g. only \(n=4\), or only \(\gamma:=d_1/d_0\)
    restricted to a proper sub-interval of \((0,1)\)); (ii) resolving the
    analogous question for the ETO and WETO polytopes \(P_{ETO}(d)\),
    \(P_{WETO}(d)\) rather than the resource theories \(RT_{ETO}(d)\),
    \(RT_{WETO}(d)\) fixed above (already answered by Theorem 10 in known,
    and logically distinct from the claim); (iii) numerical or asymptotic
    evidence, such as sampled \(p,q\) pairs or large-\(n\) approximations,
    that is not accompanied by either the full proof described above or a
    single explicit counterexample instance as described above.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
