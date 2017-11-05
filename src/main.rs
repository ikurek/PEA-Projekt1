mod file_reader;
mod print_utils;
mod brute_force_solution;
mod dynamic_programing_solution;
mod graph_generator;

use std::io;
use file_reader::file_reader as FileReaderModule;
use print_utils::print_utils as PrintUtlisModule;
use brute_force_solution::brute_force_solution as BruteForceSolution;
use dynamic_programing_solution::dynamic_programing_solution as DynamicProgramingSolution;
use graph_generator::graph_generator as GraphGenerator;


fn main() {

    println!("");
    println!("Projektowanie Efektywnych Algorytmów - Zadanie 1");
    println!("Problem komiwojażera (TSP)");
    println!("Igor Kurek, 226004");

    //Zmienna przechowująca graf w postaci macierzowej
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    loop {

        let mut selected_number = String::new();

        println!("");
        println!("Wybierz funkcję:");
        println!("1. Wczytaj plik z grafem");
        println!("2. Wygeneruj losowy graf");
        println!("3. Programowanie dynamiczne");
        println!("4. Przegląd zupełny");
        println!("5. Programowanie dynamiczne (Testy uśrednione)");
        println!("6. Przegląd zupełny (Testy uśrednione)");
        println!("0. Wyjście");
        println!("Wybór: ");

        //Wczytaj odpowiedź użytkownika
        io::stdin().read_line(&mut selected_number).expect(
            "Błąd wejścia/wyjścia",
        );

        //Zmień typ odpowiedzi na integer
        let selected_number: u32 = selected_number.trim().parse().expect("Błędna wartość");

        //Podejmij akcję w zalezności od wyboru użytkownika
        match selected_number {
            0 => std::process::exit(0x0),
            1 => {
                let mut file_name = String::new();
                println!("Nazwa pliku: ");
                io::stdin().read_line(&mut file_name).expect(
                    "Błąd wejścia/wyjścia",
                );
                matrix = FileReaderModule::read_file(String::from(file_name.trim()));
                //matrix = FileReaderModule::read_file(String::from("test_file.txt"));
                PrintUtlisModule::print_matrix(&matrix);
            }
            2 => {
                let mut number_of_nodes: String = String::new();
                println!("Ilość wierzchołków:");
                io::stdin().read_line(&mut number_of_nodes).expect(
                    "Błąd wejścia/wyjścia",
                );

                let number_of_nodes: i32 = number_of_nodes.trim().parse().expect("Błędna wartość");

                matrix = GraphGenerator::generate_random_graph(number_of_nodes);
                PrintUtlisModule::print_matrix(&matrix);

            }
            3 => {
                if matrix.is_empty() {
                    println!("Najpierw wczytaj graf z pliku!");
                } else {
                    DynamicProgramingSolution::solve(&matrix, true);
                }
            }
            4 => {
                if matrix.is_empty() {
                    println!("Najpierw wczytaj graf z pliku!");
                } else {
                    BruteForceSolution::solve(&matrix, true);
                }
            }
            5 => {
                let mut number_of_nodes: String = String::new();
                println!("Ilość wierzchołków:");
                io::stdin().read_line(&mut number_of_nodes).expect(
                    "Błąd wejścia/wyjścia",
                );
                let number_of_nodes: i32 = number_of_nodes.trim().parse().expect("Błędna wartość");

                let mut number_of_tests: String = String::new();
                println!("Ilość testów:");
                io::stdin().read_line(&mut number_of_tests).expect(
                    "Błąd wejścia/wyjścia",
                );
                let number_of_tests: i32 = number_of_tests.trim().parse().expect("Błędna wartość");

                DynamicProgramingSolution::solve_multiple(number_of_nodes, number_of_tests);

            }
            6 => {
                let mut number_of_nodes: String = String::new();
                println!("Ilość wierzchołków:");
                io::stdin().read_line(&mut number_of_nodes).expect(
                    "Błąd wejścia/wyjścia",
                );
                let number_of_nodes: i32 = number_of_nodes.trim().parse().expect("Błędna wartość");

                let mut number_of_tests: String = String::new();
                println!("Ilość testów:");
                io::stdin().read_line(&mut number_of_tests).expect(
                    "Błąd wejścia/wyjścia",
                );
                let number_of_tests: i32 = number_of_tests.trim().parse().expect("Błędna wartość");

                BruteForceSolution::solve_multiple(number_of_nodes, number_of_tests);

            }
            _ => println!("Niepoprawna wartość!"),
        };
    }
}
