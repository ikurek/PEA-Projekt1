extern crate num_traits;

pub mod dynamic_programing_solution {
    use std::time::Instant;
    use dynamic_programing_solution::num_traits::pow;

    //Rozwiązuje problem komiwojażera przy pomocy algorytmu programowania dynamicznego
    pub fn solve(matrix : &Vec<Vec<i32>>) {

        //Inicjalizacja zmiennej czasowej
        let now = Instant::now();
        println!("Liczenie...");
        calculate_tsp(matrix, matrix.len());
        
    }

    fn calculate_tsp(matrix : &Vec<Vec<i32>>, number_of_cities : usize) {

        //Odpowiednia potęga liczby 2, reprezentująca ilość podproblemów
        let power : i32 = pow(2i32, number_of_cities);
        //Macierze przechowujące wygenerowane ścieżki
        let mut connections : Vec<Vec<i32>> = Vec::new();
        let mut paths : Vec<Vec<i32>> = Vec::new();

        //Zainicjowanie stworzonych macierzy wartością -1
        for i in 0..number_of_cities {

            let mut connections_row : Vec<i32> = Vec::new();
            let mut paths_row : Vec<i32> = Vec::new();

            for j in 0..power {
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

        let mut result = recurring_tsp_step(0, power - 2, &connections, &mut paths, number_of_cities, &power, &matrix);

        println!("result: {}", result);

    }

    fn recurring_tsp_step(start : i32,
                            set : i32,
                            connections : &Vec<Vec<i32>>,
                            mut paths : &mut Vec<Vec<i32>>,
                            number_of_cities : usize,
                            power : &i32,
                            matrix : &Vec<Vec<i32>>) -> i32 {

        //Zmienne używane w pojedynczym przebiegu rekurencji
        //Maska
        let mut mask : i32;
        //wierchołem maskowany
        let mut masked : i32;
        //ostatni znaleziony wynik
        let mut result : i32 = -1;
        //Zmienna pomocnicza
        let mut temp : i32;

        //Sprawdzenie, czy badane połączenie jest już sprawdzone
        if connections[start as usize][set as usize] != -1 {
            return connections[start as usize][set as usize]
        } else {
            //jeżeli połączeni nie istnieje, należy je sprawdzić
            for i in 0..number_of_cities {
                //aktualna maska to maska maksymalna, pomniejszona o 1 i odpowiednią potegę liczby 2
                mask = power - 1 - pow(2, i);
                //Iloczyn bitowy, sprawdzający wzystkie opcje ze zbioru połączenia
                masked = set & mask;
                //Jeżeli dana opcja istnieje w zbiorze połączeń
                //TODO: co się odpierdala?
                if masked != set {
                    temp = matrix[start as usize][i as usize] + recurring_tsp_step(i as i32, masked, &connections, &mut paths, number_of_cities, power, matrix);

                    if result == -1 || result > temp {
                        result = temp;
                        paths[start as usize][set as usize] = i as i32;
                    }
                }
            }

            connections[start as usize][set as usize] == result;

            return result;
        }
    }
}