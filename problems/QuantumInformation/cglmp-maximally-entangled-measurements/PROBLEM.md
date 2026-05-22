# QF027B CGLMP maximizers on maximally entangled qudits

```xml
<problem id="QF027B">
  <status>open</status>
  <name>CGLMP maximizers on maximally entangled qudits</name>
  <keys>oqp problem 27b power of cglmp inequalities cglmp maximally entangled state maximally entangled qudits fourier measurements diagonal unitaries phase measurements computational basis optimal observables maximal violation noise resistance kullback leibler divergence statistical strength durt kaszlikowski zukowski acin gill gisin lang vertesi navascues npa ioannou rosset quantum foundations</keys>

  <claim>
    For every integer \(d\ge 3\), the local rank-one projective \(d\)-outcome
    measurements on \(\mathbb C^d\) that maximize the CGLMP Bell violation on
    the maximally entangled state
    \(|\Phi_d\rangle=d^{-1/2}\sum_{j=0}^{d-1}|j\rangle|j\rangle\) are exactly,
    up to the equivalences defined below, the Fourier-plus-diagonal CGLMP
    measurements with phases
    \[
      \alpha_1=0,\qquad \alpha_2=\frac12,\qquad
      \beta_1=\frac14,\qquad \beta_2=-\frac14 .
    \]
  </claim>

  <def>
    Let \(\mathbb Z_d=\{0,\ldots,d-1\}\), with arithmetic understood modulo
    \(d\). Alice and Bob choose settings \(x,y\in\{1,2\}\) and obtain outcomes
    \(a,b\in\mathbb Z_d\). For random variables \(U,V\) with values in
    \(\mathbb Z_d\), write
    \[
      \Pr[U=V+t]
      :=
      \sum_{r\in\mathbb Z_d}\Pr[U=r+t,\ V=r],
      \qquad t\in\mathbb Z_d .
    \]
    This entry uses the standard Collins-Gisin-Linden-Massar-Popescu
    probability convention
    \[
    \begin{gathered}
      I_d(p)
      =
      \sum_{k=0}^{\lfloor d/2\rfloor-1}
      \left(1-\frac{2k}{d-1}\right)
      \Big(
      \Pr[A_1=B_1+k]
      +\Pr[B_1=A_2+k+1]\\
      \quad
      +\Pr[A_2=B_2+k]
      +\Pr[B_2=A_1+k]
      -\Pr[A_1=B_1-k-1]\\
      \quad
      -\Pr[B_1=A_2-k]
      -\Pr[A_2=B_2-k-1]
      -\Pr[B_2=A_1-k-1]
      \Big).
    \end{gathered}
    \]
    The local CGLMP inequality is \(I_d(p)\le 2\). Maximizing the CGLMP
    violation on the fixed state \(|\Phi_d\rangle\) means maximizing \(I_d(p)\)
    over the allowed local projective measurements on that state.

    A rank-one projective \(d\)-outcome measurement on \(\mathbb C^d\) is an
    orthonormal basis, with outcome probabilities given by the corresponding
    one-dimensional projections. A measurement quadruple consists of Alice
    bases \(\{|a;x\rangle_A:a\in\mathbb Z_d\}\) for \(x=1,2\) and Bob bases
    \(\{|b;y\rangle_B:b\in\mathbb Z_d\}\) for \(y=1,2\). It induces
    \[
      p(a,b|x,y)=
      \left|\left(\langle a;x|_A\otimes \langle b;y|_B\right)
      |\Phi_d\rangle\right|^2 .
    \]

    Let \(\omega=\exp(2\pi i/d)\). The canonical Fourier-plus-diagonal CGLMP
    bases are
    \[
      |a;x\rangle_A=
      \frac{1}{\sqrt d}\sum_{j=0}^{d-1}\omega^{j(a+\alpha_x)}|j\rangle,
      \qquad
      |b;y\rangle_B=
      \frac{1}{\sqrt d}\sum_{j=0}^{d-1}\omega^{j(-b+\beta_y)}|j\rangle,
    \]
    with \(\alpha_1=0\), \(\alpha_2=1/2\), \(\beta_1=1/4\), and
    \(\beta_2=-1/4\). Equivalently, each measurement basis is obtained from the
    computational basis by a discrete Fourier transform and a diagonal phase
    unitary.

    The equivalences in the claim are exactly those that preserve this fixed
    \(I_d\) Bell functional and the fixed maximally entangled state value:
    changing the Schmidt basis by \(U\otimes\overline U\), multiplying
    individual basis vectors by phases, and applying automorphisms of the
    chosen CGLMP functional such as its outcome-shift and dihedral symmetries.
    Scenario relabelings that convert \(I_d\) into a convention-renamed Bell
    functional are not part of the fixed-functional equivalence unless they are
    automorphisms of the displayed \(I_d\). The claim is about optimization over
    measurements for the fixed state \(|\Phi_d\rangle\); it does not optimize
    over nonmaximally entangled states, higher local Hilbert-space dimensions,
    or general POVMs.
  </def>

  <known>
    This entry was collected from Open Quantum Problems, Problem 27, "The power
    of CGLMP inequalities". The Open Quantum Problems index lists Problem 27 in
    the Quantum foundations category. OQP Problem 27.B says that, numerically,
    the observables maximally violating the CGLMP inequality on a maximally
    entangled state have a very specific form involving the computational basis,
    the discrete Fourier transform, and diagonal unitaries, and asks for a proof
    that this form is necessary. OQP Problem 27.B also asks whether these
    measurements give the highest resistance of violation to noise and the best
    Kullback-Leibler discrimination against classical realism. This entry
    records only the measurement-maximizer assertion in the claim; the
    noise-resistance and Kullback-Leibler assertions are adjacent OQP subclaims
    and count as progress unless they imply the claim.

    Collins, Gisin, Linden, Massar, and Popescu introduced the CGLMP family and
    gave the Fourier-plus-diagonal measurement form for high-dimensional
    maximally entangled Bell tests. Durt, Kaszlikowski, and Zukowski numerically
    studied violations of local realism for maximally entangled quNit systems up
    to \(N=16\) using specially chosen observables. The phases in the claim are
    the standard CGLMP phase choices appearing in this line of work.

    Van Dam, Grunwald, and Gill formalized statistical strength of nonlocality
    proofs through Kullback-Leibler divergence to the local set. Acin, Gill, and
    Gisin used this statistical-strength viewpoint and found that optimal Bell
    tests do not require maximally entangled states. These results motivate the
    Kullback-Leibler part of OQP Problem 27.B but do not settle the fixed-state
    measurement classification in the claim.

    Acin, Durt, Gisin, and Latorre showed in the two three-level CGLMP setting
    that allowing nonmaximally entangled states can give a larger violation,
    equivalently stronger resistance to noise, than the maximally entangled
    state. This is not a disproof of the present claim because the present claim
    fixes \(|\Phi_d\rangle\) and optimizes only the measurements.

    Navascues, Pironio, and Acin introduced the semidefinite-programming
    hierarchy now called the NPA hierarchy and used it to obtain upper bounds on
    arbitrary Bell inequalities, including tight bounds for the Collins et al.
    inequalities. OQP Problem 27 records that Lang, Vertesi, and Navascues used
    semidefinite relaxations to show that for \(d=3\), on maximally entangled
    states of arbitrarily high dimension, the maximum CGLMP violation differs
    by at most \(1e-10\) from the local maximum found for \(d=3\) maximally
    entangled states with the special measurements. This is numerical evidence,
    not a proof for all \(d\).

    OQP Problem 27 records that Ioannou and Rosset exploited CGLMP symmetries in
    the NPA sum-of-squares framework and found analytic SOS decompositions
    proving the conjectured exact maximum CGLMP quantum violation for
    \(d=3,4\). Their arXiv abstract states that they prove the exact maximal
    quantum violation of the CGLMP inequalities for dimensions \(3\) and \(4\).
    This solves important fixed-dimension arbitrary-state value cases, but it
    does not by itself settle the all-\(d\) fixed-state measurement-maximizer
    classification in the claim.
  </known>

  <refs>
    <ref>Open Quantum Problems, The power of CGLMP inequalities, Problem 27, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/the-power-of-cglmp-inequalities, used for the OQP 27.B statement, the CGLMP background inequality, and the partial results recorded for Lang-Vertesi-Navascues, Acin-Durt-Gisin-Latorre, Navascues-Pironio-Acin, and Ioannou-Rosset.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 27, The power of CGLMP inequalities, in Quantum foundations.</ref>
    <ref>D. Collins, N. Gisin, N. Linden, S. Massar, and S. Popescu, Bell inequalities for arbitrarily high-dimensional systems, Physical Review Letters 88, 040404 (2002), DOI: 10.1103/PhysRevLett.88.040404, arXiv:quant-ph/0106024, used for the CGLMP inequality family and Fourier-plus-diagonal measurement form.</ref>
    <ref>T. Durt, D. Kaszlikowski, and M. Zukowski, Violations of local realism with quantum systems described by N-dimensional Hilbert spaces up to N=16, Physical Review A 64, 024101 (2001), DOI: 10.1103/PhysRevA.64.024101, arXiv:quant-ph/0101084, used for numerical maximally entangled quNit measurement evidence.</ref>
    <ref>W. van Dam, P. D. Grunwald, and R. D. Gill, The statistical strength of nonlocality proofs, IEEE Transactions on Information Theory 51(8), 2812-2835 (2005), DOI: 10.1109/TIT.2005.851738, arXiv:quant-ph/0307125, used for the Kullback-Leibler statistical-strength context.</ref>
    <ref>A. Acin, R. Gill, and N. Gisin, Optimal Bell tests do not require maximally entangled states, Physical Review Letters 95, 210402 (2005), DOI: 10.1103/PhysRevLett.95.210402, arXiv:quant-ph/0506225, used for the KL-optimality context and nonmaximally entangled-state caveat.</ref>
    <ref>B. Lang, T. Vertesi, and M. Navascues, Closed sets of correlations: answers from the zoo, Journal of Physics A: Mathematical and Theoretical 47, 424029 (2014), DOI: 10.1088/1751-8113/47/42/424029, arXiv:1402.2850, used for the \(d=3\) numerical upper-bound evidence recorded by OQP Problem 27.</ref>
    <ref>A. Acin, T. Durt, N. Gisin, and J. I. Latorre, Quantum nonlocality in two three-level systems, Physical Review A 65, 052325 (2002), DOI: 10.1103/PhysRevA.65.052325, used for the nonmaximally entangled-state violation caveat.</ref>
    <ref>M. Navascues, S. Pironio, and A. Acin, Bounding the set of quantum correlations, Physical Review Letters 98, 010401 (2007), DOI: 10.1103/PhysRevLett.98.010401, arXiv:quant-ph/0607119, used for the NPA hierarchy context.</ref>
    <ref>M. Ioannou and D. Rosset, Noncommutative polynomial optimization under symmetry, arXiv:2112.10803v2 (2022), DOI: 10.48550/arXiv.2112.10803, used for analytic SOS progress for CGLMP dimensions \(3\) and \(4\).</ref>
  </refs>

  <ask>
    Prove or disprove the claim. A proof must determine the global maximum of
    \(I_d(p)\) over all rank-one projective \(d\)-outcome measurement
    quadruples on the fixed state \(|\Phi_d\rangle\) for every \(d\ge 3\), show
    that the Fourier-plus-diagonal CGLMP bases attain it, and prove that every
    other maximizer is equivalent to those bases under the stated equivalences.

    A disproof must give some \(d\ge 3\) and either a projective measurement
    quadruple on \(|\Phi_d\rangle\) with a strictly larger value of \(I_d\) than
    the Fourier-plus-diagonal CGLMP measurements, or an inequivalent projective
    measurement quadruple attaining the same global maximum. The disproof must
    include a rigorous global optimality or inequivalence proof, not only a
    numerical search. Results for nonmaximally entangled states, arbitrary-state
    quantum maxima, NPA upper bounds without uniqueness, noise-resistance
    optima, Kullback-Leibler optima, or isolated dimensions are progress, not a
    solution, unless they imply the all-\(d\) fixed-state measurement
    classification or its negation.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
