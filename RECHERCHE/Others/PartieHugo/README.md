Cookie : 

Percent-encode ->  cookie :: encode() & Cookie :: parse_encoded() 
Encoded -> renvoie un wrapper. Un wrapper c’est un programme qui est appelé dans une autre fonction.  Ce wrapper affiche le nom et la valzeur du cookie.
Parse_encoded -> decode le nom et la valeur du cookie

Private -> CookieJar :: private()
Ça permet d’encrypter le cookie et d’authentifier le cookie and récupérant le parent jar

Secure -> fais private, signed, key-expansion

[dependencies.cookie]
features = ["secure", "percent-encode"]

Cookie
Representation of an HTTP cookie.
CookieBuilder
Structure that follows the builder pattern for building Cookie structs.
CookieJar
A collection of cookies that tracks its modifications.

Lien utile pour les cookies:

Firebase authenti RUST:
https://crates.io/crates/fireauth
https://docs.rs/firestore-db-and-auth/latest/firestore_db_and_auth/

Cookie en RUST:
https://docs.rs/cookie/latest/cookie/struct.Cookie.html
https://stackoverflow.com/questions/17769011/how-does-cookie-based-authentication-work
