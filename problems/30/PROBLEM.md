# 30 Is There Bound Information?

```xml
<problem id="30">
  <status>open</status>
  <kind>construction</kind>
  <field>QIT</field>
  <name>Is There Bound Information?</name>
  <keys>bound information bound intrinsic information secret key agreement secret-key agreement secret key rate secret-key rate secret key cost information of formation formation cost secrecy extraction public discussion lopc local operations public communication intrinsic information intrinsic conditional information reduced intrinsic information alice bob eve iid tripartite distribution classical secrecy bound entanglement classical analogue multipartite bound information activation open quantum problems oqp problem 47 quantum cryptography</keys>

  <claim>
    There exists a finite-alphabet tripartite probability distribution
    \(P_{XYZ}\) such that, for the i.i.d. source
    \(P_{XYZ}^{\otimes n}\) shared by Alice, Bob, and Eve, the two-way
    public-discussion secret-key rate is zero while the secret-key cost,
    equivalently the information of formation, is strictly positive:
    \[
      S(X;Y\|Z)=0
      \quad\text{and}\quad
      I_{\mathrm{form}}(X;Y|Z)\gt 0 .
    \]
  </claim>

  <def>
    All random variables in this entry have finite alphabets. Alice observes
    \(X^n\), Bob observes \(Y^n\), and Eve observes \(Z^n\), where
    \((X_i,Y_i,Z_i)\) are independent copies of \(P_{XYZ}\). Public
    discussion means an authenticated public classical channel: Alice and Bob
    may use arbitrary local randomness and any finite number of interactive
    messages, and Eve receives the entire transcript.

    A rate \(R\ge 0\) is achievable for secret-key agreement from
    \(P_{XYZ}\) if there are public-discussion protocols \(\Pi_n\) such that
    Alice and Bob output \(M_n\)-valued keys \(K_A^{(n)}\) and \(K_B^{(n)}\),
    with transcript \(C_n\), satisfying
    \[
      \liminf_{n\to\infty}{\log_2 M_n\over n}\ge R,
    \]
    \[
      \Pr[K_A^{(n)}\ne K_B^{(n)}]\to 0,
    \]
    and
    \[
      \left\|
        P_{K_A^{(n)}Z^nC_n}
        -
        U_{M_n}\otimes P_{Z^nC_n}
      \right\|_1\to 0 ,
    \]
    where \(U_{M_n}\) is the uniform distribution on the key alphabet. The
    two-way secret-key rate \(S(X;Y\|Z)\) is the supremum of achievable
    \(R\). The problem is asymptotic and permits arbitrary public-discussion
    protocols; failure of a specified protocol class is not a proof that
    \(S(X;Y\|Z)=0\).

    The secret-key cost, or information of formation,
    \(I_{\mathrm{form}}(X;Y|Z)\), is the infimum of rates \(R\) such that
    there are public-discussion protocols which, using a shared secret seed
    \(J_n\) that is uniform on a set of size \(2^{m_n}\), independent of all
    other private randomness and unknown to Eve, with
    \(\limsup_{n\to\infty}m_n/n\le R\), generate Alice and Bob outputs
    \(\widehat X^n,\widehat Y^n\) and transcript \(C_n\) with the following
    property: there is a random variable \(\widehat Z^n\) such that
    \(P_{\widehat X^n\widehat Y^n\widehat Z^n}\) converges in total variation
    to \(P_{XYZ}^{\otimes n}\), and there is a stochastic map
    \(Q_{C_n|\widehat Z^n}\) such that
    \[
      \left\|
        P_{\widehat X^n\widehat Y^n\widehat Z^n C_n}
        -
        P_{\widehat X^n\widehat Y^n\widehat Z^n}Q_{C_n|\widehat Z^n}
      \right\|_1\to0 .
    \]
    Thus the public transcript in the
    formation protocol gives Eve no more useful information than the
    \(Z^n\)-system of the target source.

    The intrinsic information of Maurer and Wolf is
    \[
      I(X;Y\downarrow Z)=
      \inf_{P_{\overline Z|Z}} I(X;Y|\overline Z),
    \]
    where the infimum is over all stochastic maps from Eve's variable \(Z\)
    to a degraded variable \(\overline Z\). The reduced intrinsic information
    of Renner and Wolf is
    \[
      I(X;Y\downarrow\downarrow Z)=
      \inf_{P_{U|XYZ}}
      \left(I(X;Y\downarrow ZU)+H(U)\right).
    \]
    These quantities are bounds, not the operational target in the claim:
    \[
      S(X;Y\|Z)
      \le I(X;Y\downarrow\downarrow Z)
      \le I(X;Y\downarrow Z)
      \le I_{\mathrm{form}}(X;Y|Z).
    \]
    Proving positive intrinsic information or positive reduced intrinsic
    information is not by itself enough to prove bound information unless
    \(S(X;Y\|Z)=0\) is also proved. Proving a gap between extraction and
    formation is not by itself enough unless the extraction rate is zero and
    the formation cost is positive for the same distribution.
  </def>

  <known>
    This entry was collected from Open Quantum Problems, Problem 47, "Is
    there bound information?". The Open Quantum Problems index lists Problem
    47 in the Quantum cryptography category, with date 2023/04/20. OQP
    Problem 47 asks for a classical secrecy analogue of bound entanglement:
    a tripartite i.i.d. distribution \(P_{XYZ}\) for which no secret key can
    be distilled by public discussion, while secret key is still required to
    form the distribution.

    Maurer, "Secret key agreement by public discussion from common
    information", IEEE Transactions on Information Theory 39(3), 733-742
    (1993), introduced the public-discussion secret-key agreement setting for
    correlated classical data. The present claim uses that two-party
    Alice-Bob-Eve source model, not a device-independent QKD model and not a
    quantum trusted-state distillable-key model.

    Maurer and Wolf, "Unconditionally secure key agreement and the intrinsic
    conditional information", IEEE Transactions on Information Theory 45(2),
    499-514 (1999), introduced intrinsic information
    \(I(X;Y\downarrow Z)\) and proved it is an upper bound on the
    public-discussion secret-key rate. Intrinsic information is not the
    secret-key rate itself.

    Renner and Wolf, "New Bounds in Secret-Key Agreement: The Gap Between
    Formation and Secrecy Extraction", EUROCRYPT 2003, Lecture Notes in
    Computer Science 2656, 562-577 (2003), introduced reduced intrinsic
    information as a stronger upper bound on the secret-key rate and proved
    that intrinsic information lower-bounds the formation cost. They also
    exhibited a strict extraction-versus-intrinsic-information gap, including
    an example with \(S(X;Y\|Z)=1\) and \(I(X;Y\downarrow Z)=3/2\). Such a
    positive-rate gap is not bound information, because OQP Problem 47
    requires \(S(X;Y\|Z)=0\).

    Gisin and Wolf, "Linking Classical and Quantum Key Agreement: Is There
    'Bound Information'?", CRYPTO 2000, Lecture Notes in Computer Science
    1880, 482-500 (2000), gave quantum-to-classical evidence and candidate
    distributions inspired by bound entangled states. Their bound intrinsic
    information terminology is treated here as evidence and progress toward
    bound information, not as the full operational claim. Their conjectural
    bridge and the measured-state candidates do not settle OQP Problem 47
    unless one proves, for a concrete classical distribution, both zero
    public-discussion secret-key rate and positive information of formation.

    Christandl and Winter, "Squashed Entanglement: An Additive Entanglement
    Measure", Journal of Mathematical Physics 45(3), 829-840 (2004),
    introduced squashed entanglement using the intrinsic-information analogy.
    This supports the analogy between secrecy and entanglement measures but
    does not prove or disprove the existence of a classical bipartite bound
    information distribution.

    Acin, Cirac, and Masanes, "Multipartite Bound Information Exists and Can
    Be Activated", Physical Review Letters 92, 107903 (2004), proved bound
    information in a multipartite honest-party scenario and activation of
    such multipartite resources. Prettico and Bae, "Superactivation,
    Unlockability, and Secrecy Distribution of Bound Information", Physical
    Review A 83, 042336 (2011), gave a further four-partite example with
    superactivation and unlockability. These results are genuine
    multipartite results and do not solve the two-honest-party Alice-Bob-Eve
    question in the claim.

    Masanes and Winter, "A Non-distillability Criterion for Secret
    Correlations", Quantum Information and Computation 10(1 and 2), 0152-0159
    (2010), introduced a computable criterion certifying that some
    Alice-Bob-Eve distributions are not asymptotically distillable into
    secret key, and explicitly described the existence of non-distillable
    correlations with positive secrecy cost as open. Prettico and Acin, "Can
    Bipartite Classical Information Resources Be Activated?", Quantum
    Information and Computation 13(3 and 4), 245-265 (2013), showed
    activation behavior for correlations with conjectured bound information.
    Activation evidence for conjectured examples is progress, not a proof of
    the exact zero-rate and positive-cost conditions.
  </known>

  <refs>
    <ref>Open Quantum Problems, Problem 47, Is there bound information?, IQOQI Vienna, https://oqp.iqoqi.oeaw.ac.at/is-there-bound-information, used for the problem statement, the secret-key rate and secret-key cost formulation, the intrinsic-information comparison, and the listed primary references.</ref>
    <ref>Open Quantum Problems, Open Quantum Problems list, https://oqp.iqoqi.oeaw.ac.at/open-quantum-problems, used for collection provenance: this entry is listed as Problem 47, Is there bound information?, in Quantum cryptography, dated 2023/04/20.</ref>
    <ref>U. M. Maurer, Secret key agreement by public discussion from common information, IEEE Transactions on Information Theory 39(3), 733-742 (1993), DOI: 10.1109/18.256484, used for the public-discussion secret-key agreement source model.</ref>
    <ref>U. M. Maurer and S. Wolf, Unconditionally secure key agreement and the intrinsic conditional information, IEEE Transactions on Information Theory 45(2), 499-514 (1999), DOI: 10.1109/18.748999, used for intrinsic information and its role as an upper bound on the secret-key rate.</ref>
    <ref>R. Renner and S. Wolf, New Bounds in Secret-Key Agreement: The Gap Between Formation and Secrecy Extraction, in Advances in Cryptology -- EUROCRYPT 2003, Lecture Notes in Computer Science 2656, 562-577 (2003), DOI: 10.1007/3-540-39200-9_35, https://crypto.ethz.ch/publications/files/RenWol03.pdf, used for reduced intrinsic information, the formation-cost interpretation, and the strict extraction-versus-formation gap that does not itself give zero-rate bound information.</ref>
    <ref>N. Gisin and S. Wolf, Linking Classical and Quantum Key Agreement: Is There 'Bound Information'?, in Advances in Cryptology -- CRYPTO 2000, Lecture Notes in Computer Science 1880, 482-500 (2000), DOI: 10.1007/3-540-44598-6_30, arXiv:quant-ph/0005042, used for the quantum-classical analogy and candidate distributions inspired by bound entanglement.</ref>
    <ref>M. Christandl and A. Winter, Squashed Entanglement: An Additive Entanglement Measure, Journal of Mathematical Physics 45(3), 829-840 (2004), DOI: 10.1063/1.1643788, arXiv:quant-ph/0308088, used for the intrinsic-information analogy behind squashed entanglement.</ref>
    <ref>A. Acin, J. I. Cirac, and Ll. Masanes, Multipartite Bound Information Exists and Can Be Activated, Physical Review Letters 92, 107903 (2004), DOI: 10.1103/PhysRevLett.92.107903, arXiv:quant-ph/0311064, used for the solved multipartite variant and activation caveat.</ref>
    <ref>L. Masanes and A. Winter, A Non-distillability Criterion for Secret Correlations, Quantum Information and Computation 10(1 and 2), 0152-0159 (2010), DOI: 10.26421/QIC10.1-2-11, arXiv:0804.4523, used for non-distillability criteria and the later explicit statement that positive-cost non-distillable two-party secret correlations remained open.</ref>
    <ref>G. Prettico and J. Bae, Superactivation, Unlockability, and Secrecy Distribution of Bound Information, Physical Review A 83, 042336 (2011), DOI: 10.1103/PhysRevA.83.042336, arXiv:1011.2120, used for four-partite bound information, superactivation, and unlockability as multipartite adjacent results.</ref>
    <ref>G. Prettico and A. Acin, Can Bipartite Classical Information Resources Be Activated?, Quantum Information and Computation 13(3 and 4), 245-265 (2013), DOI: 10.26421/QIC13.3-4-6, arXiv:1203.1445, used for activation evidence involving conjectured bipartite bound information and for distinguishing evidence from proof of OQP Problem 47.</ref>
  </refs>

  <ask>
    Prove or disprove the claim that there is a finite-alphabet
    Alice-Bob-Eve distribution \(P_{XYZ}\) with zero asymptotic
    public-discussion secret-key rate and strictly positive information of
    formation.

    A proof must specify a single distribution \(P_{XYZ}\) and prove both
    \(S(X;Y\|Z)=0\) under arbitrary asymptotic two-way public discussion and
    \(I_{\mathrm{form}}(X;Y|Z)\gt 0\). Showing that all known protocols fail,
    showing only that a reduced or intrinsic information bound is positive,
    or deriving a distribution from a bound entangled state without proving
    the exact operational zero-rate and positive-cost conditions is progress
    or a gap, not a proof.

    A disproof must prove that every finite-alphabet tripartite distribution
    with \(S(X;Y\|Z)=0\) has \(I_{\mathrm{form}}(X;Y|Z)=0\), equivalently
    that positive formation cost always implies positive
    public-discussion secret-key rate in this bipartite classical source
    model. A result about multipartite honest-party bound information,
    activation of conjectured examples, one-way public discussion, restricted
    alphabets, or quantum states is progress unless it is explicitly reduced
    to the full Alice-Bob-Eve claim above.
  </ask>

  <out>
    Return one of: proof, disproof, gap, progress.
    State all assumptions and cite every external theorem used.
  </out>
</problem>
```
