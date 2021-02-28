# Circuit commitments

## Committing to the circuit assignments

At the start of proof creation, the prover has a table of cell assignments that it claims
satisfy the constraint system. The table has $n = 2^k$ rows, and is broken into advice,
instance, and fixed columns. We define $F_{i,j}$ as the assignment in the $j$th row of
the $i$th fixed column. Without loss of generality, we'll similarly define $A_{i,j}$ to
represent the advice and instance assignments.

> We separate fixed columns here because they are provided by the verifier, whereas the
> advice and instance columns are provided by the prover. In practice, the commitments to
> instance and fixed columns are computed by both the prover and verifier, and only the
> advice commitments are stored in the proof.

To commit to these assignments, we construct Lagrange polynomials of degree $n - 1$ for
each column, over an evaluation domain of size $n$ (where $\omega$ is the $n$th primitive
root of unity):

- $a_i(X)$ interpolates such that $a_i(\omega^j) = A_{i,j}$.
- $f_i(X)$ interpolates such that $f_i(\omega^j) = F_{i,j}$.

We then create a blinding commitment to the polynomial for each column:

$$\mathbf{A} = [\text{Commit}(a_0(X)), \dots, \text{Commit}(a_i(X))]$$
$$\mathbf{F} = [\text{Commit}(f_0(X)), \dots, \text{Commit}(f_i(X))]$$

$\mathbf{F}$ is constructed as part of key generation, using a blinding factor of $1$.
$\mathbf{A}$ is constructed by the prover and sent to the verifier.

## Committing to the lookup permutations

The verifier starts by sampling $\theta$, which is used to keep individual columns within
lookups independent. Then, the prover commits to the permutations for each lookup as
follows:

- Given a lookup with input column polynomials $[A_0(X), \dots, A_{m-1}(X)]$ and table
  column polynomials $[S_0(X), \dots, S_{m-1}(X)]$, the prover constructs two compressed
  polynomials

  $$A_\text{compressed}(X) = \theta^{m-1} A_0(X) + \theta^{m-2} A_1(X) + \dots + \theta A_{m-2}(X) + A_{m-1}(X)$$
  $$S_\text{compressed}(X) = \theta^{m-1} S_0(X) + \theta^{m-2} S_1(X) + \dots + \theta S_{m-2}(X) + S_{m-1}(X)$$

- The prover then permutes $A_\text{compressed}(X)$ and $S_\text{compressed}(X)$ according
  to the [rules of the lookup argument](lookup.md), obtaining $A'(X)$ and $S'(X)$.

Finally, the prover creates blinding commitments for all of the lookups

$$\mathbf{L} = \left[ (\text{Commit}(A'(X))), \text{Commit}(S'(X))), \dots \right]$$

and sends them to the verifier.

## Committing to the equality constraint permutations

The verifier samples $\beta$ and $\gamma$.

For each equality constraint argument:

- The prover constructs a vector $P$:

$$
P_j = \prod\limits_{i=0}^{m-1} \frac{p_i(\omega^j) + \beta \cdot \delta^i \cdot \omega^j + \gamma}{p_i(\omega^j) + \beta \cdot s_i(\omega^j) + \gamma}
$$

- The prover constructs a polynomial $Z_P$ which has a Lagrange basis representation
  corresponding to a running product of $P$, starting at $Z_P(1) = 1$.

See the [Permutation argument](permutation.md#argument-specification) section for more detail.

The prover creates blinding commitments to each $Z_P$ polynomial:

$$\mathbf{Z_P} = \left[\text{Commit}(Z_P(X)), \dots \right]$$

and sends them to the verifier.

## Committing to the lookup permutation product columns

In addition to committing to the individual permuted lookups, for each lookup,
the prover needs to commit to the permutation product column:

- The prover constructs a vector $P$:

$$
P_j = \frac{(A_\text{compressed}(\omega^j) + \beta)(S_\text{compressed}(\omega^j) + \gamma)}{(A'(\omega^j) + \beta)(S'(\omega^j) + \gamma)}
$$

- The prover constructs a polynomial $Z_L$ which has a Lagrange basis representation
  corresponding to a running product of $P$, starting at $Z_L(1) = 1$.

$\beta$ and $\gamma$ are used to combine the permutation arguments for $A'(X)$ and $S'(X)$
while keeping them independent. We can reuse $\beta$ and $\gamma$ from the equality
constraint permutation here because they serve the same purpose in both places, and we
aren't trying to combine the lookup and equality constraint permutation arguments. The
important thing here is that the verifier samples $\beta$ and $\gamma$ after the prover
has created $\mathbf{A}$, $\mathbf{F}$, and $\mathbf{L}$ (and thus commited to all the
cell values used in lookup columns, as well as $A'(X)$ and $S'(X)$ for each lookup).

As before, the prover creates blinding commitments to each $Z_L$ polynomial:

$$\mathbf{Z_L} = \left[\text{Commit}(Z_L(X)), \dots \right]$$

and sends them to the verifier.
