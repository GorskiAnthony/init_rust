# Utilise une image de base officielle, ici Debian pour ARM
FROM arm64v8/debian:bullseye-slim

# Installe les dépendances nécessaires
RUN apt-get update && apt-get install -y build-essential curl

# Installe Rust avec rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Ajoute le chemin de Rust et Cargo à l'environnement PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Définir le répertoire de travail
WORKDIR /usr/src/myapp

# Copie les fichiers de votre projet Rust dans le conteneur
COPY . .

# Construit votre application Rust
RUN cargo build --release

# Exécuter votre application
CMD ["./target/release/mon_app"]
