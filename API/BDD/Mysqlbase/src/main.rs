use mysql::*;
use mysql::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[derive(Debug, PartialEq, Eq)]
struct Password {
    login: String,
    passw: u64,
}


impl Hash for Password {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.login.hash(state);
        self.passw.hash(state);
    }
}



fn PASSWORDHASH() {
	let person1 = Password {
    	login: "Thomas".to_string(),
    	passw: 0000, 
	};
    
assert_eq!(calculate_hash(&person1),calculate_hash(&person1));

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()

}

println!("{}", calculate_hash(&person1))   

}
  

fn main ()-> std::result::Result<(), Box<dyn std::error::Error>>{
     let url = "mysql://GAGA:mypass@localhost:3306/passwd";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    PASSWORDHASH();

    // Let's create a table for payments.
    conn.query_drop(
        r"CREATE TABLE password (
            login text not null,
            password int not null
        )")?;

    let passwords = vec![
        Password { login:"Thomas".to_string(), passw: 0000 },
    ];

    // Now let's insert payments to the database
    conn.exec_batch(
        r"INSERT INTO password (login, password)
          VALUES (:login, :password)",
        passwords.iter().map(|p| params! {
            "login" => &p.login,
            "password" => p.passw,
        })
    )?;

    // Let's select payments from database. Type inference should do the trick here.
    let selected_passwords = conn
        .query_map(
            "SELECT login, password from password",
            |(login, passw)| {
                Password { login, passw }
            },
        )?;

    // Let's make sure, that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows
    // without `ORDER BY`, so assume we are lucky.
    assert_eq!(passwords, selected_passwords);
    println!("Yay!");

    Ok(())  
 }
