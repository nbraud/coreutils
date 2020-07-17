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

criterion_group!(benches, floor_sqrt);
criterion_main!(benches);
