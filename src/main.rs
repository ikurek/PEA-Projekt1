mod file_reader;
mod print_utils;

use std::io;
use std::collections;
use file_reader::file_reader as FileReaderModule;
use print_utils::print_utils as PrintUtlisModule;



fn main() {

    println!("Projektowanie Efektywnych Algorytmów - Zadanie 1");
    println!("Problem komiwojażera (TSP)");
    println!("Igor Kurek, 226004");

    let mut matrix: Vec<Vec<i32>> = Vec::new();

    loop {

        let mut selected_number = String::new();

        println!("");
        println!("Wybierz funkcję:");
        println!("1. Wczytaj plik z grafem");
        println!("2. Programowanie Dynamiczne");
        println!("3. Przegląd zupełny");
        println!("0. Wyjście");
        println!("Wybór: ");

        //Wczytaj odpowiedź użytkownika
        io::stdin().read_line(&mut selected_number).expect(
            "Błąd wejścia/wyjścia",
        );

        //Zmień typ odpowiedzi na integer
        let selected_number: u32 = selected_number.trim().parse().expect("Błędna wartość");

        match selected_number {
            0 => exit_selected(),
            1 => {
                matrix = FileReaderModule::read_file(String::from("test_file.txt"));
                PrintUtlisModule::print_matrix(matrix);
            }
            2 => dynamic_programing_selected(),
            3 => brute_force_selected(),
            _ => value_error(),
        };
    }
}


fn dynamic_programing_selected() {
    unimplemented!();
}

fn brute_force_selected() {
    unimplemented!();
}

fn value_error() {
    println!("Niepoprawna wartość!");
}

fn exit_selected() {
    std::process::exit(0x0);
}