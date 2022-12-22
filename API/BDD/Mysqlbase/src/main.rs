use mysql::*;
use mysql::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[derive(Debug, PartialEq, Eq)]


//structure d'un password 
struct Password {
    id: i32,
    login: String,
    passw: String,
}

//fonction de hash
impl Hash for Password {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.passw.hash(state);
    }
}



fn PASSWORDHASH(b: String) -> String {
	let person1 = b;
assert_eq!(calculate_hash(&person1),calculate_hash(&person1));

fn calculate_hash<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()

}
println!("{}", calculate_hash(&person1));
return calculate_hash(&person1)
}



fn sel() -> String {
let mut rng = thread_rng();
let x: u32 = rng.gen();


let s: String = (&mut rng).sample_iter(Alphanumeric)
    .take(15)
    .map(char::from)
    .collect();
println!("{}", s);
return s;
}




fn main ()-> std::result::Result<(), Box<dyn std::error::Error>>{
     let url = "mysql://GAGA:mypass@localhost:3306/passwd";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    
    let y= sel();
    let concat= "4".to_string() + &y;
    let x = PASSWORDHASH(concat);
    
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS password (
            id int not null,
            login text not null,
            password text not null
        )")?;
    let passwords = vec![
        Password { id: 1, login:"gaëlle".to_string(), passw: x },
        Password { id: 2, login:"Thomas".to_string(), passw: "0000".to_string() },
        Password { id: 3, login:"corentin".to_string(), passw: "5165".to_string() },
    ];


    conn.exec_batch(
        r"INSERT INTO password (id, login, password)
          VALUES (:id, :login, :password)",
        passwords.iter().map(|p| params! {
            "id" => p.id,
            "login" => &p.login,
            "password" => &p.passw,
        })
    )?;


    let selected_passwords = conn
        .query_map(
            "SELECT * from password ",
            |(id, login, passw)| {
                Password { id, login, passw }
            },
        )?;

    assert_eq!(passwords, selected_passwords);
    println!("Yay!");

    Ok(())  
 }
