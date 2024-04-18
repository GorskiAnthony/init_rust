use std::io;

fn main() {
    // println! permet d'afficher un truc pour l'utilisateur
    println!("Devinez le nombre !");
    
    println!("Veuillez entrer un nombre.");
    
    // let pour créer une variable, comme en JS
    // mut permet de dire que la variable est modifiable, car par défaut les variables sont immuables
    let mut supposition = String::new();

    // io::stdin() permet de récupérer l'entrée utilisateur
    io::stdin()
        // read_line permet de lire l'entrée utilisateur (&mut supposition) et de la stocker dans la variable supposition créee plus haut
        .read_line(&mut supposition)
        // expect permet de gérer les erreurs, ici si la lecture de l'entrée utilisateur échoue
        .expect("Échec de la lecture de l'entrée utilisateur");

            // Retire le retour à la ligne de la fin de la chaîne
    supposition = supposition.trim_end().to_string();

    // "Votre nombre : {}" est un formatage de chaîne de caractères, {} est remplacé par la variable supposition
    println!("Votre nombre : {}", supposition);
}


