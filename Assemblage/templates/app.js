const form = document.querySelector('form');
const liste = document.querySelector('ul');
const input = document.querySelector('form input');

let toutesLesTaches = [];

form.addEventListener('submit', event => {
    event.preventDefault();

    const text = input.value.trim();
    if(text !== ''){
        // Si l'input contient du texte, on appelle la méthode rajouterUneTache
        rajouterUneTache(text);
        input.value = '';
    }
})

function rajouterUneTache(text) {
    const todo = {
        text,
        // Date.now renvoie le nb de millisecondes écoulées depuis le 1er janv 1970
        id: Date.now()
    }

    afficherListe(todo);
}

function afficherListe(todo){

    // Création du li pour le remplir avec un input (type checkbox), un span et un button qui aura son image (la croix)
    const item = document.createElement('li');
    item.setAttribute('data-key', todo.id);

    const input = document.createElement('input');
    input.setAttribute('type', 'checkbox');
    input.addEventListener('click', tacheFaite);
    item.appendChild(input);

    const txt = document.createElement('span');
    txt.innerText = todo.text;
    item.appendChild(txt);

    const btn = document.createElement('button');
    btn.addEventListener('click', supprimerTache);
    const img = document.createElement('img');
    img.setAttribute('src', 'ressources/fermer.svg');
    btn.appendChild(img);
    item.appendChild(btn);

    // On rattache li (item) au lu (liste)
    liste.appendChild(item);
    // On rajoute au tableau toutesLesTaches l'item qu'on vient de créer
    toutesLesTaches.push(item);
}

function tacheFaite(e) {
    // Si on clique sur l'input checkbox, le parent (li) aura la classe 'fin de tache'
    e.target.parentNode.classList.toggle('findeTache')
}

function supprimerTache(e) {

    //Pour chaque element du tableau
    toutesLesTaches.forEach(el => {

        // Si l'attribut du li data-key est le même que l'elèment de notre tableau
        // e.target est le bouton mais c'est le parent li qui a l'id
        if(e.target.parentNode.getAttribute('data-key') === el.getAttribute('data-key')){
            // On supprime cet élèment du DOM
            el.remove();
            // On supprime l'élèment du tableau
            // On va garder tous les li qui ont un ID strictement différent de l'élement li que je viens de retirer, on garde tous les autres, retourne un nouveau tableau
            toutesLesTaches = toutesLesTaches.filter(li => li.dataset.key !== el.dataset.key);
        }
    })
}