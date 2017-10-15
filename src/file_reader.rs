pub mod file_reader {

    use std::io;
    use std::io::BufReader;
    use std::io::BufRead;
    use std::error::Error;
    use std::fs::File;
    use std::collections;

    //Odczytuje macierz z pliku
    pub fn read_file(file_name: String) -> Vec<Vec<i32>> {

        let mut matrix: Vec<Vec<i32>> = Vec::new();

        println!("Odczytywanie pliku {}...", &file_name);

        //Stworzenie zmiennej plikowej
        let mut file = match File::open(&file_name) {
            Err(error) => {
                panic!(
                    "At the Disco! Couldn't open {}: {}",
                    &file_name,
                    Error::description(&error)
                )
            }
            Ok(file) => file,
        };

        //Stworzenie bufora ze zmiennej plikowej
        let mut buffer = BufReader::new(&file);

        //Zmienna określająca ilość wczytanych linii
        //Potrzebna aby wygodnie numerować tablice od 0
        let mut isFirstLine: bool = true;

        //Iteracja po kolejnych liniach pliku
        for line in buffer.lines() {
            //Pomiń linie bez tekstu, uszkodzone itd.
            match line {
                Ok(line) => {
                    //Wyswietl liczbę miast z pierwszej linii
                    //Pozostałe linie parsuj do wektora i dodaj jako wiersz macierzy
                    if isFirstLine {
                        println!("Liczba miast: {}", line);
                        isFirstLine = false;
                    } else {
                        matrix.push(parse_file_line(line));
                    }
                }
                Err(error) => println!("Błąd pliku: {}", error),
            }
        }

        println!("Zakończono wczytywanie pliku.");

        return matrix;
    }

    //Parsuje string z pliku do postaci wektora liczb i32
    fn parse_file_line(line: String) -> Vec<i32> {

        return line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

    }

}