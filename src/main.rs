use std::io::{stdin};
use std::cmp::{Ordering};

fn main() {
    let mut end = false;
    while !end {
        println!("Adivina que número tengo en mente");
        let mut gs = String::new();
        let n:u32 = 42;
        match stdin().read_line(&mut gs){
            Ok(_) => {
                let gu:u32 = gs.trim().parse().expect("la entrada anterior no es un número");
                let msg = match n.cmp(&gu){
                    Ordering::Equal => {end = true; "Lo tienes"},
                    Ordering::Less => "Mi número es menor",
                    Ordering::Greater => "Mi número es mayor",
                };
                println!("{}",msg);
            },
            Err(_) => {end = true; println!("error leyendo")},
        }
    }  
}
