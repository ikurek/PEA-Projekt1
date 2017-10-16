extern crate permutohedron;

pub mod brute_force_solution {

    use brute_force_solution::permutohedron::LexicalPermutation;

    pub fn solve(matrix: &Vec<Vec<i32>>) {

        //Tablica zawierająca listę przystkich wierzchołków
        let mut nodes = [0, 1, 2, 3];
        //Tablica zawierająca wszystkie możliwe permutacje wierzchołków
        let mut permutations = Vec::new();
        //Zmienna przechowująca permutację z najlepszą ścieżką
        let mut best_road = Vec::new();
        //Zmienna przechowująca wartość najkrótszej ścieżki
        let mut best_road_value: i32 = <i32>::max_value();

        //Generowanie permutacji do tablicy
        loop {
            permutations.push(nodes.to_vec());

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
    }

}