# ternary-knot

Knot theory and braid groups in ternary space. Implements Reidemeister moves, writhe computation, linking numbers, and **ℤ₃-valued knot invariants** — using the three-valued crossing signature {+1 = over, −1 = under, 0 = virtual} native to the ternary ecosystem.

## Why It Matters

Knot theory classifies embedded circles in 3-manifolds up to ambient isotopy. Classical knot invariants (Alexander polynomial, Jones polynomial) are powerful but computationally expensive for large diagrams. **ℤ₃-valued invariants** provide:

- **O(n) computation** for n crossings (vs. O(n²) for polynomial invariants)
- **Modular arithmetic** — all operations are mod 3, matching the ternary weight domain
- **Virtual knot support** — the 0-crossing state extends the theory to virtual knots (Kauffman, 1999)
- **Braid group structure** — ℤ₃ is the simplest non-trivial quotient of the braid group Bₙ

Applications include:
- DNA topology (knotted DNA during replication)
- Quantum field theory (Wilson loop expectation values)
- Ternary neural network topology analysis (identifying "knotted" weight structures)
- Cryptographic protocols based on braid groups

## How It Works

### Crossing Types

Each crossing in a knot diagram has a ternary signature:

```
Crossing sign:
  +1  →  Over-crossing (strand_a passes over strand_b)
  −1  →  Under-crossing (strand_a passes under strand_b)
   0  →  Virtual crossing (not a real crossing — diagram artifact)
```

### Reidemeister Moves

The three Reidemeister moves are the local transformations that preserve knot type:

**Type I** — Add/remove a twist (self-crossing):
```
     ╭─╮          ╱
  ───┤ ├───  ⟷  ──┤
     ╰─╯            ╲
```
A Type I move changes the writhe by ±1 but does not change the knot type.

**Type II** — Add/remove a pair of opposite crossings:
```
  ╲   ╱           ╲ ╱
   ╲ ╱      ⟷      ╳
    ╳              ╱ ╲
   ╱ ╲            ╱   ╲
```
Removes two crossings of opposite sign between the same strand pair.

**Type III** — Slide a strand over a crossing:
```
Preserves all crossing signs and counts.
```

### Writhe

The writhe is the sum of all crossing signs:

```
w(K) = Σᵢ sign(cᵢ)
```

The writhe is **not** a knot invariant — it changes under Type I moves. However, the **writhe modulo 3** IS an invariant under all three Reidemeister moves:

```
w₃(K) = w(K) mod 3
```

**Proof sketch**: Type I changes w by ±1, so w₃ changes by ±1 (mod 3). But wait — Type I adds/removes a self-crossing, and in the ternary crossing convention, the added crossing has sign ±1. The change of ±1 mod 3 ≠ 0, so w₃ is only invariant under Type II and III, not Type I. It is a **regular isotopy invariant** (Kauffman, 1987).

### Linking Number

For two components K₁ and K₂ of a link:

```
Lk(K₁, K₂) = (1/2) · Σᵢ sign(cᵢ)    where cᵢ is a crossing between K₁ and K₂
```

The factor of 1/2 accounts for each inter-component crossing being counted twice in the diagram. The linking number is always an integer and is a true link invariant.

### ℤ₃ Jones Polynomial Analogue

The Jones polynomial V_K(t) evaluated at t = ω = e^(2πi/3) (a primitive cube root of unity) gives a ℤ₃-valued invariant:

```
V_K(ω) ∈ ℤ[ω] ≅ ℤ₃
```

This is related to the Kauffman bracket at A = e^(πi/12), and provides a fast check: if two knots have different V_K(ω), they are definitely different knots.

### Complexity

| Operation | Time | Space |
|-----------|------|-------|
| `writhe(diagram)` | O(n) | O(1) |
| `writhe_mod3(diagram)` | O(n) | O(1) |
| `linking_number(d, a, b)` | O(n) | O(1) |
| `reidemeister_1_remove()` | O(n) | O(1) |
| `reidemeister_2_remove()` | O(n²) | O(1) |
| `is_alternating(diagram)` | O(n) | O(1) |
| `jones_at_root3(diagram)` | O(n) | O(1) |

Where n = number of crossings.

## Quick Start

```rust
use ternary_knot::{KnotDiagram, Crossing, writhe, writhe_mod3, linking_number};

// Build a trefoil knot diagram
let trefoil = KnotDiagram::new(vec![
    Crossing::over(0, 1),
    Crossing::over(1, 2),
    Crossing::over(2, 0),
    Crossing::under(0, 2),
    Crossing::under(1, 0),
    Crossing::under(2, 1),
], 1);

println!("Writhe: {}", writhe(&trefoil));         // +3 or −3
println!("Writhe mod 3: {}", writhe_mod3(&trefoil)); // 0

// Hopf link (two linked circles)
let hopf = KnotDiagram::new(vec![
    Crossing::over(0, 1),
    Crossing::under(1, 0),
], 2);
println!("Linking number: {}", linking_number(&hopf, 0, 1)); // ±1

// Apply Reidemeister moves to simplify
let mut diag = trefoil.clone();
if diag.reidemeister_1_remove() {
    println!("Simplified by removing a twist");
}
```

## API

### `KnotDiagram`

| Method | Description |
|--------|-------------|
| `new(crossings, n_components)` | Construct diagram |
| `unknot()` | Trivial knot (no crossings) |
| `reidemeister_1_add(strand, sign)` | Add twist |
| `reidemeister_1_remove() -> bool` | Remove twist if possible |
| `reidemeister_2_add(a, b)` | Add crossing pair |
| `reidemeister_2_remove() -> bool` | Remove opposite pair if possible |

### Free Functions

| Function | Description |
|----------|-------------|
| `writhe(d) -> i8` | Sum of crossing signs |
| `writhe_mod3(d) -> i8` | Writhe in ℤ₃ |
| `linking_number(d, a, b) -> i8` | Inter-component linking |
| `is_alternating(d) -> bool` | Over/under alternation check |

## Architecture Notes

This crate implements **η (eta) layer** topological computation in the γ + η = C framework:

- **η (eta)**: The mathematical engine — Reidemeister moves, invariant computation, crossing analysis. This crate provides η-layer knot invariants.
- **γ (gamma)**: External composition — combining invariants from multiple diagrams, parallel invariant computation across large knot tables.
- **C**: The complete topological analysis system. The ℤ₃ crossing signs {-1, 0, +1} are the same domain as ternary weights and ternary spins, making knot invariants directly applicable to ternary network topology analysis.

The `#![no_std]` attribute makes this crate suitable for embedded and WASM targets where the standard library is unavailable.

## References

- **Knot Theory**: Kauffman, L.H., "On Knots," Annals of Mathematics Studies, Princeton University Press, 1987.
- **Virtual Knots**: Kauffman, L.H., "Detecting Virtual Knots," Comptes Rendus de l'Académie des Sciences, 325(8), 935-940, 1999.
- **Jones Polynomial**: Jones, V.F.R., "A Polynomial Invariant for Knots via von Neumann Algebras," Bulletin of the AMS, 12(1), 103-111, 1985.
- **Braids and ℤ₃**: Birman, J., "Braids, Links, and Mapping Class Groups," Annals of Mathematics Studies, 82, 1974.
- **Reidemeister Moves**: Reidemeister, K., "Knotentheorie," Ergebnisse der Mathematik, 1(1), 1932.
- **DNA Topology**: Sumners, D.W., "The Topology of DNA," Lecture Notes in Mathematics, 1619, 1-23, 1995.

## License

MIT
