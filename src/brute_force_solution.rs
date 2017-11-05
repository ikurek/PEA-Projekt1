extern crate permutohedron;
extern crate time;

pub mod brute_force_solution {
    use brute_force_solution::permutohedron::LexicalPermutation;
    use brute_force_solution::time::PreciseTime;
    use print_utils::print_utils as PrintUtlisModule;
    use graph_generator::graph_generator as GraphGeneratorModule;

    pub fn solve_multiple(number_of_nodes: i32, number_of_tests: i32) {
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        let mut times: Vec<i64> = Vec::new();
        let mut average: i64 = 0;

        for i in 0..number_of_tests {
            println!("Test {}", i);
            matrix = GraphGeneratorModule::generate_random_graph(number_of_nodes);
            times.push(solve(&matrix, false));
        }

        for x in &times {
            average = average + x;
        }

        average = average / (times.len() as i64);
        println!("Średni czas: {}ns", average);
    }

    //Rozwiązuje problem komiwojażera przy pomocy algorytmu przeglądu zupełnego
    pub fn solve(matrix: &Vec<Vec<i32>>, print_info: bool) -> i64 {

        //Inicjalizacja zmiennej do pomiaru czasu
        let timer_start = PreciseTime::now();
        //Tablica zawierająca listę przystkich wierzchołków
        let mut nodes: Vec<i32> = (0..matrix.len() as i32).collect();
        //Tablica zawierająca wszystkie możliwe permutacje wierzchołków
        let mut permutations = Vec::new();
        //Zmienna przechowująca permutację z najlepszą ścieżką
        let mut best_road = Vec::new();
        //Zmienna przechowująca wartość najkrótszej ścieżki
        let mut best_road_value: i32 = <i32>::max_value();

        //Generowanie permutacji do tablicy
        if print_info {
            println!("Generowanie permutacji...");
        }
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

        if print_info {
            println!("Całkowita liczba permutacji: {}", permutations.len());
            println!("Liczenie...");
        }

        //Sprawdzenie każdej permutacji
        for permutation in permutations {

            let mut road_value: i32 = 0;
            let mut index: i32 = 0;

            while (index + 1) < permutation.len() as i32 {
                road_value = road_value + matrix[permutation[index as usize] as usize][permutation[(index + 1) as usize] as usize];
                index = index + 1;
            }

            if road_value < best_road_value {
                best_road_value = road_value;
                best_road = permutation.to_vec();
            }
        }

        let timer_stop = PreciseTime::now();
        let duration_in_ns = timer_start.to(timer_stop).num_nanoseconds().unwrap();
        if print_info {
            println!("Zakończono!");
            PrintUtlisModule::print_result(best_road_value, best_road, duration_in_ns);
        }
        return duration_in_ns;
    }

}
