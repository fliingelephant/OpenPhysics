# QC024 Secret Key From All Entangled States

```xml
<problem id="QC024">
  <status>open</status>
  <name>Secret Key From All Entangled States</name>
  <keys>secret key distillable key zero distillable key key-undistillable entangled key-undistillable states private key cryptographic key key distillation quantum key agreement lopc locc public communication eve purification trusted state bipartite entanglement entangled states bound entanglement bound entangled states ppt positive partial transpose npt distillable entanglement private states pbits pdits devetak winter devetak-winter one-way key rate quantum communication open quantum problems oqp problem 24</keys>

  <claim>
    For every finite-dimensional bipartite entangled state
    \(\rho_{AB}\in\mathcal{D}(\mathcal{H}_A\otimes\mathcal{H}_B)\),
    with \(\dim\mathcal{H}_A,\dim\mathcal{H}_B\lt\infty\), the trusted-state
    distillable secret-key rate against a purifying eavesdropper is positive:
    \[
      K_D(\rho_{AB})\gt 0 .
    \]
  </claim>

  <def>
    A bipartite state \(\rho_{AB}\) is entangled if it is not separable, i.e.,
    it is not in the closed convex hull of product states
    \(\alpha_A\otimes\beta_B\). Let
    \(\psi_{ABE}=|\psi\rangle\langle\psi|_{ABE}\) be any purification of
    \(\rho_{AB}\). Different purifications are related by an isometry on Eve's
    system and give the same key-distillation problem.

    An LOPC protocol is a finite-round protocol consisting of local quantum
    operations by Alice and Bob and authenticated public classical
    communication whose transcript is available to Eve. Equivalently, for
    blocklength \(n\) it is represented by a tripartite channel
    \[
      \Lambda_n\in
      \mathrm{LOPC}(A^n:B^n|E^n\to K_n:K'_n|E'_n),
    \]
    where Alice and Bob output classical registers \(K_n,K'_n\) and Eve keeps
    her quantum side information together with all public messages in \(E'_n\).
    In the bipartite trusted-state formulation, the same admissible operations
    are often described as LOCC between Alice and Bob, with the induced public
    transcript included in Eve's final system.

    For an integer \(M\ge 1\), a perfect \(M\)-valued secret key independent of
    Eve has the form
    \[
      \tau^M_{KK'E'}
      =
      \frac{1}{M}\sum_{x=0}^{M-1}
      |x\rangle\langle x|_K\otimes |x\rangle\langle x|_{K'}
      \otimes \sigma_{E'}
    \]
    for some state \(\sigma_{E'}\). A rate \(R\ge 0\) is achievable from
    \(\rho_{AB}\) if there are integers \(M_n\) and LOPC protocols
    \(\Lambda_n\) such that
    \[
      \liminf_{n\to\infty}\frac{\log_2 M_n}{n}\ge R
    \]
    and
    \[
      \inf_{\sigma_{E'_n}}
      \left\|
        \Lambda_n(\psi_{ABE}^{\otimes n})
        -
        \tau^{M_n}_{K_nK'_nE'_n}
      \right\|_1
      \longrightarrow 0 .
    \]
    The distillable secret-key rate \(K_D(\rho_{AB})\) is the supremum of all
    achievable \(R\). This is a trusted-state, device-dependent asymptotic
    definition. It is not a device-independent QKD claim and not the
    untrusted-source verification setting in which an adversary supplies states
    whose quality Alice and Bob must test.
  </def>

  <known>
    This entry was collected from Open Quantum Problems, Problem 24, "Secret
    key from all entangled states". The Open Quantum Problems index lists
    Problem 24 in the Quantum communication category. OQP Problem 24 asks
    whether all bipartite entangled states can be used to generate secret keys,
    and its background formulates key generation from many copies of a
    tripartite state \(\rho_{ABE}\) by LOPC protocols whose public
    communication is overheard by Eve. The OQP page does not state a dimension
    restriction. This entry records the finite-dimensional trusted-state form,
    matching the finite-dimensional bipartite-state setting of the cited
    distillable-key examples and avoiding additional energy-constrained
    infinite-dimensional conventions not specified by OQP Problem 24.

    Horodecki, Horodecki, Horodecki, and Oppenheim, "Secure key from bound
    entanglement", Phys. Rev. Lett. 94, 160502 (2005), show that some bound
    entangled states have positive distillable secret key. This separates key
    distillation from the strategy of first distilling singlets. It does not
    prove that every entangled state has positive distillable key.

    Devetak and Winter, "Distillation of secret key and entanglement from
    quantum states", Proc. R. Soc. A 461, 207-235 (2005), give one-way public
    communication distillation theorems for secret key and entanglement,
    including formulas when Eve holds a purification of Alice and Bob's state.
    These one-way lower bounds are sufficient for many positive-key examples
    but do not settle the all-entangled-states claim.

    Horodecki, Pankowski, Horodecki, and Horodecki, "Low-Dimensional Bound
    Entanglement With One-Way Distillable Cryptographic Key", IEEE Trans. Inf.
    Theory 54, 2621-2625 (2008), provide a class of bound entangled states with
    positive one-way distillable key, including examples in \(4\otimes 4\).
    These examples show that positive key in bound entanglement is not only a
    high-dimensional phenomenon.

    Horodecki, Leung, Lo, and Oppenheim, "Quantum key distribution based on
    arbitrarily weak distillable entangled states", Phys. Rev. Lett. 96, 070501
    (2006), study extraction from untrusted states after privacy verification,
    even when distillable entanglement is arbitrarily small. That result is a
    related QKD-verification result, not the trusted-state LOPC claim stated
    here.

    Horodecki, Sikorski, Das, and Wilde, "Cost of quantum secret key", Quantum
    10, 2098 (2026), develop a resource theory of quantum secret key while
    explicitly operating under the assumption that entangled states with zero
    distillable key do not exist, and state that their results would remain
    valid if such states were shown to exist. This is modern evidence that the
    existence or nonexistence of entangled states with \(K_D=0\) is still a
    live assumption in the literature, not a proved theorem.
  </known>

  <refs>
    <ref>Open Quantum Problems, Secret key from all entangled states, Problem 24, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/secret-key-from-all-entangled-states, used for the problem statement, the LOPC background, and the listed partial results.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 24, Secret key from all entangled states, in Quantum communication.</ref>
    <ref>K. Horodecki, M. Horodecki, P. Horodecki, and J. Oppenheim, Secure key from bound entanglement, Physical Review Letters 94, 160502 (2005), DOI: 10.1103/PhysRevLett.94.160502, arXiv:quant-ph/0309110, used for positive distillable key from some bound entangled states and the private-key paradigm.</ref>
    <ref>I. Devetak and A. Winter, Distillation of secret key and entanglement from quantum states, Proceedings of the Royal Society A 461, 207-235 (2005), DOI: 10.1098/rspa.2004.1372, arXiv:quant-ph/0306078, used for one-way secret-key distillation and the purifying-Eve formulation.</ref>
    <ref>K. Horodecki, L. Pankowski, M. Horodecki, and P. Horodecki, Low-Dimensional Bound Entanglement With One-Way Distillable Cryptographic Key, IEEE Transactions on Information Theory 54(6), 2621-2625 (2008), DOI: 10.1109/TIT.2008.921709, arXiv:quant-ph/0506203, used for low-dimensional bound entangled states with positive one-way distillable key.</ref>
    <ref>K. Horodecki, D. Leung, H.-K. Lo, and J. Oppenheim, Quantum key distribution based on arbitrarily weak distillable entangled states, Physical Review Letters 96, 070501 (2006), DOI: 10.1103/PhysRevLett.96.070501, arXiv:quant-ph/0510067, used for the related untrusted-state QKD-verification variant.</ref>
    <ref>K. Horodecki, L. Sikorski, S. Das, and M. M. Wilde, Cost of quantum secret key, Quantum 10, 2098 (2026), DOI: 10.22331/q-2026-05-06-2098, arXiv:2402.17007v2, used for the modern resource-theory status and the explicit assumption concerning entangled states with zero distillable key.</ref>
  </refs>

  <ask>
    Prove or disprove the claim that every finite-dimensional bipartite
    entangled state has \(K_D(\rho_{AB})\gt 0\) under the trusted-state
    asymptotic LOPC definition above.

    A proof must apply to every finite pair of local dimensions and every
    entangled density operator on
    \(\mathcal{H}_A\otimes\mathcal{H}_B\), with Eve holding a purification,
    and must exhibit or characterize LOPC protocols with a strictly positive
    asymptotic secret-key rate. A proof for all distillable entangled states,
    for all pure entangled states, for a family of bound entangled states, or
    for a particular class such as PPT private states is progress unless it
    covers all finite-dimensional entangled states.

    A disproof must give a finite-dimensional entangled state \(\rho_{AB}\) and
    prove \(K_D(\rho_{AB})=0\) for the trusted-state purifying-Eve model. A
    result showing that a state is not useful for a particular one-way
    protocol, for a fixed measurement scheme, for a device-independent
    protocol, or for an untrusted-source verification protocol is progress or a
    gap unless it proves zero distillable key under arbitrary asymptotic LOPC
    protocols as defined here.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
