//! # ternary-knot
//!
//! Knot theory and braid groups in ternary space.
//! Reidemeister moves, linking numbers, writhe, and Z₃-valued knot invariants.

#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};

/// A crossing in a knot diagram.
/// +1 = over-crossing, -1 = under-crossing, 0 = virtual crossing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Crossing {
    pub sign: i8,     // +1, -1, or 0
    pub strand_a: usize,
    pub strand_b: usize,
}

impl Crossing {
    pub fn over(a: usize, b: usize) -> Self {
        Self { sign: 1, strand_a: a, strand_b: b }
    }

    pub fn under(a: usize, b: usize) -> Self {
        Self { sign: -1, strand_a: a, strand_b: b }
    }

    pub fn virtual_crossing(a: usize, b: usize) -> Self {
        Self { sign: 0, strand_a: a, strand_b: b }
    }

    /// Flip the crossing (over ↔ under)
    pub fn flip(&self) -> Self {
        Self { sign: -self.sign, strand_a: self.strand_a, strand_b: self.strand_b }
    }
}

/// A knot diagram: a sequence of crossings.
#[derive(Debug, Clone)]
pub struct KnotDiagram {
    pub crossings: Vec<Crossing>,
    pub n_components: usize,
}

impl KnotDiagram {
    pub fn new(crossings: Vec<Crossing>, n_components: usize) -> Self {
        Self { crossings, n_components }
    }

    pub fn unknot() -> Self {
        Self { crossings: vec![], n_components: 1 }
    }

    /// Apply Reidemeister I: add or remove a twist (self-crossing)
    pub fn reidemeister_1_remove(&mut self) -> bool {
        for i in 0..self.crossings.len() {
            let c = &self.crossings[i];
            if c.strand_a == c.strand_b {
                self.crossings.remove(i);
                return true;
            }
        }
        false
    }

    pub fn reidemeister_1_add(&mut self, strand: usize, sign: i8) {
        self.crossings.push(Crossing { sign, strand_a: strand, strand_b: strand });
    }

    /// Apply Reidemeister II: remove a pair of opposite crossings
    pub fn reidemeister_2_remove(&mut self) -> bool {
        for i in 0..self.crossings.len() {
            for j in (i + 1)..self.crossings.len() {
                let ci = &self.crossings[i];
                let cj = &self.crossings[j];
                if ci.strand_a == cj.strand_a && ci.strand_b == cj.strand_b && ci.sign == -cj.sign {
                    // Remove both — remove higher index first
                    self.crossings.remove(j);
                    self.crossings.remove(i);
                    return true;
                }
            }
        }
        false
    }

    pub fn reidemeister_2_add(&mut self, a: usize, b: usize) {
        self.crossings.push(Crossing::over(a, b));
        self.crossings.push(Crossing::under(a, b));
    }
}

/// Compute the writhe of a knot diagram (sum of crossing signs)
pub fn writhe(diagram: &KnotDiagram) -> i8 {
    diagram.crossings.iter().map(|c| c.sign).sum()
}

/// Compute the writhe modulo 3 (Z₃ invariant)
pub fn writhe_mod3(diagram: &KnotDiagram) -> i8 {
    let w = writhe(diagram);
    ((w % 3) + 3) % 3
}

/// Linking number between two components
/// Counts signed crossings where strand_a and strand_b belong to different components
pub fn linking_number(diagram: &KnotDiagram, comp_a: usize, comp_b: usize) -> i8 {
    let mut count: i8 = 0;
    for c in &diagram.crossings {
        if (c.strand_a == comp_a && c.strand_b == comp_b) ||
           (c.strand_a == comp_b && c.strand_b == comp_a) {
            count += c.sign;
        }
    }
    count / 2
}

/// Check if crossings alternate between over and under
pub fn is_alternating(diagram: &KnotDiagram) -> bool {
    if diagram.crossings.len() < 2 {
        return true;
    }
    for i in 1..diagram.crossings.len() {
        if diagram.crossings[i].sign == diagram.crossings[i - 1].sign {
            return false;
        }
    }
    true
}

/// Z₃ knot invariant: sum of crossing signs weighted by strand indices, mod 3
pub fn ternary_knot_invariant(diagram: &KnotDiagram) -> i8 {
    let mut val: i8 = 0;
    for (i, c) in diagram.crossings.iter().enumerate() {
        // Weight by index and crossing sign
        val += ((i as i8 + 1) * c.sign * ((c.strand_a as i8 + c.strand_b as i8 + 1) % 3 + 1));
    }
    ((val % 3) + 3) % 3
}

/// A braid generator: σ_i (positive) or σ_i^{-1} (negative)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BraidGen {
    pub index: usize,   // which strand crossing (0-indexed)
    pub positive: bool, // true = σ, false = σ^{-1}
}

/// A braid word: sequence of generators
#[derive(Debug, Clone)]
pub struct BraidWord {
    pub n_strands: usize,
    pub generators: Vec<BraidGen>,
}

impl BraidWord {
    pub fn new(n_strands: usize) -> Self {
        Self { n_strands, generators: vec![] }
    }

    pub fn sigma(&mut self, i: usize) -> &mut Self {
        self.generators.push(BraidGen { index: i, positive: true });
        self
    }

    pub fn sigma_inv(&mut self, i: usize) -> &mut Self {
        self.generators.push(BraidGen { index: i, positive: false });
        self
    }

    /// Close the braid into a knot diagram (Alexander's theorem)
    pub fn close(&self) -> KnotDiagram {
        let mut crossings = vec![];
        for g in &self.generators {
            let sign = if g.positive { 1 } else { -1 };
            crossings.push(Crossing {
                sign,
                strand_a: g.index,
                strand_b: g.index + 1,
            });
        }
        KnotDiagram::new(crossings, 1)
    }

    /// Count the number of crossings in the braid
    pub fn crossing_count(&self) -> usize {
        self.generators.len()
    }

    /// Compute the braid's exponent sum (sum of signs)
    pub fn exponent_sum(&self) -> i8 {
        self.generators.iter().map(|g| if g.positive { 1 } else { -1 }).sum()
    }
}

/// Simplified Kauffman bracket for Z₃
/// Computes a Jones-like polynomial mod 3
pub fn jones_polynomial_ternary(diagram: &KnotDiagram) -> i8 {
    // Simplified: Kauffman bracket <K> = A^(writhe) * sum_over_states
    // For Z₃, we compute the bracket mod 3
    let n = diagram.crossings.len();
    if n == 0 {
        return 1; // unknot
    }

    // Each crossing has 2 smoothings. We enumerate all 2^n states.
    let n_states = 1usize << n.min(12); // cap at 4096 states for sanity
    let mut bracket: i8 = 0;

    for state in 0..n_states {
        let mut a_power: i8 = 0;
        let mut loops = 1i8;

        for (i, _c) in diagram.crossings.iter().enumerate() {
            if i >= 12 { break; }
            let smooth = (state >> i) & 1;
            if smooth == 0 {
                a_power += 1; // A-smoothing contributes A
            } else {
                a_power -= 1; // B-smoothing contributes A^{-1}
            }
        }
        // Simplified: each state contributes A^(a_power) * (-2)^(loops-1) mod 3
        // In Z₃: (-2) ≡ 1, so loops contribute 1
        // A ≡ 1 mod 3 (ternary: A^k ≡ 1 for all k)
        // So each state contributes 1 mod 3
        bracket += 1;
    }

    // Multiply by (-A^3)^writhe
    let w = writhe(diagram);
    // In Z₃: (-A^3)^w ≡ (-1)^w. Since -1 ≡ 2 mod 3, 2^w mod 3 cycles {1, 2, 1, ...}
    let writhe_factor = if w % 2 == 0 { 1i8 } else { 2i8 };

    let result = bracket * writhe_factor;
    ((result % 3) + 3) % 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crossing_creation() {
        let c = Crossing::over(0, 1);
        assert_eq!(c.sign, 1);
        assert_eq!(c.strand_a, 0);
        assert_eq!(c.strand_b, 1);
    }

    #[test]
    fn test_crossing_flip() {
        let c = Crossing::over(0, 1);
        let flipped = c.flip();
        assert_eq!(flipped.sign, -1);
    }

    #[test]
    fn test_virtual_crossing() {
        let c = Crossing::virtual_crossing(0, 1);
        assert_eq!(c.sign, 0);
    }

    #[test]
    fn test_unknot() {
        let k = KnotDiagram::unknot();
        assert_eq!(k.crossings.len(), 0);
        assert_eq!(writhe(&k), 0);
    }

    #[test]
    fn test_writhe_trefoil() {
        // Right-handed trefoil: three positive crossings
        let k = KnotDiagram::new(vec![
            Crossing::over(0, 1),
            Crossing::over(0, 1),
            Crossing::over(0, 1),
        ], 1);
        assert_eq!(writhe(&k), 3);
    }

    #[test]
    fn test_writhe_mod3() {
        let k = KnotDiagram::new(vec![
            Crossing::over(0, 1),
            Crossing::over(0, 1),
            Crossing::over(0, 1),
        ], 1);
        assert_eq!(writhe_mod3(&k), 0); // 3 mod 3 = 0
    }

    #[test]
    fn test_reidemeister_1_remove() {
        let mut k = KnotDiagram::new(vec![
            Crossing { sign: 1, strand_a: 0, strand_b: 0 },
        ], 1);
        assert!(k.reidemeister_1_remove());
        assert_eq!(k.crossings.len(), 0);
    }

    #[test]
    fn test_reidemeister_1_no_remove() {
        let mut k = KnotDiagram::new(vec![
            Crossing::over(0, 1),
        ], 1);
        assert!(!k.reidemeister_1_remove());
    }

    #[test]
    fn test_reidemeister_2_remove() {
        let mut k = KnotDiagram::new(vec![
            Crossing::over(0, 1),
            Crossing::under(0, 1),
        ], 1);
        assert!(k.reidemeister_2_remove());
        assert_eq!(k.crossings.len(), 0);
    }

    #[test]
    fn test_reidemeister_2_no_remove() {
        let mut k = KnotDiagram::new(vec![
            Crossing::over(0, 1),
            Crossing::over(0, 1),
        ], 1);
        assert!(!k.reidemeister_2_remove());
    }

    #[test]
    fn test_linking_number_hopf() {
        // Hopf link: two crossings between components 0 and 1
        let k = KnotDiagram::new(vec![
            Crossing::over(0, 1),
            Crossing::over(0, 1),
        ], 2);
        assert_eq!(linking_number(&k, 0, 1), 1);
    }

    #[test]
    fn test_linking_number_unlinked() {
        let k = KnotDiagram::new(vec![
            Crossing::over(0, 0),
            Crossing::over(1, 1),
        ], 2);
        assert_eq!(linking_number(&k, 0, 1), 0);
    }

    #[test]
    fn test_is_alternating() {
        let k = KnotDiagram::new(vec![
            Crossing::over(0, 1),
            Crossing::under(0, 1),
            Crossing::over(0, 1),
        ], 1);
        assert!(is_alternating(&k));
    }

    #[test]
    fn test_not_alternating() {
        let k = KnotDiagram::new(vec![
            Crossing::over(0, 1),
            Crossing::over(0, 1),
        ], 1);
        assert!(!is_alternating(&k));
    }

    #[test]
    fn test_ternary_invariant_unknot() {
        let k = KnotDiagram::unknot();
        assert_eq!(ternary_knot_invariant(&k), 0);
    }

    #[test]
    fn test_braid_word() {
        let mut b = BraidWord::new(3);
        b.sigma(0).sigma(1).sigma_inv(0);
        assert_eq!(b.crossing_count(), 3);
        assert_eq!(b.exponent_sum(), 1);
    }

    #[test]
    fn test_braid_close() {
        let mut b = BraidWord::new(2);
        b.sigma(0).sigma(0).sigma(0);
        let k = b.close();
        assert_eq!(k.crossings.len(), 3);
    }

    #[test]
    fn test_jones_polynomial_unknot() {
        let k = KnotDiagram::unknot();
        assert_eq!(jones_polynomial_ternary(&k), 1);
    }
}
