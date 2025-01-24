use std::{path::Path, time::Instant};

use matrix_test::{self, compute_with_cpu, io::read_file, parser::get_matrix_pairs_from_string};

fn main() {
    let contents = read_file(Path::new("task.txt"));
    let work_pairs = get_matrix_pairs_from_string(contents);

    let now = Instant::now();
    compute_with_cpu(work_pairs, 4);
    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
}
