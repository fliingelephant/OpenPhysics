# 13 Lockable Distillable Entanglement

```xml
<problem id="13">
  <status>open</status>
  <kind>construction</kind>
  <field>QIT</field>
  <name>Lockable Distillable Entanglement</name>
  <keys>lockable entanglement measures locking entanglement single qubit local discard loss of one qubit dephasing measurement flag flag construction two-way distillable entanglement distillible entanglement distillable entanglement d two way one-way distillable entanglement one-way ed ed to locc lopc secret key rate distillable key kd ab-locking e-locking private states squashed entanglement entanglement cost entanglement of formation logarithmic negativity relative entropy of entanglement entanglement of purification open quantum problems oqp problem 25 entanglement theory</keys>

  <claim>
    The two-way LOCC distillable entanglement \(D_{\leftrightarrow}\) is
    lockable by local loss of one qubit: there exist finite-dimensional
    systems \(A_0,A_j,B_j\), with \(A_0\cong\mathbb{C}^2\), and bipartite
    states \(\rho^{(j)}_{A_0A_jB_j}\) such that
    \[
      \Delta_j
      =
      D_{\leftrightarrow}^{A_0A_j:B_j}(\rho^{(j)}_{A_0A_jB_j})
      -
      D_{\leftrightarrow}^{A_j:B_j}
      \bigl(\operatorname{tr}_{A_0}\rho^{(j)}_{A_0A_jB_j}\bigr)
      \longrightarrow +\infty
    \]
    as \(j\to\infty\).
  </claim>

  <def>
    All Hilbert spaces in this entry are finite-dimensional. For a bipartite
    state \(\omega_{XY}\), the two-way distillable entanglement
    \(D_{\leftrightarrow}^{X:Y}(\omega)\) is the supremum of all rates
    \(R\ge 0\) for which there are integers \(M_n\) and two-way LOCC
    protocols
    \[
      \Lambda_n\in\mathrm{LOCC}_{\leftrightarrow}
      (X^n:Y^n\to \widehat X_n:\widehat Y_n)
    \]
    satisfying
    \[
      \liminf_{n\to\infty}{\log_2 M_n\over n}\ge R
      \quad\text{and}\quad
      \left\|
        \Lambda_n(\omega_{XY}^{\otimes n})
        -
        \Phi^{M_n}_{\widehat X_n\widehat Y_n}
      \right\|_1\to 0 .
    \]
    Here
    \[
      |\Phi_M\rangle
      =
      {1\over\sqrt M}\sum_{x=0}^{M-1}|x\rangle_{\widehat X}|x\rangle_{\widehat Y}
    \]
    and \(\Phi^M=|\Phi_M\rangle\langle\Phi_M|\). The protocols may use
    arbitrary finite rounds of local quantum operations and two-way classical
    communication for each blocklength \(n\).

    For a bipartite entanglement or key measure \(E\), local-discard
    lockability means that there are finite-dimensional systems
    \(A_0,A_j,B_j\), with \(\dim A_0=2\), and states
    \(\rho^{(j)}_{A_0A_jB_j}\) such that
    \[
      E^{A_0A_j:B_j}(\rho^{(j)})
      -
      E^{A_j:B_j}(\operatorname{tr}_{A_0}\rho^{(j)})
      \to +\infty .
    \]
    The operation in this entry is exactly the local trace
    \(\operatorname{tr}_{A_0}\), so exactly one qubit is removed from Alice's
    side. Replacing \(\operatorname{tr}_{A_0}\) by local dephasing,
    nonselective measurement, or selective measurement defines a nearby
    locking variant; such a result must be related back to the local-discard
    gap above to prove or disprove the claim.

    The OQP page also asks about the distillable secret-key rate. For a
    bipartite state \(\omega_{XY}\), let \(\psi_{XYE}\) be a purification.
    A rate \(R\ge 0\) is achievable for the trusted-state distillable key
    \(K_D^{X:Y}(\omega)\) if there are integers \(M_n\) and LOPC protocols,
    equivalently LOCC protocols between Alice and Bob with the public
    transcript included in Eve's final system, such that the output classical
    registers \(K_n,K'_n\) satisfy
    \[
      \liminf_{n\to\infty}{\log_2 M_n\over n}\ge R
    \]
    and
    \[
      \inf_{\sigma_{E'_n}}
      \left\|
        \Lambda_n(\psi_{XYE}^{\otimes n})
        -
        {1\over M_n}\sum_{x=0}^{M_n-1}
        |x x\rangle\langle x x|_{K_nK'_n}\otimes\sigma_{E'_n}
      \right\|_1
      \to 0 .
    \]
    \(K_D^{X:Y}(\omega)\) is the supremum of achievable rates. This entry
    defines \(K_D\) to record the second OQP target, but the single canonical
    claim above concerns \(D_{\leftrightarrow}\) only.
  </def>

  <known>
    This entry was collected from Open Quantum Problems, Problem 25,
    "Lockable entanglement measures". The Open Quantum Problems index lists
    Problem 25 in the Entanglement theory category. OQP Problem 25 asks
    whether two-way distillible entanglement and secret key rate are lockable.
    This entry fixes one canonical mathematical claim, the
    \(D_{\leftrightarrow}\) local-discard claim above, because a solution for
    \(D_{\leftrightarrow}\) and a solution for \(K_D\) are not logically the
    same statement without an additional comparison theorem.

    The OQP Problem 25 page describes lockability as extreme sensitivity to
    the loss of a single qubit by one partner. It cites Horodecki, Horodecki,
    Horodecki, and Oppenheim, "Locking entanglement measures with a single
    qubit", arXiv:quant-ph/0404096. The published PRL version, titled "Locking
    Entanglement with a Single Qubit", Phys. Rev. Lett. 94, 200501 (2005),
    proves large drops under one-qubit dephasing or measurement for entanglement
    of formation, entanglement cost, and logarithmic negativity. The published
    PRL version also gives a one-qubit flag example for one-way distillable
    entanglement \(E_D^\to\). The same article proves a one-qubit flag locking
    theorem for convex LOCC monotones \(E\) that obey a subextensive bound
    \(E(\rho)\le C\log d\) and are not asymptotically continuous. It proves that
    the relative entropy of entanglement is nonlockable and states that
    unrestricted distillable entanglement was not settled there. These results
    motivate the present target, but neither the dephasing or measurement
    examples, the one-way \(E_D^\to\) flag example, nor the general flag theorem
    settles two-way distillable entanglement.

    Christandl and Winter, "Uncertainty, Monogamy, and Locking of Quantum
    Correlations", IEEE Trans. Inf. Theory 51, 3159-3165 (2005), show
    locking phenomena for squashed entanglement and entanglement of
    purification, including arbitrarily large drops caused by losing one local
    qubit. This is progress on the landscape of lockable correlation measures,
    not a proof or disproof of the \(D_{\leftrightarrow}\) claim.

    Devetak and Winter, "Distillation of secret key and entanglement from
    quantum states", Proc. R. Soc. A 461, 207-235 (2005), give asymptotic
    distillation theorems for secret key and entanglement in one-way settings
    and provide the purifying-Eve formulation used in the definition of
    \(K_D\) above. Their one-way formulas do not decide whether the two-way
    distillable entanglement \(D_{\leftrightarrow}\), or the two-way
    distillable secret-key rate \(K_D\), has an unbounded one-qubit
    local-discard gap. The OQP \(K_D\) target is AB-locking by local loss from
    Alice or Bob, not E-locking by changing Eve's side information; known
    E-locking or ccq/Holevo-key variants do not settle that target.
  </known>

  <refs>
    <ref>Open Quantum Problems, Problem 25, Lockable entanglement measures, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/lockable-entanglement-measures, used for the problem statement, locking background, and collection provenance.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for the Problem 25 number and Entanglement theory category.</ref>
    <ref>K. Horodecki, M. Horodecki, P. Horodecki, and J. Oppenheim, Locking entanglement measures with a single qubit, arXiv:quant-ph/0404096 (2004), cited by OQP Problem 25 for the original preprint provenance.</ref>
    <ref>K. Horodecki, M. Horodecki, P. Horodecki, and J. Oppenheim, Locking Entanglement with a Single Qubit, Physical Review Letters 94, 200501 (2005), DOI: 10.1103/PhysRevLett.94.200501, used for single-qubit locking definitions, measurement or dephasing examples for several measures, the one-way distillable-entanglement flag example in the published PRL version, the convex-subextensive-nonasymptotically-continuous flag-locking theorem, nonlockability of relative entropy of entanglement, and the open status there of unrestricted distillable entanglement.</ref>
    <ref>M. Christandl and A. Winter, Uncertainty, Monogamy, and Locking of Quantum Correlations, IEEE Transactions on Information Theory 51(9), 3159-3165 (2005), DOI: 10.1109/TIT.2005.853338, arXiv:quant-ph/0501090, used for locking of squashed entanglement and related correlation measures.</ref>
    <ref>I. Devetak and A. Winter, Distillation of secret key and entanglement from quantum states, Proceedings of the Royal Society A 461, 207-235 (2005), DOI: 10.1098/rspa.2004.1372, arXiv:quant-ph/0306078, used for the purifying-Eve secret-key formulation and asymptotic distillation-rate definitions.</ref>
  </refs>

  <ask>
    Prove or disprove the claim that \(D_{\leftrightarrow}\) has an unbounded
    local-discard locking gap caused by removing one qubit from one party.

    A proof must give a sequence
    \(\rho^{(j)}_{A_0A_jB_j}\) with \(\dim A_0=2\), prove the stated
    asymptotic separation of
    \(D_{\leftrightarrow}^{A_0A_j:B_j}(\rho^{(j)})\) from
    \(D_{\leftrightarrow}^{A_j:B_j}(\operatorname{tr}_{A_0}\rho^{(j)})\),
    and justify all LOCC distillation-rate bounds used on both sides of the
    gap.

    A disproof must prove a dimension-independent finite upper bound on
    \[
      D_{\leftrightarrow}^{A_0A:B}(\rho_{A_0AB})
      -
      D_{\leftrightarrow}^{A:B}(\operatorname{tr}_{A_0}\rho_{A_0AB})
    \]
    for every finite-dimensional state \(\rho_{A_0AB}\) with
    \(\dim A_0=2\), or otherwise prove that no sequence can make this
    difference diverge. A bound for one family of states, for one local
    dimension, or for one-way LOCC is progress unless it covers the full
    two-way finite-dimensional domain above. A result only about \(K_D\), or
    only about a dephasing or measurement version of locking, is progress on
    an adjacent OQP target unless it is explicitly connected to the
    local-discard \(D_{\leftrightarrow}\) claim.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
