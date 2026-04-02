# The Modern Freelancer Hub

A blogging website for digital freelancers. Four categories: Graphic Design, Content Writing, Video Editing, and Social Media Management — all with real, human-written posts based on earned experience.

**Target audience:** Students, office workers looking for side hustles, aspiring digital nomads.

---

## Quick Start

### Option A: Open the standalone HTML (no server needed)

Just open `index.html` in any browser. Everything is self-contained — React loads from CDN, all posts are embedded in the file. Works offline once loaded.

### Option B: Run with the Rust API backend

**Prerequisites:** Node.js 18+, Rust + Cargo (install via [rustup.rs](https://rustup.rs))

```bash
# 1. Start the Rust API server
cd backend
cargo run
# API will be at http://localhost:3001

# 2. Serve the frontend
# Option: Use any static file server
npx serve .
# Or with Python:
python3 -m http.server 8080
# Then open http://localhost:8080
```

---

## Project Structure

```
freelancer-hub/
├── index.html              # Complete self-contained React frontend
│                           # (React loaded via CDN, all posts embedded)
│
├── backend/
│   ├── Cargo.toml          # Rust dependencies (Axum, Tokio, Serde)
│   └── src/
│       └── main.rs         # Axum HTTP server
│                           # Routes: /api/posts, /api/categories, /health
│
└── README.md               # This file
```

---

## Frontend Features

- **Home page** — Hero post + featured sidebar + full post grid
- **Category pages** — Filtered view per skill category
- **Post reader** — Full article with related posts sidebar
- **Search** — Client-side full-text search across all posts
- **Responsive** — Works on mobile and desktop
- **Sticky navbar** — Category navigation always accessible

## Backend API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/api/posts` | All posts (supports `?q=`, `?category=`, `?limit=`) |
| GET | `/api/posts/featured` | Featured posts only |
| GET | `/api/posts/:slug` | Single post by slug |
| GET | `/api/categories` | All categories |
| GET | `/api/categories/:id` | Single category |

---

## Content Categories

| Category | Posts | Featured |
|----------|-------|---------|
| Graphic Design | 4 | ✓ |
| Content Writing | 4 | ✓ |
| Video Editing | 4 | ✓ |
| Social Media Management | 3 | ✓ |

---

## Next Steps / Expansion

**To add a CMS:**
- Connect [Contentful](https://contentful.com) or [Sanity.io](https://sanity.io) and fetch posts via API instead of hardcoded data
- Alternatively: store posts as Markdown files and parse at build time (works well with Vite + MDX)

**To upgrade to a full React project:**
```bash
npm create vite@latest freelancer-hub -- --template react
```
Then split `index.html` into proper component files under `src/`.

**To add comments:**
Connect [Giscus](https://giscus.app) (GitHub Discussions-based, free) or Disqus.

**To add a newsletter:**
Embed a [ConvertKit](https://convertkit.com) or [Mailchimp](https://mailchimp.com) form in the footer.

**For monetization (per the plan):**
- Affiliate links: add them naturally within post content
- Lead magnets: link to PDF checklists in the category sidebars
- AdSense: add unit after traffic reaches 1k+ visitors/month

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | React 18 (embedded via CDN) |
| Styling | Custom CSS with CSS variables |
| Backend | Rust + Axum 0.7 |
| Runtime | Tokio async runtime |
| Serialization | Serde + serde_json |
| CORS | tower-http CorsLayer |

---

*Prepared for Zara — 2025*
