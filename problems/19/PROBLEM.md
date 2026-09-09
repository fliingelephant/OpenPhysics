# 19 Genuine multipartite nonlocality of pure states

```xml
<problem id="19">
  <status>open</status>
  <kind>proof</kind>
  <field>QIT</field>
  <name>Genuine multipartite nonlocality of pure states</name>
  <keys>oqp problem 32c gisin conjecture genuine multipartite nonlocality gmnl genuine n-party nonlocality genuine multipartite entanglement gme pure states bell inequalities biseparable nonsignalling hybrid nonsignaling hybrid ns2 svetlichny bancal barrett gisin pironio popescu rohrlich generic quantum nonlocality hardy test yu oh three qubit pure states symmetric multiqubit qubit stabilizer subspaces self testing balanzo juando coladangelo supic makuta augusiak quantum foundations</keys>

  <claim>
    For every integer \(n\ge 3\), every tuple of finite local dimensions
    \(d_1,\ldots,d_n\), and every genuinely multipartite entangled pure state
    \[
      |\psi\rangle\in
      \mathbb C^{d_1}\otimes\cdots\otimes\mathbb C^{d_n},
    \]
    there exist finite local input sets \(X_i\), finite local output sets
    \(A_i\), and local POVMs
    \(\{E^{(i),x_i}_{a_i}\}_{a_i\in A_i}\) on \(\mathbb C^{d_i}\) such that
    the behavior
    \[
      p(a_1,\ldots,a_n|x_1,\ldots,x_n)
      =
      \langle\psi|
      \bigotimes_{i=1}^n E^{(i),x_i}_{a_i}
      |\psi\rangle
    \]
    is outside the biseparable nonsignalling hybrid polytope
    \(\mathcal B_{\mathrm{NS\text{-}bisep}}\) defined below. Equivalently,
    \(p\) strictly violates a finite Bell inequality that is valid for every
    nonsignalling behavior that is local across at least one bipartition.
  </claim>

  <def>
    For a positive integer \(r\), write \([r]=\{1,\ldots,r\}\). In a finite
    \(n\)-party Bell scenario, party \(i\) receives an input \(x_i\in X_i\) and
    returns an output \(a_i\in A_i\), where each \(X_i\) and \(A_i\) is finite.
    A behavior is an array
    \[
      p=\{p(a|x):a\in A_1\times\cdots\times A_n,\,
      x\in X_1\times\cdots\times X_n\}
    \]
    of nonnegative probabilities normalized for every input tuple \(x\).

    A pure state \(|\psi\rangle\) is genuinely multipartite entangled, or GME,
    when for every nonempty proper subset \(S\subset[n]\) it is not a product
    vector across the bipartition \(S|\bar S\). Equivalently, for every such
    \(S\), the reduced density operator
    \(\operatorname{Tr}_{\bar S}|\psi\rangle\langle\psi|\) is mixed.
    This entry is about GME pure states; biseparable pure states are outside
    the claim.

    For a nonempty proper subset \(S\subset[n]\), write
    \(a_S=(a_i)_{i\in S}\) and \(x_S=(x_i)_{i\in S}\). A behavior is
    nonsignalling within the parties of \(S\) if every marginal on a subfamily
    \(T\subset S\) depends only on \(x_T\), not on the inputs
    \(x_{S\setminus T}\). The same convention applies to \(\bar S\).

    A behavior is nonsignalling-local across the bipartition \(S|\bar S\) if
    it has a decomposition
    \[
      p(a|x)=
      \sum_{\lambda} \mu_\lambda\,
      q^\lambda_S(a_S|x_S)\,
      q^\lambda_{\bar S}(a_{\bar S}|x_{\bar S}),
    \]
    where \(\mu_\lambda\ge 0\), \(\sum_\lambda\mu_\lambda=1\), and
    \(q^\lambda_S\) and \(q^\lambda_{\bar S}\) are arbitrary normalized
    nonsignalling behaviors for the grouped parties in \(S\) and \(\bar S\).
    The shared variable \(\lambda\) may be taken finite because the scenario is
    finite.

    The biseparable nonsignalling hybrid polytope is
    \[
      \mathcal B_{\mathrm{NS\text{-}bisep}}
      =
      \operatorname{conv}\!
      \bigcup_{\emptyset\ne S\subsetneq[n]}
      \mathcal B_{S|\bar S},
    \]
    where \(\mathcal B_{S|\bar S}\) is the set of behaviors
    nonsignalling-local across \(S|\bar S\). The union may be taken over one
    representative of each unordered bipartition without changing the set.
    This is the nonsignalling-bipartition convention used in modern GMNL work;
    it differs from ordinary multipartite locality, where every hidden-variable
    component factorizes over all individual parties.

    A real Bell functional is
    \[
      \ell(p)=\sum_{a,x} c_{a,x}p(a|x).
    \]
    A GMNL Bell inequality in this entry is an inequality
    \(\ell(p)\le \beta_{\mathrm{NS\text{-}bisep}}\), where
    \[
      \beta_{\mathrm{NS\text{-}bisep}}
      =
      \max_{b\in\mathcal B_{\mathrm{NS\text{-}bisep}}}\ell(b).
    \]
    A behavior is genuinely multipartite nonlocal, or GMNL, when it violates
    such an inequality, equivalently when it is not in
    \(\mathcal B_{\mathrm{NS\text{-}bisep}}\). Since the scenario is finite,
    the equivalence follows from separation of a point from a closed convex
    polytope. A violation of a fully local Bell inequality proves ordinary
    multipartite Bell nonlocality but is not enough for this entry unless it
    also separates the behavior from
    \(\mathcal B_{\mathrm{NS\text{-}bisep}}\).
  </def>

  <known>
    Open Quantum Problems, Problem 32, "Bell inequalities: many questions, a
    few answers", lists subquestion C: find genuine \(n\)-party inequalities
    violated by all \(n\)-party pure entangled states. The Open Quantum
    Problems index lists Problem 32 in the Quantum foundations category, with
    Nicolas Gisin as contact and last progress dated 2016/12/01. Gisin, "Bell
    inequalities: many questions, a few answers", in Essays in Honour of Abner
    Shimony, Springer (2009), discusses an ordinary multipartite Bell
    nonlocality route using bipartite projections and asks whether a single
    inequality can replace the resulting family. Svetlichny-type and
    nonsignalling-bipartition notions of genuine multipartite nonlocality are
    separate from that original ordinary-nonlocality discussion.

    This entry is therefore not a literal transcription of Gisin's single
    ordinary-Bell-inequality question. It records a modern strengthening
    motivated by OQP 32C and by later GMNL literature: whether every GME pure
    state can generate behavior outside the nonsignalling-bipartition
    biseparable polytope, with the Bell scenario and separating inequality
    allowed to depend on the state. A single state-independent inequality or
    explicit universal family for a specified domain would be stronger
    progress on this entry and would also address the closer Gisin-style
    reading when the relevant inequality notion matches the source question.

    Gisin, "Bell inequalities: many questions, a few answers",
    arXiv:quant-ph/0702021, is the source named by Open Quantum Problems for
    Problem 32 and includes the same subquestion in a list of fundamental Bell
    inequality questions. Gisin's formulation did not fix one of the later
    inequivalent definitions of genuine multipartite nonlocality. The claim in
    this entry uses the nonsignalling-bipartition hybrid set because it matches
    the operational GMNL convention developed after Svetlichny's original
    tripartite criterion.

    Popescu and Rohrlich, "Generic quantum nonlocality", Physics Letters A
    166, 293-297 (1992), argued that every pure entangled multiparticle state
    violates some Bell inequality. Gachechiladze and Guhne, "Completing the
    proof of 'Generic quantum nonlocality'", Physics Letters A 381, 1281-1285
    (2017), identified a gap in the Popescu-Rohrlich proof and supplied a
    construction completing the ordinary multipartite Bell-nonlocality result.
    Yu, Chen, Zhang, Lai, and Oh, "All entangled pure states violate a single
    Bell's inequality", Physical Review Letters 109, 120402 (2012), also prove
    an ordinary pure-state Bell-nonlocality theorem. These results do not settle
    the present claim because a Bell violation outside the fully local set can
    still lie inside the larger biseparable nonsignalling hybrid polytope.

    Svetlichny, "Distinguishing three-body from two-body nonseparability by a
    Bell-type inequality", Physical Review D 35, 3066-3069 (1987), introduced
    a tripartite Bell-type test for genuine three-body nonseparability. Bancal,
    Barrett, Gisin, and Pironio, "The definition of multipartite nonlocality",
    Physical Review A 88, 014102 (2013), compared Svetlichny's notion with
    nonsignalling-bipartition definitions of \(n\)-way nonlocality and reported
    numerical evidence that all three-way entangled pure quantum states can
    produce three-way nonlocal correlations. Bancal, Brunner, Gisin, and Liang,
    "Detecting Genuine Multipartite Quantum Nonlocality", Physical Review
    Letters 106, 020405 (2011), constructed GMNL Bell inequalities for
    arbitrary numbers of parties and arbitrary local dimensions, but not a
    proof that their family or any other family detects every GME pure state.

    Yu and Oh, "Tripartite entangled pure states are tripartite nonlocal",
    arXiv:1306.5330 (2013), prove using a Hardy-type test that every
    tripartite entangled pure state, regardless of local dimensions, is genuine
    tripartite nonlocal against nonsignalling local hybrid models. Thus the
    \(n=3\), arbitrary-local-dimension case of the modern equivalence
    formulation is known. This does not solve arbitrary \(n\), nor does it give
    a single fixed \(n=3\) Bell inequality violated by all tripartite GME pure
    states.

    Curchod, Almeida, and Acin, "A versatile construction of Bell inequalities
    for the multipartite scenario", New Journal of Physics 21, 023016 (2019),
    give Bell inequality families witnessing several strengths of multipartite
    nonlocality. They analytically prove GMNL for a large class of GME pure
    states for arbitrary \(n\), including some states close to product states,
    and give numerical evidence that one family is violated by all GME pure
    states of three and four qubits. This is substantial progress but not an
    all-\(n\), all-dimension proof.

    Stachura, Makuta, and Augusiak, "Single Bell inequality to detect genuine
    nonlocality in three-qubit genuinely entangled states", New Journal of
    Physics 26, 093029 (2024), prove that one improved Bell inequality detects
    GMNL in every three-qubit GME pure state. This gives a single-inequality
    result for the three-qubit case, stronger than state-dependent
    three-qubit detection, but it does not cover higher local dimensions or
    arbitrary numbers of parties.

    Wojcik, Bruzda, Stachura, and Augusiak, "Equivalence of Genuine
    Multipartite Entanglement and Nonlocality of Nearly Symmetric Multiqubit
    Pure States", arXiv:2510.13296 (2025), prove GMNL for highly symmetric
    GME multipartite qubit pure states by combining the Stachura-Makuta-Augusiak
    inequality with Hardy's paradox and a canonical pure-state decomposition.
    This advances the symmetric multiqubit direction but leaves nonsymmetric
    multiqubit states and higher-dimensional systems outside its scope.

    Makuta and Augusiak, "All genuinely entangled stabilizer subspaces are
    multipartite fully nonlocal", npj Quantum Information 11, 144 (2025),
    establish that every genuinely entangled qubit stabilizer subspace is
    multipartite fully nonlocal, and hence prove an equivalence between GME and
    GMNL for a broad qubit stabilizer-formalism class including mixed states
    supported on such subspaces. They also treat qudit graph-state cases. This
    is stronger than the present claim on those classes, but it does not imply
    the claim for arbitrary finite-dimensional pure states.

    Zwerger, Dur, Bancal, and Sekatski, "Device-independent detection of
    genuine multipartite entanglement for all pure states", Physical Review
    Letters 122, 060502 (2019), show that GME of all finite-dimensional
    multipartite pure states can be detected device-independently by using
    bipartite Bell inequalities on states deterministically generated by local
    operations from the initial state. This is a device-independent GME
    detection theorem, not a proof that the original single-copy \(n\)-party
    behavior is outside \(\mathcal B_{\mathrm{NS\text{-}bisep}}\).

    Balanzo-Juando, Coladangelo, Augusiak, Acin, and Supic, "All pure
    multipartite entangled states of qubits can be self-tested", Nature
    Communications 17, 4463 (2026), prove that every pure \(n\)-qubit
    entangled state has standard Bell-scenario correlations that self-test it,
    up to local isometries and complex conjugation. This is adjacent
    device-independent progress, but self-testing or device-independent GME
    certification does not by itself prove exclusion from
    \(\mathcal B_{\mathrm{NS\text{-}bisep}}\), and the theorem is for qubits
    rather than arbitrary local dimensions.

    Renner, Lobo, Konderak, Augusiak, and Acin, "All pure entangled states can
    lead to fully nonlocal correlations", arXiv:2604.26605 (2026), prove
    bipartite full-nonlocality results for some nonmaximally entangled states
    and many-copy activation of full nonlocality for all pure entangled states.
    They also show that in every dimension some pure entangled states do not
    exhibit full nonlocality in the studied single-copy sense. These results
    concern bipartite full nonlocality and many-copy activation, not the
    single-copy GMNL claim for arbitrary GME pure states.
  </known>

  <refs>
    <ref>Open Quantum Problems, Bell inequalities: many questions, a few answers, Problem 32, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/bell-inequalities-many-questions-a-few-answers, used for OQP32C provenance and the original genuine \(n\)-party Bell-inequality question that motivates this modern GMNL strengthening.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is derived from Problem 32, Bell inequalities: many questions, a few answers, listed in Quantum foundations.</ref>
    <ref>N. Gisin, Bell inequalities: many questions, a few answers, in Essays in Honour of Abner Shimony, Western Ontario Series in Philosophy of Science, Springer, 2009, pp. 125-140, DOI: 10.1007/978-1-4020-9107-0_9, arXiv:quant-ph/0702021, used for the original list of Bell inequality questions and subquestion C.</ref>
    <ref>S. Popescu and D. Rohrlich, Generic quantum nonlocality, Physics Letters A 166(5-6), 293-297 (1992), DOI: 10.1016/0375-9601(92)90711-T, used for the ordinary pure-state multipartite nonlocality theorem.</ref>
    <ref>M. Gachechiladze and O. Guhne, Completing the proof of "Generic quantum nonlocality", Physics Letters A 381(15), 1281-1285 (2017), DOI: 10.1016/j.physleta.2016.10.001, arXiv:1607.02948, used for the completed proof of ordinary pure-state multipartite nonlocality.</ref>
    <ref>S. Yu, Q. Chen, C. Zhang, C. H. Lai, and C. H. Oh, All entangled pure states violate a single Bell's inequality, Physical Review Letters 109, 120402 (2012), DOI: 10.1103/PhysRevLett.109.120402, arXiv:1205.1179, used for the ordinary pure-state Bell-nonlocality distinction.</ref>
    <ref>G. Svetlichny, Distinguishing three-body from two-body nonseparability by a Bell-type inequality, Physical Review D 35(10), 3066-3069 (1987), DOI: 10.1103/PhysRevD.35.3066, used for the original tripartite genuine-nonlocality criterion.</ref>
    <ref>J.-D. Bancal, J. Barrett, N. Gisin, and S. Pironio, The definition of multipartite nonlocality, Physical Review A 88, 014102 (2013), DOI: 10.1103/PhysRevA.88.014102, arXiv:1112.2626, used for nonsignalling-bipartition definitions and three-way pure-state numerical evidence.</ref>
    <ref>J.-D. Bancal, N. Brunner, N. Gisin, and Y.-C. Liang, Detecting Genuine Multipartite Quantum Nonlocality: A Simple Approach and Generalization to Arbitrary Dimensions, Physical Review Letters 106, 020405 (2011), DOI: 10.1103/PhysRevLett.106.020405, used for GMNL Bell inequalities for arbitrary parties and local dimensions.</ref>
    <ref>S. Yu and C. H. Oh, Tripartite entangled pure states are tripartite nonlocal, arXiv:1306.5330 (2013), DOI: 10.48550/arXiv.1306.5330, used for the arbitrary-local-dimension tripartite GME-pure-state GMNL result via a Hardy-type test.</ref>
    <ref>F. J. Curchod, M. L. Almeida, and A. Acin, A versatile construction of Bell inequalities for the multipartite scenario, New Journal of Physics 21, 023016 (2019), DOI: 10.1088/1367-2630/aaff2d, arXiv:1808.10688, used for analytic arbitrary-party progress and numerical three- and four-qubit evidence.</ref>
    <ref>I. Stachura, O. Makuta, and R. Augusiak, Single Bell inequality to detect genuine nonlocality in three-qubit genuinely entangled states, New Journal of Physics 26, 093029 (2024), DOI: 10.1088/1367-2630/ad7753, arXiv:2406.14078, used for the all three-qubit GME pure-state case.</ref>
    <ref>J. Wojcik, W. Bruzda, I. Stachura, and R. Augusiak, Equivalence of Genuine Multipartite Entanglement and Nonlocality of Nearly Symmetric Multiqubit Pure States, arXiv:2510.13296 (2025), DOI: 10.48550/arXiv.2510.13296, used for highly symmetric multiqubit pure-state progress.</ref>
    <ref>O. Makuta and R. Augusiak, All genuinely entangled stabilizer subspaces are multipartite fully nonlocal, npj Quantum Information 11, 144 (2025), DOI: 10.1038/s41534-025-01080-3, arXiv:2312.08757, used for stabilizer-subspace GMNL progress.</ref>
    <ref>M. Zwerger, W. Dur, J.-D. Bancal, and P. Sekatski, Device-independent detection of genuine multipartite entanglement for all pure states, Physical Review Letters 122, 060502 (2019), DOI: 10.1103/PhysRevLett.122.060502, arXiv:1809.02381, used for the device-independent GME-detection distinction.</ref>
    <ref>M. Balanzo-Juando, A. Coladangelo, R. Augusiak, A. Acin, and I. Supic, All pure multipartite entangled states of qubits can be self-tested, Nature Communications 17, 4463 (2026), DOI: 10.1038/s41467-026-70829-x, used for adjacent qubit self-testing progress.</ref>
    <ref>M. J. Renner, E. P. Lobo, A. Konderak, R. Augusiak, and A. Acin, All pure entangled states can lead to fully nonlocal correlations, arXiv:2604.26605 (2026), DOI: 10.48550/arXiv.2604.26605, used for current adjacent full-nonlocality and many-copy activation progress.</ref>
  </refs>

  <ask>
    Prove or disprove the claim. A proof must show that for every finite
    \(n\ge 4\), every finite-dimensional GME pure state \(|\psi\rangle\), and
    some finite single-copy local POVM Bell scenario, possibly depending on
    \(|\psi\rangle\), the induced behavior is not in
    \(\mathcal B_{\mathrm{NS\text{-}bisep}}\). Equivalently, the proof may
    construct or certify a Bell functional with a strict gap
    \[
      \ell(p)-\beta_{\mathrm{NS\text{-}bisep}}\gt 0.
    \]

    A disproof must give a finite \(n\ge 3\), finite local dimensions
    \(d_1,\ldots,d_n\), and a GME pure state \(|\psi\rangle\) such that for
    every finite choice of local POVMs on the single copy of \(|\psi\rangle\),
    the resulting behavior belongs to
    \(\mathcal B_{\mathrm{NS\text{-}bisep}}\) in the corresponding Bell
    scenario. The disproof must prove membership for all finite local
    measurements, not just for projective measurements, two-outcome
    measurements, full-correlation inequalities, a fixed inequality family, or
    numerically searched scenarios.

    Ordinary multipartite Bell nonlocality, the known tripartite Hardy-type
    theorem, postselected local-projection arguments, stochastic filtering,
    many-copy activation, device-independent GME witnesses based on generated
    lower-party states, fixed families such as GHZ, W, Dicke, graph, cluster,
    stabilizer, or symmetric states, single state-independent inequalities for
    only \(n=3\), and qubit-only self-testing theorems are progress unless
    they imply the stated all-finite-dimensional single-copy GMNL claim or its
    negation.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
