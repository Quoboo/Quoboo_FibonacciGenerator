use std::io;

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 0..n-1 {
        let temp = a;
        a = b;
        b = temp + b;
    }

    b
}

fn main() {
    println!("Gib eine Zahl ein, um die entsprechende Fibonacci-Zahl zu berechnen:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fehler beim Einlesen der Zeile");
    let n: u32 = input.trim().parse().expect("Ungültige Eingabe");

    let result = fibonacci(n);
    println!("Die {}-te Fibonacci-Zahl ist: {}", n, result);
}
