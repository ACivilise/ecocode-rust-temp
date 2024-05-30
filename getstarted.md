
# Création du network docker

```bash
docker network create host_network
```

# lancement des containers

```bash
docker compose up
```

# configuration

## connexion

Se connecter à Sonar en local : http://localhost:9000/
avec comme identifants :
- login: admin
- password: admin

changer le mdp

## configuration 

Installer le plugin Rust language analyzer dans la market place.
Générer un token d'authentification #token
Créer un projet d'analyse local #project-name

Modifier le fichier sonar-project.properties

```bash
sonar.token=#token
sonar.projectKey=#project-name
sonar.host.url=http://localhost:9000
sonar.sources=./src
# --- optional properties ---
# defaults to project key
sonar.projectName=Run-test
community.rust.clippy.reportPaths=./target/clippy-report.json
```

# lancement de l'analyse

```bash
cargo clippy --message-format=json &> <CLIPPY REPORT FILE>
cargo clippy --message-format=json *> <CLIPPY REPORT FILE>
```

Ensuite aller dans le container sonar-scanner et lancer la commande 

```bash
sonar-scanner
```