use rust_ml_cli::init_summarization_model;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

// fn sort_arr_benchmark(c: &mut Criterion) {
//     let mut arr = black_box(
//         [6, 2, 4, 1, 9, -2, 5]
//     );
//
//     c.bench_function(
//         "sorting algorithm",
//         |b| b.iter(|| sort_arr(&mut arr))
//     );
// }


fn nlp_benchmark(c: &mut Criterion) {
    let mut inputs = black_box(
        ["ss"]
    );
    let summarization_model = init_summarization_model();

    c.bench_function(
        "nlp algorithm",
        |b| b.iter(|| summarization_model.summarize(&inputs))
    );
}
criterion_group!(benches, nlp_benchmark);
criterion_main!(benches);