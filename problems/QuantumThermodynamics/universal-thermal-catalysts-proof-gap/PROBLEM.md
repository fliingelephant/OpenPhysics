# QTD004 Catalytic Universality in the Genuine Catalysis Regime of Thermal Operations

```xml
<problem id="QTD004">
  <status>open</status>
  <name>Catalytic Universality in the Genuine Catalysis Regime of Thermal Operations</name>
  <keys>quantum catalysis thermal operations catalytic thermal operations universal catalysts catalytic universality genuine catalysis regime embezzlement regime multi-copy catalyst many-copy catalyst n-copy catalyst inexact catalysis approximate catalysis second laws of thermodynamics second laws of quantum thermodynamics generalized free energy Renyi divergence Renyi relative entropy energy-incoherent states block-diagonal states diagonal states resource theory of quantum thermodynamics resource theory of athermality majorization-based resource theories thermo-majorization Lipka-Bartosik Skrzypczyk Theorem 2 Lemma 4 gentle unitary lemma Stinespring dilation lemma proof gap invalid lemma broken proof Reviews of Modern Physics catalysis review RMP catalysis review any state catalyzes any transition all states are universal catalysts</keys>

  <claim>
    Fix a finite-dimensional quantum system \(S\) of dimension \(d_S\) with
    Hamiltonian \(H_S=\sum_{i=1}^{d_S}E_i|i\rangle\langle i|_S\) in some
    orthonormal energy eigenbasis \(\{|i\rangle\}_{i=1}^{d_S}\) (the
    \(E_i\in\mathbb R\) need not be distinct), and an inverse temperature
    \(\beta\in(0,\infty)\). Let \(\rho_S,\sigma_S\) be energy-incoherent
    states on \(S\) (defined below), represented by the probability vectors
    \(p=\mathrm{diag}[\rho_S]\), \(q=\mathrm{diag}[\sigma_S]\), and let
    \(g=\mathrm{diag}[\gamma_S]\) be the probability vector of the Gibbs
    state \(\gamma_S\) (defined below). Suppose
    \[
      F_\alpha(p,g) \gt F_\alpha(q,g) \qquad \text{for every }
      \alpha\in[0,\infty),
    \]
    where \(F_\alpha\) is the generalized free energy defined below.

    Then: for every finite-dimensional catalyst system \(C\) of dimension
    \(d_C\) with an arbitrary Hamiltonian \(H_C\), for every
    energy-incoherent state \(\omega_C\) on \(C\) (represented by
    \(c=\mathrm{diag}[\omega_C]\)), and for every \(\kappa\in(0,1)\), there
    exist an integer \(n_0\) and a real constant \(M\gt0\) (both may depend
    on \(\rho_S,\sigma_S,\omega_C,H_S,H_C,\beta,\kappa\)) such that, for
    every integer \(n\ge n_0\), there is a thermal operation \(T_{SC}\) on
    \(S\otimes C\) at inverse temperature \(\beta\) (defined below) and a
    state \(\sigma'_{SC}\) on \(S\otimes C\) with
    \[
      T_{SC}\bigl(\rho_S\otimes\omega_C^{\otimes n}\bigr)=\sigma'_{SC},
      \qquad
      \operatorname{tr}_C[\sigma'_{SC}]=\sigma_S,
    \]
    \[
      \bigl\|\operatorname{tr}_S[\sigma'_{SC}]-\omega_C^{\otimes n}\bigr\|_1
      \le M\,e^{-n^\kappa}.
    \]
  </claim>

  <def>
    System and Hamiltonian. \(S\) is a Hilbert space of finite dimension
    \(d_S\); \(H_S=\sum_{i=1}^{d_S}E_i|i\rangle\langle i|_S\) for some
    orthonormal basis \(\{|i\rangle\}_{i=1}^{d_S}\) of \(S\) and real
    numbers \(E_i\) (possibly repeated, i.e. \(H_S\) may have degenerate
    levels). The Gibbs state of \(S\) at inverse temperature \(\beta\) is
    \(\gamma_S=e^{-\beta H_S}/Z_S\), \(Z_S=\operatorname{tr}e^{-\beta H_S}\).

    Energy-incoherent state and diagonal representation. A state \(\rho_S\)
    on \(S\) is energy-incoherent iff \([\rho_S,H_S]=0\), equivalently
    \(\rho_S=\sum_ip_i|i\rangle\langle i|_S\) for a probability vector
    \(p=(p_1,\dots,p_{d_S})\) (\(p_i\ge0\), \(\sum_ip_i=1\)); write
    \(p=\mathrm{diag}[\rho_S]\), \(p_i=\langle i|\rho_S|i\rangle\), for this
    vector. Likewise \(q=\mathrm{diag}[\sigma_S]\) and
    \(g=\mathrm{diag}[\gamma_S]\), \(g_i=e^{-\beta E_i}/Z_S\gt0\).

    Generalized free energy. For probability vectors
    \(x=(x_1,\dots,x_{d_S})\) and \(g=(g_1,\dots,g_{d_S})\) with every
    \(g_i\gt0\), the Rényi relative entropy is, for
    \(\alpha\in(0,1)\cup(1,\infty)\),
    \[
      D_\alpha(x\|g):=\frac{1}{\alpha-1}\log\sum_ix_i^\alpha g_i^{1-\alpha},
    \]
    with the \(\alpha\to1\) limit
    \(D_1(x\|g):=\sum_ix_i(\log x_i-\log g_i)\) and the \(\alpha\to0\)
    limit \(D_0(x\|g):=-\log\sum_{i:x_i\gt0}g_i\). The generalized free
    energy of order \(\alpha\ge0\) of \(x\) relative to \(g\) is
    \[
      F_\alpha(x,g):=\frac{1}{\beta}\bigl[D_\alpha(x\|g)-\log Z_S\bigr],
    \]
    so that \(F_\alpha(g,g)=-\beta^{-1}\log Z_S\) for every \(\alpha\).

    Thermal operation. For a finite-dimensional system \(X\) with
    Hamiltonian \(H_X\), a thermal operation on \(X\) at inverse
    temperature \(\beta\) is a CPTP map
    \[
      \Phi_X(\rho)=
      \operatorname{tr}_B\!\left[
        U_{XB}(\rho\otimes\tau_B)U_{XB}^\dagger
      \right],
      \qquad
      [U_{XB},\,H_X\otimes I_B+I_X\otimes H_B]=0,
    \]
    where \(B\) is a finite-dimensional bath with Hamiltonian \(H_B\) and
    Gibbs state \(\tau_B=e^{-\beta H_B}/\operatorname{tr}e^{-\beta H_B}\);
    \(B\), \(H_B\), and \(U_{XB}\) are otherwise unconstrained and may be
    chosen freely for each \(\Phi_X\). A thermal operation \(T_{SC}\) on the
    composite \(S\otimes C\) is a thermal operation per this definition
    with \(X=S\otimes C\) and \(H_X=H_S\otimes I_C+I_S\otimes H_C\).

    Catalyst system. \(C\) is a second, independent finite-dimensional
    Hilbert space of dimension \(d_C\) with its own Hamiltonian \(H_C\)
    (unrelated to \(H_S\)); \(\omega_C\) is an energy-incoherent state on
    \(C\) (\([\omega_C,H_C]=0\)) with diagonal representation
    \(c=\mathrm{diag}[\omega_C]\). \(\omega_C^{\otimes n}\) is the state of
    \(n\) independent identical copies of \(C\), on \(C^{\otimes n}\) with
    Hamiltonian \(\sum_{k=1}^nH_C^{(k)}\) (energy additive across copies).

    Trace norm. For a Hermitian operator \(A\),
    \(\|A\|_1:=\max\{\operatorname{tr}[PA]\mid0\preceq P\preceq I\}\),
    equivalently the sum of the absolute values of the eigenvalues of
    \(A\).
  </def>

  <known>
    Lipka-Bartosik and Skrzypczyk, All states are universal catalysts in
    quantum thermodynamics, Physical Review X 11, 011061 (2021), Section
    III ("Results"), state exactly the claim above as their Theorem 2, in
    their Eqs. (13)-(16), with \(p=\mathrm{diag}[\rho_S]\),
    \(q=\mathrm{diag}[\sigma_S]\), \(g=\mathrm{diag}[\gamma_S]\),
    \(c=\mathrm{diag}[\omega_C]\) matching the notation above and their
    \(F_\alpha\) (their Eq. (6)) matching the definition above. The same
    Section III also states, immediately before Theorem 2, an unconditional
    companion result as their Theorem 1 ("embezzlement regime"): for every
    \(\rho_S,\sigma_S\) (no second-laws hypothesis at all) and every
    catalyst state \(\omega_C\), there is, for every \(n\), a thermal
    operation with system error
    \(\varepsilon_S=\|\operatorname{tr}_C[\sigma'_{SC}]-\sigma_S\|_1=
    O(1/n)\) (their Eq. (12)) and catalyst error
    \(\varepsilon_C=\|\operatorname{tr}_S[\sigma'_{SC}]-\omega_C^{\otimes
    n}\|_1=O(1/\sqrt n)\) (their Eq. (11)), both vanishing only
    polynomially and neither exactly, in contrast to Theorem 2's exactly-
    \(\sigma_S\) system output and sub-exponential catalyst error. This
    entry's claim is Theorem 2 alone; Theorem 1 carries no second-laws
    restriction on \(\rho_S,\sigma_S\) and is a logically distinct
    statement from the claim above.

    Lipka-Bartosik and Skrzypczyk defer the proofs of both theorems to
    their Appendix C, built on a shared preprocessing/postprocessing
    protocol (the unlabeled part of Appendix C preceding its subsection
    "a. Embezzlement regime") that converts the \(n\)-copy catalyst
    \(\omega_C^{\otimes n}\) into an intermediate state \(\eta_C\), applies
    a catalytic step specific to each regime, and then reverses the
    conversion to approximately restore the catalyst. This reversal step,
    in the shared construction and again explicitly within "a.
    Embezzlement regime", invokes their Lemma 4 (Appendix B): for a
    thermal operation \(T\) on a block-diagonal system with
    \(\|T[\rho_S]-\sigma_S\|_1\le\epsilon\), there exists a unitary
    \(U_{SB}\) conserving \(H_S+H_B\), acting on \(S\) together with the
    same environment \(B\), such that
    \(\|U_{SB}(\rho_S\otimes\tau_B)U_{SB}^\dagger-\sigma_S\otimes\tau_B\|_1
    \le\epsilon\) -- i.e. any approximate thermal operation can be dilated
    to an energy-conserving unitary that induces no larger an error on the
    environment than the original error on the system. Within "b. Genuine
    catalysis regime" the exact catalytic step is instead constructed
    directly from thermo-majorization, but its stated construction of the
    required permutation unitary explicitly reuses "the methods described
    in the proof of Lemma 4".

    Lipka-Bartosik, Wilming, and Ng, Catalysis in quantum information
    theory, Reviews of Modern Physics 96, 025005 (2024), Section V.C.3
    ("Thermal operations"), report on this result directly: "Ref.
    (Lipka-Bartosik and Skrzypczyk, 2021a) claimed to formally prove that
    for any pair of energy-incoherent states satisfying the second laws of
    Eq. (148) for \(\alpha\ge0\), any energy-incoherent quantum state can
    act as an approximate catalyst for the transition, as long as enough
    copies of the catalyst are available. Unfortunately, it was later
    discovered that the proof of this statement has a gap (i.e. Lemma 4
    therein is not valid), which leaves this claim unresolved." Their own
    Eq. (148), restricted to \(\alpha\ge0\), is the same second-laws
    hypothesis \(F_\alpha(p,g)\gt F_\alpha(q,g)\ \forall\alpha\ge0\) used
    above. This is the direct source of the claim above and the direct
    report that its only published proof is invalid; the review does not
    supply, cite, or attempt a replacement proof, and it does not extend
    this specific report of invalidity to Theorem 1 above.

    The necessity direction adjacent to this claim is separately
    established and is not in question here: Brandão, Horodecki, Ng,
    Oppenheim, and Wehner, The second laws of quantum thermodynamics,
    Proceedings of the National Academy of Sciences 112 (11), 3275-3279
    (2015), prove that \(F_\alpha(p,g)\ge F_\alpha(q,g)\) for every
    \(\alpha\ge0\) is necessary for the existence of any catalyst realizing
    \(\rho_S\to\sigma_S\) with arbitrarily small, nonzero error; this is
    cited by Lipka-Bartosik and Skrzypczyk (their Ref. [11]) for their Eq.
    (5), the "only if" half of the second laws that their Theorem 2
    attempts to strengthen to "if, for every energy-incoherent catalyst
    shape, given enough copies". Horodecki and Oppenheim, Fundamental
    limitations for quantum and nanoscale thermodynamics, Nature
    Communications 4, 2059 (2013), prove the thermo-majorization criterion
    for exact, catalyst-free thermal-operation convertibility of
    block-diagonal states (cited by Lipka-Bartosik and Skrzypczyk as their
    Ref. [22]) that Lipka-Bartosik and Skrzypczyk's protocol repeatedly
    invokes to construct the catalytic step itself; this criterion is not
    in question here.

    Neither source located for this entry supplies a corrected proof, a
    replacement for Lemma 4, or a disproof of Theorem 2. The arXiv record
    for Lipka-Bartosik and Skrzypczyk (arXiv:2006.16290) carries only a v1
    (29 June 2020) and a v2 (25 February 2021, matching the published PRX
    text) with no later corrective revision, and the Reviews of Modern
    Physics passage quoted above is the only discussion of this specific
    result found in that review: it ends with "which leaves this claim
    unresolved" and the review's next paragraph moves to an unrelated
    topic (elementary and Markovian thermal operations with limited
    control).
  </known>

  <refs>
    <ref>P. Lipka-Bartosik and P. Skrzypczyk, All states are universal catalysts in quantum thermodynamics, Physical Review X 11, 011061 (2021), DOI: 10.1103/PhysRevX.11.011061, arXiv:2006.16290, used for Theorem 1 (Section III, "Results", Eqs. (10)-(12)) and Theorem 2 (Section III, Eqs. (13)-(16), the claim above), the definitions of \(F_\alpha\) (Eq. (6)) and the trace norm, and for Lemma 4 (Appendix B) as the step invoked in the shared preprocessing/postprocessing construction of Appendix C and again explicitly in Appendix C's subsections "a. Embezzlement regime" and "b. Genuine catalysis regime".</ref>
    <ref>P. Lipka-Bartosik, H. Wilming, and N. H. Y. Ng, Catalysis in quantum information theory, Reviews of Modern Physics 96, 025005 (2024), DOI: 10.1103/RevModPhys.96.025005, arXiv:2306.00798, Section V.C.3 ("Thermal operations"), used for the explicit report that the proof of Lipka-Bartosik and Skrzypczyk's Theorem 2 has a gap because its Lemma 4 is not valid, leaving the claim above unresolved, and for Eq. (148) restated above as the second-laws hypothesis.</ref>
    <ref>F. G. S. L. Brandão, M. Horodecki, N. Ng, J. Oppenheim, and S. Wehner, The second laws of quantum thermodynamics, Proceedings of the National Academy of Sciences 112 (11), 3275-3279 (2015), DOI: 10.1073/pnas.1411728112, arXiv:1305.5278, used for the proof that the generalized free energy conditions \(F_\alpha\ge0\) for \(\alpha\ge0\) are necessary for catalytic convertibility with arbitrarily small error, cited by Lipka-Bartosik and Skrzypczyk as their Ref. [11] for the necessity direction that this entry's claim does not put in question.</ref>
    <ref>M. Horodecki and J. Oppenheim, Fundamental limitations for quantum and nanoscale thermodynamics, Nature Communications 4, 2059 (2013), DOI: 10.1038/ncomms3059, arXiv:1111.3834, used for the thermo-majorization criterion for exact, catalyst-free thermal-operation convertibility of block-diagonal states, cited by Lipka-Bartosik and Skrzypczyk as their Ref. [22] and repeatedly invoked in their protocol's construction of the catalytic step.</ref>
  </refs>

  <ask>
    A proof must establish the claim above exactly as stated, for the full
    domain: every finite \(d_S\), every \(H_S\), every
    \(\beta\in(0,\infty)\), every energy-incoherent pair \(\rho_S,\sigma_S\)
    with \(F_\alpha(p,g)\gt F_\alpha(q,g)\) for every \(\alpha\in[0,\infty)\),
    every finite \(d_C\), every \(H_C\), every energy-incoherent
    \(\omega_C\), and every \(\kappa\in(0,1)\); either by repairing Lemma 4
    above, by replacing it with a different construction that still
    yields both an exactly-\(\sigma_S\) system output and a catalyst error
    bounded by \(M\,e^{-n^\kappa}\), or by any other valid argument.

    A disproof must exhibit \(d_S,H_S,\beta,\rho_S,\sigma_S\) satisfying
    the second-laws hypothesis above, together with \(d_C,H_C,\omega_C\)
    energy-incoherent and \(\kappa\in(0,1)\), and prove that no \(n_0,M\)
    exist making the stated conclusion hold for every \(n\ge n_0\);
    equivalently, prove that for every \(M\gt0\) there are arbitrarily
    large \(n\) for which every thermal operation \(T_{SC}\) on
    \(S\otimes C\) with
    \(\operatorname{tr}_C[T_{SC}(\rho_S\otimes\omega_C^{\otimes n})]
    =\sigma_S\) exactly satisfies
    \(\|\operatorname{tr}_S[T_{SC}(\rho_S\otimes\omega_C^{\otimes n})]
    -\omega_C^{\otimes n}\|_1\gt M\,e^{-n^\kappa}\). Because the claim
    above is a single statement universally quantified over these
    instances, one such counterexample instance disproves it.

    The following count as progress, not as a solution to the claim,
    unless combined with a proof removing the stated restriction: (i) a
    valid proof restricted to a proper subclass of
    \(\rho_S,\sigma_S,\omega_C,H_S,H_C\), such as fixed or bounded
    \(d_S\) or \(d_C\), a full-rank or otherwise structurally restricted
    \(\omega_C\), or a fixed \(\kappa\); (ii) a corrected replacement for
    Lemma 4 that yields a weaker catalyst-error rate than \(e^{-n^\kappa}\)
    (for instance \(O(1/\sqrt n)\), matching only Theorem 1's
    embezzlement-regime rate) or that requires \(\varepsilon_S\gt0\)
    rather than an exactly-\(\sigma_S\) system output; (iii) a proof or
    disproof of Theorem 1 (the embezzlement-regime claim recalled above),
    which carries no second-laws hypothesis and is a logically distinct
    statement from the claim above; (iv) numerical evidence for specific
    finite \(d_S,d_C,n\), of the kind already reported in Lipka-Bartosik
    and Skrzypczyk's own Appendix D, without an analytic bound valid for
    every sufficiently large \(n\); (v) results for energy-coherent
    \(\rho_S\), \(\sigma_S\), or \(\omega_C\), a regime this entry's claim
    does not address (see the companion entries QTD001 and QTD003 on
    energy-coherent convertibility).
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
