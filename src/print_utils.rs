pub mod print_utils {

    //Drukuje na ekran macierz
    pub fn print_matrix(matrix: &Vec<Vec<i32>>) {

        println!("Macierz grafu:");

        for i in 0..matrix.len() {

            println!("{:?}", matrix[i as usize]);
        }
    }
}