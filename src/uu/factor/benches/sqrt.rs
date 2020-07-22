use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use uu_factor::numeric;

fn floor_sqrt(c: &mut Criterion) {
    let inputs = {
        // Deterministic RNG; use an explicitely-named RNG to guarantee stability
        use rand::{RngCore, SeedableRng};
        use rand_chacha::ChaCha8Rng;
        const SEED: u64 = 0xb007_d007;
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);

        std::iter::repeat_with(move || rng.next_u64())
    };

    let mut group = c.benchmark_group("floor_sqrt");
    for n in inputs.take(10) {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| numeric::floor_sqrt(n));
        });
    }
    group.finish()
}

fn exact_sqrt(c: &mut Criterion) {
    // Deterministic RNG; use an explicitely-named RNG to guarantee stability
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    let random_inputs = {
        const SEED: u64 = 0x5b47_eef4_dba9_93c0;
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);
        std::iter::repeat_with(move || rng.next_u64())
    };
    let square_inputs = {
        const SEED: u64 = 0x74cb_7058_4b06_d988;
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);
        std::iter::repeat_with(move || {
            let n = rng.next_u32() as u64;
            n * n
        })
    };

    let mut group = c.benchmark_group("exact_sqrt");
    for n in random_inputs.take(10).chain(square_inputs.take(10)) {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| numeric::exact_sqrt(n));
        });
    }
    group.finish()
}

criterion_group!(benches, floor_sqrt, exact_sqrt);
criterion_main!(benches);
