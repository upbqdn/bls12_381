use super::*;

#[test]
fn test_direct_subtraction_matches_negation_addition() {
    use rand_core::SeedableRng;

    fn check(a: Fp, b: Fp) {
        assert_eq!(a - b, Fp::add(&Fp::neg(&b), &a));
        assert_eq!((a - b) + b, a);
    }

    let mut near_modulus = MODULUS;
    near_modulus[0] -= 1;
    let boundaries = [
        Fp::zero(),
        Fp::one(),
        -Fp::one(),
        Fp([1, 0, 0, 0, 0, 0]),
        Fp(near_modulus),
    ];
    for a in boundaries {
        for b in boundaries {
            check(a, b);
        }
        // Cross each limb boundary, exercising borrow propagation in both directions.
        for limb in 0..6 {
            let mut raw = [0; 6];
            raw[limb] = 1;
            let b = Fp(raw);
            for b in [b, b - Fp([1, 0, 0, 0, 0, 0])] {
                check(a, b);
                check(b, a);
            }
        }
    }
    let mut rng = rand_xorshift::XorShiftRng::from_seed([0x61; 16]);
    for _ in 0..1024 {
        let a = Fp::try_random(&mut rng).unwrap();
        let b = Fp::try_random(&mut rng).unwrap();
        check(a, b);
        check(b, a);
        check(a, a);
    }
}

#[cfg(feature = "pairings")]
#[test]
fn test_vartime_inversion_matches_constant_time() {
    use rand_core::SeedableRng;

    fn check(value: Fp) {
        assert_eq!(value.invert_vartime(), Option::<Fp>::from(value.invert()));
        if let Some(inverse) = value.invert_vartime() {
            assert_eq!(value * inverse, Fp::one());
        }
    }

    check(Fp::zero());
    check(Fp::one());
    check(-Fp::one());
    let mut near_modulus = MODULUS;
    near_modulus[0] -= 1;
    check(Fp(near_modulus));
    assert_eq!(Fp(MODULUS).invert_vartime(), None);
    assert_eq!(Fp([u64::MAX; 6]).invert_vartime(), None);
    // All canonical one-bit raw representations, including even GCD inputs.
    for bit in 0..381 {
        let mut raw = [0; 6];
        raw[bit / 64] = 1 << (bit % 64);
        check(Fp(raw));
    }
    let mut rng = rand_xorshift::XorShiftRng::from_seed([0x62; 16]);
    for _ in 0..256 {
        check(Fp::try_random(&mut rng).unwrap());
    }
}
