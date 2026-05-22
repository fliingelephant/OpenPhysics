# QCOMP014 Tough Error Models

```xml
<problem id="QCOMP014">
  <status>open</status>
  <name>Tough Error Models</name>
  <keys>tough error models quantum error correction qec knill laflamme condition knill-laflamme condition general noise arbitrary noise error model operator subspace operator space kraus operators quantum code code dimension guaranteed code dimension quantum anticlique noncommutative graph operator system higher rank numerical range projective measurement lueders von neumann measurement radon theorem tverberg theorem knill laflamme viola open quantum problems oqp quantum computation</keys>

  <claim>
    For every pair of positive integers \((e,n)\), determine the exact integer
    \(c(e,n)\) defined below: in the nonvacuous range \(1\le e\le n^2\),
    \(c(e,n)\) is the largest integer \(c\) such that every \(e\)-dimensional
    operator subspace \(\mathcal E\le B(\mathcal H)\) on every
    \(n\)-dimensional Hilbert space \(\mathcal H\) has a quantum code
    \(\mathcal C\le\mathcal H\) of dimension at least \(c\) satisfying
    \[
      P_{\mathcal C} A^*B P_{\mathcal C}
      =
      \lambda(A,B)P_{\mathcal C}
      \qquad\text{for all }A,B\in\mathcal E .
    \]
    Equivalently, determine
    \[
      c(e,n)=
      \min_{\substack{\mathcal E\le B(\mathcal H)\\ \dim\mathcal E=e}}
      \max\{\dim\mathcal C:\mathcal C\text{ corrects }\mathcal E\}.
    \]
    In the vacuous range \(e\gt n^2\), use the convention \(c(e,n)=n\)
    because no \(e\)-dimensional subspace of \(B(\mathcal H)\) exists. In the
    nonvacuous range, also exhibit tough error models \(\mathcal E\) for which
    the inner maximum equals \(c(e,n)\), characterize such models where
    possible, and give quantitatively stated tight or nearly tight families in
    asymptotic regimes where exact minimizers are not known.
  </claim>

  <def>
    Throughout, \(\mathcal H\) is a complex Hilbert space with
    \(\dim\mathcal H=n\), and \(B(\mathcal H)\) is the \(n^2\)-dimensional
    complex vector space of linear operators on \(\mathcal H\). An
    \(e\)-dimensional error model is a complex vector subspace
    \(\mathcal E\le B(\mathcal H)\) with \(\dim\mathcal E=e\). The
    nonvacuous parameter range is therefore \(1\le e\le n^2\).

    A quantum code is a nonzero subspace \(\mathcal C\le\mathcal H\), and
    \(P_{\mathcal C}\) denotes the orthogonal projection onto \(\mathcal C\).
    The code \(\mathcal C\) corrects \(\mathcal E\) when there is a sesquilinear
    scalar form \(\lambda:\mathcal E\times\mathcal E\to\mathbb C\) such that
    the Knill-Laflamme condition
    \[
      P_{\mathcal C} A^*B P_{\mathcal C}
      =
      \lambda(A,B)P_{\mathcal C}
    \]
    holds for all \(A,B\in\mathcal E\). Equivalently,
    \(P_{\mathcal C}SP_{\mathcal C}\in\mathbb C P_{\mathcal C}\) for every
    \(S\) in the operator system
    \[
      \mathcal S_{\mathcal E}
      =
      \operatorname{span}\{I,A^*B:A,B\in\mathcal E\}.
    \]
    In noncommutative graph language, \(P_{\mathcal C}\) is a quantum
    anticlique for \(\mathcal S_{\mathcal E}\).

    For a fixed error model \(\mathcal E\), write
    \[
      m(\mathcal E)
      =
      \max\{\dim\mathcal C:\mathcal C\le\mathcal H
      \text{ corrects }\mathcal E\}.
    \]
    Then \(c(e,n)\) is the worst-case value of \(m(\mathcal E)\) over
    \(e\)-dimensional \(\mathcal E\). Since every one-dimensional
    \(\mathcal C\) satisfies the Knill-Laflamme condition, \(c(e,n)\ge 1\)
    whenever \(1\le e\le n^2\).

    For \(1\le e\le n^2\), a tough error model for \((e,n)\) means an \(e\)-dimensional
    \(\mathcal E\le B(\mathcal H)\) with \(m(\mathcal E)=c(e,n)\). A family
    \(\mathcal E_{e,n}\) is called nearly tight only relative to an explicit
    quantitative statement, for example
    \(m(\mathcal E_{e,n})/c(e,n)\to 1\) in a specified limit, or a specified
    constant-factor upper bound on \(m(\mathcal E_{e,n})/c(e,n)\).
  </def>

  <known>
    Open Quantum Problems, Problem 14, "Tough error models", asks, for fixed
    \(e\) and \(n\), for the largest guaranteed code dimension \(c(e,n)\) for
    an \(e\)-dimensional vector space of operators on an \(n\)-dimensional
    Hilbert space, and asks for tough error models for which that guarantee is
    tight or nearly tight. The Open Quantum Problems list records Problem 14
    in the Quantum computation category.

    Knill, Laflamme, Ashikhmin, Barnum, Viola, and Zurek, "Introduction to
    Quantum Error Correction", arXiv:quant-ph/0207170 (2002), give background
    on quantum error correction, including the decoding and subsystem
    viewpoints, error models, and the correctability criterion used in this
    entry.

    Knill, Laflamme, and Viola, "Theory of Quantum Error Correction for General
    Noise", Physical Review Letters 84, 2525-2528 (2000), arXiv:quant-ph/9908066,
    prove existence of large quantum codes for general noise. Reimpell,
    "Quantum Information and Convex Optimization", Section 3.3.4, records the
    Radon-theorem lower-bound strategy as giving the integer sufficient bound
    \[
      c(e,n)\ge
      \max\left\{1,
      \left\lfloor
      \frac{\lceil n/e^2\rceil+e^2}{e^2+1}
      \right\rfloor
      \right\}.
    \]
    The OQP Problem 14 page summarizes the same Knill-Laflamme-Viola source by
    displaying the coarse real-valued estimate
    \[
      c(e,n)\gt \frac{n}{e^2(e^2+1)} .
    \]
    Since \(c(e,n)\) is an integer, the floor and ceiling formulation is the
    statement to use in endpoint cases; neither displayed estimate is an exact
    formula for \(c(e,n)\) or a classification of tough models.

    The OQP Problem 14 page gives a trivial upper bound from a
    Lueders-von Neumann projective measurement error model. For \(1\le e\le n\),
    take \(e\) mutually orthogonal projections summing to \(I_{\mathcal H}\)
    whose ranks are as equal as possible, so the largest rank is
    \(\lceil n/e\rceil\). Any code correcting the span of these projections has
    dimension at most the largest projection rank, and the largest projection
    range itself is a code. Thus this family gives
    \(c(e,n)\le\lceil n/e\rceil\) for \(1\le e\le n\). For
    \(n\lt e\le n^2\), including the \(n\) rank-one coordinate projections and
    extending their span to dimension \(e\) gives \(m(\mathcal E)=1\), so
    \(c(e,n)=1=\lceil n/e\rceil\) in this range. The latter extension is a
    simple consequence of the same projective-measurement obstruction; the OQP
    page states the projective-measurement upper bound as
    \(c(e,n)\le\lceil n/e\rceil\).

    Reimpell, "Quantum Information and Convex Optimization", Ph.D. thesis,
    Technische Universitaet Braunschweig (2008), Section 3.3.4, studies the
    same tough-error-model question in the Kraus-operator formulation, records
    the Knill-Laflamme-Viola lower-bound strategy based on Radon's theorem,
    proves another lower bound by flattening spectra, and gives numerical
    upper-bound evidence from random and \(SU(2)\)-covariant channels. The
    thesis concludes that there remains a large gap between known lower and
    upper bounds, suggesting either tougher error models or better bounds are
    still needed.

    The exact one-dimensional case is known. If
    \(\mathcal E=\operatorname{span}\{A\}\), the condition is the existence of
    a rank-\(k\) projection \(P\) with
    \[
      P A^\dagger A P=\lambda P .
    \]
    Choi, Kribs, and Zyczkowski, "Quantum error correcting codes from the
    compression formalism", Reports on Mathematical Physics 58(1), 77-91
    (2006), identify this with the higher-rank numerical range of the Hermitian
    operator \(A^\dagger A\). Their Hermitian formula implies existence for
    \(k\le\lceil n/2\rceil\). Taking \(A^\dagger A\) with simple spectrum gives
    no scalar compression of rank larger than \(\lceil n/2\rceil\), hence
    \(c(1,n)=\lceil n/2\rceil\).

    Later operator-system and higher-rank-numerical-range literature reformulates
    the same Knill-Laflamme compression condition as the existence of quantum
    anticliques for product spans or noncommutative graphs. This gives related
    bounds and special cases, but it does not provide an exact determination of
    \(c(e,n)\) for all \(1\le e\le n^2\) nor a general classification of tough
    error models.
  </known>

  <refs>
    <ref>Open Quantum Problems, Tough error models, Problem 14, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/tough-error-models, used for the problem statement, the Knill-Laflamme condition, the lower bound \(c(e,n)\gt n/(e^2(e^2+1))\), and the projective-measurement upper bound \(c(e,n)\le\lceil n/e\rceil\).</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 14, Tough error models, in Quantum computation.</ref>
    <ref>E. Knill, R. Laflamme, A. Ashikhmin, H. Barnum, L. Viola, and W. H. Zurek, Introduction to Quantum Error Correction, arXiv:quant-ph/0207170 (2002), DOI: 10.48550/arXiv.quant-ph/0207170, used for background on quantum error correction, error models, decoding, subsystems, and the correctability criterion.</ref>
    <ref>E. Knill, R. Laflamme, and L. Viola, Theory of Quantum Error Correction for General Noise, Physical Review Letters 84(11), 2525-2528 (2000), DOI: 10.1103/PhysRevLett.84.2525, arXiv:quant-ph/9908066, used for the general-noise quantum-code existence bound cited by OQP and the Radon-theorem lower-bound method.</ref>
    <ref>M. Reimpell, Quantum Information and Convex Optimization, Ph.D. thesis, Technische Universitaet Braunschweig (2008), https://www.osti.gov/etdeweb/servlets/purl/21161752, Section 3.3.4, used for subsequent analytic and numerical work on tough error models, the integer form of the Radon-theorem lower-bound strategy, comparison of lower bounds, and evidence that the exact finite-parameter problem remained open.</ref>
    <ref>N. Weaver, The "quantum" Turan problem for operator systems, Pacific Journal of Mathematics 301(1), 335-349 (2019), DOI: 10.2140/pjm.2019.301.335, arXiv:1802.07394, used for the operator-system terminology of quantum anticliques and related dimension-bound context.</ref>
    <ref>M.-D. Choi, D. W. Kribs, and K. Zyczkowski, Quantum error correcting codes from the compression formalism, Reports on Mathematical Physics 58(1), 77-91 (2006), DOI: 10.1016/S0034-4877(06)80041-8, used for the compression and higher-rank-numerical-range reformulation of Knill-Laflamme error-correction conditions.</ref>
  </refs>

  <ask>
    Give a theorem that determines the integer \(c(e,n)\) for every positive
    pair \((e,n)\), with proof for the nonvacuous range \(1\le e\le n^2\) and
    with the convention \(c(e,n)=n\) for \(e\gt n^2\). The theorem must prove
    both sides: for every \(e\)-dimensional \(\mathcal E\le B(\mathcal H)\),
    construct or prove existence of a correcting code of dimension at least
    \(c(e,n)\), and exhibit \(e\)-dimensional error models whose largest
    correcting code has dimension exactly \(c(e,n)\). For \(e\gt n^2\), the
    convention \(c(e,n)=n\) is only a bookkeeping convention; no tough error
    model is required because no \(e\)-dimensional error model exists.

    A result giving only lower bounds, only upper bounds, only asymptotic
    bounds, only numerical evidence, only random or covariant examples, only
    channel-constrained Kraus families, only projective-measurement examples,
    only one previously unrecorded value of \((e,n)\), or only one regime such
    as fixed \(e\) and \(n\to\infty\), is progress unless it implies the exact
    determination of \(c(e,n)\) in every finite nonvacuous case. Rediscovering
    the \(e=1\) case above is not new progress. A claimed nearly tight family
    must state and prove the approximation ratio or asymptotic error term
    relative to \(c(e,n)\).

    Because this is a minimax determination problem, a claimed disproof should
    identify the proposed formula, classification, algorithm, or asymptotic
    assertion being refuted. A single hard error model disproves only proposed
    lower bounds above its value of \(m(\mathcal E)\); a single new code
    construction disproves only proposed upper bounds below its dimension.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
