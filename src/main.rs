use askama::Template;
use axum::{response::Html, routing::get, Router};
use tower_http::{compression::CompressionLayer, services::ServeDir};

struct Project {
    name: &'static str,
    date: &'static str,
    desc: &'static str,
    stack: &'static [&'static str],
    url: Option<&'static str>,
}

const PROJECTS: &[Project] = &[
    Project { name: "Secret Santa", date: "Oct. 2026", desc: "Application web de Secret Santa pour une classe : tirage contrôlé par l'admin, comptes Argon2id, CSRF, CSP stricte, animation d'enveloppe.", stack: &["Rust", "Axum", "Askama", "SQLite"], url: Some("https://github.com/Teamfliyou/secret-santa-rust") },
    Project { name: "Qalamio", date: "Oct. 2026", desc: "Système d'information scolaire de l'association musulmane audomaroise : classes, élèves, paiements famille, présences, notes et compétences Coran.", stack: &["TypeScript", "JWT"], url: None },
    Project { name: "Lernoviq — Coach TOEIC", date: "Oct. 2026", desc: "Entraînement au nouveau format TOEIC Reading avec coach IA DeepSeek qui explique chaque erreur. Design liquid glass.", stack: &["TypeScript", "DeepSeek"], url: Some("https://github.com/Teamfliyou/toeic-reading-ai-coach") },
    Project { name: "Vooby PrestaShop", date: "Oct. 2026", desc: "Boutique e-commerce PrestaShop.", stack: &["PrestaShop"], url: None },
    Project { name: "Tableau de bord étudiant", date: "Oct. 2026", desc: "Emploi du temps en temps réel (iCal ADE), kanban de devoirs et plan de révision généré par Gemini.", stack: &["FastAPI", "Vue.js", "SQLite", "Gemini"], url: Some("https://github.com/Teamfliyou/Tablau_de_bord_projet") },
    Project { name: "Optima Rénovation", date: "Août 2026", desc: "Site vitrine avec API de devis et espace admin, déployé sur Railway.", stack: &["Node.js", "Express"], url: None },
    Project { name: "Veille concurrentielle literie", date: "Août 2026", desc: "Analyse marketing automatisée des concurrents par scraping Playwright et LLM.", stack: &["Python", "FastAPI", "Playwright", "DeepSeek"], url: Some("https://github.com/Teamfliyou/marketing-analyzer") },
    Project { name: "TOEIC Backend", date: "Juil. 2026", desc: "API du coach TOEIC.", stack: &["JavaScript"], url: None },
    Project { name: "Consommation énergétique", date: "Mars 2026", desc: "Analyse de la consommation de gaz et d'électricité par département (projet CPI, Dunkerque).", stack: &["Python", "Tkinter"], url: Some("https://github.com/Teamfliyou/projet-informatique") },
    Project { name: "SIS AMA", date: "Déc. 2025", desc: "Première version de gestion scolaire : absences, notes, bulletins PDF, paiements.", stack: &["PHP", "Docker"], url: Some("https://github.com/Teamfliyou/ASSO-AMA-sis-") },
];

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    projects: &'static [Project],
}

async fn index() -> Html<String> {
    Html(Index { projects: PROJECTS }.render().expect("template"))
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CompressionLayer::new());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    println!("http://localhost:{port}");
    axum::serve(listener, app).await.unwrap();
}
