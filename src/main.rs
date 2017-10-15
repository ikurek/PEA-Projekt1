use std::io;
use std::collections;


fn main() {

    println!("Projektowanie Efektywnych Algorytmów - Zadanie 1");
    println!("Problem komiwojażera (TSP)");
    println!("Igor Kurek, 226004");

    let mut vector_of_nodes : Vec<module::node::Node> = Vec::new();

    loop {

        let mut selected_number = String::new();

        println!("Wybierz funkcję:");
        println!("1. Wczytaj plik z grafem");
        println!("2. Programowanie Dynamiczne");
        println!("3. Przegląd zupełny");
        println!("0. Wyjście");
        println!("Wybór: ");

        //Wczytaj odpowiedź użytkownika
        io::stdin().read_line(&mut selected_number)
            .expect("Błąd wejścia/wyjścia");

        //Zmień typ odpowiedzi na integer
        let selected_number: u32 = selected_number.trim().parse()
            .expect("Błędna wartość");

        match selected_number {
            0 => exit(),
            1 => read_file(),
            2 => dynamic_programing(),
            3 => brute_force(),
            _ => value_error(),
        };
    }
}

fn read_file() {
    unimplemented!();
}

fn dynamic_programing() {
    unimplemented!();
}

fn brute_force() {
    unimplemented!();
}

fn value_error() {
    println!("Niepoprawna wartość!");
}

fn exit() {
    exit();
}