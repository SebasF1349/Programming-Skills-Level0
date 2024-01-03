use std::io::stdin;

struct User {
    name: String,
    password: String,
    balance: u64,
}

fn main() {
    let mut user = User {
        name: "Sebas".to_string(),
        password: "123456".to_string(),
        balance: 2000,
    };
    let mut wrong_credentials = 0;
    let mut input = String::new();
    println!("Ingrese su nombre de usuario: ");
    stdin().read_line(&mut input).expect("Error");
    if input.trim() != user.name {
        println!("Nombre de usuario incorrecto.");
        return;
    }
    if wrong_credentials == 3 {
        println!("Su cuenta se encuentra bloqueada. Acerquese al banco para blanquear su clave.");
        return;
    }
    while wrong_credentials < 3 {
        input = String::new();
        println!("Ingrese su contrasenia: ");
        stdin().read_line(&mut input).expect("Error");
        if input.trim() != user.password {
            println!("Contrasenia incorrecta.{}-{}-", input.trim(), user.password);
            wrong_credentials += 1;
            continue;
        }
        wrong_credentials = 0;
        break;
    }
    if wrong_credentials == 3 {
        println!("Su cuenta se encuentra bloqueada. Acerquese al banco para blanquear su clave.");
        return;
    }
    loop {
        input = String::new();
        println!("\nQue deseas hacer? Elije un numero.");
        println!("1. Depositar");
        println!("2. Retirar");
        println!("3. Ver Saldo");
        println!("4. Transferir");
        println!("9. Salir");
        stdin().read_line(&mut input).expect("Error");
        let option = input.trim().parse::<u8>().unwrap_or_else(|v| {
            println!("{v} no es un numero valido");
            0
        });
        input = String::new();
        match option {
            1 => {
                println!("Cuanto dinero desea depositar?");
                stdin().read_line(&mut input).expect("Error");
                let option = input.trim().parse::<u64>();
                match option {
                    Ok(m) => {
                        println!("Operacion realizada con exito.");
                        user.balance += m;
                    }
                    Err(m) => println!("Valor incorrecto: {m}"),
                }
            }
            2 => {
                println!("Cuanto dinero desea retirar?");
                stdin().read_line(&mut input).expect("Error");
                let option = input.trim().parse::<u64>();
                match option {
                    Ok(m) if m <= user.balance => {
                        println!("Operacion realizada con exito.");
                        user.balance -= m;
                    }
                    Ok(_) => println!("No puede retirar mas dinero del que se posee."),
                    Err(m) => println!("Valor incorrecto: {m}"),
                }
            }
            3 => println!("Su saldo es de ${}.", user.balance),
            4 => {
                println!("Cuanto dinero desea transferir?");
                stdin().read_line(&mut input).expect("Error");
                let option = input.trim().parse::<u64>();
                match option {
                    Ok(m) if m <= user.balance => {
                        println!("Operacion realizada con exito.");
                        user.balance -= m;
                    }
                    Ok(_) => println!("No puede transferir mas dinero del que se posee."),
                    Err(m) => println!("Valor incorrecto: {m}"),
                }
            }
            9 => break,
            v => println!("{v} no es un numero valido"),
        }
    }
}
