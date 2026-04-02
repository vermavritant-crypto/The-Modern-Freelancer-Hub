// freelancer-hub API — built with Axum (Rust)
// Run: cargo run
// Serves the React frontend + JSON API for posts/categories

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/* ─── MODELS ──────────────────────────────────────────────────────────── */

#[derive(Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub slug: String,
    pub category: String,
    pub featured: bool,
    pub title: String,
    pub excerpt: String,
    pub date: String,
    pub read_time: String,
    pub content: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub category: Option<String>,
    pub limit: Option<usize>,
}

/* ─── STATE ───────────────────────────────────────────────────────────── */

#[derive(Clone)]
pub struct AppState {
    posts: Arc<Vec<Post>>,
    categories: Arc<Vec<Category>>,
}

impl AppState {
    fn new() -> Self {
        let categories = vec![
            Category {
                id: "graphic-design".into(),
                name: "Graphic Design".into(),
                description: "Earning with visual skills — from client logos to passive template income.".into(),
            },
            Category {
                id: "content-writing".into(),
                name: "Content Writing".into(),
                description: "The honest path from SEO content mills to high-ticket copywriting clients.".into(),
            },
            Category {
                id: "video-editing".into(),
                name: "Video Editing".into(),
                description: "Short-form, long-form, retainers — how video editors are building income.".into(),
            },
            Category {
                id: "social-media".into(),
                name: "Social Media".into(),
                description: "Strategy, community management, and what SMM really looks like day to day.".into(),
            },
        ];

        // In production: load from a database (PostgreSQL, SQLite, etc.)
        // For launch: posts are served from this in-memory list.
        let posts = vec![
            Post {
                id: 1,
                slug: "first-10000-selling-templates".into(),
                category: "graphic-design".into(),
                featured: true,
                title: "How I Made My First ₹10,000 Selling Templates — Without a Single Client".into(),
                excerpt: "I kept waiting to feel ready. Three months later I finally just started. The first sale came in on a random Tuesday.".into(),
                date: "March 28, 2025".into(),
                read_time: "7 min".into(),
                content: "Full content stored in database or CMS.".into(),
            },
            Post {
                id: 2,
                slug: "fiverr-vs-upwork-graphic-design".into(),
                category: "graphic-design".into(),
                featured: false,
                title: "Fiverr vs Upwork for Graphic Designers: What 6 Months Taught Me".into(),
                excerpt: "Both platforms work. They attract completely different kinds of buyers.".into(),
                date: "March 15, 2025".into(),
                read_time: "5 min".into(),
                content: "Full content stored in database or CMS.".into(),
            },
            Post {
                id: 5,
                slug: "seo-writing-vs-copywriting".into(),
                category: "content-writing".into(),
                featured: true,
                title: "SEO Writing Pays Bills. Copywriting Builds Wealth.".into(),
                excerpt: "After 300+ SEO articles, I switched to sales copywriting. My income doubled in 90 days.".into(),
                date: "March 22, 2025".into(),
                read_time: "8 min".into(),
                content: "Full content stored in database or CMS.".into(),
            },
            Post {
                id: 9,
                slug: "short-form-video-gold-rush".into(),
                category: "video-editing".into(),
                featured: true,
                title: "Short-Form Video is a Gold Rush. Here's How I Got In.".into(),
                excerpt: "I was editing YouTube vlogs for ₹5,000 a video. A client asked me to cut some Reels. My rate tripled in three weeks.".into(),
                date: "March 20, 2025".into(),
                read_time: "7 min".into(),
                content: "Full content stored in database or CMS.".into(),
            },
            Post {
                id: 13,
                slug: "40k-smm-side-hustle".into(),
                category: "social-media".into(),
                featured: true,
                title: "The ₹40,000/Month SMM Side Hustle Nobody Takes Seriously".into(),
                excerpt: "Spreadsheets and scheduling tools and a lot of client education. Not glamorous. But it works.".into(),
                date: "March 18, 2025".into(),
                read_time: "8 min".into(),
                content: "Full content stored in database or CMS.".into(),
            },
        ];

        Self {
            posts: Arc::new(posts),
            categories: Arc::new(categories),
        }
    }
}

/* ─── HANDLERS ────────────────────────────────────────────────────────── */

/// GET /api/posts
/// Optional query params: ?q=search&category=graphic-design&limit=10
async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Json<Vec<Post>> {
    let mut posts: Vec<Post> = state.posts.as_ref().clone();

    // Filter by category
    if let Some(ref cat) = params.category {
        posts.retain(|p| &p.category == cat);
    }

    // Search filter
    if let Some(ref q) = params.q {
        let q_lower = q.to_lowercase();
        posts.retain(|p| {
            p.title.to_lowercase().contains(&q_lower)
                || p.excerpt.to_lowercase().contains(&q_lower)
                || p.category.to_lowercase().contains(&q_lower)
        });
    }

    // Limit results
    if let Some(limit) = params.limit {
        posts.truncate(limit);
    }

    Json(posts)
}

/// GET /api/posts/:slug
async fn get_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Post>, StatusCode> {
    state
        .posts
        .iter()
        .find(|p| p.slug == slug)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// GET /api/posts/featured
async fn featured_posts(State(state): State<AppState>) -> Json<Vec<Post>> {
    let featured: Vec<Post> = state
        .posts
        .iter()
        .filter(|p| p.featured)
        .cloned()
        .collect();
    Json(featured)
}

/// GET /api/categories
async fn list_categories(State(state): State<AppState>) -> Json<Vec<Category>> {
    Json(state.categories.as_ref().clone())
}

/// GET /api/categories/:id
async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Category>, StatusCode> {
    state
        .categories
        .iter()
        .find(|c| c.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// GET /health
async fn health() -> &'static str {
    "OK"
}

/* ─── ROUTER ──────────────────────────────────────────────────────────── */

fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/posts", get(list_posts))
        .route("/posts/featured", get(featured_posts))
        .route("/posts/:slug", get(get_post))
        .route("/categories", get(list_categories))
        .route("/categories/:id", get(get_category))
        .with_state(state)
}

fn app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health))
        .nest("/api", api_router(state))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

/* ─── MAIN ────────────────────────────────────────────────────────────── */

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new();
    let app = app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind port 3001");

    tracing::info!("Freelancer Hub API listening on http://0.0.0.0:3001");
    tracing::info!("Endpoints:");
    tracing::info!("  GET /api/posts");
    tracing::info!("  GET /api/posts/:slug");
    tracing::info!("  GET /api/posts/featured");
    tracing::info!("  GET /api/categories");
    tracing::info!("  GET /health");

    axum::serve(listener, app).await.unwrap();
}
