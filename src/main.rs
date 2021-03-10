use std::io;

fn main() {

    colour::dark_cyan_ln!("Would you like to convert from | (1) °F to °C | or | (2) °C to °F | ?");

    //Pega o input do teclado e garante que será um número
    let scale: u8 = {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("It wasn't possible to access stdin");
        buffer.trim().parse().expect("Please, just numbers!")
    };
    
    if scale == 1 {
        f_to_c();
    } else {
        c_to_f();
    }

    println!("Type anything to exit...");
        let mut buffer = String::new(); 
        let _= io::stdin()
            .read_line(&mut buffer);
}

fn f_to_c() {
    colour::dark_cyan_ln!("Type your value in °F:");

    //Pega o input do teclado e garante que será um número
    let f: f32 = {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("It wasn't possible to access stdin");
        buffer.trim().parse().expect("Please, just numbers!")
    };

    // Faz o cálculo para transformar °F em °C
    let result: f32 = (f - 32.0) * 5.0/9.0;

    // Mostra o resultado
    colour::dark_red!("\n{}°F ", f);
    colour::dark_cyan!("= ");
    colour::dark_red_ln!("{:.2}°C", result);
    colour::magenta_ln!("formula utilizada: (°F - 32) * 5/9\n");
}

fn c_to_f() {
    colour::dark_cyan_ln!("Type your value in °C:");

    // Pega o input do teclado e garante que será um número
    let c: f32 = {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("It wasn't possible to access stdin");
        buffer.trim().parse().expect("Please, just numbers!")
    };

    // Faz o cálculo para transformar °C em °F
    let result: f32 = (c * 9.0/5.0) + 32.0;

    //Mostra o resultado
    colour::dark_red!("\n{}°C ", c);
    colour::dark_cyan!("= ");
    colour::dark_red!("{:.2}°F\n", result);
    colour::magenta_ln!("formula utilizada: (°c - * 9/5) + 32\n");
}

