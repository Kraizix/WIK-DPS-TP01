# WIK-DPS-TP01

Serveur web développé en rust en utilisant le moins de dépendances possible.
Le serveur est une API qui renvoit les headers d'une requête sur la route /ping (HTTP GET)).
Toutes les autres routes renvoient vers une erreur 404
Le port d'écoute du serveur est modifiable via le fichier .env (variable PING_LISTEN_PORT).

## Lancer le projet :

Dans un premier temps il faut télécharger [rust](https://www.rust-lang.org/tools/install) ainsi que cargo.

Ensuite il faut cloner le git contenant le projet : `git clone https://github.com/Kraizix/WIK-DPS-TP01.git`

Il ne reste plus qu'à build et run le projet via l'invite de commandes dans le dossier du projet:

- `cd WIK-DPS-TP01`
- `cargo build`
- `cargo run`
