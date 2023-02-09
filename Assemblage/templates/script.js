	function Securisation() {
	var str = document.getElementById("password").value;
	console.log(str.length);
	console.log(str);
	//checkPassword();
	var specialChar = /[ !@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]/;
	if (document.getElementById('login').value == ""){
		alert("Veuillez rentrer votre nom d'utilisateur");
	}
	else if (document.getElementById('password').value == ""){
		alert("Veuillez rentrer un mot de passe");
    }
	else if (document.getElementById('password').value != document.getElementById('password2').value){
		alert("Veuillez rentrer le même mot de passe");
    }
	else if (document.getElementById("password").value.length < 8 ){
		alert("Veuillez rentrer un mot de passe de plus de 8 caractères");
	}

	else if (str === str.toLowerCase()) {
	   alert("Veuillez rentrer une majuscule");
	}
	else if (!specialChar.test(str)){
		alert("Veuillez rentrer un caractère spécial");
	}
	else {
			console.log("Mot de passe sécurisé");
			Check();
			document.getElementById("signup").style.display = "inline-block";
			document.getElementById("test").style.display = "none";
			document.getElementById("login").style.display = "none";
			document.getElementById("password").style.display = "none";
			document.getElementById("password2").style.display = "none";
	}
	
	}
	
	function cacherBouton() {
    document.getElementById("bouton2").style.display = "none";
  }

  window.onload = function() {
    cacherBouton();
    document.getElementById("signup").addEventListener("click", afficherBouton);
  }
	
	function sel(){
		let sel = makeid(10)
		console.log(sel);
		console.log(document.getElementById('password').value);
		let mdphash = document.getElementById('password').value+sel;
		console.log(mdphash);
		let hash = sha256(mdphash)
		console.log(hash);
		alert(hash);
		var inputElement = document.getElementById("coco");
		inputElement.value = hash;
		var inputElement = document.getElementById("coco2");
		inputElement.value = document.getElementById('login').value;
		 var inputElement = document.getElementById("coco3");
		inputElement.value = sel;
		//window.location.href="menu.html"
	}
	

	function makeid(length) {
		var result           = '';
		var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
		var charactersLength = characters.length;
		for ( var i = 0; i < length; i++ ) {
			result += characters.charAt(Math.floor(Math.random() * charactersLength));
		}
		return result;
	}

	function Check()
	{
			var phrase = document.getElementById("password").value;
			console.log(phrase);
			//var nbMaj = phrase.match(/[A-Z]/g).length;
			//alert(nbMaj);

	if (phrase.match( /[^a-zA-Z\d]/g)){
	  console.log('Contient des caractères spéciaux');
	  sel();
	} else {
	  alert("Veuillez rentrer un caractère spécial");
	}
	}
	