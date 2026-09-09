# 41 Refined BMV Lower Bound

```xml
<problem id="41">
  <status>open</status>
  <kind>proof</kind>
  <field>QIT</field>
  <name>Refined BMV Lower Bound</name>

  <keys>bmv conjecture bessis-moussa-villani conjecture refined bmv refinement of bmv golden-thompson inequality golden thompson trace inequality matrix exponential trace exponential positive definite matrices word average normalized word average pnm coefficient p_nm oqp problem 40 iqoqi open quantum problems hagele werner lieb-seiringer reformulation stahl proof partition function derivative noncommutative positivity matrix logarithm sandwich inequality surviving lower bound cha lee counterexample refined bmv upper bound refutation dinh pinching correction adnane</keys>

  <claim>
    Fix an integer \(d\ge1\). For every pair of integers \(n,m\ge1\) and
    every pair of Hermitian positive definite matrices
    \(A,B\in\mathbb{C}^{d\times d}\),
    \[
      \operatorname{tr}\exp\bigl(n\log A+m\log B\bigr)\;\le\;p_{n,m}(A,B).
    \]
  </claim>

  <def>
    \(d\ge1\) is a fixed matrix dimension. A matrix \(A\in\mathbb{C}^{d
    \times d}\) is Hermitian positive definite, written \(A\succ0\), if
    \(A=A^\dagger\) and \(\langle\psi|A|\psi\rangle\gt0\) for every nonzero
    \(\psi\in\mathbb{C}^d\); equivalently every eigenvalue of \(A\) is a
    strictly positive real number. \(A\succeq0\) (positive semidefinite)
    only requires \(\langle\psi|A|\psi\rangle\ge0\), allowing zero
    eigenvalues.

    For \(A\succ0\) with spectral decomposition
    \(A=\sum_{i=1}^d a_i|\phi_i\rangle\langle\phi_i|\), \(a_i\gt0\), the
    matrix logarithm is
    \(\log A:=\sum_{i=1}^d(\log a_i)|\phi_i\rangle\langle\phi_i|\), a
    Hermitian matrix. For a Hermitian matrix
    \(H=\sum_ih_i|\chi_i\rangle\langle\chi_i|\), the matrix exponential is
    \(\exp H:=\sum_ie^{h_i}|\chi_i\rangle\langle\chi_i|\).

    For integers \(n,m\ge0\), a word of type \((n,m)\) is a sequence
    \((\ell_1,\dots,\ell_{n+m})\in\{A,B\}^{n+m}\) with exactly \(n\) entries
    equal to \(A\) and exactly \(m\) entries equal to \(B\); write
    \(\mathcal{W}_{n,m}\) for the set of all such sequences, so
    \(|\mathcal{W}_{n,m}|=\binom{n+m}{n}\). For
    \(W=(\ell_1,\dots,\ell_{n+m})\in\mathcal{W}_{n,m}\), let
    \(W(A,B):=\ell_1\ell_2\cdots\ell_{n+m}\) denote the ordinary matrix
    product obtained by substituting the matrices \(A,B\) for the letters
    in order. The normalized word average is
    \[
      p_{n,m}(A,B):=\frac{1}{\binom{n+m}{n}}\sum_{W\in\mathcal{W}_{n,m}}
      \operatorname{tr}W(A,B),
    \]
    equivalently the coefficient extracted from the two-variable trace
    polynomial via
    \(\binom{n+m}{n}\,p_{n,m}(A,B)=[t^ns^m]\operatorname{tr}(tA+sB)^{n+m}\),
    where \([t^ns^m]\) denotes the coefficient of the monomial \(t^ns^m\)
    in the expansion of \(\operatorname{tr}(tA+sB)^{n+m}\) as a homogeneous
    polynomial of degree \(n+m\) in the commuting scalars \(t,s\).
  </def>

  <known>
    Bessis, Moussa, and Villani, Monotonic converging variational
    approximations to the functional integrals in quantum statistical
    mechanics, J. Math. Phys. 16, 2318 (1975), conjectured that for
    Hermitian \(A\) and positive semidefinite \(B\), the map
    \(\lambda\mapsto\operatorname{tr}\exp(A-\lambda B)\) is the Laplace
    transform of a positive measure on \([0,\infty)\) (the BMV conjecture).

    Lieb and Seiringer, Equivalent forms of the Bessis-Moussa-Villani
    conjecture, J. Stat. Phys. 115, 185 (2004), proved the BMV conjecture
    equivalent to: every coefficient of the one-variable polynomial
    \(\operatorname{tr}(A+tB)^p\), expanded in powers of \(t\), is
    nonnegative whenever \(A,B\succeq0\) and \(p\in\mathbb{N}\); via the
    coefficient-extraction identity in the definitions above this is the
    same statement as \(p_{n,m}(A,B)\ge0\) for every \(n,m\ge0\) with
    \(n+m=p\) and every \(A,B\succeq0\). Stahl, Proof of the BMV
    conjecture, Acta Math. 211, 255 (2013), proved this positivity
    statement. It is strictly weaker than the claim recorded here, which
    asserts the specific quantitative lower bound
    \(p_{n,m}(A,B)\ge\operatorname{tr}\exp(n\log A+m\log B)\) rather than
    mere nonnegativity of \(p_{n,m}(A,B)\).

    A refinement of BMV attributed to Daniel Hägele, communicated by R. F.
    Werner, is catalogued as Open Quantum Problems Problem 40 (IQOQI
    Vienna, Refinement of the Bessis-Moussa-Villani conjecture,
    https://oqp.iqoqi.oeaw.ac.at/refinement-of-the-bessis-moussa-villani-conjecture).
    Cha and Lee, arXiv:2603.19927, Eqs. (1)-(2), restate this refinement
    with the same \(p_{n,m}(A,B)\) notation as the two-sided sandwich
    \(\operatorname{tr}(A^nB^m)\ge p_{n,m}(A,B)\ge\operatorname{tr}\exp(n
    \log A+m\log B)\) for \(A,B\succeq0\). Equality holds throughout when
    \(A\) and \(B\) commute. When \(n=m=1\) the lower bound recorded as
    the claim here reduces to the classical Golden-Thompson inequality
    \(\operatorname{tr}e^{A+B}\le\operatorname{tr}(e^Ae^B)\) (Golden, Lower
    bounds for the Helmholtz function, Phys. Rev. 137, B1127 (1965);
    Thompson, Inequality with applications in statistical mechanics, J.
    Math. Phys. 6, 1812 (1965)).

    OQP Problem 40 and the Cha-Lee restatement (arXiv:2603.19927, Eq. (2))
    fix \(p_{n,m}(A,B)\) and the upper-bound term \(\operatorname{tr}(A^n
    B^m)\) for merely positive semidefinite \(A,B\), but the lower-bound
    term \(\operatorname{tr}\exp(n\log A+m\log B)\) needs \(\log A,\log B\)
    to be defined, which needs \(A,B\succ0\); neither source specifies a
    convention for singular \(A\) or \(B\) in the lower bound. The claim
    recorded here therefore uses the positive-definite domain, the precise
    reading under which every term is unambiguously defined, rather than a
    limiting extension to singular matrices. Dinh, arXiv:2605.17782, Eq.
    (1.2), independently states the same two-sided sandwich "for positive
    definite matrices \(A,B\), with the positive semidefinite case obtained
    by approximation," confirming positive-definiteness as the domain of
    the exact statement rather than an artifact of this entry's phrasing.

    Cha and Lee, One-parameter counterexamples to the refined
    Bessis-Moussa-Villani conjecture, arXiv:2603.19927 (2026), refute the
    upper half of the sandwich: their Proposition 6 and Corollary 2 exhibit
    an explicit one-parameter family of \(3\times3\) positive semidefinite
    matrices \((A_x,B_x)_{x\ge0}\) for which the ratio
    \(p_{n,m}(A_x,B_x)/\operatorname{tr}(A_x^nB_x^m)\) diverges as
    \(x\to0^+\) whenever \(\min\{n,m\}\gt4\) (worked out explicitly for
    \(n=m=5\)), so \(p_{n,m}(A,B)\le\operatorname{tr}(A^nB^m)\) fails in
    general. Their abstract, introduction, and Section VII (Discussion)
    confine this refutation to what they call "the first inequality in
    Eq. (2)," i.e. the upper bound; Cha and Lee, arXiv:2603.19927, state
    and use no result about the lower bound \(\operatorname{tr}\exp(n\log
    A+m\log B)\le p_{n,m}(A,B)\), which is the claim recorded here.

    Dinh, On the Failure of the Upper Bound in the Refined BMV Conjecture
    and a Pinching Correction, arXiv:2605.17782 (2026), responds to the
    Cha-Lee counterexamples with a corrected upper-side bound built from a
    pinching map \(E_A\), proving
    \(p_{n,2}(A,E_A(B))\le p_{n,2}(A,B)\le\operatorname{tr}(A^nB^2)\) in
    the two-letter case. This again addresses only the upper side of the
    sandwich and does not treat the lower bound.
  </known>

  <refs>
    <ref>Open Quantum Problems, Problem 40: Refinement of the Bessis-Moussa-Villani conjecture, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/refinement-of-the-bessis-moussa-villani-conjecture, attributed to Daniel Hägele, communicated by R. F. Werner; used for the problem statement and the two-sided sandwich inequality. (Direct access returned HTTP 503 at verification time; content confirmed via search-result excerpt and via the identical restatement in Cha and Lee, arXiv:2603.19927, Eqs. (1)-(2) and reference [24], which records this listing and gives the same problem number.)</ref>
    <ref>Mostafa Adnane, Refinement of the Bessis-Moussa-Villani Conjecture (2017); cited as reference [27] in Cha and Lee, arXiv:2603.19927, alongside the OQP 40 listing as the source of the refined conjecture, and matched by web search to a 2017 note by Mostafa Adnane (École Polytechnique, Département de Physique) hosted on ResearchGate. Full text not independently read for this entry; no DOI located at verification time.</ref>
    <ref>Hyunho Cha and Jungwoo Lee, One-parameter counterexamples to the refined Bessis-Moussa-Villani conjecture, arXiv:2603.19927 [quant-ph] (2026), Eqs. (1)-(2) for the definition of \(p_{n,m}(A,B)\) and the sandwich inequality, Proposition 6 and Corollary 2 for the counterexample family, Section VII for confirming the refutation is confined to the upper inequality.</ref>
    <ref>Trung Hoa Dinh, On the Failure of the Upper Bound in the Refined BMV Conjecture and a Pinching Correction, arXiv:2605.17782 [quant-ph] (2026); the pinching-corrected upper bound for the two-letter case.</ref>
    <ref>D. Bessis, P. Moussa, and M. Villani, Monotonic converging variational approximations to the functional integrals in quantum statistical mechanics, J. Math. Phys. 16, 2318 (1975); origin of the BMV conjecture.</ref>
    <ref>E. H. Lieb and R. Seiringer, Equivalent forms of the Bessis-Moussa-Villani conjecture, J. Stat. Phys. 115, 185 (2004); the positivity-of-coefficients reformulation.</ref>
    <ref>H. R. Stahl, Proof of the BMV conjecture, Acta Math. 211, 255 (2013); proof of the reformulated BMV positivity statement.</ref>
    <ref>S. Golden, Lower bounds for the Helmholtz function, Phys. Rev. 137, B1127 (1965); one half of the Golden-Thompson inequality reached at \(n=m=1\).</ref>
    <ref>C. J. Thompson, Inequality with applications in statistical mechanics, J. Math. Phys. 6, 1812 (1965); the other half of the Golden-Thompson inequality reached at \(n=m=1\).</ref>
  </refs>

  <ask>
    Prove the claim by establishing \(\operatorname{tr}\exp(n\log A+m\log
    B)\le p_{n,m}(A,B)\) for every \(d\ge1\), every \(n,m\ge1\), and every
    Hermitian positive definite \(A,B\in\mathbb{C}^{d\times d}\). A proof
    restricted to a fixed dimension \(d\), fixed \(n,m\), commuting
    \(A,B\), or the already established \(n=m=1\) Golden-Thompson case is
    progress, not a proof of the general claim.

    Disprove the claim by exhibiting one instance: some \(d\ge1\),
    \(n,m\ge1\), and Hermitian positive definite \(A,B\in\mathbb{C}^{d
    \times d}\) with \(\operatorname{tr}\exp(n\log A+m\log
    B)\gt p_{n,m}(A,B)\). A single such instance suffices, since the claim
    is a universally quantified inequality; the counterexample family in
    Cha and Lee, arXiv:2603.19927, does not itself provide one, since it
    is built to violate the reversed, upper-bound inequality
    \(p_{n,m}(A,B)\le\operatorname{tr}(A^nB^m)\), not the lower bound.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
