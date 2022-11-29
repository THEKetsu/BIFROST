use std::io;

fn main() {
     println!("voici le menu");
     println!("1) Afficher coucou");
     println!("2) Afficher thomas");
     println!("3) Afficher Jai taff tas vu ?");
     println!("4) Exit");
     
    
     

     loop{ 

        let mut input = String::new();
        io::stdin()
        .read_line(&mut input).expect("error: unable to read user input");

        let input: u32 = input.trim().parse().expect("Veuillez entrer un nombre !");

   
        match input{
        1=>println!("coucou"),
        2 =>println!("thomas"),
        3 =>println!("Jai taff tas vu ?"),
        4 =>println!("Exit"),
         _ =>println!("Choix non valide"),
        }

        if input == 4{
            break;
        }
    }

}
