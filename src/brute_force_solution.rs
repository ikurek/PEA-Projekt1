extern crate permutohedron;

pub mod brute_force_solution {
    use brute_force_solution::permutohedron::LexicalPermutation;
    use print_utils::print_utils as PrintUtlisModule;
    use std::time::Instant;

    //Rozwiązuje problem komiwojażera przy pomocy algorytmu przeglądu zupełnego
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
        println!("Generowanie permutacji...");
        loop {
            //Pobranie permutacji z tablicy nodes jako wektor
            let mut permutation = nodes.to_vec();
            //Ustawia pierwszy wierzchołek permutacji jako końcowy
            let starting_node = permutation[0];
            permutation.push(starting_node);
            //Dodaje permutację do zbioru permutacji
            permutations.push(permutation);

            //Pętla kończy się, gdy nie ma kolejnej permutacji
            if !nodes.next_permutation() {
                break;
            }
        }

        println!("Całkowita liczba permutacji: {}", permutations.len());
        println!("Liczenie...");


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
        }

        let elapsed_time = now.elapsed();
        println!("Zakończono!");
        PrintUtlisModule::print_result(best_road_value, best_road, elapsed_time.subsec_nanos() as i32);
    }

}
