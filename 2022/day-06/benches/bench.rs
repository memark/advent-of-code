#[macro_use]
extern crate bencher;

use bencher::Bencher;
use day_06::{file, solve_with_window_size, solve_with_window_size_vec_deque};

fn iter(bench: &mut Bencher) {
    let input = file("input");

    bench.iter(|| solve_with_window_size(&input, 14))
}

fn vec_deque(bench: &mut Bencher) {
    let input = file("input");

    bench.iter(|| solve_with_window_size_vec_deque(&input, 14))
}

benchmark_group!(benches, vec_deque, iter);
benchmark_main!(benches);
