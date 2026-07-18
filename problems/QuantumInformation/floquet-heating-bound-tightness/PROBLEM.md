# QTD033 Floquet Heating-Time Bound for the Driven Bose-Hubbard Chain

```xml
<problem id="QTD033">
  <status>open</status>
  <name>Floquet Heating-Time Bound for the Driven Bose-Hubbard Chain</name>
  <keys>floquet heating, floquet prethermalization, periodically driven bose-hubbard model, lattice bosons, unbounded local hilbert space, unbounded local hamiltonian, quasi-conserved effective hamiltonian, heating time, energy absorption rate, exponentially slow heating, high-frequency driving, abanin de roeck ho huveneers, effective hamiltonians prethermalization slow energy absorption, floquet-magnus expansion, renormalization of time-dependent hamiltonian, local norm, k-local hamiltonian, occupation-number truncation, hard-wall truncation, fast arnold diffusion, statistical floquet prethermalization, rubio-abadal, dalla torre dentelski, chirikov vecheslavov, tightness of heating-time bound, matching lower bound on heating rate, quantum kicked rotor, superfluid to mott insulator</keys>

  <claim>
    There exist real numbers \(J,U,v\gt0\), a frequency \(\omega_0\gt0\), and a
    function \(\tau:[\omega_0,\infty)\to(0,\infty)\) that grows faster than
    every polynomial (\(\lim_{\omega\to\infty}\tau(\omega)\,\omega^{-k}=\infty\)
    for every \(k\in\mathbb N\)), such that for every positive integer
    \(n_c\) (occupation-number truncation), every integer \(N\ge3\) (chain
    length), and every \(\omega\ge\omega_0\), the time-periodic Hamiltonian
    \(H_{N,n_c}(t)\) of the Def section (built from \(J,U,v,\omega\)) admits
    a Hermitian operator \(H_*^{N,n_c}(\omega)\) on \(\mathcal
    H_{N,n_c}=(\mathbb C^{n_c+1})^{\otimes N}\) such that, for some
    \(C,\kappa\gt0\) depending only on \(J,U,v\) (not on \(n_c\), \(N\),
    \(\omega\), or \(\tau\)):

    (a) [closeness to the static part]
    \(\displaystyle\frac1N\big\|H_*^{N,n_c}(\omega)-H_{N,n_c}^0\big\|\le C\);

    (b) [locality] writing \(H_*^{N,n_c}(\omega)=\sum_{X}\Phi_X\) with each
    \(\Phi_X\) Hermitian and supported on an interval \(X\) of consecutive
    sites of \(\Lambda_N\) (Def section),
    \(\displaystyle\sup_{i\in\Lambda_N}\sum_{X\ni i}\|\Phi_X\|\,e^{\kappa|X|}
    \le C\);

    (c) [quasi-conservation, i.e. slow heating] for every solution
    \(|\psi(t)\rangle\) of \(i\partial_t|\psi(t)\rangle=H_{N,n_c}(t)
    |\psi(t)\rangle\) with arbitrary \(|\psi(0)\rangle\), and every positive
    integer \(m\) with \(mT\le\tau(\omega)\) (\(T=2\pi/\omega\)),
    \[
      \frac1N\Big|\langle\psi(mT)|H_*^{N,n_c}(\omega)|\psi(mT)\rangle
      -\langle\psi(0)|H_*^{N,n_c}(\omega)|\psi(0)\rangle\Big|\;\le\;1 .
    \]

    Informally: does the Abanin-De Roeck-Ho-Huveneers / Theorem-1-style
    conclusion (a local, quasi-conserved
    effective Hamiltonian that controls the dynamics up to an
    exponentially-in-\(\omega\), or at least super-polynomially-in-\(\omega\),
    long time) survive intact as the occupation-number cutoff \(n_c\) is
    removed, for some genuinely interacting (\(J,U,v\gt0\)) periodically
    driven Bose-Hubbard chain, with a single triple \((C,\kappa,\tau)\) that
    does not degrade with \(n_c\)? A proof that the stronger, exact
    Abanin-De-Roeck-Ho-Huveneers form \(\tau(\omega)=e^{c\omega}\) (some fixed
    \(c\gt0\)) holds under (a)-(c) resolves this claim a fortiori; see Ask.

    Ambiguity flagged. The survey lead this entry is drawn from names two
    open flanks of the 2015-17 exponentially-slow-heating theorems:
    (i) whether the bound is tight, i.e. whether a matching upper bound on
    the achievable heating time (equivalently, a matching lower bound on the
    heating rate) can be proved for the bounded-local-dimension case already
    covered by those theorems; and (ii) whether any rigorous bound survives
    for lattice bosons, where unbounded occupations break the proofs. This
    entry adopts only flank (ii), stated above as claim (a)-(c), because
    neither Abanin, De Roeck, Ho, and Huveneers, Physical Review B 95,
    014112 (2017), nor Ho, Mori, Abanin, and Dalla
    Torre, Annals of Physics 454, 169297 (2023), states flank (i) as a named
    open problem with a precise target statement: both papers prove
    one-directional bounds only (an upper bound on the heating rate /
    lower bound on the heating time) and never formulate, even informally,
    what a matching converse would assert. Constructing such a converse
    claim from scratch, for the bounded-dimension case, would require
    inventing a precise statement not present in either source and risks
    misattribution; flank (ii), by contrast, is stated by name as an
    "outstanding challenge" in Ho, Mori, Abanin, and Dalla Torre, Section
    VII (Known section), giving it a directly citable, unambiguous open
    status. Flank (i) is recorded, not adopted, in the Known section below.

    A second ambiguity concerns the precise formalization of flank (ii)
    itself, since neither source states one. This entry adopts the
    occupation-number-truncation formulation above (claim (a)-(c), with
    \(\tau\) merely super-polynomial and \((C,\kappa,\omega_0,J,U,v)\)
    existentially, not universally, quantified) as the weakest precise
    reading that still captures the content of the named challenge: some
    single, non-degenerating bound must survive the removal of a cutoff
    that ordinary Hilbert-space truncation trivially requires (see Known).
    A stronger universally-quantified version (over all \(J,U,v\gt0\), or
    demanding the exact exponential rate) is a strictly stronger claim and
    is not the one adopted here.
  </claim>

  <def>
    Single-site bosonic Fock space and truncation. Let \(\mathfrak
    h=\ell^2(\mathbb Z_{\ge0})\) with orthonormal basis \(\{|n\rangle\}_{n\ge
    0}\), and let \(b,b^\dagger,\hat n=b^\dagger b\) be the annihilation,
    creation, and number operators, \(b|n\rangle=\sqrt n\,|n-1\rangle\)
    (\(n\ge1\)), \(b|0\rangle=0\). For a positive integer \(n_c\), let
    \(P^{(n_c)}\) be the orthogonal projector of \(\mathfrak h\) onto
    \(\mathrm{span}\{|0\rangle,\ldots,|n_c\rangle\}\cong\mathbb
    C^{n_c+1}\), and set \(b^{(n_c)}:=P^{(n_c)}bP^{(n_c)}\),
    \(\hat n^{(n_c)}:=P^{(n_c)}\hat nP^{(n_c)}\) (\(=\hat nP^{(n_c)}\), since
    \(\hat n\) already preserves the truncated subspace), both operators on
    \(\mathbb C^{n_c+1}\). Directly from
    \(b^{(n_c)}|k\rangle=\sqrt k\,|k-1\rangle\) for \(1\le k\le n_c\) and
    \(b^{(n_c)}|0\rangle=0\), \(\|b^{(n_c)}\|=\|b^{(n_c)\dagger}\|=\sqrt{n_c}\).

    Chain, Hamiltonian, and drive. Fix an integer \(N\ge3\) and let
    \(\Lambda_N=\mathbb Z/N\mathbb Z=\{1,\ldots,N\}\) with periodic
    neighbor structure \(N+1\equiv1\); \(|X|\) is the cardinality and
    \(\mathrm{diam}(X)\) the graph diameter of \(X\subset\Lambda_N\) with
    respect to the ring metric. For \(i\in\Lambda_N\), attach a copy
    \(b_i^{(n_c)},\hat n_i^{(n_c)}\) of the truncated operators above,
    acting on the \(i\)-th factor of \(\mathcal
    H_{N,n_c}:=(\mathbb C^{n_c+1})^{\otimes N}\). For real \(J,U,v\gt0\) and
    \(\omega\gt0\) (period \(T:=2\pi/\omega\)), define
    \[
      H_{N,n_c}^0:=-J\sum_{i=1}^N\Big(b_i^{(n_c)\dagger}b_{i+1}^{(n_c)}
      +b_{i+1}^{(n_c)\dagger}b_i^{(n_c)}\Big)
      +\frac U2\sum_{i=1}^N\hat n_i^{(n_c)}\big(\hat n_i^{(n_c)}-1\big),
    \]
    \[
      V_{N,n_c}(t):=v\cos(\omega t)\sum_{i=1}^N\hat n_i^{(n_c)},
      \qquad
      H_{N,n_c}(t):=H_{N,n_c}^0+V_{N,n_c}(t).
    \]
    \(H_{N,n_c}(t)\) is Hermitian, time-periodic with period \(T\)
    (\(\int_0^TV_{N,n_c}(t)\,dt=0\)), and, being an operator on the
    finite-dimensional space \(\mathcal H_{N,n_c}\) for every finite
    \(n_c\), generates well-defined unitary dynamics with no domain
    subtleties. \(H_{N,n_c}^0\) is the truncated Bose-Hubbard Hamiltonian
    (hopping amplitude \(J\), on-site interaction \(U\)) and
    \(V_{N,n_c}(t)\) an on-site number-diagonal periodic drive of amplitude
    \(v\); other periodic on-site drive shapes are a natural generalization
    not adopted here, and \(J,U,v\) are required strictly positive so that
    the drive does not commute with the full Hamiltonian termwise (at
    \(J=0\), \(V_{N,n_c}(t)\) commutes with \(H^0_{N,n_c}\) at every \(t\),
    so the model never heats at all, for every \(n_c\) and \(\omega\),
    which would make claim (a)-(c) hold vacuously and carry no content).

    Local norm of the truncated model (elementary; used only in the Known
    section, not part of the claim). With the operator-norm bounds above,
    \(\|{-}J(b_i^{(n_c)\dagger}b_{i+1}^{(n_c)}+b_{i+1}^{(n_c)\dagger}
    b_i^{(n_c)})\|\le2Jn_c\) per bond and
    \(\|\frac U2\hat n_i^{(n_c)}(\hat n_i^{(n_c)}-1)+v\cos(\omega t)\hat
    n_i^{(n_c)}\|\le\frac U2n_c(n_c-1)+vn_c\) per site, so with the local
    norm \(\Lambda(t)=\max_i\sum_{X\ni i,|X|\le2}\|h_X(t)\|\) of Ho, Mori,
    Abanin, and Dalla Torre (their eq. (3), \(k=2\)) applied term by term to
    \(H_{N,n_c}(t)=\sum_{X:|X|\le2}h_X(t)\),
    \[
      \Lambda_{n_c}:=\max_t\Lambda(t)\;\le\;\frac U2n_c^2+(4J+v)n_c
      \;=\;O(Un_c^2)\quad(n_c\to\infty).
    \]

    Bose-Hubbard model. The physics term "Bose-Hubbard model" refers to
    \(H^0_{N,n_c}\) above (or its \(n_c=\infty\), i.e. untruncated, limit);
    "lattice bosons" and "unbounded local Hilbert space" refer to the
    \(n_c=\infty\) case, where \(\hat n_i\) and \(b_i\) are unbounded
    operators on \(\mathfrak h\), so no finite \(h\) with \(\|H_i\|\le h\)
    exists in the sense of Abanin, De Roeck, Ho, and Huveneers, eq. (4).
  </def>

  <known>
    Abanin, De Roeck, Ho, and Huveneers, "Effective Hamiltonians,
    prethermalization and slow energy absorption in periodically driven
    many-body systems," Physical Review B 95, 014112
    (2017), DOI 10.1103/PhysRevB.95.014112, arXiv:1510.03405, Sec. II,
    restrict explicitly to "the case of a lattice system with locally
    bounded Hilbert space. In other words, the Hilbert space of site \(i\)
    is finite-dimensional, as is the case for fermions, spins, as well as
    hard-core bosons" (eq. (2)-(4): local terms \(H_i,V_i(t)\) of range
    \(R\) with \(\|H_i\|\le h\), \(\|V_i(t)\|\le h\)). Under this hypothesis
    and \(hT\ll1\), their Theorem (eq. (1), eq. (31)) proves a quasi-conserved
    local \(H_*\) exists with heating time \(\tau_*\sim e^{c\omega/h}\),
    \(c=O(1)\); ordinary (soft-core) lattice bosons, whose local Hilbert
    space is infinite-dimensional and whose on-site interaction is an
    unbounded operator, are outside this hypothesis by construction, so the
    paper neither proves nor claims to prove anything about them.

    Ho, Mori, Abanin, and Dalla Torre, "Quantum and classical Floquet
    prethermalization," Annals of Physics 454, 169297 (2023), DOI
    10.1016/j.aop.2023.169297, arXiv:2212.00041, Sec. II.A, Theorem 1,
    restate the same kind of result in the language used in the Def and
    Claim sections above: for a \(k\)-local, time-periodic \(H(t)\) with
    finite local norm \(\Lambda\) (their eq. (3)), there is
    \(H_{\mathrm{eff}}\) with \(\frac1N\|H_{\mathrm{eff}}-H_0\|=O(\Lambda/
    \Omega)\) (eq. (4)) and
    \(\frac1N|\langle\psi(t)|H_{\mathrm{eff}}|\psi(t)\rangle-\langle\psi(0)
    |H_{\mathrm{eff}}|\psi(0)\rangle|\le e^{-O(\Omega/\Lambda)}\Lambda^2t\)
    (eq. (5)) at stroboscopic times, for an arbitrary initial state.
    Applying Theorem 1 to \(H_{N,n_c}(t)\) at any fixed finite \(n_c\) is
    legitimate, since \(\Lambda_{n_c}\lt\infty\) (Def section); its stated
    eq. (4)-(5) directly give (a) and (c) of the Claim above, with
    \(\tau(\omega)\sim\exp\!\big(\Theta(\omega/\Lambda_{n_c})\big)\), valid
    only once \(\omega\gtrsim\Lambda_{n_c}=O(Un_c^2)\). Property (b) is not
    part of eq. (4)-(5) itself; at this same fixed \(n_c\), it follows
    instead from the constructive renormalization procedure of Abanin, De
    Roeck, Ho, and Huveneers, eq. (18) and eq. (29)-(30), which produces
    \(H_*^{N,n_c}(\omega)\) as an explicit sum of terms of range growing
    linearly in the expansion order and norm decaying exponentially in
    that order (their eq. (24), eq. (27)). All constants \(C,\kappa\) so
    obtained depend on \(n_c\) (through \(\Lambda_{n_c}\)). Because this
    threshold diverges as
    \(n_c\to\infty\) at fixed \(\omega\), direct term-by-term application of
    Theorem 1 to the truncated model gives, for large enough \(n_c\), no
    information whatsoever at any fixed \(\omega\); it does not establish,
    and is not claimed by the source to establish, claim (a)-(c) above,
    which demands a single \((C,\kappa,\tau,\omega_0)\) working for every
    \(n_c\) simultaneously. This is the precise sense in which "unbounded
    occupations break the proofs."

    The same review, Sec. III, shows the bounded-local-dimension hypothesis
    can be relaxed to allow the local Hilbert-space \emph{dimension} to grow
    (e.g. quantum spins of increasing size \(S\to\infty\), recovering
    classical spin chains), precisely because in that construction the
    local \emph{norm} \(\Lambda\) stays finite as the dimension grows (their
    Sec. III, discussion after eq. (27): "we find that eq. (27) is still
    2-local, and its local norm \(\Lambda\) is now finite even in the
    classical limit"). This is the contrasting case that isolates the
    actual obstruction for lattice bosons: it is the local Hamiltonian
    \emph{norm}, not the local Hilbert-space dimension per se, that must
    stay bounded for Theorem 1 to apply, and for the Bose-Hubbard on-site
    term \(\frac U2\hat n_i(\hat n_i-1)\) this norm is unbounded already at
    finite occupation cutoff (\(\Lambda_{n_c}=O(Un_c^2)\to\infty\)), let
    alone in the untruncated theory.

    The same review, Sec. IV, states directly: "Two ... important examples
    of unbounded local Hamiltonians are interacting bosons on a lattice
    where the potential energy is proportional to the square of the number
    of bosons per site, and rotors whose kinetic energy is proportional to
    the square of the angular velocity," for which, upon driving, "it is
    not clear that an exponentially large suppression of the heating rate
    is to be expected at high frequencies," contrasting this with periodically
    kicked classical rotors (another unbounded-Hamiltonian system) where
    Chirikov and Vecheslavov, "Theory of fast Arnold diffusion in
    many-frequency systems," Journal of Statistical Physics 71, 243 (1993),
    found the heating rate depends only \emph{polynomially} on the ratio of
    kick strength to frequency ("fast Arnold diffusion") -- i.e., no
    exponential, or even super-polynomial, suppression at all in that
    analogous unbounded case.

    The same review, Sec. VI (discussion of Rubio-Abadal et al.,
    "Floquet prethermalization in a Bose-Hubbard system," Physical Review X
    10, 021044 (2020)), reports that this experiment/numerics, at
    approximately one particle per site, "found" a "prethermal regime with
    exponentially suppressed heating rates," and states explicitly: "the
    Bose-Hubbard model has an unbounded local Hamiltonian (due to the
    possibility to pile many bosonic particles on a single site) and in
    principle cannot be described using rigorous theorems. Nevertheless,
    the statistical approach reviewed in Sec. IV could qualitatively
    capture the non-monotonic behavior of the heating rate," citing Dalla
    Torre and Dentelski, "Statistical Floquet prethermalization of the
    Bose-Hubbard model," SciPost Physics 11, 040 (2021), whose "statistical"
    argument (Sec. IV of the review, generally) is explicitly a
    non-rigorous, probabilistic estimate of resonance rates, not a proof,
    and gives no operator bound of the form (a)-(c).

    The same review, Sec. VII (Summary and Discussion), lists as one of
    the "outstanding challenges": "One avenue where experiments and
    numerics suggest that there may be room for rigorous results, is that
    of driven systems of interacting bosons on a lattice, such as the
    Bose-Hubbard model studied in ref. 69 [Rubio-Abadal et al.]. These
    studies also demonstrated exponentially slow heating, yet current
    rigorous approaches to Floquet prethermalization cannot handle this
    setup, owing to the unbounded nature of the local Hilbert space." No
    theorem resolving this is presented anywhere else in the paper; the
    two other items in the same "outstanding challenges" list (closing the
    linear-response/Magnus-expansion gap for long-range interactions, and
    strengthening bounds under multiple conservation laws) concern
    different, logically independent questions and are not part of the
    claim above.

    Flank (i) (tightness), not adopted in the Claim (see Ambiguity note).
    Neither source states or attempts a matching converse to \(\tau_*\sim
    e^{c\omega/h}\); Abanin, De Roeck, Ho, and Huveneers, Sec. V, assert
    only informally, for the case where \(H_*\) is ergodic, that "the
    system will absorb energy and relax to a featureless, infinite-
    temperature state beyond times \(t\sim\tau_*\)," which is a physical
    expectation grounded in the eigenstate thermalization hypothesis for
    \(H_*\) (itself not proved in general), not a theorem bounding the
    heating time from above. This
    residual doubt -- that a real, precisely-statable open problem plausibly
    underlies flank (i) as well, just not one either cited source
    formulates -- is recorded here rather than incorporated into the claim.
  </known>

  <refs>
    <ref>D. A. Abanin, W. De Roeck, W. W. Ho, and F. Huveneers, Effective Hamiltonians, prethermalization and slow energy absorption in periodically driven many-body systems, Physical Review B 95, 014112 (2017), DOI 10.1103/PhysRevB.95.014112, arXiv:1510.03405, Sec. II (eq. (2)-(4), bounded-local-Hilbert-space hypothesis) and Sec. III/V (eq. (1), eq. (31), heating time \(\tau_*\sim e^{c\omega/h}\); Sec. V discussion of heating at \(t\sim\tau_*\) for ergodic \(H_*\)), used for the Def-section local-norm formalism and the Known-section baseline theorem.</ref>
    <ref>W. W. Ho, T. Mori, D. A. Abanin, and E. G. Dalla Torre, Quantum and classical Floquet prethermalization, Annals of Physics 454, 169297 (2023), DOI 10.1016/j.aop.2023.169297, arXiv:2212.00041, Sec. II.A (Theorem 1, eq. (3)-(5)), Sec. III (large-spin/growing-dimension extension and finiteness of \(\Lambda\)), Sec. IV (unbounded local Hamiltonians, Bose-Hubbard example, contrast with fast Arnold diffusion), Sec. VI (Bose-Hubbard experiment discussion), and Sec. VII (list of outstanding challenges naming driven lattice bosons), used for the Claim's target statement and the Known section throughout.</ref>
    <ref>A. Rubio-Abadal, M. Ippoliti, S. Hollerith, D. Wei, J. Rui, S. L. Sondhi, V. Khemani, C. Gross, and I. Bloch, Floquet Prethermalization in a Bose-Hubbard System, Physical Review X 10, 021044 (2020), DOI 10.1103/PhysRevX.10.021044, cited by Ho, Mori, Abanin, and Dalla Torre (2023), Sec. VI and Sec. VII as ref. 69, used only for the numerical/experimental evidence of exponentially slow heating in the driven Bose-Hubbard model that motivates, but does not prove, the claim above.</ref>
    <ref>E. G. Dalla Torre and D. Dentelski, Statistical Floquet prethermalization of the Bose-Hubbard model, SciPost Physics 11, 040 (2021), DOI 10.21468/SciPostPhys.11.2.040, cited by Ho, Mori, Abanin, and Dalla Torre (2023), Sec. VI as ref. 72, used only for the non-rigorous statistical (resonance-counting) argument noted in the Known section as not constituting a proof.</ref>
    <ref>B. V. Chirikov and V. V. Vecheslavov, Theory of fast Arnold diffusion in many-frequency systems, Journal of Statistical Physics 71, 243-258 (1993), DOI 10.1007/BF01048098, cited by Ho, Mori, Abanin, and Dalla Torre (2023), Sec. IV as ref. 41, used only for the contrasting polynomial-heating-rate phenomenon noted in the Known section, relevant to the disproof direction of the Ask section.</ref>
  </refs>

  <ask>
    Prove or disprove the claim that \(J,U,v,\omega_0,\tau,C,\kappa\) as in
    the Claim section exist.

    A proof must exhibit \(J,U,v\gt0\), \(\omega_0\gt0\), a
    super-polynomial \(\tau\), and \(C,\kappa\gt0\) independent of \(n_c\),
    \(N\), and \(\omega\), and must rigorously establish (a)-(c) for every
    \(n_c\in\mathbb N\), every integer \(N\ge3\), and every \(\omega\ge
    \omega_0\) simultaneously -- not merely for each fixed \(n_c\) with
    constants that are then shown, or left, to depend on \(n_c\). A proof
    that instead establishes the stronger Abanin-De-Roeck-Ho-Huveneers rate
    \(\tau(\omega)=e^{c\omega}\) for a fixed \(c\gt0\) resolves the claim a
    fortiori. A proof restricted to a fixed finite \(n_c\) (as already
    follows from Theorem 1, Known section) or to a fixed finite \(N\) is
    progress, not a resolution.

    A disproof must show that for every choice of \(J,U,v\gt0\),
    \(\omega_0\gt0\), super-polynomial \(\tau\), and \(C,\kappa\gt0\), there
    exist \(n_c\in\mathbb N\), an integer \(N\ge3\), and \(\omega\ge
    \omega_0\), such that every Hermitian \(H_*^{N,n_c}(\omega)\) on
    \(\mathcal H_{N,n_c}\) satisfying (a) and (b) with that \(C,\kappa\)
    fails (c) at some \(m\) with \(mT\le\tau(\omega)\). An explicit
    dynamical mechanism showing that, no matter
    how \(H_*\) is chosen, order-\(1\) relative energy change is forced by a
    time growing only polynomially in \(\omega\) as \(n_c\to\infty\) --
    analogous to the fast Arnold diffusion mechanism of the Known section,
    but established for the driven Bose-Hubbard chain of the Def section --
    would constitute such a disproof. Numerical evidence of exponentially
    suppressed heating at particular, fixed values of \(n_c\), \(N\), and
    \(\omega\) (as in Rubio-Abadal et al., Known section) is progress
    (evidence for the claim), not a proof, since it does not by itself rule
    out degradation of the bound as \(n_c\to\infty\); the non-rigorous
    statistical argument of Dalla Torre and Dentelski (Known section) is
    likewise progress, not a proof, since it does not exhibit operators
    \(H_*^{N,n_c}(\omega)\) or establish (a)-(c) rigorously.

    A resolution, in either direction, of flank (i) (tightness of the
    bounded-local-dimension bound, recorded in the Known section) is
    progress on a related but logically separate question, not a proof or
    disproof of the claim above.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
