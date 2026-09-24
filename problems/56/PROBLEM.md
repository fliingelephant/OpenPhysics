# 56 Achievability of Non-Commuting Quantum Dichotomy Transformation Rates

```xml
<problem id="56">
  <status>open</status>
  <kind>proof</kind>
  <field>QTD</field>
  <name>Achievability of Non-Commuting Quantum Dichotomy Transformation Rates</name>
  <keys>quantum dichotomy quantum dichotomies Blackwell order approximate Blackwell order relative majorisation relative majorization non-commuting target states noncommuting target dichotomy second-order asymptotics second order rate small deviation regime state interconversion rate state conversion rate coherent thermodynamics coherent quantum thermodynamics sesquinormal distribution Rayleigh-Normal distribution reversibility parameter relative entropy variance quantum Stein's lemma second order asymptotics achievability conjecture optimality bound converse bound Conjecture 10 Conjecture 2 hypothesis testing achievability thermal operations Lipka-Bartosik Chubb Renes Tomamichel Korzekwa PRX Quantum 020335</keys>

  <claim>
    Fix $\epsilon\in(0,1)$. Let $(\rho_1,\sigma_1)$ be a quantum dichotomy on
    a fixed finite-dimensional Hilbert space $\mathcal H_1$ and
    $(\rho_2,\sigma_2)$ a quantum dichotomy on a fixed finite-dimensional
    Hilbert space $\mathcal H_2$, such that
    $\operatorname{supp}\rho_1\subseteq\operatorname{supp}\sigma_1$,
    $\operatorname{supp}\rho_2\subseteq\operatorname{supp}\sigma_2$,
    $D(\rho_1\|\sigma_1),V(\rho_1\|\sigma_1),D(\rho_2\|\sigma_2),V(\rho_2\|\sigma_2)\in(0,\infty)$,
    $[\rho_1,\sigma_1]\neq0$, and $[\rho_2,\sigma_2]\neq0$. Then
    \[
    \lim_{n\to\infty}\ \sqrt n\left[\,R_n^*(\epsilon)\ -\ \frac{D(\rho_1\|\sigma_1)+\sqrt{V(\rho_1\|\sigma_1)/n}\;S_{1/\xi}^{-1}(\epsilon)}{D(\rho_2\|\sigma_2)}\,\right]=0,
    \]
    i.e. the right-hand side, already proved to be a valid upper bound on
    $R_n^*(\epsilon)$ up to $o(1/\sqrt n)$ for every such $(\rho_1,\sigma_1)$,
    $(\rho_2,\sigma_2)$ by Lipka-Bartosik, Chubb, Renes, Tomamichel, and
    Korzekwa, "Quantum Dichotomies and Coherent Thermodynamics beyond
    First-Order Asymptotics," PRX Quantum 5, 020335 (2024), Theorem 3, Eq.
    (26), is in fact achieved up to $o(1/\sqrt n)$, i.e. equality holds, even
    though $\rho_2$ and $\sigma_2$ do not commute.
  </claim>

  <def>
    A quantum dichotomy is a pair $(\rho,\sigma)$ of density operators on a
    common finite-dimensional Hilbert space. For dichotomies $(\rho,\sigma)$
    on $\mathcal H$ and $(\rho',\sigma')$ on $\mathcal H'$, and
    $\epsilon_\rho,\epsilon_\sigma\ge0$, write
    \[
    (\rho,\sigma)\succeq_{(\epsilon_\rho,\epsilon_\sigma)}(\rho',\sigma')
    \]
    iff there exists a completely positive trace-preserving (CPTP) linear
    map $\mathcal E$, from states on $\mathcal H$ to states on $\mathcal
    H'$, with
    \[
    T(\mathcal E(\rho),\rho')\le\epsilon_\rho,\qquad
    T(\mathcal E(\sigma),\sigma')\le\epsilon_\sigma,
    \]
    where $T(X,Y):=\tfrac12\lVert X-Y\rVert_1$ is the trace distance and
    $\lVert X\rVert_1:=\operatorname{tr}\sqrt{X^\dagger X}$. For a
    dichotomy $(\rho,\sigma)$, $[\rho,\sigma]:=\rho\sigma-\sigma\rho$, so
    $[\rho,\sigma]\neq0$ means $\rho$ and $\sigma$ do not commute.

    For $n\in\mathbb N$ and $\epsilon_n\in[0,1]$,
    \[
    R_n^*(\epsilon_n):=\sup\Bigl\{R\ge0:\ (\rho_1^{\otimes n},\sigma_1^{\otimes n})\succeq_{(\epsilon_n,0)}\bigl(\rho_2^{\otimes\lfloor Rn\rfloor},\sigma_2^{\otimes\lfloor Rn\rfloor}\bigr)\Bigr\}
    \]
    is the optimal asymptotic transformation rate from $(\rho_1,\sigma_1)$
    to $(\rho_2,\sigma_2)$ at error $\epsilon_n$: the $\sigma$-component
    must be reproduced exactly, and only the $\rho$-component may incur
    error $\epsilon_n$, measured in trace distance.

    For a dichotomy $(\rho,\sigma)$ with
    $\operatorname{supp}\rho\subseteq\operatorname{supp}\sigma$, the
    (Umegaki) relative entropy and relative entropy variance are
    \[
    D(\rho\|\sigma):=\operatorname{tr}\bigl[\rho(\log\rho-\log\sigma)\bigr],
    \qquad
    V(\rho\|\sigma):=\operatorname{tr}\bigl[\rho(\log\rho-\log\sigma)^2\bigr]-D(\rho\|\sigma)^2,
    \]
    with $\log,\exp$ taken in a fixed but arbitrary base used consistently
    throughout. For dichotomies $(\rho_1,\sigma_1)$, $(\rho_2,\sigma_2)$
    with $D(\rho_1\|\sigma_1),D(\rho_2\|\sigma_2)>0$ and
    $V(\rho_1\|\sigma_1),V(\rho_2\|\sigma_2)$ finite, the reversibility
    parameter is
    \[
    \xi:=\frac{V(\rho_1\|\sigma_1)/D(\rho_1\|\sigma_1)}{V(\rho_2\|\sigma_2)/D(\rho_2\|\sigma_2)}.
    \]

    Let $\Phi$ be the standard normal cumulative distribution function and
    $\Phi^{-1}$ its inverse. For $\nu\in(0,\infty)$ and $\epsilon\in(0,1)$,
    Lipka-Bartosik, Chubb, Renes, Tomamichel, and Korzekwa (cited in full
    below), Lemma 1, Eq. (17), define
    \[
    S_\nu^{-1}(\epsilon):=\min_{x\in(\epsilon,1)}\bigl[\sqrt{\nu}\,\Phi^{-1}(x)-\Phi^{-1}(x-\epsilon)\bigr],
    \]
    proving this is the inverse cumulative distribution function of a
    probability distribution $S_\nu$ (their "sesquinormal distribution",
    defined implicitly by an optimisation over CDFs in their Eq. (15))
    that interpolates between the normal distribution ($\nu\to0$) and the
    half-normal distribution ($\nu=1$). In the claim above, $\nu=1/\xi$.
  </def>

  <known>
    Lipka-Bartosik, Chubb, Renes, Tomamichel, and Korzekwa, "Quantum
    Dichotomies and Coherent Thermodynamics beyond First-Order
    Asymptotics," PRX Quantum 5, 020335 (2024), DOI:
    10.1103/PRXQuantum.5.020335, arXiv:2303.05524v3 (12 May 2024), Theorem
    2 (Section III.2, first-order rate), prove
    $\lim_{n\to\infty}R_n^*(\epsilon)=D(\rho_1\|\sigma_1)/D(\rho_2\|\sigma_2)$
    for constant $\epsilon\in(0,1)$ whenever $[\rho_2,\sigma_2]=0$, and only
    the upper bound $\limsup_{n\to\infty}R_n^*(\epsilon)\le
    D(\rho_1\|\sigma_1)/D(\rho_2\|\sigma_2)$ for general (possibly
    non-commuting) $(\rho_2,\sigma_2)$, with $(\rho_1,\sigma_1)$ arbitrary
    throughout.

    Lipka-Bartosik, Chubb, Renes, Tomamichel, and Korzekwa's Theorem 3
    (Section III.2, small-deviation rate, same paper as above) proves the
    second-order refinement of this pattern: for constant $\epsilon\in(0,1)$
    and $[\rho_2,\sigma_2]=0$, $R_n^*(\epsilon)$ equals the right-hand side
    of the claim above up to $o(1/\sqrt n)$ (their Eq. (25)); for general
    $(\rho_2,\sigma_2)$, i.e. dropping $[\rho_2,\sigma_2]=0$, the same
    expression remains a valid upper bound up to $o(1/\sqrt n)$ (their Eq. (26))
    -- this is the "known optimal rate" of the claim above, and it is
    proved for arbitrary $(\rho_1,\sigma_1)$, commuting or not.
    Lipka-Bartosik et al.'s proofs of the equality half in Theorem 3 (and
    of the analogous equality halves of Theorems 4, 5, 6, 7, 8, covering
    the moderate-deviation, large-deviation, zero-error, and extreme
    regimes respectively) all proceed via an achievability construction
    built on a hypothesis-testing reformulation of the Blackwell order
    (their Eq. (2), attributed there to an earlier reference) that
    Lipka-Bartosik et al. state (Section II.1) is known to characterise
    the Blackwell order only for commuting dichotomies, and to fail, once
    both dichotomies are non-commuting, beyond two-dimensional density
    matrices. No matching achievability construction for non-commuting
    $(\rho_2,\sigma_2)$ is given for any of these regimes.

    Immediately after stating Theorem 8, Lipka-Bartosik et al. write: "all
    of the achievability bounds rely on connections to hypothesis testing
    that only apply for commuting targets $[\rho_2,\sigma_2]=0$, while all
    of our optimality bounds apply for general states. We conjecture that
    there might exist alternative protocols capable of saturating these
    bounds," and state this as their Conjecture 10: "All of the optimality
    bounds in [Theorems 3-8] are achievable, for general states,
    $[\rho_1,\sigma_1]\neq0$ and $[\rho_2,\sigma_2]\neq0$." The claim
    recorded above is exactly the $\epsilon\in(0,1)$, small-deviation
    ($n\to\infty$ at fixed $\epsilon$) instance of Lipka-Bartosik et al.'s
    Conjecture 10, i.e. Conjecture 10 restricted to Theorem 3.
    Lipka-Bartosik et al.'s Conjecture 10 also asserts the analogous
    achievability statement for the moderate-deviation (Theorem 4),
    large-deviation (Theorems 5-6), zero-error (Theorem 7), and extreme
    (Theorem 8) regimes; those regimes use different limiting formulas
    (involving Renyi relative entropies rather than only $D$ and $V$) and
    are not certified open by this entry, which records only the
    small-deviation instance as its single canonical claim.

    Lipka-Bartosik et al.'s Conjecture 9, stated immediately before
    Conjecture 10 in the same discussion, is a different open problem: it
    asks whether the small-deviation formula (Eq. (25)) generalises from
    trace distance to a general statistical distance $\delta$ while
    keeping $[\rho_2,\sigma_2]=0$ (commuting targets). Conjecture 9 varies
    the error metric at fixed commutativity; Conjecture 10, recorded here,
    varies commutativity at fixed error metric (trace distance). The two
    are independent claims about the same Theorem 3 formula.

    Lipka-Bartosik, Chubb, Renes, Tomamichel, and Korzekwa's own numbered
    label for the claim recorded here is Conjecture 10, not Conjecture 2;
    "Conjecture 2" does not identify any numbered statement in
    arXiv:2303.05524v3 or in PRX Quantum 5, 020335.
  </known>

  <refs>
    <ref>Lipka-Bartosik, P., Chubb, C. T., Renes, J. M., Tomamichel, M.,
    and Korzekwa, K., "Quantum Dichotomies and Coherent Thermodynamics
    beyond First-Order Asymptotics," PRX Quantum 5, 020335 (2024). DOI:
    10.1103/PRXQuantum.5.020335. arXiv:2303.05524v3 (12 May 2024). Lemma 1
    (Section III.1, sesquinormal distribution and its inverse CDF, Eqs.
    (14)-(20)); Theorem 2 (Section III.2, first-order rate, Eqs.
    (22)-(23)); Theorem 3 (Section III.2, small-deviation rate, Eqs.
    (25)-(26)); Conjecture 10 (Section III.2, paragraph following Theorem
    8).</ref>
  </refs>

  <ask>
    Prove the claim -- for every finite-dimensional dichotomy pair
    $(\rho_1,\sigma_1)$, $(\rho_2,\sigma_2)$ meeting the stated domain
    conditions and every $\epsilon\in(0,1)$, prove the existence of, for
    each $n$, a sequence of CPTP maps witnessing $R_n^*(\epsilon)$ within
    $o(1/\sqrt n)$ of the stated formula -- or disprove it by exhibiting one such
    pair of dichotomies and one $\epsilon\in(0,1)$ for which
    \[
    \limsup_{n\to\infty}\ \sqrt n\left[\frac{D(\rho_1\|\sigma_1)+\sqrt{V(\rho_1\|\sigma_1)/n}\;S_{1/\xi}^{-1}(\epsilon)}{D(\rho_2\|\sigma_2)}-R_n^*(\epsilon)\right]>0,
    \]
    i.e. a persistent gap, not vanishing faster than $1/\sqrt n$, between the
    known upper bound and the true optimal rate. Partial progress -- e.g.
    achievability restricted to $\dim\mathcal H_2=2$, to a specific family
    of $(\rho_2,\sigma_2)$, or with the $o(1/\sqrt n)$ requirement weakened to a
    coarser convergence rate -- should be reported as such rather than as
    a full proof or disproof.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
