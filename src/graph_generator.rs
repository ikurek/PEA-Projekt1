pub mod graph_generator {
    extern crate rand;
    use self::rand::Rng;
    use self::rand::distributions::Range;
    use self::rand::distributions::IndependentSample;

    pub fn generate_random_graph(number_of_nodes : i32) -> Vec<Vec<i32>> {

        let mut matrix : Vec<Vec<i32>> = Vec::new();
        let rng_range = Range::new(1, 100);
        let mut rng = rand::thread_rng();

        //Iterowanie po wierszach macierzy
        for i in 0..number_of_nodes {

            //Zmienna tymczasowa przechowująca powstający w pętli poniżej wiersz macierzy
            let mut row : Vec<i32> = Vec::new();

            //Iterowanie po kolumnach macierzy
            for j in 0..number_of_nodes {
                if i == j {
                    row.push(-1);
                } else {
                    row.push(rng_range.ind_sample(&mut rng));
                }
            }

            matrix.push(row);
        }

        return matrix;
    }
}