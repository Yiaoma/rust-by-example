use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn transponse(matrix: Matrix) -> Matrix {
    let new_matrix = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);

    new_matrix
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transponse(matrix));
}
