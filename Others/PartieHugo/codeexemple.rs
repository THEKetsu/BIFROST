use std::io;

fn main() {
    println!("voici le menu");
    println!("1) Afficher coucou");
    println!("2) Afficher thomas");
    println!("3) Afficher Jai taff tas vu ?");
    println!("4) Exit");
    println!("Que voulez vous faire ?");

    let mut choix = String::new();
   
    loop{
        io::stdin()
        .read_line(&mut choix)
        .expect("Error");

     match choix.as_str(){
        "1"=>println!("coucou"),
        "2"=>println!("thomas"),
        "3"=>println!("Jai taff tas vu ?"),
        _=>println!("Entrer non valide"),
        }
    if choix=="4"{
        break;
    }
    }
}
