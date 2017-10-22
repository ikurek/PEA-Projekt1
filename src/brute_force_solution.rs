extern crate permutohedron;

pub mod brute_force_solution {
    use brute_force_solution::permutohedron::LexicalPermutation;
    use std::time::Instant;

    pub fn solve(matrix: &Vec<Vec<i32>>) {

        //Inicjalizacja zmiennej do pomiaru czasu
        let now = Instant::now();
        //Tablica zawierająca listę przystkich wierzchołków
        let mut nodes : Vec<i32> = (0..matrix.len() as i32).collect();
        //Tablica zawierająca wszystkie możliwe permutacje wierzchołków
        let mut permutations = Vec::new();
        //Zmienna przechowująca permutację z najlepszą ścieżką
        let mut best_road = Vec::new();
        //Zmienna przechowująca wartość najkrótszej ścieżki
        let mut best_road_value: i32 = <i32>::max_value();

        //Generowanie permutacji do tablicy
        loop {
            let mut permutation = nodes.to_vec();
            let mut starting_node = permutation[0];
            //Ustawia powrót do wierzchołka początkowego
            permutation.push(starting_node);
            permutations.push(permutation);

            if !nodes.next_permutation() {
                break;
            }
        }

        println!("Całkowita liczba permutacji: {}", permutations.len());
        println!("");


        //Sprawdzenie każdej permutacji
        for permutation in permutations {

            let mut road_value: i32 = 0;
            let mut index: i32 = 0;

            while (index + 1) < permutation.len() as i32 {
                road_value = road_value +
                    matrix[permutation[index as usize] as usize][permutation[(index + 1) as usize] as usize];
                index = index + 1;
            }

            if road_value < best_road_value {
                best_road_value = road_value;
                best_road = permutation.to_vec();
            }

            println!("Permutacja: {:?}, Koszt: {}", permutation, road_value);

        }

        println!("Najlepsza trasa: {:?}, Koszt: {}", best_road, best_road_value);
        let elapsed_time = now.elapsed();
        println!("Czas wykonania algorytmu: {}ms", (elapsed_time.as_secs() * 1_000) + (elapsed_time.subsec_nanos() / 1_000_000) as u64);
    }

}