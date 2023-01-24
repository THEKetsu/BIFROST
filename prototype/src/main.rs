use actix_web::{web, App,HttpServer, ResponseError};
use serde::{Deserialize};
use askama::Template;
use mysql::*;
use mysql::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use actix_web::HttpResponse;
use std::fmt::Debug;
use std::fmt;

use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use argon2::{self, Config};





#[derive(Debug)]
pub struct MyError(String); // <-- needs debug and display


//structure d'un password 

#[derive(Debug, PartialEq, Eq)]
struct Password {
    sel_1: String,
    sel_2: String,
    login: String,
    passw: String,
}

//fonction de hash
impl Hash for Password {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.passw.hash(state);
    }
}



fn passwordhash(a:String ,b: String) -> String {
    let password = b.as_bytes();
    let salt = a.as_bytes();
    let config = Config::default();
    let encoded_hash = argon2::hash_encoded(password, salt, &config).unwrap();
    let hash = encoded_hash.as_str();
    println!("{}", hash);
    return hash.to_string();
    }
    



fn salt() -> String {
let mut rng = thread_rng();
let _x: u32 = rng.gen();


let s: String = (&mut rng).sample_iter(Alphanumeric)
    .take(15)
    .map(char::from)
    .collect();
println!("{}", s);
return s;
}



fn chiffrement (entree: String) -> String {

    let mut array: [u8; 16] = [0;16];
    let mut array2: [u8; 16] = [0;16];
    let mut array3: [u8; 16] = [0;16];
    let mut array4: [u8; 16] = [0;16];
    let mut array5: [u8; 16] = [0;16];
    let mut array6: [u8; 16] = [0;16];
    let mut i =0;
    let mut crypt="".to_owned();
    //Remplissage du tableau à chiffrer
    for c in entree.chars() {
        println!("{}", c);
        let my_string = c as u8;
        println!("{} valeur en u8", my_string);
        if i < 16{
            array[i]=my_string;
        }
        if i > 15 && i <32 {
            array2[i-16]=my_string;
        }
        if i > 31 && i <48 {
            array3[i-32]=my_string;
        }
        if i >47 && i<64{
            array4[i-48]=my_string;
        }
        if i > 63 && i <80{
            array5[i-64]=my_string;
        }
        if i > 79 {
            array6[i - 80]=my_string;
        }
        i+=1;
    }
    println!("{}+{}+{}", array[0],array[1],array[2]);
    println!("{}{}{}", array2[0],array2[1],array2[2]);
    

    // Définition de la clé et des blocks
    let key = GenericArray::from([10,3,5,1,3,5,1,3,5,1,3,5,1,3,5,6]);
    let mut block = GenericArray::from(array);
    let mut block2=GenericArray::from(array2);
    let mut block3=GenericArray::from(array3);
    let mut block4=GenericArray::from(array4);
    let mut block5=GenericArray::from(array5);
    let mut block6=GenericArray::from(array6);



    println!("Clé AES: {:?}",key);
    println!("Block clair: {:?}",block);
    println!("Block clair 2: {:?}",block2);
    println!("Block clair 3: {:?}",block3);
    println!("Block clair 4: {:?}",block4);
    println!("Block clair 5: {:?}",block5);
    println!("Block clair 6: {:?}",block6);
    //Composants pour chiffrer 

    let cipher = Aes128::new(&key);

    let block_copy = block.clone();
    let block_copy2 = block2.clone();
    let block_copy3   = block3.clone();
    let block_copy4 = block4.clone();
    let block_copy5 = block5.clone();
    let block_copy6 = block6.clone();


    cipher.encrypt_block(&mut block);
    cipher.encrypt_block(&mut block2);
    cipher.encrypt_block(&mut block3);
    cipher.encrypt_block(&mut block4);
    cipher.encrypt_block(&mut block5);
    cipher.encrypt_block(&mut block6);


    println!("Block 0: {:?}, Block 1 {}, Block_Copy {}", block[0], block[1], block[2]);
    println!("Block2 0: {:?}, Block2 1 {}, Block_Copy2 {}", block2[0], block2[1], block_copy2[0]);

    for a in block{
        let texte= a.to_string();
        crypt=crypt + &texte;
    }
    for b in block2{
        let texte= b.to_string();
        crypt=crypt + &texte;
    }
    for z in block3{
        let texte= z.to_string();
        crypt=crypt + &texte;
    }
    for e in block4{
        let texte= e.to_string();
        crypt=crypt + &texte;
    }
    for r in block5{
        let texte= r.to_string();
        crypt=crypt + &texte;
    }
    for t in block6{
        let texte= t.to_string();
        crypt=crypt + &texte;
    }

    println!("Texte final: {}", crypt);
    return crypt;
}
/////////////////////////////////////////////////////////////////////////////



impl fmt::Display for MyError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let c = formatter.fill();
        if let Some(width) = formatter.width() {
            for _ in 0..width {
                write!(formatter, "{c}")?;
            }
            Ok(())
        } else {
            write!(formatter, "{c}")
        }
    }
}
 

impl ResponseError for MyError {} // <-- key // je crée l'instance erreur avec la macro du dessus


#[derive(Debug, Template)] //déclaration de la 1ére template html (le formulaire)
#[template(path = "index.html")]
struct Index {}

#[derive(Debug, Template)] // déclaration de la 2éme template html (affichage de la variable)
#[template(path = "show.html")]
struct Show {
    thing_to_show:String,
    thing_to_show2:String,
    thing_to_show3:String,
}



#[derive(Debug, Deserialize)] // pour adapter la donnée
struct FormData {
    thing_to_show:String,
    thing_to_show2:String,
    thing_to_show3:String,
}




impl std::fmt::Display for FormData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FormData")
         .field("thing_to_show", &self.thing_to_show)
         .finish()
    }
}

async fn index() ->std::result::Result<HttpResponse, Box<dyn std::error::Error>> { //fonction pour afficher le premier rendu html
    let html = Index{}.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}


/*
async fn showthis(form_data: web::Form<FormData>) -> String  { //fonction pour afficher le 2éme rendu html
    let html = Show{ thing_to_show: form_data.thing_to_show.to_string() }.render().unwrap();
    println!("{}",html);
    let y= salt();
    println!("{}",y);
    let concat = html + &y.to_string();
    println!("{}",concat);
    let x = passwordhash(concat);
    println!("{}",x);
    return x;


}

*/

/*async fn cequetuveux() -> String{
    let formdata = FormData { thing_to_show: String::new() };
    let data = showthis(actix_web::web::Form(formdata)).await.to_string();
    //let up = data;
    println!("{}", data);
    let y= salt();
    let concat = data + &y.to_string();
    let x = passwordhash(concat);
    return x;
}

*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/showthis", web::post().to(bdd_create))
            
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
    
}



async fn bdd_create(form_data: web::Form<FormData>) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>>{
    let url = "mysql://aimasu:BIFROST@localhost:3306/TEST";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    
    let hashlolo = form_data.thing_to_show.to_string();    
    let log = form_data.thing_to_show2.to_string();    
    let sellolo= form_data.thing_to_show3.to_string();
    let y= salt();
    let sel= y.clone();
    let concat = hashlolo + &y.to_string();
    let x = passwordhash(y, concat);
    let aes = chiffrement(x);
    
    println!("{}", aes);
  	
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS password (
            sel_1 text not null,
            sel_2 text not null,
            login text not null,
            password text not null
        )")?;
    let _passwords = vec![
        Password { sel_1: sellolo  , sel_2:sel , login: log , passw: aes },
    ];


    conn.exec_batch(
        r"INSERT INTO password (sel_1, sel_2, login, password)
          VALUES (:sel_1, :sel_2, :login, :password)",
        _passwords.iter().map(|p| params! {
            "sel_1" => &p.sel_1,
            "sel_2" => &p.sel_2,
            "login" => &p.login,
            "password" => &p.passw,
        })
    )?;
Ok(HttpResponse::Ok().content_type("text/html").body("super"))
 
 }
 



async fn bdd_research(form_data: web::Form<FormData>)->std::result::Result<HttpResponse, Box<dyn std::error::Error>>{

let url = "mysql://aimasu:BIFROST@localhost:3306/TEST";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let hashlolo = form_data.thing_to_show.to_string();    
    let log = form_data.thing_to_show2.to_string();    
    let sellolo= form_data.thing_to_show3.to_string();
    let y= salt();
    let sel= y.clone();
    let concat = hashlolo + &y.to_string();
    let x = passwordhash(y, concat);
    let aes = chiffrement(x);
    
     let _passwords = vec![
        Password {sel_1: sellolo  , sel_2:sel , login: log , passw: aes},
    ];
    
    
let selected_passwords = conn
    	.query_map(
            "SELECT * FROM password where login='thomas' ",
            |(sel_1, sel_2, login, passw)| {
                Password { sel_1, sel_2, login, passw }
            },
        )?;

let mut log_extrait = "".to_string();

for burnout in selected_passwords {
log_extrait.push_str(&burnout.sel_1);

}


println!("{}", log_extrait);


//assert_eq!(selected_passwords, _passwords);

		
	
Ok(HttpResponse::Ok().content_type("text/html").body("super"))
 
}	

