<h1 align="center">Bienvenue sur init Rust 👋</h1>
<p>
  <a href="https://twitter.com/Gorski_anthony" target="_blank">
    <img alt="Twitter: Gorski_anthony" src="https://img.shields.io/twitter/follow/Gorski_anthony.svg?style=social" />
  </a>
</p>

> Ce projet est un template pour initialiser un projet Rust avec les outils de base.

## Documentation

Je me sert de la documentation officiel de Rust pour initialiser un projet.

-   [Le langage de programmation Rust - Le langage de programmation Rust](https://jimskapt.github.io/rust-book-fr/title-page.html)

## Architecture

```text
.
├── Cargo.toml
├── Dockerfile
├── README.md
├── docker-compose.yml
└── src
    └── main.rs
```

J'utilise docker pour lancer mon projet Rust.

Pourquoi ? Car pour l'instant, je n'ai pas envie de m'embêter avec l'installation de Rust sur ma machine.

## Utilisation

```sh
# Lancer le projet
docker-compose up
```

### Pour le jeu du plus ou du moins

```sh
# Lancer le projet
docker-compose run --rm rust-app
```

-   [Programmer le jeu du plus ou du moins - Le langage de programmation Rust](https://jimskapt.github.io/rust-book-fr/ch02-00-guessing-game-tutorial.html#%C3%89tendre-les-fonctionnalit%C3%A9s-de-rust-avec-une-crate)

## Auteur

👤 **Anthony Gorski**

-   𝕏 - (Twitter): [@Gorski_Anthony](https://twitter.com/Gorski_Anthony)
-   GitHub: [@GorskiAnthony](https://github.com/GorskiAnthony)

## Affichez votre soutien

## Donnez un ⭐️ si ce projet vous a aidé !

### 🗃️ Version

-   **v0.1.0** - First commit

---

### 👋 Qui suis-je ?

Je suis **Anthony Gorski**, développeur web et formateur à la [Wild Code School](https://www.wildcodeschool.com/fr-FR).
