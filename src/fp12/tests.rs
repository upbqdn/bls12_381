use super::*;

#[cfg(test)]
impl Fp12 {
    fn mul_by_014_reference(&self, c0: &Fp2, c1: &Fp2, c4: &Fp2) -> Fp12 {
        let aa = self.c0.mul_by_01(c0, c1);
        let bb = self.c1.mul_by_1(c4);
        let o = c1 + c4;
        let c1 = self.c1 + self.c0;
        let c1 = c1.mul_by_01(c0, &o);
        let c1 = c1 - aa - bb;
        let c0 = bb;
        let c0 = c0.mul_by_nonresidue();
        let c0 = c0 + aa;

        Fp12 { c0, c1 }
    }
}

#[cfg(feature = "pairings")]
#[test]
fn test_fused_sparse_product_matches_original_and_dense() {
    use rand_core::SeedableRng;

    fn check(f: Fp12, c0: Fp2, c1: Fp2, c4: Fp2) {
        let dense = Fp12 {
            c0: Fp6 {
                c0,
                c1,
                c2: Fp2::zero(),
            },
            c1: Fp6 {
                c0: Fp2::zero(),
                c1: c4,
                c2: Fp2::zero(),
            },
        };
        assert_eq!(
            f.mul_by_014(&c0, &c1, &c4),
            f.mul_by_014_reference(&c0, &c1, &c4)
        );
        assert_eq!(f.mul_by_014(&c0, &c1, &c4), f * dense);
    }

    // Canonical raw limbs just below p maximize the six-term accumulator.
    let high = Fp::from_raw_unchecked([
        0xb9fe_ffff_ffff_aaaa,
        0x1eab_fffe_b153_ffff,
        0x6730_d2a0_f6b0_f624,
        0x6477_4b84_f385_12bf,
        0x4b1b_a7b6_434b_acd7,
        0x1a01_11ea_397f_e69a,
    ]);
    let high = Fp2 { c0: high, c1: high };
    let dense_high = Fp6 {
        c0: high,
        c1: high,
        c2: high,
    };
    for f in [
        Fp12::zero(),
        Fp12::one(),
        -Fp12::one(),
        Fp12 {
            c0: dense_high,
            c1: dense_high,
        },
    ] {
        for c0 in [Fp2::zero(), Fp2::one(), -Fp2::one(), high] {
            for c1 in [Fp2::zero(), Fp2::one(), -Fp2::one(), high] {
                for c4 in [Fp2::zero(), Fp2::one(), -Fp2::one(), high] {
                    check(f, c0, c1, c4);
                }
            }
        }
    }
    let mut rng = rand_xorshift::XorShiftRng::from_seed([0x64; 16]);
    for _ in 0..256 {
        check(
            Fp12::try_random(&mut rng).unwrap(),
            Fp2::try_random(&mut rng).unwrap(),
            Fp2::try_random(&mut rng).unwrap(),
            Fp2::try_random(&mut rng).unwrap(),
        );
    }
}
