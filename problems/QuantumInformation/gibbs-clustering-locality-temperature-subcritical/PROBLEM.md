# QTD028 Exponential Clustering of Quantum Gibbs States up to the Phase-Transition Threshold in Two and Higher Dimensions

```xml
<problem id="QTD028">
  <status>open</status>
  <name>Exponential Clustering of Quantum Gibbs States up to the Phase-Transition Threshold in Two and Higher Dimensions</name>

  <keys>gibbs state thermal state quantum spin lattice exponential clustering clustering of correlations decay of correlations correlation decay connected correlation function covariance generalized covariance two-point function phase transition critical temperature critical inverse temperature curie temperature order-disorder transition free energy density thermodynamic limit analyticity of free energy real-analytic free energy analytic continuation yang-lee zeros lee-yang theorem sharpness of the phase transition complete analyticity dobrushin-shlosman high temperature expansion cluster expansion locality of temperature kliesch gogolin kastoryano riera eisert growth constant lattice animal universal critical temperature subcritical temperature high-temperature threshold non-commuting hamiltonian finite-range interaction translation-invariant interaction two-dimensional lattice higher-dimensional lattice ising model capel moscolari teufel wessel decay-of-correlations hypothesis local perturbations perturb locally lppl local indistinguishability stability of gibbs state quantum belief propagation lieb-robinson bound kms state kms condition infinite-volume gibbs state quantum markov property</keys>

  <claim>
    Fix an integer \(D\ge2\). For every translation-invariant, finite-range
    interaction \(\Phi=\{h_X\}\) on \(\mathbb Z^D\) (Def section: bounded,
    self-adjoint, finite-range terms, invariant under lattice translations),
    let \(\beta_{PT}(\Phi)\in[0,\infty]\) be the phase-transition threshold
    and \(\beta_c(\Phi)\in[0,\infty]\) the exponential-clustering threshold
    of the Def section. Then
    \[
      \beta_c(\Phi)\ \ge\ \beta_{PT}(\Phi),
    \]
    i.e.\ \(\Phi\) has exponential clustering (Def section) at every
    \(\beta\lt\beta_{PT}(\Phi)\) -- every inverse temperature at which
    \(\Phi\) has no phase transition in the sense of the Def section.

    Operationalization and quantifier note (weaker reading chosen;
    ambiguity flagged). Kliesch, Gogolin, Kastoryano, Riera, and Eisert
    (Known section) speak of "physical critical temperatures such as the
    Curie temperature" without giving a general definition applicable to an
    arbitrary, possibly non-commuting \(\Phi\); their only worked numerical
    example is the classical (diagonal, commuting) two-dimensional Ising
    Hamiltonian, whose critical point is known independently of their
    method. This entry operationalizes "the actual phase transition" of the
    survey lead via real-analyticity of the free energy density \(f_\Phi\)
    (Def section) -- the standard, model-general notion of a thermodynamic
    phase transition in equilibrium statistical mechanics, well defined for
    every translation-invariant finite-range \(\Phi\) without further
    hypotheses, and logically independent of the clustering property
    defined below it (so the claim is not circular). A different
    operationalization of "no phase transition" -- e.g.\ via uniqueness of
    the infinite-volume KMS state, or via vanishing long-range order in a
    specified order parameter -- could define a different threshold in
    general; whether such alternative thresholds coincide with
    \(\beta_{PT}(\Phi)\) is itself not settled by the cited sources and is
    not assumed here. The claim is stated only as the inequality
    \(\beta_c(\Phi)\ge\beta_{PT}(\Phi)\) (clustering persists throughout the
    region of analyticity), the weaker of the two natural readings; the
    converse inequality \(\beta_c(\Phi)\le\beta_{PT}(\Phi)\), and the
    equality obtained by combining both, are different, stronger statements
    not addressed by the cited sources and not included in this entry's
    claim.
  </claim>

  <def>
    Lattice, interaction, Gibbs state. Fix an integer \(D\ge2\) and an
    integer \(q\ge2\) (local dimension); every site of \(\mathbb Z^D\)
    carries a Hilbert space \(\mathbb C^q\). For \(x,y\in\mathbb Z^D\),
    \(\|x-y\|_1:=\sum_{i=1}^D|x_i-y_i|\); for finite \(X,Y\subset\mathbb
    Z^D\), \(\operatorname{dist}(X,Y):=\min\{\|x-y\|_1:x\in X,y\in Y\}\) and
    \(\operatorname{diam}(X):=\max\{\|x-y\|_1:x,y\in X\}\). A
    translation-invariant, finite-range interaction is a family
    \(\Phi=\{h_X\}\), indexed by finite \(X\subset\mathbb Z^D\), of
    Hermitian operators \(h_X\) acting on \(\bigotimes_{x\in X}\mathbb C^q\)
    (extended by the identity elsewhere), with a fixed range \(r\)
    (\(h_X=0\) whenever \(\operatorname{diam}(X)\gt r\)), a fixed norm
    bound \(J\) (\(\|h_X\|\le J\) for every \(X\)), and translation
    invariance: for every \(v\in\mathbb Z^D\) and every finite
    \(X\subset\mathbb Z^D\), \(h_{X+v}\) equals \(h_X\) under the canonical
    identification of \(\bigotimes_{x\in X+v}\mathbb C^q\) with
    \(\bigotimes_{x\in X}\mathbb C^q\) given by the lattice shift by \(v\)
    (so \(\Phi\) is fixed by finitely many "shapes" up to translation). For
    the cube \(\Lambda_L:=\{0,\ldots,L-1\}^D\) (\(L\in\mathbb N\),
    \(n(L):=L^D=|\Lambda_L|\)), the Hamiltonian is
    \(H_{\Lambda_L}:=\sum_{X\subseteq\Lambda_L}h_X\) (free boundary
    condition), the partition function is
    \(Z_{\Lambda_L}(\beta):=\operatorname{Tr}e^{-\beta H_{\Lambda_L}}\), and
    the Gibbs state at inverse temperature \(\beta\ge0\) is
    \(\rho_{\beta,\Lambda_L}:=e^{-\beta H_{\Lambda_L}}/Z_{\Lambda_L}(\beta)\).

    Free energy density. For \(\beta\gt0\) set
    \(f_{\Lambda_L}(\beta):=-\frac{1}{\beta\,n(L)}\ln Z_{\Lambda_L}(\beta)\).
    For each fixed \(L\), \(Z_{\Lambda_L}(\beta)=\sum_k e^{-\beta E_k}\) is a
    finite sum of exponentials of the eigenvalues \(E_k\) of
    \(H_{\Lambda_L}\), strictly positive for real \(\beta\), so
    \(f_{\Lambda_L}\) is real-analytic (indeed entire) on \((0,\infty)\).
    By a standard subadditivity argument for translation-invariant,
    finite-range interactions (a standard fact of quantum statistical
    mechanics, not otherwise used in this entry), the limit
    \(f_\Phi(\beta):=\lim_{L\to\infty}f_{\Lambda_L}(\beta)\) exists for
    every \(\beta\gt0\).

    Phase-transition threshold \(\beta_{PT}(\Phi)\). Set
    \[
      \beta_{PT}(\Phi)\ :=\ \sup\{\beta_1\ge0:\ f_\Phi\text{ is
      real-analytic on the open interval }(0,\beta_1)\}\ \in[0,\infty].
    \]
    This is the standard Yang-Lee/Ruelle-style analyticity threshold of the
    free energy density: it is well defined for every translation-invariant
    finite-range \(\Phi\) with no further hypothesis, and its definition
    does not reference the clustering property defined next.

    Exponential clustering and the threshold \(\beta_c(\Phi)\). For finite
    \(X\subset\mathbb Z^D\) let \(\partial X:=\{x\in X:
    \operatorname{dist}(\{x\},\mathbb Z^D\setminus X)\le r\}\) (the sites of
    \(X\) within interaction range \(r\), as fixed above, of its
    complement). Say \(\Phi\) has exponential clustering at \(\beta\) if
    there exist \(\xi=\xi(\beta)\gt0\) and a function \(a\mapsto
    C(\beta,a)\lt\infty\) on \(a\in\mathbb N\), independent of \(L\) and of
    \(\operatorname{dist}(X,Y)\), such that, for every \(L\), every finite
    \(X,Y\subset\Lambda_L\), and every operators \(A\) on \(X\), \(B\) on
    \(Y\) with \(\|A\|,\|B\|\le1\), setting \(a:=\min(|\partial
    X|,|\partial Y|)\),
    \[
      \bigl|\operatorname{Tr}[\rho_{\beta,\Lambda_L}(A\otimes B)]
      -\operatorname{Tr}[\rho_{\beta,\Lambda_L}(A\otimes\mathbb 1)]\,
      \operatorname{Tr}[\rho_{\beta,\Lambda_L}(\mathbb 1\otimes B)]\bigr|
      \ \le\ C(\beta,a)\,e^{-\operatorname{dist}(X,Y)/\xi}.
    \]
    This boundary-size-dependent prefactor is the form in which such
    bounds are actually proved by Lieb-Robinson/quantum-belief-propagation
    methods (Known section); it is weaker than requiring a single
    \(C(\beta)\) uniform over all region sizes at once. Set
    \(\beta_c(\Phi):=\sup\{\beta_1\ge0:\Phi\text{ has exponential
    clustering at every }\beta\lt\beta_1\}\in[0,\infty]\). Unlike
    \(\beta_{PT}(\Phi)\), this threshold is defined purely from the decay
    of a two-point function evaluated in the finite-volume Gibbs states
    themselves.
  </def>

  <known>
    Michael Kliesch, Christian Gogolin, Michael J. Kastoryano, Arnau Riera,
    and Jens Eisert, Locality of Temperature, Physical Review X 4, 031019
    (2014), DOI: 10.1103/PhysRevX.4.031019, arXiv:1309.0816, Theorem 2
    (Sec. II.4) and Sec. III.1, prove, for a local Hamiltonian on any
    interaction hypergraph \((V,E)\) (not assumed translation-invariant,
    not assumed commuting, not assumed to be \(\mathbb Z^D\)) with growth
    constant \(\alpha\) (the number of connected sub-hypergraphs of size
    \(m\) containing a fixed edge is at most \(\alpha^m\)) and uniform
    local-term norm bound \(J\), that, setting
    \(\beta^*:=\ln[(1+\sqrt{1+4/\alpha})/2]/(2J)\), a generalized covariance
    of any two operators \(A,B\) decays exponentially in graph distance for
    every \(|\beta|\lt\beta^*\): explicitly,
    \(|\operatorname{cov}^\tau_{g(\beta)}(A,B)|\le4a\|A\|_\infty\|B\|_\infty\,
    e^{-d(A,B)/\xi(\beta)}/[\ln(3)(1-e^{-1/\xi(\beta)})]\) for every
    \(d(A,B)\ge L_0(\beta,a)\), where \(\xi(\beta)\) is a finite
    correlation length and \(a:=\min\{|\partial A|,|\partial B|\}\) is the
    smaller of the two operators' boundary sizes in \((V,E)\); the
    prefactor \(4a\) and the minimum-distance threshold \(L_0(\beta,a)\)
    grow with \(a\), so this is a bound at fixed boundary size, not one
    uniform over all region sizes at once. At the parameter value
    corresponding to ordinary covariance, this generalized covariance is
    exactly the two-point function of the Def section above, so the
    theorem gives exponential clustering in the Def section's
    boundary-size-dependent sense throughout \(|\beta|\lt\beta^*\). For
    nearest-neighbor interactions on \(\mathbb Z^D\), the growth
    constant obeys \(\alpha\le2De\), giving an explicit \(\beta^*(D,J)\)
    depending only on \(D\) and \(J\) -- never on the finer structure of
    \(\Phi\) -- so that \(\beta_c(\Phi)\ge\beta^*(D,J)\) for every
    nearest-neighbor \(\Phi\) of norm bound \(J\) on \(\mathbb Z^D\); the
    same theorem extends to any fixed finite range \(r\) via a
    correspondingly larger, still \(\Phi\)-independent, growth constant.
    This is the only unconditional, general-\(D\), general-(possibly
    non-commuting)-\(\Phi\) lower bound on \(\beta_c(\Phi)\) located for
    this entry. Kliesch, Gogolin, Kastoryano, Riera, and Eisert's own worked
    example -- the classical, commuting two-dimensional Ising Hamiltonian,
    whose true critical point is known
    independently of this method -- gives \(1/(\beta^*J)\approx24.58\)
    against the true \(1/(\beta_{PT}J)\approx2.27\), an order-of-magnitude
    gap; because \(\beta^*(D,J)\) depends only on \(D\) and \(J\), it cannot
    by construction reflect any model-specific reduction of that gap for a
    particular \(\Phi\). Sec. III.1 explicitly leaves as future work both
    tightening \(\beta^*\) "if more specific properties of a model are
    taken into account" and, separately, whether any model saturates the
    universal bound \(\beta^*(D,J)\) at its own transition; it neither
    states nor proves \(\beta_c(\Phi)\ge\beta_{PT}(\Phi)\) for general
    \(D\ge2\), possibly non-commuting \(\Phi\), and this entry's search did
    not locate that statement, or its disproof, elsewhere.

    Ángela Capel, Massimo Moscolari, Stefan Teufel, and Tom Wessel, From
    Decay of Correlations to Locality and Stability of the Gibbs State,
    Communications in Mathematical Physics 406, 43 (2025), DOI:
    10.1007/s00220-024-05198-x, arXiv:2310.09182 (v3, 2025 -- final version
    corresponding to the published article), Definition 1, Theorem 22
    ("LPPL from correlations in the unperturbed state"), and Theorem 29
    (Sec. 6, "Local indistinguishability from uniform LPPL"), prove, in any
    dimension \(D\) and requiring only locality of the Hamiltonian, that if
    a Gibbs state satisfies a quantitative decay-of-correlations hypothesis
    (their Definition 1: \(\operatorname{Cov}_\rho(X;Y)\le|X|^n
    f_{\operatorname{Cov}}(|Y|)\,\zeta_{\operatorname{Cov}}(
    \operatorname{dist}(X,Y))\) for continuous
    \(\zeta_{\operatorname{Cov}},f_{\operatorname{Cov}}:[0,\infty)\to[0,\infty)\)
    and \(n\ge0\)),
    then the Gibbs state is stable against local perturbations (Theorem
    22) and, composing with Theorem 29, locally indistinguishable from
    finite- and infinite-volume versions of itself. Theorem 31 (Sec. 7,
    "Uniform decay of correlations from local indistinguishability")
    proves the converse implication; the paper states, as a corollary, that decay of
    correlations, uniform LPPL, and local indistinguishability are three
    equivalent properties of the Gibbs state. These theorems take decay of
    correlations as an input hypothesis for the direction used above; they
    do not prove it in general dimension. For dimension \(D\ge2\), Capel,
    Moscolari, Teufel, and Wessel's own general short-range instance of
    that hypothesis (Conjecture 12, Sec. 3.4) is unproved -- stated only
    as a conjecture -- and their Corollary 13 derives LPPL and local
    indistinguishability from it conditionally on the conjecture. Their
    Remark 14 notes that the one case in which Conjecture 12 is proved,
    finite-range \(\Phi\), was already established by Kliesch, Gogolin,
    Kastoryano, Riera, and Eisert, and that composing that decay of
    correlations bound with their own Theorem 22 and Theorem 29 only
    recovers the Kliesch-Gogolin-Kastoryano-Riera-Eisert result; so even
    this unconditional instance is again only a
    Kliesch-Gogolin-Kastoryano-Riera-Eisert-type high-temperature-threshold
    result, not an extension of \(\beta_c(\Phi)\) toward
    \(\beta_{PT}(\Phi)\); their applications to one-dimensional chains
    include clustering at every temperature for
    polynomially-decaying translation-invariant interactions (Sec. 3.3,
    restating Kimura-Kuwahara's result as Theorem 10 and deriving the
    LPPL consequence themselves as Corollary 11, proof in Appendix A.2
    -- an instance of the decay-of-correlations hypothesis established
    elsewhere and invoked, not reproved, there) and, as a new result
    proved by Capel, Moscolari, Teufel, and Wessel themselves,
    exponential clustering above a threshold temperature for
    exponentially-decaying one-dimensional interactions (Sec. 9, Theorem
    6); neither
    one-dimensional instance addresses \(D\ge2\). Capel, Moscolari, Teufel,
    and Wessel's results instantiate the survey lead's description of "the
    assumption every 2024-25 rigorous paper imports": for general
    \(D\ge2\), decay of correlations below the
    Kliesch-Gogolin-Kastoryano-Riera-Eisert threshold remains an unverified
    hypothesis their general theorems consume rather than establish, and
    their introduction does not discuss extending that hypothesis's range
    toward \(\beta_{PT}(\Phi)\) as a target.

    Together, these two sources leave the claim above open exactly as
    stated, for every \(D\ge2\): no source located for this entry proves or
    disproves \(\beta_c(\Phi)\ge\beta_{PT}(\Phi)\) for a general
    translation-invariant, finite-range \(\Phi\) on \(\mathbb Z^D\); the
    best unconditional lower bound on \(\beta_c(\Phi)\) remains the
    \(\Phi\)-independent (beyond range and norm bound) \(\beta^*(D,J)\) of
    Kliesch, Gogolin, Kastoryano, Riera, and Eisert, whose own worked
    example shows this bound can be an order of magnitude below
    \(\beta_{PT}(\Phi)\). Theorem and section numbers for Kliesch, Gogolin,
    Kastoryano, Riera, and Eisert above were obtained from automated
    fetches of arXiv abstract and HTML (ar5iv) renderings rather than from
    a manual read of the typeset PDF, and are reported here with that
    caveat. The Capel, Moscolari, Teufel, and Wessel numbers above have
    been checked directly against the typeset text of the arXiv v3 PDF (54
    pages, matching the "final version corresponding to the published
    article" cited above); they may still not exactly match the final
    Communications in Mathematical Physics typesetting, which was not
    consulted for this entry.
  </known>

  <refs>
    <ref>Michael Kliesch, Christian Gogolin, Michael J. Kastoryano, Arnau Riera, and Jens Eisert, Locality of Temperature, Physical Review X 4, 031019 (2014), DOI: 10.1103/PhysRevX.4.031019, arXiv:1309.0816, Theorem 2 (Sec. II.4) and Sec. III.1, used for the explicit, \(\Phi\)-independent universal threshold \(\beta^*(D,J)\) below which exponential clustering is unconditionally proven, and for its own numerical comparison of \(\beta^*\) to the true critical point of the two-dimensional Ising Hamiltonian and its framing of tightening this bound as future work.</ref>
    <ref>Ángela Capel, Massimo Moscolari, Stefan Teufel, and Tom Wessel, From Decay of Correlations to Locality and Stability of the Gibbs State, Communications in Mathematical Physics 406, 43 (2025), DOI: 10.1007/s00220-024-05198-x, arXiv:2310.09182 (v3, 2025), Definition 1, Theorem 22, Theorem 29 (Sec. 6), Theorem 31 (Sec. 7), Conjecture 12, Corollary 13, and Remark 14 (Sec. 3.4), Theorem 10 and Corollary 11 (Sec. 3.3, Appendix A.2), and Theorem 6 (Sec. 9), used for the conditional (decay-of-correlations-as-hypothesis) results this entry's claim is contrasted against, and for confirming that this line of work does not itself extend the proven clustering range beyond Kliesch, Gogolin, Kastoryano, Riera, and Eisert's threshold in dimension \(D\ge2\).</ref>
  </refs>

  <ask>
    A proof exhibits, for every \(D\ge2\) and every translation-invariant,
    finite-range interaction \(\Phi\) on \(\mathbb Z^D\) (Def section),
    functions \(C(\beta,a)\lt\infty\) and \(\xi(\beta)\gt0\) such that the
    exponential-clustering bound of the Def section holds for every
    \(\beta\lt\beta_{PT}(\Phi)\) and every \(L\) -- i.e.\ shows
    \(\beta_c(\Phi)\ge\beta_{PT}(\Phi)\) for every such \(\Phi\), not merely
    for \(\beta\) below the \(\Phi\)-independent threshold \(\beta^*(D,J)\)
    of Kliesch, Gogolin, Kastoryano, Riera, and Eisert (Known section). A
    proof of the equality \(\beta_c(\Phi)=\beta_{PT}(\Phi)\) is strictly
    stronger and also counts as a full proof of the claim above.

    A disproof exhibits some \(D\ge2\), some translation-invariant
    finite-range \(\Phi\) on \(\mathbb Z^D\), and some
    \(\beta_0\lt\beta_{PT}(\Phi)\), together with a proof that \(\Phi\)
    fails exponential clustering at \(\beta_0\) in the sense of the Def
    section (for instance a rigorous non-exponential lower bound, such as a
    power law, on some connected correlation function at \(\beta_0\), or a
    proof of long-range order at \(\beta_0\)) -- establishing
    \(\beta_c(\Phi)\lt\beta_{PT}(\Phi)\) for that one \(\Phi\) and refuting
    the universally-quantified claim above, even leaving every other
    \(\Phi\) undecided.

    The following count as progress, not as a solution, unless combined
    with a proof removing the stated restriction: (i) an improved but still
    \(\Phi\)-independent threshold \(\beta^{**}(D,J)\gt\beta^*(D,J)\),
    without a proof that \(\beta^{**}(D,J)\ge\beta_{PT}(\Phi)\) for every
    \(\Phi\) of that norm bound (exactly the tightening Kliesch, Gogolin,
    Kastoryano, Riera, and Eisert's own Sec. III.1 flags as future work,
    Known section); (ii) a proof restricted to commuting \(\Phi\), or to
    one specific solvable model (e.g.\ the classical Ising Hamiltonian on
    \(\mathbb Z^D\), whose classical sharp-threshold behavior is a separate
    body of results not verified as part of this entry); (iii) a proof of a
    weaker decay -- polynomial, or exponential only outside a shrinking
    neighborhood of \(\beta_{PT}(\Phi)\) -- throughout
    \((0,\beta_{PT}(\Phi))\), rather than exponential decay with
    \(L\)-uniform \(C(\beta,a),\xi(\beta)\) at every such \(\beta\); (iv) a
    proof of the claim conditional on an unproven extra hypothesis (e.g.\
    an assumed spectral gap, or an assumed decay-of-correlations input of
    the kind Capel, Moscolari, Teufel, and Wessel's theorems take as given,
    Known section) without removing that hypothesis; (v) a proof or
    disproof of the analogous statement obtained by operationalizing "the
    actual phase transition" via uniqueness of the infinite-volume KMS
    state, or via long-range order in a specified order parameter, instead
    of via \(\beta_{PT}(\Phi)\) (Def section), without also settling the
    claim exactly as stated above.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
