# QTD022 Finite-Bath Landauer Erasure Sharp Constant

```xml
<problem id="QTD022">
  <status>open</status>
  <name>Finite-Bath Landauer Erasure Sharp Constant</name>
  <keys>landauer principle landauer erasure landauer bound landauer limit finite-size correction finite-size corrections finite-size reservoir finite bath finite-dimensional reservoir n-qubit bath qubit erasure information erasure cost entropy production excess dissipation extra dissipation excess heat reeb-wolf reeb wolf bound tight bound sharp constant optimal constant quadratic convergence quadratic scaling 1/n^2 scaling interacting environment interacting reservoir non-interacting reservoir cooling protocol thermodynamic irreversibility second law of thermodynamics dissipation heat dissipation maximally mixed qubit thermal reservoir gibbs state bath thermal bath Lipka-Bartosik Perarnau-Llobet phase transition reservoir critical reservoir near-critical bath max-cooling permutation swap process fixed system dimension asymptotic erasure sequence relative entropy dimension bound</keys>

  <claim>
    Fix natural-log (nat) entropy throughout. For \(n\ge1\) let
    \(\mathcal P_n\) be the set of pairs \((H,U)\) with \(H\) an arbitrary
    Hermitian operator on \((\mathbb C^2)^{\otimes n}\) (no locality or
    interaction restriction) and \(U\) an arbitrary unitary on
    \(\mathbb C^2\otimes(\mathbb C^2)^{\otimes n}\); for \((H,U)\in\mathcal
    P_n\) let \(q(H,U)\in[0,\tfrac12]\), \(\Delta S(H,U)\le0\), and
    \(\Sigma(H,U)\ge0\) be the residual excited-state population, system
    entropy change, and entropy production of the erasure process
    \((H,U)\) (all defined below). Call a sequence
    \((H_n,U_n)_{n\ge1}\), \((H_n,U_n)\in\mathcal P_n\), an
    \emph{asymptotic erasure sequence} if \(q(H_n,U_n)\to0\) as
    \(n\to\infty\), and define
    \[
      c^\star:=\inf\Bigl\{\ \limsup_{n\to\infty}n^2\,\Sigma(H_n,U_n)\ :\
      (H_n,U_n)_{n\ge1}\text{ an asymptotic erasure sequence}\ \Bigr\}.
    \]
    By known facts below, \(2\le c^\star\le2\pi^2\). Determine whether
    \(c^\star=2\).

    This is the weaker, precisely quantified reading of survey lead #22
    ("the extra dissipation scales as \(c/n^2\) ... pinned to \(2\le
    c\le2\pi^2\)"), phrased as the sharp dichotomy directly beneath the
    two proven bounds; see known, "Reading note (claim-strength choice)."
  </claim>

  <def>
    Von Neumann entropy \(S(\rho):=-\Tr[\rho\ln\rho]\), natural logarithm
    throughout (nats), matching the exclusive convention of both refs
    below.

    Binary entropy \(h(q):=-q\ln q-(1-q)\ln(1-q)\) for \(q\in(0,\tfrac12]\),
    \(h(0):=0\).

    For \(n\ge1\), bath Hilbert space \(\mathcal H_B^{(n)}:=(\mathbb
    C^2)^{\otimes n}\), \(\dim\mathcal H_B^{(n)}=2^n\) ("an \(n\)-qubit
    bath"). \(\mathcal P_n\) is the set of pairs \((H,U)\), \(H=H^\dagger\)
    any Hermitian operator on \(\mathcal H_B^{(n)}\), \(U\) any unitary on
    \(\mathbb C^2\otimes\mathcal H_B^{(n)}\) (system qubit tensor bath); no
    constraint (e.g. locality, few-body structure) beyond Hermiticity
    resp. unitarity is imposed on \(H\) or \(U\).

    Inverse temperature is fixed at \(\beta=1\), without loss of
    generality: replacing \(H\) by \(H/\beta\) for any \(\beta\gt0\)
    reproduces the same thermal state and the same achievable set of
    \((\Delta S,\Sigma)\) pairs for any \(\beta\), since \(H\) ranges over
    \emph{all} Hermitian operators on \(\mathcal H_B^{(n)}\); this
    matches Reeb and Wolf (2014, full citation in refs), Theorem 6, which
    holds unchanged for every \(\beta\in[-\infty,+\infty]\).

    For \((H,U)\in\mathcal P_n\): \(\tau(H):=e^{-H}/\Tr[e^{-H}]\) (thermal
    bath state); \(\rho_S:=\tfrac12\mathbb I_2\) (initial maximally mixed
    system qubit, fixed, independent of \(n\)); \(\sigma_{SB}(H,U):=U(\rho_S
    \otimes\tau(H))U^\dagger\); \(\sigma_S(H,U):=\Tr_B\sigma_{SB}(H,U)\),
    \(\sigma_B(H,U):=\Tr_S\sigma_{SB}(H,U)\); \(q(H,U):=\) the smaller of
    the two eigenvalues of \(\sigma_S(H,U)\), so \(q(H,U)\in[0,\tfrac12]\)
    and \(S(\sigma_S(H,U))=h(q(H,U))\); \(\Delta S(H,U):=S(\sigma_S(H,U))-
    S(\rho_S)=h(q(H,U))-\ln2\) (system entropy change; \(\le0\) since
    \(h(q)\le\ln2=h(\tfrac12)\) for every \(q\), with equality only at
    \(q=\tfrac12\)); \(Q(H,U):=\Tr[H(\sigma_B(H,U)-\tau(H))]\) (heat
    dissipated into the bath); and
    \[
      \Sigma(H,U):=Q(H,U)+\Delta S(H,U).
    \]
    \(\Sigma(H,U)\ge0\) always, by the general equality \(\Sigma(H,U)=
    I(\mathsf S:\mathsf B)_{\sigma_{SB}}+D(\sigma_B(H,U)\|\tau(H))\) (both
    terms non-negative), which is the content of Lipka-Bartosik and
    Perarnau-Llobet (2025, full citation in refs), Eq. (5), itself
    citing Reeb and Wolf (2014), Theorem 3 [their Eq. (22), in the
    opposite-sign convention \(\Delta S_{\rm RW}:=S(\rho_S)-S(\rho_S')\ge0\)
    for erasure, so \(\Delta S_{\rm RW}(H,U)=-\Delta S(H,U)\) and
    \(\Sigma(H,U)=\beta\Delta Q-\Delta S_{\rm RW}\) in that paper's
    notation]. \(\Sigma\) is the quantity the survey lead calls "the
    extra dissipation": the heat paid beyond the strict Landauer minimum
    \(-\Delta S(H,U)\).

    \(q(H_n,U_n)\to0\) as \(n\to\infty\) is called an \emph{asymptotic
    erasure sequence} (claim above): the qubit is driven arbitrarily
    close to its pure ground state as the bath grows, i.e.
    \(\Delta S(H_n,U_n)\to-\ln2\).

    \(c^\star\) (claim above) is thus the best asymptotic constant, over
    every way of asymptotically completely erasing the qubit with an
    \(n\)-qubit bath of unrestricted Hamiltonian, at which \(\Sigma\)
    can be driven to \(0\) at rate \(1/n^2\); \(c^\star\in[0,\infty]\) is
    well-defined (non-empty infimum set, by known fact 2 below).
  </def>

  <known>
    1. General finite-size lower bound (Reeb and Wolf, "An improved
    Landauer Principle with finite-size corrections," New Journal of
    Physics 16, 103011 (2014) [full citation in refs], Theorem 6, Eq.
    (36), first case, together with the explicit valid choice \(N=
    \tfrac14\log^2(d-1)+1\) given in the sentence immediately following
    Eq. (36)). For any process in the sense of their Theorem 3 (system of
    any dimension, reservoir of Hilbert-space dimension \(2\le d\lt
    \infty\) and arbitrary Hamiltonian, initially uncorrelated product
    state with the reservoir thermal, joint unitary evolution — exactly
    matching \((H,U)\in\mathcal P_n\) with \(d=2^n\) here) and any \(\Delta
    S_{\rm RW}\ge0\), \(\beta\Delta Q\ge\Delta S_{\rm RW}+(\Delta S_{\rm
    RW})^2/(2N)\) for \(N=\tfrac14\log^2(d-1)+1\), i.e.
    \[
      \Sigma\ \ge\ \frac{2(\Delta S_{\rm RW})^2}{\log^2(d-1)+4}
    \]
    (here and in fact 2 below, "\(\log\)" reproduces the source papers'
    own symbol for the natural logarithm, identical to the "\(\ln\)"
    fixed in def above; both papers state this convention explicitly).
    Lipka-Bartosik and Perarnau-Llobet (2025, full citation in refs)
    restate exactly this inequality as their Eq. (11), attributing it to
    Reeb and Wolf, and describe it (their text immediately below Eq.
    (11)) as "valid for sufficiently large \(n\) and appl[ying] to any
    environment, regardless of the specific system-environment
    interaction and process being implemented" — i.e. unconditionally on
    all of \(\mathcal P_n\), not merely on some restricted subfamily.
    Substituting \(d=2^n\) and \(\Delta S_{\rm RW}=-\Delta S(H_n,U_n)\to
    \ln2\) (asymptotic erasure sequence) gives, since \(\log(2^n-1)=n\ln2+
    o(1)\),
    \[
      n^2\Sigma(H_n,U_n)\ \ge\ \frac{2n^2(\Delta S_{\rm RW}(n))^2}
      {\log^2(2^n-1)+4}\ \xrightarrow[n\to\infty]{}\ \frac{2(\ln2)^2}
      {(\ln2)^2}=2,
    \]
    so \(\liminf_n n^2\Sigma(H_n,U_n)\ge2\) for every asymptotic erasure
    sequence, hence \(c^\star\ge2\). This substitution and limit are
    performed here by combining Theorem 6/Eq. (11) with \(d=2^n\) and
    \(n\to\infty\); the closed asymptotic value \(2\) is not stated in
    this form in either source.

    2. Explicit achieving construction (Lipka-Bartosik and Perarnau-Llobet
    2025, Sec. III.2 and Appendix B, Eqs. (36)-(37), (43), (72), (75),
    (78)-(79)). For any fixed real \(\alpha\gt2\), the bath Hamiltonian
    with degeneracies \(\Omega_0=1\), \(\Omega_i=2^{i-1}\) (\(i\ge1\)) and
    energies \(\beta\epsilon_i=\log\Omega_{i+1}-\log r_i\), \(r_i:=1+
    n^{-\alpha}-\cos(2\pi i/n)\) [their Eqs. (36), (43)], together with
    the permutation unitary \(U\) that maximizes the system's ground-state
    occupation (the "max-cooling permutation," their Eq. (37)), define a
    pair \((H_n,U_n)\in\mathcal P_n\) with residual population \(q(H_n,
    U_n)=\tfrac12n^{-\alpha}\to0\) (their Eq. (72): an asymptotic erasure
    sequence for every fixed \(\alpha\gt2\)) and, by their fully worked
    derivation culminating in Eq. (79),
    \[
      \Sigma(H_n,U_n)\ \le\ \frac{2\pi^2}{n^2}+o(1/n^2)\qquad(n\to\infty),
    \]
    where their own notation "\(\mathcal O(1/n^2)\)" is explicitly defined
    (text below their Eq. (16)) to mean terms vanishing strictly faster
    than \(n^{-2}\), i.e. \(o(1/n^2)\) in standard notation. Hence
    \(\limsup_n n^2\Sigma(H_n,U_n)\le2\pi^2\) for this sequence, so
    \(c^\star\le2\pi^2\).

    Combined, facts 1-2 give \(2\le c^\star\le2\pi^2\), as used in claim.

    3. Neither cited source claims the constant \(2\pi^2\) is optimal.
    Lipka-Bartosik and Perarnau-Llobet (2025), Conclusion, state only
    that the \(1/n^2\) \emph{scaling} (exponent) is optimal — "This is in
    fact the best possible scaling of the entropy production with \(n\),
    which follows from the lower bound derived in Ref. [Reeb-Wolf]" — a
    claim about the exponent \(-2\), citing exactly fact 1's \(O(1/n^2)\)
    behavior, not a claim that \(2\pi^2\) itself is the optimal constant.
    Their Abstract and Introduction repeat this same "best possible
    scaling" language without ever asserting exact optimality of the
    prefactor \(2\pi^2\).

    4. Original provenance of the open gap (Reeb and Wolf 2014, Appendix
    B, paragraph immediately following their Eq. (97)). Reeb and Wolf
    prove tightness of their Theorem 6 bound for \(\Delta S_{\rm RW}\ge0\)
    (i.e. that some process attains \(\beta\Delta Q=\Delta S_{\rm RW}+
    M(\Delta S_{\rm RW},d)\), where \(M\) is their exact tight
    relative-entropy-vs-dimension function, Eq. (34), which is \(\ge\)
    the quadratic bound used in fact 1) \emph{only} via a swap process
    (their Example 1) that requires the system dimension to equal the
    reservoir dimension, \(d_S=d\) — not the \(d_S=2\) fixed, \(d=2^n\to
    \infty\) regime of \(\mathcal P_n\) here. They state explicitly: "We
    leave open the question whether for \(\Delta S\gt0\) the bound can be
    exactly tight even if one fixes \(d_S\) independently of \(d\)." This
    sentence is the original statement of exactly the obstruction that
    keeps \(c^\star\) open: it is not known whether the tight,
    dimension-only-dependent bound underlying fact 1 is achievable at all
    once the system dimension is pinned to \(2\) independently of the
    growing bath dimension \(d=2^n\). Fact 2 makes partial progress on
    this exact question (an explicit \(d_S=2\)-independent-of-\(d\)
    construction attaining the correct \(1/n^2\) order) without matching
    the constant \(2\) from fact 1's bound.

    5. Reading note (claim-strength choice). Survey lead #22 states the
    range "\(2\le c\le2\pi^2\)" as if \(c\) were already a well-defined
    single number; facts 1-2 establish this range rigorously only as
    bounds on \(c^\star\) as defined in claim (an infimum over limsups of
    an explicitly quantified family of processes), which is the weakest
    precise object for which both a matching lower bound (fact 1) and a
    matching achievability (fact 2) are actually proved in the cited
    sources. The claim is phrased as the dichotomy \(c^\star=2\) (rather
    than "determine \(c^\star\) exactly") because \(c^\star=2\) is the
    natural sharp yes/no question directly beneath fact 4's original open
    question, and admits clean proof/disproof conditions (ask below);
    "determine \(c^\star\) exactly" is not by itself a provable/
    disprovable assertion.

    6. Residual doubt (recorded, not resolved by further search per this
    entry's instructions). (i) No search was performed for work after
    Lipka-Bartosik and Perarnau-Llobet's November 2024 posting (or its
    2025 publication) that might improve the upper bound below \(2\pi^2\),
    prove a matching lower bound, or otherwise resolve \(c^\star=2\); the
    survey stage is treated as having already screened for this. (ii) The
    asymptotic-constant computation in fact 1 (that Eq. (11) with \(d=2^n\)
    gives leading coefficient exactly \(2\)) is a direct substitution
    performed for this entry, not a closed-form asymptotic stated
    verbatim in either source, though it follows immediately from the
    quoted formulas. (iii) The function \(M(x,d)\) underlying Reeb and
    Wolf's Theorem 6 and its stated tightness (fact 4) is itself imported
    from a companion paper of theirs, cited there as "[RW13]," which was
    not independently read for this entry; this entry relies on Reeb and
    Wolf (2014)'s own restatement and use of it. (iv) Lipka-Bartosik and
    Perarnau-Llobet's main-text Eq. (16) is printed with "\(\ge\)"
    ("\(\Sigma\ge2\pi^2/n^2+\mathcal O(1/n^2)\)"), which if read literally
    contradicts the achievability role the sentence around it claims for
    that equation and contradicts their own Abstract ("\(\Sigma=2\pi^2/
    n^2\) for \(n\gg1\)"); this entry follows the fully worked Appendix B
    derivation, which culminates unambiguously in "\(\le\)" (their Eq.
    (79), quoted in fact 2), as authoritative, and treats the main-text
    Eq. (16) as an apparent sign/direction slip in Lipka-Bartosik and
    Perarnau-Llobet (2025) rather than as a separate, additional claim.
  </known>

  <refs>
    <ref>D. Reeb and M. M. Wolf, An improved Landauer Principle with finite-size corrections, New Journal of Physics 16, 103011 (2014), DOI: 10.1088/1367-2630/16/10/103011, arXiv:1306.4352, used for: Theorem 6 and Eq. (36) with the choice \(N=\tfrac14\log^2(d-1)+1\) (general finite-size lower bound on excess dissipation for any finite-dimensional reservoir, any Hamiltonian, any system dimension); Appendix B, in particular the paragraph following Eq. (97) (proof that the bound is tight for \(\Delta S\ge0\) only via a swap process requiring \(d_S=d\), and the explicit statement that tightness with \(d_S\) fixed independently of \(d\) is left open); Section 3.3 and Section 6 (finite reservoirs cannot achieve exact full-rank-reducing erasure, motivating the \(q\to0\) rather than \(q=0\) formulation used here).</ref>
    <ref>P. Lipka-Bartosik and M. Perarnau-Llobet, Minimizing Dissipation via Interacting Environments: Quadratic Convergence to Landauer Bound, Physical Review Letters 135, 170404 (2025), DOI: 10.1103/5w1r-1nzs, arXiv:2411.00944, used for: Eq. (1) and Eq. (5) (definition of entropy production \(\Sigma:=\beta Q+\Delta S_{\mathsf S}\) and its equality decomposition); Eq. (10)/Eq. (2) (non-interacting-reservoir \(O(1/n)\) lower bound, cited here only as context for why interactions are needed); Eq. (11) (restatement of Reeb-Wolf's Theorem 6 as a general \(O(1/n^2)\) lower bound for any environment); Sec. III.2 and Appendix B, Eqs. (36)-(37), (43), (72), (75), (78)-(79) (explicit \(n\)-qubit bath Hamiltonian and max-cooling permutation achieving \(\Sigma\le2\pi^2/n^2+o(1/n^2)\) for any fixed \(\alpha\gt2\) in \(q=\tfrac12n^{-\alpha}\)); Conclusion (statement that only the \(O(1/n^2)\) exponent, not the constant \(2\pi^2\), is claimed to be optimal).</ref>
  </refs>

  <ask>
    A proof of \(c^\star=2\) must, given the known \(c^\star\ge2\)
    (known fact 1), establish \(c^\star\le2\): for every \(\varepsilon\gt
    0\) exhibit an asymptotic erasure sequence \((H_n,U_n)_{n\ge1}\in
    \prod_n\mathcal P_n\) with \(q(H_n,U_n)\to0\) and \(\limsup_{n\to
    \infty}n^2\Sigma(H_n,U_n)\le2+\varepsilon\) (a single sequence
    attaining \(\limsup_n n^2\Sigma(H_n,U_n)=2\) suffices for all
    \(\varepsilon\) at once). Reaching any fixed constant strictly above
    \(2\) does not establish the claim.

    A disproof must establish \(c^\star\gt2\): exhibit \(\delta\gt0\) and
    prove \(\limsup_{n\to\infty}n^2\Sigma(H_n,U_n)\ge2+\delta\) for
    \emph{every} asymptotic erasure sequence \((H_n,U_n)_{n\ge1}\), i.e. a
    universal lower bound on \(\mathcal P_n\) strictly stronger than known
    fact 1's asymptotic constant \(2\), specific to the regime where the
    system dimension (\(2\)) is fixed independently of the growing bath
    dimension \(d=2^n\). A bound that only rules out a specific
    subfamily of constructions (e.g. only permutation/max-cooling
    processes, or only Hamiltonians of the specific form in known fact 2)
    does not establish the claim.

    The following count as progress, not as a resolution of the claim,
    unless combined with a matching argument as above: (i) an explicit
    asymptotic erasure sequence with \(\limsup_n n^2\Sigma(H_n,U_n)\)
    strictly below \(2\pi^2\) but not shown to equal or approach \(2\)
    (improving known fact 2's construction without reaching the floor);
    (ii) a universal lower bound with constant strictly above \(2\) that
    is not shown to be tight (improving known fact 1's constant without
    a matching construction); (iii) resolving Reeb and Wolf's original
    open question (known fact 4) for some specific finite \(d_S\)
    independent of \(d\), without addressing the \(n\to\infty\) leading
    asymptotic constant \(c^\star\) posed here; (iv) any result that
    depends on locality or few-body restrictions on \(H\) not present in
    \(\mathcal P_n\) (both known bounds place no such restriction).
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
