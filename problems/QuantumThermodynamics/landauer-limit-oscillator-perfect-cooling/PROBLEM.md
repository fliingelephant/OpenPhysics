# QTD021 Landauer-Saturating Perfect Cooling of a Harmonic Oscillator at Finite Control Complexity

```xml
<problem id="QTD021">
  <status>open</status>
  <name>Landauer-Saturating Perfect Cooling of a Harmonic Oscillator at Finite Control Complexity</name>
  <keys>landauer limit landauer bound landauer principle landauer's principle landauer cost perfect cooling ground state cooling ground-state cooling harmonic oscillator cooling oscillator cooling cooling a quantum system third law of thermodynamics nernst unattainability principle nernst's principle carnot-landauer limit carnot-landauer bound control complexity effective dimension finite control complexity finite effective dimension finite means diverging time diverging control complexity coherent control paradigm quantum thermodynamics cooling protocol infinite-dimensional quantum system continuous variable quantum system continuous-variable cooling gaussian operations non-gaussian operations gaussian unitary beam splitter two-level exchange protocol Taranto Bakhshinezhad Bluhm Silva Friis Lock Vitagliano Binder Debarba Schwarzhans Clivaz Huber landauer versus nernst landauer vs nernst true cost of cooling a quantum system PRX Quantum 4 010332 quantum battery thermal machine reeb-wolf inequality reeb wolf information erasure bit erasure cost quantum information erasure appendix E open question single-mode harmonic oscillator bosonic mode cooling work source coherent work source saturating the landauer bound landauer-saturating protocol energy cost of cooling asymptotic ground state preparation pure state preparation cost hilbert hotel cooling protocol sufficiency for optimal cooling qtd021</keys>

  <claim>
    Fix $\omega_S\gt0$ and $\beta\in(0,\infty)$, and let $H_S:=\omega_S a^\dagger a$ be
    the Hamiltonian of a single harmonic-oscillator mode $S$ (def below). A
    *finite-step protocol* is a pair $\bigl(M,\{U^{(k)}\}_{k\ge1}\bigr)$ -- an
    auxiliary system $M$ with a discrete-spectrum Hamiltonian $H_M$ on a separable
    Hilbert space $\mathcal H_M$, together with a sequence of unitaries $U^{(k)}$
    on $\mathcal H_S\otimes\mathcal H_M$ -- such that the effective dimension
    (def below) obeys $d^{\rm eff}(U^{(k)})\lt\infty$ for every $k=1,2,\ldots$
    ("finite means at every step"). Call such a protocol *cooling* if
    $\lim_{N\to\infty}\bigl\|\varrho_S^{(N)}-|0\rangle\langle0|_S\bigr\|_1=0$ (def
    below), and for a cooling protocol let
    $G^{(N)}:=\beta\Delta E_M^{(N)}-\widetilde\Delta S_S^{(N)}$ ($G^{(N)}\ge0$
    for every $N$, known) be its Landauer gap after $N$ steps. Let

    \[
    \Gamma^\ast:=\inf\Bigl\{\ \liminf_{N\to\infty}G^{(N)}\ :\
    \bigl(M,\{U^{(k)}\}_{k\ge1}\bigr)\text{ a cooling finite-step protocol}\
    \Bigr\}.
    \]

    This infimum is over a nonempty set of nonnegative numbers (known below
    exhibits one cooling finite-step protocol with finite $\liminf_N G^{(N)}$),
    so $\Gamma^\ast\in[0,\infty)$ is a well-defined real number.

    Determine whether $\Gamma^\ast=0$ (Landauer's bound is approachable to
    arbitrary precision by a sequence of finite-step protocols that perfectly
    cool $S$ to its ground state, each step of finite effective dimension) or
    $\Gamma^\ast\gt0$ (a strictly positive residual energy cost above Landauer's
    bound is unavoidable for every protocol restricted to finite-effective-
    dimension interactions at each individual step).
  </claim>

  <def>
    Units. $\hbar=k_B=1$ throughout, matching the source's convention. $\mathbb
    N_0:=\{0,1,2,\ldots\}$.

    Target oscillator. $\mathcal H_S$ has orthonormal (Fock) basis
    $\{|n\rangle\}_{n\in\mathbb N_0}$ with mode operators $a|n\rangle=\sqrt
    n\,|n-1\rangle$, $a^\dagger|n\rangle=\sqrt{n+1}\,|n+1\rangle$,
    $[a,a^\dagger]=1$, so $H_S|n\rangle=\omega_S n|n\rangle$.

    Thermal state. For a Hamiltonian $H$ with discrete spectrum bounded below
    and $\mathrm{tr}[e^{-\beta H}]\lt\infty$, $\tau(\beta,H):=e^{-\beta
    H}/\mathrm{tr}[e^{-\beta H}]$; in particular
    $\tau(\beta,H_S)=(1-e^{-\beta\omega_S})\sum_{n\ge0}e^{-\beta\omega_S
    n}|n\rangle\langle n|$.

    Machine and protocol. $M$ carries a Hamiltonian $H_M$ (discrete spectrum,
    bounded below) on a separable Hilbert space $\mathcal H_M$, initially in
    $\tau(\beta,H_M)$, uncorrelated with $S$, at the same $\beta$ as $S$ (the
    standing "coherent-control" assumption of the source). Write
    \[
    \varrho_{SM}^{(0)}:=\tau(\beta,H_S)\otimes\tau(\beta,H_M),\qquad
    \varrho_{SM}^{(k)}:=U^{(k)}\varrho_{SM}^{(k-1)}\bigl(U^{(k)}\bigr)^\dagger\
    (k\ge1),
    \]
    for a sequence of unitaries $U^{(k)}$ on $\mathcal H_S\otimes\mathcal H_M$,
    and set $\varrho_S^{(N)}:=\mathrm{tr}_M\bigl[\varrho_{SM}^{(N)}\bigr]$,
    $\varrho_M^{(N)}:=\mathrm{tr}_S\bigl[\varrho_{SM}^{(N)}\bigr]$.

    Effective dimension (Taranto et al., Definition 1). For a unitary $U$ on a
    Hilbert space $\mathcal K$,
    \[
    d^{\rm eff}(U):=\min\bigl\{\dim\mathcal A:\mathcal A\subseteq\mathcal
    K\text{ a closed subspace},\ U=U_{\mathcal A}\oplus\mathbb
    1_{\mathcal A^\perp}\bigr\},
    \]
    equivalently $d^{\rm eff}(U)=\mathrm{rank}(U-\mathbb1)$, finite iff
    $U-\mathbb1$ has finite rank, i.e. $U$ acts as the identity outside some
    finite-dimensional subspace. (Consequently, if $d^{\rm
    eff}(U^{(k)})\lt\infty$ for every $k\le N$, then $\varrho_{SM}^{(N)}$
    differs from $\varrho_{SM}^{(0)}$ only on a finite-rank perturbation, so
    $\varrho_S^{(N)}$, $\varrho_M^{(N)}$, and every quantity below are
    well-defined and finite for every finite $N$.)

    Von Neumann entropy and trace norm.
    $S(\varrho):=-\mathrm{tr}[\varrho\log\varrho]$;
    $\|X\|_1:=\mathrm{tr}\bigl[\sqrt{X^\dagger X}\,\bigr]$.

    Energy cost and entropy change after $N$ steps.
    \[
    \Delta E_M^{(N)}:=\mathrm{tr}\bigl[H_M\bigl(\varrho_M^{(N)}-\tau(\beta,H_M)
    \bigr)\bigr],\qquad
    \widetilde\Delta S_S^{(N)}:=S\bigl(\tau(\beta,H_S)\bigr)-S\bigl(\varrho_S^{
    (N)}\bigr),
    \]
    matching the source's sign convention (entropy *decrease* of $S$ is counted
    positively). The Landauer gap is $G^{(N)}:=\beta\Delta E_M^{(N)}-
    \widetilde\Delta S_S^{(N)}$.

    "Saturating the Landauer bound" (the asymptotic sense used throughout the
    source for protocols requiring diverging resources) means the vanishing of
    the gap, $G^{(N)}\to0$; the claim's $\Gamma^\ast$ measures how close a
    finite-step cooling protocol can be made to get to this, in the
    $\liminf$ sense.
  </def>

  <known>
    The Landauer bound itself (proven, general). Taranto, Bakhshinezhad, Bluhm,
    Silva, Friis, Lock, Vitagliano, Binder, Debarba, Schwarzhans, Clivaz, and
    Huber, "Landauer vs. Nernst: What is the True Cost of Cooling a Quantum
    System?", PRX Quantum 4, 010332 (2023) [full citation in refs], Eqs.
    (3)-(4) (p.4): for every global unitary on $S\otimes M$ with $M$ initially
    thermal, $\beta\Delta E_M\ge\widetilde\Delta S_S$, with equality iff the
    final mutual information $I(S:M)$ and the final machine's relative entropy
    to its own initial state both vanish; the equality form is proved as their
    Theorem 9 (Appendix A1, p.17), which the source attributes to David Reeb
    and Michael M. Wolf, "An improved Landauer principle with finite-size
    corrections," New Journal of Physics 16, 103011 (2014), their Ref. [29]
    (not independently re-verified here; see residual doubt below). Applied to
    $\varrho'_{SM}=\varrho_{SM}^{(N)}$ for each $N$ (a valid global unitary
    transform of the initial thermal product state), this gives $G^{(N)}\ge0$
    for every $N\ge0$, as used in the claim.

    Necessary structural condition on the machine (proven, general; does not by
    itself resolve the open question). Taranto et al., Corollary 2 (p.6) and
    Theorem 3 (p.7): a cooling protocol that saturates the Landauer bound must
    couple $S$ to an infinite-dimensional subspace of $M$ overall (their
    $d^{\rm eff}\to\infty$ for the single joint unitary obtained by
    recompressing the whole protocol into unit time). This forces $M$, or the
    composite of all steps taken together, to be infinite-dimensional in
    total; it does not bar each *individual* step from having finite
    $d^{\rm eff}(U^{(k)})$, and is explicitly satisfied by the finite-per-step
    protocol below (there $M$ is a single, hence infinite-dimensional,
    harmonic oscillator).

    Finite time and finite complexity together already exclude any finite
    energy cost, hence exclude the (finite) Landauer cost (proven, general;
    this is why the open case is specifically the diverging-time regime).
    Taranto et al., Appendix B2 (pp.19-20): for a *finite* number of steps of
    finite effective dimension, perfect cooling of a full-rank initial state
    forces the energy cost to diverge (concluding sentence, p.20: "we see that
    within the resource trinity of energy, time, and control complexity, if
    the latter two are finite, energy must diverge to asymptotically achieve a
    pure state"). A finite-step protocol in the sense used here is therefore,
    by construction, restricted to a countably infinite (diverging) number of
    steps whenever it is to cool at any finite energy cost at all -- matching
    the source's own framing of this appendix and of Appendix E below.

    An explicit finite-per-step-complexity protocol that perfectly cools but
    leaves a strictly positive gap (proven witness). Taranto et al., Appendix
    E2b (pp.41-43): with $M$ a single harmonic-oscillator mode at the same
    frequency $\omega$ and temperature as $S$, a sequence of two-level
    population-exchange unitaries $U_{SM}^{(k)}$ (their Eq. E25) has
    $d^{\rm eff}\bigl(U_{SM}^{(k)}\bigr)=2$ for every $k$ (each a rank-2
    subspace swap), and $\lim_{k\to\infty}\mathrm{tr}_M\bigl[\varrho_{SM}^{(k)}
    \bigr]=|0\rangle\langle0|_S$ exactly (their Eq. E26, obtained by a
    Hilbert-hotel-style argument that places every one of the infinitely many
    eigenvalues of the joint initial state into the target's ground-state
    subspace). They compute closed-form values for the machine's final energy
    (Eq. E28, giving $\Delta E_M$ in Eq. E30) and for $\widetilde\Delta
    S_S=S[\tau(\beta)]$ (Eq. E31), and show algebraically, directly below Eq.
    E31 on p.43, that $\beta\Delta E_M-\widetilde\Delta S_S\gt0$ strictly, for
    every nonzero initial temperature. This protocol is a finite-step cooling
    protocol in the sense of the claim, with a specific finite value of
    $\liminf_N G^{(N)}$ (their limiting $\beta\Delta E_M-\widetilde\Delta
    S_S$); it shows $\Gamma^\ast$ is a well-defined finite number (the
    nonempty-domain fact used in the claim) but does not itself show
    $\Gamma^\ast=0$: the source states only that "the presented protocol does
    not minimise the energy cost to saturate the Landauer bound," without
    proving that no better finite-step protocol exists (p.43, same paragraph).

    A Landauer-saturating protocol exists, but outside the finite-per-step
    class (proven, contrast case). Taranto et al., Appendix E2a (pp.39-41): a
    diverging-time protocol using Gaussian (beam-splitter-type) swaps between
    $S$ and a growing sequence of machine oscillators of increasing frequency
    does saturate the Landauer bound asymptotically ($\lim_N G^{(N)}=0$,
    matching their finite-dimensional Eq. C19 in the machine-dimension $\to
    \infty$ limit). However, every individual step is a two-mode Gaussian
    unitary (Eq. E14), and the source notes explicitly, discussing precisely
    this fact (Appendix E introduction, p.37): "even a single two-mode swap,
    which cannot cool perfectly at Landauer cost, would have infinite control
    complexity" -- so $d^{\rm eff}(U^{(k)})=\infty$ for every $k$ in this
    protocol; it is not a finite-step protocol in the sense of the claim.
    (Appendix E3, pp.43-44, gives a third construction saturating the
    Landauer bound via a single joint Gaussian unitary, of diverging effective
    dimension, acting in unit time on infinitely many machine modes at once --
    again not finite per-step complexity and not diverging time; recorded
    here only as further context, not as bearing on $\Gamma^\ast$.)

    The open-question statements (verbatim, two independent loci in the same
    paper). Main text, Sec. IV.B.1 "Sufficiency for Optimal Cooling" (p.8):
    "In contrast, we then present a protocol that demonstrates that perfect
    cooling is possible given diverging time and operations acting on only a
    finite effective dimensionality (i.e., using non-Gaussian operations),
    with a finite energy cost that is greater than the Landauer limit; whether
    or not a similar protocol that saturates the Landauer limit exists in this
    setting remains an open question." Appendix E introduction (p.37):
    "Whether or not a similar protocol exists that also saturates the
    Landauer bound remains an open question." Appendix E2b's own closing
    discussion (p.43) restates the same unresolved status in different words:
    "Although the presented protocol does not minimise the energy cost to
    saturate the Landauer bound, we cannot at this point conclusively say that
    it is not possible to do so in this setting." No later section of the
    paper (Discussion, p.13; Conclusions and Outlook, pp.13-14) revisits or
    resolves this question.

    Reading note (scope choices made here; weaker precise formulation
    adopted). (a) The claim requires $d^{\rm eff}(U^{(k)})\lt\infty$ for each
    step individually, without requiring a bound uniform in $k$. The source's
    own witness (E2b) happens to be uniform ($d^{\rm eff}\equiv2$), but its
    own phrasing of the open question, "operations acting on only a finite
    effective dimensionality" (p.8) and "finite control complexity in each of
    the diverging number of steps" (p.41), is naturally read per-operation,
    not uniformly across operations; the per-step (non-uniform) reading is
    the more permissive, logically weaker version of "finite means at every
    step" and is the one adopted in the claim. (b) $M$ is allowed to be any
    discrete-spectrum system on any separable Hilbert space, not restricted to
    harmonic-oscillator modes; Appendix E restricts its own two witness
    constructions to oscillator machines as part of the paper's broader
    continuous-variable programme (p.37, "we now analyse the case of cooling
    infinite-dimensional quantum systems in detail... ensembles of harmonic
    oscillators"), not because the open question is stated as being
    specifically about oscillator machines. Both choices widen the admissible
    protocol class relative to the source's own explicit constructions, making
    $\Gamma^\ast=0$ easier to witness and $\Gamma^\ast\gt0$ correspondingly
    harder to prove -- the weaker, more cautious existence claim.

    Residual doubt (recorded, not resolved by further search per this entry's
    task instructions). This entry verifies only Taranto et al. (2023)
    itself, read in full (main text and all lettered appendices); it does not
    independently confirm the Reeb-Wolf equality form of Landauer's principle
    (above) against Reeb and Wolf (2014) directly, relying on Taranto et
    al.'s own restatement (their Theorem 9). Whether any paper subsequent to
    March 2023 (the arXiv v3 date) has settled the finite-per-step-complexity
    question specifically has not been searched for here, per this task's
    instruction to verify openness against the cited source rather than
    conduct a solving-literature search; the source's own Discussion and
    Conclusions sections present the question as open with no forward
    reference to work in preparation that would resolve it.
  </known>

  <refs>
    <ref>Philip Taranto, Faraj Bakhshinezhad, Andreas Bluhm, Ralph Silva,
    Nicolai Friis, Maximilian P. E. Lock, Giuseppe Vitagliano, Felix C.
    Binder, Tiago Debarba, Emanuel Schwarzhans, Fabien Clivaz, and Marcus
    Huber, "Landauer vs. Nernst: What is the True Cost of Cooling a Quantum
    System?", PRX Quantum 4, 010332 (2023). DOI: 10.1103/PRXQuantum.4.010332.
    arXiv:2106.05151v3 (28 March 2023, matching the published version). Used:
    Eq. (1) (machine initial thermal state); Eqs. (2)-(4) and surrounding text,
    p.4 (coherent-control energy/entropy accounting and the Landauer bound);
    p.6, Corollary 2, and p.7, Definition 1 and Theorem 3 (effective
    dimension; necessity of an infinite-dimensional machine to saturate the
    Landauer bound); p.8, Sec. IV.B.1 "Sufficiency for Optimal Cooling"
    (verbatim open-question statement); Appendix A1, p.17, Theorem 9 (equality
    form of Landauer's principle, attributed to Reeb and Wolf 2014); Appendix
    B2, pp.19-20 (necessity of diverging energy when time and control
    complexity are both finite); Appendix E introduction, p.37 (verbatim
    open-question statement; the two-mode-swap infinite-complexity remark);
    Appendix E2a, pp.39-41, Eq. (E14) (Gaussian diverging-time
    Landauer-saturating protocol, infinite per-step complexity); Appendix E2b,
    pp.41-43, Eqs. (E23)-(E31) (non-Gaussian diverging-time
    finite-per-step-complexity protocol; explicit strict-inequality gap above
    the Landauer bound); Appendix E3, pp.43-44 (unit-time diverging-complexity
    Landauer-saturating protocol, given as contrast).</ref>
  </refs>

  <ask>
    A proof (of $\Gamma^\ast=0$) must, for every $\epsilon\gt0$, exhibit an
    explicit machine $M$ (a Hamiltonian $H_M$ on a separable Hilbert space)
    and an explicit sequence of unitaries $\{U^{(k)}\}_{k\ge1}$ on $\mathcal
    H_S\otimes\mathcal H_M$ with $d^{\rm eff}(U^{(k)})\lt\infty$ for every $k$,
    such that $\lim_{N\to\infty}\|\varrho_S^{(N)}-|0\rangle\langle0|_S\|_1=0$
    and $\liminf_{N\to\infty}G^{(N)}\lt\epsilon$ for that protocol --
    equivalently, an explicit sequence of finite-step cooling protocols with
    Landauer gaps tending to $0$. Exhibiting perfect cooling alone (as
    Appendix E2b, known above, already does) or a gap that merely decreases
    without a proof that it can be driven below every $\epsilon$ does not
    suffice.

    A disproof (of $\Gamma^\ast=0$, i.e. establishing $\Gamma^\ast\gt0$) must
    exhibit $c\gt0$ and prove $\liminf_{N\to\infty}G^{(N)}\ge c$ for every
    finite-step cooling protocol -- a bound holding for every admissible
    discrete-spectrum machine $M$ and every admissible sequence $\{U^{(k)}\}$
    with each $d^{\rm eff}(U^{(k)})\lt\infty$, not merely for a particular
    family of protocols. A quantitative lower bound on $\Gamma^\ast$ itself
    (e.g. as an explicit function of $\beta$ and $\omega_S$) would strengthen,
    but is not required beyond, the strict positivity of some such $c$.

    The following count as progress, not as resolving the claim: (i) a
    finite-step cooling protocol with a smaller gap than the explicit value
    from Appendix E2b (known above), without a proof that a whole sequence of
    such protocols drives the gap to $0$; (ii) a resolution restricted to a
    specific sub-class of machines (e.g. $M$ a fixed finite number of
    oscillator modes, or $M$ restricted to qudits) rather than the general
    class of def above; (iii) a resolution under a bound on $d^{\rm
    eff}(U^{(k)})$ uniform in $k$ (the narrower reading flagged in known,
    "Reading note," point (a)) that does not settle the per-step-only,
    non-uniform reading adopted in the claim; (iv) convergence of
    $\varrho_S^{(N)}$ to the ground state in a topology weaker than trace
    norm (e.g. only in energy, or only on a dense domain), without trace-norm
    convergence; (v) any strengthening of the diverging-time Gaussian protocol
    (Appendix E2a) or of the unit-time diverging-complexity protocol
    (Appendix E3) -- both of which already saturate the Landauer bound but lie
    outside the finite-step class by construction (known above) -- unless it
    is turned into a genuine finite-per-step-complexity construction.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
