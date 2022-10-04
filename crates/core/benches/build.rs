
use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[inline]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}
pub fn build_bench(c: &mut Criterion){
  c.bench_function("build", |b| b.iter(|| {fibonacci(black_box(20))}));
}
criterion_group!(benches, build_bench);
criterion_main!(benches);