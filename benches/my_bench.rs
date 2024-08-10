use cranevm::engine::engine::Engine;
use criterion::{criterion_group, criterion_main, Criterion};
#[inline(never)]
fn run(engine: &mut Engine) {
    engine.run();
}

fn benchmark(c: &mut Criterion) {
    let mut engine = Engine::new("test.cb");
    engine.load_file();
    c.bench_function("function_to_benchmark", |b| {
        b.iter(|| run(&mut engine));
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
