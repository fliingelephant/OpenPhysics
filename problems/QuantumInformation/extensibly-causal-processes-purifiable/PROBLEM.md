# QF043 Extensibly causal process purifiability

```xml
<problem id="QF043">
  <status>open</status>
  <name>Extensibly causal process purifiability</name>
  <keys>oqp problem 43 extensibly causal processes purifiable process matrices process matrix purification postulate pure process indefinite causal order no causal order causal inequality causal inequalities causal process causally separable process extensibly causally separable ecs process quantum switch wocb process wopt noisy wopt quantum comb superchannel unitary extension unitarily extendible quantum foundations</keys>

  <claim>
    Every finite-dimensional ordinary multipartite quantum process matrix with
    trivial global past and trivial global future that is extensibly causal is
    purifiable in the sense of Araújo, Feix, Navascués, and Brukner, "A
    purification postulate for quantum mechanics with indefinite causal order",
    Quantum 1, 10 (2017).
  </claim>

  <def>
    All Hilbert spaces in this entry are finite-dimensional complex Hilbert
    spaces. For party \(A_i\), write \(A^i_I\) and \(A^i_O\) for its input and
    output spaces. An ordinary \(N\)-partite process matrix is an operator
    \[
      W\in\mathcal L\!\left(\bigotimes_{i=1}^N
      (A^i_I\otimes A^i_O)\right)
    \]
    whose global past \(P\) and global future \(F\) are the one-dimensional
    spaces. For a completely positive map
    \(\mathcal M:\mathcal L(A_I)\to\mathcal L(A_O)\), use the
    Oreshkov-Costa-Brukner and Oreshkov-Giarmatzi Choi-Jamiołkowski convention
    \[
      M=\left[(\mathrm{id}\otimes\mathcal M)
      (|\Phi^+\rangle\langle\Phi^+|)\right]^{\mathrm T},
      \qquad |\Phi^+\rangle=\sum_j |j\rangle|j\rangle ,
    \]
    with the transpose taken in fixed local bases. Then \(M\ge 0\), and
    \(\mathcal M\) is trace-preserving iff
    \(\operatorname{Tr}_{A_O}M=\mathbb I_{A_I}\). The process \(W\) assigns
    probabilities to local CP-map outcomes by
    \[
      p(M_1,\ldots,M_N)=
      \operatorname{Tr}\!\left[
      (M_1\otimes\cdots\otimes M_N)W\right].
    \]
    It is a valid ordinary process matrix iff \(W\ge 0\) and these
    probabilities are normalized for every choice of local CPTP maps, including
    the Oreshkov-Costa-Brukner extension in which parties receive arbitrary
    ancillary input systems in a joint quantum state.

    A finite setting-outcome process is causal if its conditional
    probabilities admit the Oreshkov-Giarmatzi canonical causal decomposition:
    a convex decomposition in which a nonempty set of first parties is chosen,
    the first parties have a non-signaling reduced process, and, conditional on
    their events, meaning the relevant local setting and outcome data
    \((s^i,o^i)\), the remaining parties again have a causal process, iterated
    until no parties remain. Equivalently, in fixed finite classical
    setting-outcome scenarios, causal correlations are exactly the correlations
    in the causal polytope; violations of facets of this polytope are causal
    inequality violations. An ordinary quantum process matrix is causal if, for
    every choice of local instruments with finite settings and outcomes, the
    resulting probabilities are causal in this sense.

    An ordinary quantum process matrix is causally separable if it is causal and
    has an Oreshkov-Giarmatzi causal decomposition in which every process
    appearing recursively in the decomposition is itself a valid quantum process
    matrix. In the bipartite case this is the usual form
    \(W=q\,W^{A\preceq B}+(1-q)W^{B\preceq A}\), where
    \(W^{A\preceq B}\) permits no signaling from \(B\) to \(A\),
    \(W^{B\preceq A}\) permits no signaling from \(A\) to \(B\), and both
    summands are valid process matrices after their conventional normalization.

    An ordinary process matrix \(W\) is extensibly causal if, for every choice
    of finite-dimensional ancillary input spaces \(A^{i'}_I\) and every joint
    density operator
    \[
      \rho\in\mathcal D\!\left(\bigotimes_i A^{i'}_I\right),
    \]
    the extended process \(W\otimes\rho\), with local operations now acting
    from \(A^i_I\otimes A^{i'}_I\) to \(A^i_O\), is causal. It is extensibly
    causally separable, or ECS, if \(W\otimes\rho\) is causally separable for
    every such ancillary state \(\rho\). ECS implies extensible causality, but
    Oreshkov and Giarmatzi, "Causal and causally separable processes", New
    Journal of Physics 18, 093020 (2016), prove that the two notions are not
    equivalent in general.

    To define purifiability, allow global past and future spaces \(P\) and
    \(F\). A generalized process
    \[
      W\in\mathcal L\!\left(P\otimes\bigotimes_i(A^i_I\otimes A^i_O)
      \otimes F\right)
    \]
    maps local CPTP maps to a CPTP map from \(P\) to \(F\). Translating the
    Araújo-Feix-Navascués-Brukner link-product definition to the transposed
    Oreshkov-Costa-Brukner/OQP CJ convention used above, a process \(S\) on
    \(P\otimes P'\), the local laboratories, and \(F\otimes F'\) is pure if,
    whenever the parties apply unitary channels, possibly with arbitrary
    through-wires in their laboratories, the induced transformation from the
    through-wire inputs and global past to the through-wire outputs and global
    future is unitary. Equivalently,
    \(S=|U_S\rangle\!\rangle\langle\!\langle U_S|\) for a unitary
    transformation from the tensor product of all laboratory outputs and past
    systems to the tensor product of all laboratory inputs and future systems,
    with the corresponding CJ convention.

    An ordinary process matrix \(W\) is purifiable if there exist
    finite-dimensional ancillary global past and future spaces \(P'\) and
    \(F'\), a fixed pure state \(|0\rangle\in P'\), and a pure process \(S\)
    with \(P=F=\mathbb C\) such that
    \[
      W=S*(|0\rangle\langle0|^{P'}\otimes\mathbb I^{F'})
    \]
    in the link-product notation of Araújo, Feix, Navascués, and Brukner. In
    coordinates this is the OQP trace formula with the transpose prescribed by
    the same CJ/link-product convention. This definition is not the weaker
    rank-one-process notion of purity used in some causal-witness literature;
    that weaker notion would make purifiability trivial and is not the OQP43
    target.
  </def>

  <known>
    Open Quantum Problems, Are all extensibly causal processes purifiable?,
    Problem 43, asks whether every extensibly causal process is purifiable. The
    Open Quantum Problems index lists Problem 43 in Quantum foundations, with
    Mateus Araújo and Časlav Brukner as contacts and date 2017/05/22. This
    entry records the ordinary finite-dimensional process-matrix reading of the
    OQP question, because OQP cites Oreshkov and Giarmatzi, "Causal and
    causally separable processes", New Journal of Physics 18, 093020 (2016),
    for extensible causality, and that definition is stated for ordinary process
    matrices with trivial global past and future. Variants allowing nontrivial
    global \(P\) and \(F\), infinite-dimensional systems, approximate
    purification, or theory-independent process frameworks are adjacent
    variants unless they imply or refute the finite-dimensional ordinary claim
    above.

    Oreshkov, Costa, and Brukner, "Quantum correlations with no causal order",
    Nature Communications 3, 1092 (2012), introduce ordinary process matrices,
    the CJ probability rule, validity by well-defined probabilities for all
    local quantum operations, and the first process \(W_{\mathrm{OCB}}\)
    violating a causal inequality. Araújo, Branciard, Costa, Feix, Giarmatzi,
    and Brukner, "Witnessing causal nonseparability", New Journal of Physics
    17, 102001 (2015), introduce causal witnesses and show that the quantum
    switch is causally nonseparable while not violating causal inequalities.
    Branciard, Araújo, Feix, Costa, and Brukner, "The simplest causal
    inequalities and their violation", New Journal of Physics 18, 013008
    (2016), characterize the simplest bipartite binary causal polytope and give
    process-matrix violations, including \(W_{\max}\). These sources establish
    the process-matrix and causal-inequality setting, but they do not settle
    OQP43.

    Oreshkov and Giarmatzi, "Causal and causally separable processes", New
    Journal of Physics 18, 093020 (2016), define the canonical notion of causal
    process, the causal polytope, causal separability, extensible causality, and
    ECS. They prove that some causally separable processes become non-causal
    when extended with entangled input ancillas, motivating extensible
    causality. They also prove that extensibly causal and ECS processes are not
    equivalent in general, using a causally nonseparable tripartite process
    based on the quantum switch that remains causal under entangled ancillary
    inputs. This separation is not a disproof of the present claim because the
    process is not proved non-purifiable; quantum-switch-type processes are
    among the processes expected to be purifiable.

    Araújo, Feix, Navascués, and Brukner, "A purification postulate for quantum
    mechanics with indefinite causal order", Quantum 1, 10 (2017), define pure
    processes as those preserving unitarity of local laboratory operations and
    define purifiable processes as reductions of pure processes. They derive
    necessary and sufficient conditions for purifiability and a computable
    necessary condition. Applying that condition, they prove that
    \(W_{\mathrm{OCB}}\) and \(W_{\max}\) are not purifiable. These are not
    counterexamples to OQP43 because \(W_{\mathrm{OCB}}\) and \(W_{\max}\)
    violate causal inequalities and hence are not extensibly causal.

    The same Araújo-Feix-Navascués-Brukner analysis discusses
    \[
      W_{\mathrm{opt}}=\frac14\left[
      \mathbb I\mathbb I\mathbb I\mathbb I+
      \frac{1}{\sqrt3}Z\mathbb I XZ+
      \frac{\sqrt3-1}{3}(\mathbb IXX\mathbb I+
      \mathbb IYY\mathbb I+\mathbb IZZ\mathbb I)\right],
    \]
    introduced by Feix, Araújo, and Brukner, "Quantum superposition of the
    order of parties as a communication resource", Physical Review A 92,
    052326 (2015). \(W_{\mathrm{opt}}\) is causal by itself but becomes
    non-causal with entangled ancillary inputs. Araújo, Feix, Navascués, and
    Brukner report that their purifiability test is inconclusive for
    \(W_{\mathrm{opt}}\) and for the noisy version conjectured there to be
    unable to violate causal inequalities. The noisy \(W_{\mathrm{opt}}\)
    therefore is not a certified OQP43 counterexample unless one proves both
    extensible causality and non-purifiability.

    Later literature found no proof of the converse in the claim and no
    explicit extensibly causal process proved non-purifiable. Wechs, Abbott,
    and Branciard, "On the definition and characterisation of multipartite
    causal (non)separability", New Journal of Physics 21, 013027 (2019),
    clarify multipartite causal separability and relate their preferred
    definition to extensible causal separability; this concerns ECS, not the
    larger class of extensibly causal processes in the claim. Yokojima,
    Quintino, Soeda, and Murao, "Consequences of preserving reversibility in
    quantum superchannels", Quantum 5, 441 (2021), characterize bipartite pure
    superchannels and prove that every purifiable bipartite process matrix is
    extensibly causal, equivalently that it cannot violate causal inequalities
    even after arbitrary shared input ancillas; this is the direction
    \(\mathrm{purifiable}\Rightarrow\mathrm{extensibly\ causal}\), not a proof
    of the OQP43 converse. Barrett, Lorenz, and Oreshkov, "Cyclic quantum
    causal models", Nature Communications 12, 885 (2021), show that all
    unitarily extendible bipartite processes are causally separable and analyze
    unitary cyclic causal structure; this again does not prove the OQP43
    converse for all extensibly causal processes. Wechs, Dourdent, Abbott, and
    Branciard, "Quantum circuits with classical versus quantum control of
    causal order", PRX Quantum 2, 030335 (2021), give physically realizable
    circuit subclasses that generate only causal correlations. Vanrietvelde,
    Ormrod, Kristjánsson, and Barrett, "Consistent circuits for indefinite
    causal order", Quantum 9, 1923 (2025), construct routed consistent circuits
    for many exotic processes and conjecture coverage of all unitarily
    extendible processes. These routed, circuit, and cyclic-model results
    supply positive physical implementations and structural subclasses, but
    they do not settle whether the entire extensibly causal set is purifiable.
  </known>

  <refs>
    <ref>Open Quantum Problems, Are all extensibly causal processes purifiable?, Problem 43, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/are-all-extensibly-causal-processes-purifiable, used for the OQP43 question, process-matrix probability rule, extensible-causality motivation, and purifiability formula.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems index, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is derived from Problem 43, Are all extensibly causal processes purifiable?, listed in Quantum foundations.</ref>
    <ref>O. Oreshkov, F. Costa, and Č. Brukner, Quantum correlations with no causal order, Nature Communications 3, 1092 (2012), DOI: 10.1038/ncomms2076, arXiv:1105.4464, used for the ordinary process-matrix framework, CJ probability rule, validity conditions, and the \(W_{\mathrm{OCB}}\) causal-inequality-violating example.</ref>
    <ref>M. Araújo, C. Branciard, F. Costa, A. Feix, C. Giarmatzi, and Č. Brukner, Witnessing causal nonseparability, New Journal of Physics 17, 102001 (2015), DOI: 10.1088/1367-2630/17/10/102001, arXiv:1506.03776, used for causal witnesses, causal nonseparability, and the quantum-switch distinction from causal-inequality violation.</ref>
    <ref>O. Oreshkov and C. Giarmatzi, Causal and causally separable processes, New Journal of Physics 18, 093020 (2016), DOI: 10.1088/1367-2630/18/9/093020, arXiv:1506.05449, used for causal processes, causal polytopes, causal inequalities, causal separability, extensible causality, extensible causal separability, tripartite ECS structure, and known separations among these classes.</ref>
    <ref>C. Branciard, M. Araújo, A. Feix, F. Costa, and Č. Brukner, The simplest causal inequalities and their violation, New Journal of Physics 18, 013008 (2016), DOI: 10.1088/1367-2630/18/1/013008, arXiv:1508.01704, used for the bipartite binary causal polytope and the \(W_{\max}\) causal-inequality-violating process later tested for purifiability.</ref>
    <ref>M. Araújo, A. Feix, M. Navascués, and Č. Brukner, A purification postulate for quantum mechanics with indefinite causal order, Quantum 1, 10 (2017), DOI: 10.22331/q-2017-04-26-10, arXiv:1611.08535, used for pure processes, purifiable processes, the purifiability criteria, non-purifiability of \(W_{\mathrm{OCB}}\) and \(W_{\max}\), the inconclusive status of \(W_{\mathrm{opt}}\) and noisy \(W_{\mathrm{opt}}\), and the statement that the extensibly-causal-to-purifiable question remained open.</ref>
    <ref>A. Feix, M. Araújo, and Č. Brukner, Quantum superposition of the order of parties as a communication resource, Physical Review A 92, 052326 (2015), DOI: 10.1103/PhysRevA.92.052326, arXiv:1508.07840, used for the \(W_{\mathrm{opt}}\) process and activation-by-entangled-ancilla context.</ref>
    <ref>J. Wechs, A. A. Abbott, and C. Branciard, On the definition and characterisation of multipartite causal (non)separability, New Journal of Physics 21, 013027 (2019), DOI: 10.1088/1367-2630/aaf352, arXiv:1807.10557, used for later clarification of multipartite causal separability and extensible causal separability.</ref>
    <ref>W. Yokojima, M. T. Quintino, A. Soeda, and M. Murao, Consequences of preserving reversibility in quantum superchannels, Quantum 5, 441 (2021), DOI: 10.22331/q-2021-04-26-441, arXiv:2003.05682, used for the bipartite pure-superchannel characterization and the theorem that purifiable bipartite process matrices are extensibly causal.</ref>
    <ref>J. Barrett, R. Lorenz, and O. Oreshkov, Cyclic quantum causal models, Nature Communications 12, 885 (2021), DOI: 10.1038/s41467-020-20456-x, arXiv:2002.12157, used for the cyclic quantum causal model analysis and the theorem that all unitarily extendible bipartite processes are causally separable.</ref>
    <ref>J. Wechs, H. Dourdent, A. A. Abbott, and C. Branciard, Quantum circuits with classical versus quantum control of causal order, PRX Quantum 2, 030335 (2021), DOI: 10.1103/PRXQuantum.2.030335, arXiv:2101.08796, used for physically realizable circuit subclasses with classical or quantum control of causal order and their causal-correlation limitations.</ref>
    <ref>A. Vanrietvelde, N. Ormrod, H. Kristjánsson, and J. Barrett, Consistent circuits for indefinite causal order, Quantum 9, 1923 (2025), DOI: 10.22331/q-2025-12-02-1923, arXiv:2206.10042, used for routed consistent-circuit constructions for exotic processes and the conjectured coverage of all unitarily extendible processes.</ref>
  </refs>

  <ask>
    Prove or disprove the claim. A proof must show that for every finite
    \(N\), every finite choice of laboratory input and output Hilbert spaces,
    and every valid ordinary process matrix \(W\) that remains causal after
    tensoring arbitrary finite-dimensional joint input ancillas, there exist
    finite-dimensional \(P'\), \(F'\), a pure state \(|0\rangle\in P'\), and a
    pure process \(S\) whose reduction is \(W\) under the Araújo-Feix-Navascués-
    Brukner purifiability definition.

    A disproof must give a concrete finite-dimensional ordinary process matrix
    \(W\), prove validity of \(W\), prove extensible causality of \(W\) for
    arbitrary finite-dimensional joint quantum input ancillas and arbitrary
    finite local instruments, and prove that no finite-dimensional pure-process
    dilation \(S\) as defined above reduces to \(W\). A process that merely is
    causal before adding entangled ancillas, is only conjectured to be
    extensibly causal, violates a causal inequality, is merely causally
    nonseparable, is known not to be purifiable but not known to be extensibly
    causal, or belongs to a purifiable physical subclass such as fixed-order
    circuits, the quantum switch, quantum-controlled circuits, or routed
    unitarily extendible examples is progress unless it proves the universal
    claim or supplies the certified counterexample just described.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
