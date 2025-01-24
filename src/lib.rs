use std::{sync::Arc, thread::{self, JoinHandle}};

pub mod io;
pub mod parser;

pub const MATRIX_ORDER: usize = 4;

pub struct MatrixNxN {
    data: [[i64; MATRIX_ORDER]; MATRIX_ORDER]
}

impl MatrixNxN {
    pub fn get_element(&self, r: usize, c: usize) -> &i64 {
        if c < MATRIX_ORDER && r < MATRIX_ORDER {
            Some(&self.data[r][c]).unwrap()
        }
        else {
            None.expect("Can't access non-existent elements!")
        }
    }

    pub fn get_column(&self, c: usize) -> [&i64; MATRIX_ORDER] {
        let mut col: [&i64; MATRIX_ORDER] = [&0; MATRIX_ORDER];
        for r in 0..MATRIX_ORDER {
            col[r] = &self.data[r][c];
        }
        col
    }

    pub fn get_data(&self) -> &[[i64; MATRIX_ORDER]; MATRIX_ORDER] {
        &self.data
    }

    pub fn set_element(&mut self, r: usize, c: usize, e: i64) {
        self.data[r][c] = e;
    }

    pub fn set_matrix(&mut self, vec_data: Vec<i64>) {
        for r in 0..MATRIX_ORDER {
            for c in 0..MATRIX_ORDER {
                self.data[r][c] = vec_data[r * MATRIX_ORDER + c];
            }
        }
    }

    pub fn scale_column_by_factor(col: [&i64; MATRIX_ORDER], factor: &i64) -> [i64; MATRIX_ORDER] {
        let mut result_col: [i64; MATRIX_ORDER] = [0; MATRIX_ORDER];
        for r in 0..MATRIX_ORDER {
            result_col[r] = *col[r] * *factor;
        }
        result_col
    }

    pub fn dot_product(&self, other_matrix: &MatrixNxN, other_col_number: usize) -> [i64; MATRIX_ORDER] {
        let mut result_col = [0; MATRIX_ORDER];
        let other_column = other_matrix.get_column(other_col_number);
        if other_col_number < MATRIX_ORDER {
            for r in 0..MATRIX_ORDER {
                for c in 0..MATRIX_ORDER {
                    result_col[c] += Self::scale_column_by_factor(self.get_column(r), other_column[r])[c]
                }
            }
            result_col
        }
        else {
            panic!("Can't access non-existent columns!")
        }
    }

    pub fn transpose(data: [[i64; MATRIX_ORDER]; MATRIX_ORDER]) -> [[i64; MATRIX_ORDER]; MATRIX_ORDER] {
        let mut transposed = [[0; MATRIX_ORDER]; MATRIX_ORDER];
        for r in 0..MATRIX_ORDER {
            for c in 0..MATRIX_ORDER {
                transposed[r][c] = data[c][r];
            }
        }
        transposed
    }

    pub fn make_marix_from_vec(vec_data: Vec<i64>) -> Self {
        let mut data = [[0i64; MATRIX_ORDER]; MATRIX_ORDER];
        for r in 0..MATRIX_ORDER {
            for c in 0..MATRIX_ORDER {
                data[r][c] = vec_data[r * MATRIX_ORDER + c];
            }
        }
        Self { data }
    }

    pub fn make_marix(data: [[i64; MATRIX_ORDER]; MATRIX_ORDER]) -> Self {
        Self { data }
    }

    pub fn zero_matrix() -> Self {
        MatrixNxN { data: [[0; MATRIX_ORDER]; MATRIX_ORDER] }
    }

    pub fn show_matrix(&self) {
        for r in 0..MATRIX_ORDER {
            for c in 0..MATRIX_ORDER {
                print!("{} ", self.get_element(r, c));
            }
            println!();
        }
    }
}

pub struct MatrixPair(
    pub(crate) MatrixNxN,
    pub(crate) MatrixNxN
);

impl MatrixPair {
    pub fn get_matrix_product(&self) -> MatrixNxN {
        let mut data = [[0i64; MATRIX_ORDER]; MATRIX_ORDER];
        for c in 0..MATRIX_ORDER {
            data[c] = self.0.dot_product(&self.1, c);
        }
        MatrixNxN { data: MatrixNxN::transpose(data) }
    }

    fn split_work(work_size: usize, count: usize) -> Vec<usize> {
        let mut split_indices = Vec::new();
        for i in 0..=count {
            split_indices.push(i * work_size / count);
        }
        split_indices
    }
}

#[allow(unused_must_use)]
pub fn compute_with_cpu(pairs: Vec<MatrixPair>, thread_count: usize) {
    let split_indices = MatrixPair::split_work(pairs.len(), thread_count);
    let mut handles = Vec::<JoinHandle<()>>::new();
    let arc_pairs = Arc::<Vec<MatrixPair>>::from(pairs);

    for i in 0..thread_count {
        let start_index = split_indices[i];
        let last_index = split_indices[i + 1];
        let pairs = Arc::<Vec<MatrixPair>>::clone(&arc_pairs);

        handles.push(
            thread::spawn(
                move || {
                    for j in start_index..last_index {
                        &pairs[j].get_matrix_product();
                    }
                }
            )
        );
    }

    for handle in handles {
        handle.join().unwrap();
    }
}