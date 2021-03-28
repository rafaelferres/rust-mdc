use std::io;

fn convert_int(s: &str) -> i32 {
    s.trim().parse::<i32>().unwrap()
}

fn main() {
    let mut number1 = String::new();
    let mut number2 = String::new();
    let mut resto = 1;

    io::stdin()
        .read_line(&mut number1)
        .expect("Erro ao ler number1");
    io::stdin()
        .read_line(&mut number2)
        .expect("Erro ao ler number2");

    let mut inumber1 = convert_int(&number1);
    let mut inumber2 = convert_int(&number2);

    while inumber2 != 0 {
        resto = inumber1 % inumber2;
        inumber1 = inumber2;
        inumber2 = resto;
    }

    println!("O mdc é {}", inumber1);
}
