# Portfolio — TeamFliyou

Site vitrine personnel présentant mes projets GitHub, écrit en **Rust** avec **Axum** (serveur HTTP) et **Askama** (templates HTML compilés).

Contact : zaydfliyou25@gmail.com — étudiant à l'EILCO (CP2).

## Pourquoi Rust ?

- **Un seul binaire** : pas de Node, pas de dépendances à l'exécution, déploiement trivial.
- **Templates vérifiés à la compilation** : une faute dans `templates/index.html` (variable inconnue, balise mal fermée) casse le build, pas la prod.
- **Rapide et léger** : le site est rendu côté serveur, compressé en gzip, sans JavaScript côté client.

## Architecture

```
portfolio/
├── Cargo.toml            dépendances (axum, askama, tokio, tower-http)
├── src/main.rs           liste des projets + routes + démarrage du serveur
├── templates/index.html  page unique (hero, à propos, compétences, parcours, projets, contact)
├── static/main.js        animations : machine à écrire, apparition au scroll, compteurs, filtres
└── static/style.css      styles, animations CSS, thème clair/sombre automatique
```

### Comment ça marche

1. `PROJECTS` (dans `src/main.rs`) est une constante : chaque `Project` a un nom, une date, une description, une stack et un lien optionnel (`None` = dépôt privé).
2. La route `GET /` rend le template `Index` avec cette liste.
3. `/static/*` sert le CSS via `ServeDir`, et `CompressionLayer` compresse les réponses.
4. Le port se règle avec la variable d'environnement `PORT` (3000 par défaut).

## Lancer le projet

Prérequis : [Rust](https://rustup.rs) stable.

```bash
cargo run
```

Puis ouvrir <http://localhost:3000>.

Version optimisée :

```bash
cargo build --release
./target/release/portfolio
```

> Le binaire lit `static/` depuis le dossier courant : le lancer depuis la racine du projet.

> Windows (toolchain GNU) : si la compilation échoue avec une erreur `dlltool` / `Can't open def file`, le chemin du projet est trop long. Utiliser un dossier cible court : `$env:CARGO_TARGET_DIR="C:\t\pf"`.

## Ajouter un projet

Ajouter une entrée dans `PROJECTS` (`src/main.rs`) :

```rust
Project {
    name: "Mon projet",
    date: "Nov. 2026",
    desc: "Ce que fait le projet.",
    stack: &["Rust", "Axum"],
    url: Some("https://github.com/Teamfliyou/mon-projet"), // None si privé
},
```

## Projets présentés

Secret Santa (Rust), Qalamio, Lernoviq (coach TOEIC IA), Vooby PrestaShop, Tableau de bord étudiant, Optima Rénovation, Veille concurrentielle literie, TOEIC Backend, Consommation énergétique, SIS AMA.

## Stack

Rust · Axum 0.7 · Askama 0.12 · Tokio · tower-http · CSS pur

## Fonctionnalités du site

- **Hero animé** : formes flottantes, titre en dégradé, texte qui s'écrit tout seul.
- **Apparition au scroll** (`IntersectionObserver`), barre de progression, compteurs animés.
- **Compétences** avec barres qui se remplissent.
- **Parcours** : frise chronologique des dates de chaque projet.
- **Projets** filtrables par technologie, cartes avec effet au survol.
- Responsive, thème clair/sombre, respect de `prefers-reduced-motion`.
