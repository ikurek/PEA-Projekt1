extern crate num_traits;
extern crate time;

pub mod dynamic_programing_solution {
    use dynamic_programing_solution::time::PreciseTime;
    use dynamic_programing_solution::num_traits::pow;
    use print_utils::print_utils as PrintUtlisModule;

    //Rozwiązuje problem komiwojażera przy pomocy algorytmu programowania dynamicznego
    pub fn solve(matrix: &Vec<Vec<i32>>) {
        println!("Liczenie...");

        //Inicjalizacja zmiennej czasowej
        let timer_start = PreciseTime::now();
        //Wierzchołek początkowy
        let start: i32 = 0;
        //Liczba miast
        let number_of_cities = matrix.len();
        //Odpowiednia potęga liczby 2, reprezentująca ilość podproblemów
        let power: i32 = pow(2i32, number_of_cities);
        //Zmienna przechowująca wartość najkrótszej ścieżki
        let result: i32;
        //Tablica przechowująca wszystkie kolejne wierzchołki w najkrótszej ścieżce
        //Na początku zawiera jedynie wierzchołek startowy
        let mut result_path: Vec<i32> = Vec::new();
        result_path.push(start);
        //Macierze przechowujące wygenerowane ścieżki
        let mut connections: Vec<Vec<i32>> = Vec::new();
        let mut paths: Vec<Vec<i32>> = Vec::new();

        //Zainicjowanie stworzonych macierzy wartością -1
        for _i in 0..number_of_cities {

            let mut connections_row: Vec<i32> = Vec::new();
            let mut paths_row: Vec<i32> = Vec::new();

            for _j in 0..power {
                connections_row.push(-1);
                paths_row.push(-1);
            }

            connections.push(connections_row);
            paths.push(paths_row);
        }

        //Krok pierwszy algorytmu Helda-karpa
        //Inicjalizacja połączeń zestawem danych
        for i in 0..number_of_cities {
            connections[i][0] = matrix[i][0];
        }

        //Wylicza wynik, wywołując kolejne rekurencje algorytmu Helda-Karpa
        result = recurring_tsp_step(
            start,
            power - 2,
            &connections,
            &mut paths,
            number_of_cities,
            &power,
            &matrix,
        );
        let timer_stop = PreciseTime::now();
        let duration_in_ns = timer_start.to(timer_stop).num_nanoseconds().unwrap();
        //Pobiera kolejne wierchołki najkrótszej scieżki
        result_path = get_path(&mut paths, &mut result_path, &power, start, power - 2);
        //Dodaje do tablicy ostatni wierchołek, jako wierzchołek końcowy
        result_path.push(start);
        //Drukuje informacje o wyniku
        println!("Zakończono!");
        PrintUtlisModule::print_result(result, result_path, duration_in_ns);

    }

    fn recurring_tsp_step(
        start: i32,
        set: i32,
        connections: &Vec<Vec<i32>>,
        mut paths: &mut Vec<Vec<i32>>,
        number_of_cities: usize,
        power: &i32,
        matrix: &Vec<Vec<i32>>,
    ) -> i32 {

        //Zmienne używane w pojedynczym przebiegu rekurencji
        //Maska
        let mut mask: i32;
        //wierchołem maskowany
        let mut masked: i32;
        //ostatni znaleziony wynik
        let mut result: i32 = -1;
        //Zmienna pomocnicza
        let mut temp: i32;

        //Sprawdzenie, czy badane połączenie jest już sprawdzone
        if connections[start as usize][set as usize] != -1 {
            return connections[start as usize][set as usize];
        } else {
            //jeżeli połączenie nie istnieje, należy je sprawdzić
            for i in 0..number_of_cities {
                //aktualna maska to maska maksymalna, pomniejszona o 1 i odpowiednią potegę liczby 2
                mask = power - 1 - pow(2, i);
                //Iloczyn bitowy, sprawdzający wzystkie opcje ze zbioru połączenia
                masked = set & mask;
                //Jeżeli dana opcja istnieje w zbiorze połączeń
                if masked != set {
                    //Wykonaj kolejną rekurencję, uwzględniając wartość poprzedniej ścieżki
                    temp = matrix[start as usize][i as usize] +
                        recurring_tsp_step(
                            i as i32,
                            masked,
                            &connections,
                            &mut paths,
                            number_of_cities,
                            power,
                            matrix,
                        );
                    //Jeżeli kolejny uzyskany wynik jest mniejszy od poprzedniego, oznac go jako minimalny
                    if result == -1 || result > temp {
                        result = temp;
                        paths[start as usize][set as usize] = i as i32;
                    }
                }
            }
            //Zapisz wynik dla danego połączenia w tablicy
            connections[start as usize][set as usize] == result;

            return result;
        }
    }

    fn get_path(mut paths: &Vec<Vec<i32>>, mut result_path: &mut Vec<i32>, power: &i32, start: i32, set: i32) -> Vec<i32> {

        //Powtarza kroki algorytmu, zapisując do tablicy jedynie numery kolejnych wierzchołków
        let current_node = paths[start as usize][set as usize];
        if current_node == -1 {
            return result_path.to_vec();
        } else {
            let mask = power - 1 - pow(2, current_node as usize);
            let masked = set & mask;

            result_path.push(current_node);
            get_path(&mut paths, &mut result_path, &power, current_node, masked);
        }

        return result_path.to_vec();
    }
}
