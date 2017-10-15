pub mod print_utils {
    use std::io;
    use std::collections;

    //Drukuje na ekran macierz
    pub fn print_matrix(matrix: Vec<Vec<i32>>) {

        println!("Macierz grafu:");

        for i in 0..matrix.len() {

            println!("");

            for j in 0..matrix[i as usize].len() {

                print!("{} ", matrix[i as usize][j as usize]);

            }
        }

        println!("");
    }

}