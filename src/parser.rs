use crate::{MatrixNxN, MatrixPair, MATRIX_ORDER};

pub fn get_matrix_pairs_from_string(file_content: String) -> Vec<MatrixPair> {
    let mut matrix_pairs = Vec::<MatrixPair>::new();

    let pairs: Vec<&str> = file_content.split::<&str>(",\n").collect();
    
    let mut line_pair: Vec<&str>;

    let mut line_left: Vec<&str>;
    let mut numbers_left: Vec<i64>;

    let mut line_right: Vec<&str>;
    let mut numbers_right: Vec<i64>;

    for i in 0..pairs.len() {
        
        numbers_left = Vec::new();
        let mut m1: MatrixNxN = MatrixNxN::zero_matrix();

        numbers_right = Vec::new();
        let mut m2: MatrixNxN = MatrixNxN::zero_matrix();


        for line in pairs[i].lines() {
            line_pair = line.split("   ").collect();

            line_left = line_pair[0].split(" ").collect();
            for l in 0..MATRIX_ORDER {
                numbers_left.push(line_left[l].parse().unwrap());
            }
            line_right = line_pair[1].split(" ").collect();
            for r in 0..MATRIX_ORDER {
                numbers_right.push(line_right[r].parse().unwrap());
            }
        }

        m1.set_matrix(numbers_left);
        m2.set_matrix(numbers_right);
        
        matrix_pairs.push(MatrixPair(m1, m2));
    }

    matrix_pairs
}