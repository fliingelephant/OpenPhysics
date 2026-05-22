# QI003 QCRB Saturability

```xml
<problem id="QI003">
  <status>open</status>
  <name>QCRB Saturability</name>
  <keys>qcrb quantum cramer rao bound quantum fisher information multiparameter quantum metrology single-copy saturability attainability symmetric logarithmic derivative sld covariance matrix quantum estimation probe state</keys>

  <claim>
    For a general finite-dimensional quantum statistical model
    \[
      \theta=(\theta_1,\ldots,\theta_N)\mapsto \rho_\theta,
      \qquad N \gt 1,
    \]
    provide necessary and sufficient conditions, depending on the unspecified
    probe state \(\rho_\theta\) and its symmetric logarithmic derivatives, for
    the single-copy saturability of the symmetric-logarithmic-derivative quantum
    Cramer-Rao bound
    \[
      \operatorname{Cov}(\hat{\theta}) \succeq Q(\theta)^{-1}
    \]
    at regular parameter points where \(Q(\theta)\) is nonsingular.
  </claim>

  <def>
    The model \(\rho_\theta\) is a smooth family of density operators on a
    finite-dimensional Hilbert space. For each parameter \(\theta_k\), the
    symmetric logarithmic derivative \(L_k\) is defined by
    \[
      2\partial_k\rho_\theta=L_k\rho_\theta+\rho_\theta L_k,
    \]
    where \(\partial_k=\partial/\partial\theta_k\). The symmetric-logarithmic-
    derivative quantum Fisher information matrix is
    \[
      Q_{km}(\theta)=\frac{1}{2}\operatorname{tr}
      \left[\rho_\theta(L_kL_m+L_mL_k)\right].
    \]
    The matrix inequality \(A\succeq B\) means that \(A-B\) is positive
    semidefinite. The inverse \(Q(\theta)^{-1}\) is taken at regular parameter
    points where \(Q(\theta)\) is nonsingular. Single-copy saturability means
    that the equality
    \[
      \operatorname{Cov}(\hat{\theta})=Q(\theta)^{-1}
    \]
    can be achieved at the parameter point under consideration by a measurement
    on one copy of \(\rho_\theta\), together with an unbiased or locally
    unbiased estimator in the setting of the quantum Cramer-Rao bound.
  </def>

  <known>
    The published PRX Quantum paper states Problem 3 as asking for general
    necessary and sufficient conditions, depending on an unspecified probe state
    \(\rho_\theta\), for saturability of the QCRB in multiparameter quantum
    metrology on the single-copy level.

    The problem is not the asymptotic many-copy setting with collective
    measurements. In the asymptotic setting, a weaker commutativity condition is
    known to characterize saturability. On the single-copy level, full
    commutativity
    \[
      [L_k,L_m]=0,\qquad 1\le k,m\le N,
    \]
    is sufficient. For pure states, the weaker condition
    \[
      \operatorname{tr}\!\left(\rho_\theta[L_k,L_m]\right)=0,
      \qquad 1\le k,m\le N,
    \]
    is necessary and sufficient. For general mixed states on the single-copy
    level, the published PRX paper records stronger necessary conditions, but
    not a general necessary-and-sufficient characterization.

    The requested conditions must not assume a fixed number of parameters, a
    fixed Hilbert-space dimension, a pure probe state, or unitary parameter
    encoding of the form \(\rho_\theta=U_\theta\varrho U_\theta^\dagger\).
  </known>

  <refs>
    <ref>P. Horodecki, Ł. Rudnicki, and K. Życzkowski, Five Open Problems in Quantum Information Theory, PRX Quantum 3, 010101 (2022), Problem 3.</ref>
    <ref>https://doi.org/10.1103/PRXQuantum.3.010101</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is related to the Problem 3 listing, QCRB saturability, in Quantum information.</ref>
  </refs>

  <ask>
    Give a general necessary-and-sufficient criterion for single-copy
    saturability of the SLD QCRB in the setting above. A valid solution must
    specify both necessity and sufficiency and must apply to arbitrary mixed
    probe states and arbitrary \(N\gt 1\). A result restricted to pure states,
    unitary parameter encoding, commuting SLDs, a fixed dimension, or the
    asymptotic many-copy setting is progress only, not a solution.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
