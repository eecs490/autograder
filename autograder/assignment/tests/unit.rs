mod our_tests {
    use array_macro::array;
    use assignment::solution::fib;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    #[should_panic]
    #[test]
    fn panic() {
        submission::solution::fib(-1);
    }

    #[test]
    fn random() {
        let seed: [u8; 32] = array![|x| x as u8; 32];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        for _ in 0..10 {
            let n: i32 = rng.gen_range(0, 10);
            assert_eq!(submission::solution::fib(n), fib(n), "input: {}", n);
        }
    }
}
