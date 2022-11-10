use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct GroupeTaches { // crée la structure GroupeTaches
    operateurs: Vec<Operateur>,
    envoi: mpsc::Sender<Mission>,
}

type Mission = Box<dyn FnOnce() + Send + 'static>;

impl GroupeTaches {
    pub fn new(taille: usize) -> GroupeTaches {	//valide le nombre de tâches
        assert!(taille > 0);

        let (envoi, reception) = mpsc::channel();

        let reception = Arc::new(Mutex::new(reception));

        let mut operateurs = Vec::with_capacity(taille);	//crée l'espace de rangement 

        for id in 0..taille {
            operateurs.push(Operateur::new(id, Arc::clone(&reception))); // génère un id pour chaque opérateur et stock l'id
        }

        GroupeTaches { operateurs, envoi }
    }

    pub fn executer<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mission = Box::new(f);

        self.envoi.send(mission).unwrap();
    }
}
struct Operateur { //crée un opérateur 
    id: usize,
    tache: thread::JoinHandle<()>,
}

impl Operateur { // implémente l'opérateur et affiche dans la console
    fn new(id: usize, reception: Arc<Mutex<mpsc::Receiver<Mission>>>) -> Operateur {
        let tache = thread::spawn(move || {
            while let Ok(mission) = reception.lock().unwrap().recv() {
                println!("L'opérateur {} a obtenu une mission ; il l'exécute.", id);

                mission();
            }
        });

        Operateur { id, tache }
    }
}
