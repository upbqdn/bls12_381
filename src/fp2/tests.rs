#[cfg(feature = "pairings")]
use super::*;

#[cfg(feature = "pairings")]
#[test]
fn test_vartime_inversion_matches_constant_time() {
    use rand_core::SeedableRng;

    let mut rng = rand_xorshift::XorShiftRng::from_seed([0x63; 16]);
    for value in [
        Fp2::zero(),
        Fp2::one(),
        -Fp2::one(),
        Fp2 {
            c0: Fp::zero(),
            c1: Fp::one(),
        },
    ] {
        assert_eq!(value.invert_vartime(), Option::<Fp2>::from(value.invert()));
    }
    for _ in 0..256 {
        let value = Fp2::try_random(&mut rng).unwrap();
        let inverse = value.invert_vartime().unwrap();
        assert_eq!(inverse, value.invert().unwrap());
        assert_eq!(value * inverse, Fp2::one());
    }
}
