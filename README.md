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

### Docker:

Build l'image :
`docker build -t webapi .`

Run le container :
`docker run -p 8080:7878 --rm --name webapi1 webapi`

Scan de l'image :
`docker scan webapi`

```
@kevin ➜ wik_dps_tp01 git(main) docker scan webapi

Testing webapi...

Package manager:   apk
Project name:      docker-image|webapi
Docker image:      webapi
Platform:          linux/amd64
Base image:        alpine:3.16.2

✔ Tested 26 dependencies for known vulnerabilities, no vulnerable paths found.
```
