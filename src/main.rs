use pulldown_cmark::{html, Parser};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct PostMeta {
    title: String,
    date: String,
    description: Option<String>,
}

struct Post {
    meta: PostMeta,
    content_html: String,
    slug: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Locale {
    En,
    Es,
    Fr,
}

impl Locale {
    fn all() -> [Locale; 3] {
        [Locale::En, Locale::Es, Locale::Fr]
    }

    /// BCP-47 code for the <html lang> attribute.
    fn code(self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Es => "es",
            Locale::Fr => "fr",
        }
    }

    /// Output sub-directory prefix. English lives at the root.
    fn prefix(self) -> &'static str {
        match self {
            Locale::En => "",
            Locale::Es => "es/",
            Locale::Fr => "fr/",
        }
    }

    /// Content sub-directory holding this locale's markdown posts.
    fn content_dir(self) -> &'static str {
        match self {
            Locale::En => "content/posts/en",
            Locale::Es => "content/posts/es",
            Locale::Fr => "content/posts/fr",
        }
    }

    /// Flag emoji shown in the language switcher.
    fn flag(self) -> &'static str {
        match self {
            Locale::En => "🇬🇧",
            Locale::Es => "🇪🇸",
            Locale::Fr => "🇫🇷",
        }
    }

    /// The language's own name, shown in the switcher.
    fn label(self) -> &'static str {
        match self {
            Locale::En => "English",
            Locale::Es => "Español",
            Locale::Fr => "Français",
        }
    }
}

/// All translatable UI chrome for a locale.
struct Ui {
    site_title: &'static str,
    tagline: &'static str,
    nav_home: &'static str,
    nav_about: &'static str,
    nav_writing: &'static str,
    nav_aria: &'static str,
    skip_link: &'static str,
    meta_description: &'static str,
    author: &'static str,
    footer1: &'static str,
    footer2: &'static str,
    choose_lang: &'static str,
    toggle_theme: &'static str,
    blog_heading: &'static str,
    blog_intro: &'static str,
    page_home: &'static str,
    page_about: &'static str,
}

fn ui(locale: Locale) -> Ui {
    match locale {
        Locale::En => Ui {
            site_title: "Nature &amp; Technology",
            tagline: "Exploring sustainability and AI safety for a humane future",
            nav_home: "Home",
            nav_about: "About",
            nav_writing: "Writing",
            nav_aria: "Main navigation",
            skip_link: "Skip to main content",
            meta_description:
                "Exploring the intersection of sustainability, AI safety, and our collective future.",
            author: "Vincent Rizzo",
            footer1: "Crafted with care using Rust - Designed for sustainability",
            footer2: "Growing ideas for a thriving planet and aligned AI",
            choose_lang: "Choose language",
            toggle_theme: "Toggle light/dark theme",
            blog_heading: "Writing",
            blog_intro:
                "Reflections on sustainability, AI safety, and cultivating a humane technological future.",
            page_home: "Home",
            page_about: "About",
        },
        Locale::Es => Ui {
            site_title: "Naturaleza y Tecnología",
            tagline: "Explorando la sostenibilidad y la seguridad de la IA para un futuro humano",
            nav_home: "Inicio",
            nav_about: "Acerca de",
            nav_writing: "Escritos",
            nav_aria: "Navegación principal",
            skip_link: "Saltar al contenido principal",
            meta_description:
                "Explorando la intersección de la sostenibilidad, la seguridad de la IA y nuestro futuro colectivo.",
            author: "Vincent Rizzo",
            footer1: "Hecho con cariño usando Rust - Diseñado para la sostenibilidad",
            footer2: "Cultivando ideas para un planeta próspero y una IA alineada",
            choose_lang: "Elegir idioma",
            toggle_theme: "Cambiar tema claro/oscuro",
            blog_heading: "Escritos",
            blog_intro:
                "Reflexiones sobre la sostenibilidad, la seguridad de la IA y el cultivo de un futuro tecnológico humano.",
            page_home: "Inicio",
            page_about: "Acerca de",
        },
        Locale::Fr => Ui {
            site_title: "Nature et Technologie",
            tagline: "Explorer la durabilité et la sécurité de l'IA pour un avenir humain",
            nav_home: "Accueil",
            nav_about: "À propos",
            nav_writing: "Articles",
            nav_aria: "Navigation principale",
            skip_link: "Aller au contenu principal",
            meta_description:
                "Explorer l'intersection de la durabilité, de la sécurité de l'IA et de notre avenir collectif.",
            author: "Vincent Rizzo",
            footer1: "Conçu avec soin en Rust - Pensé pour la durabilité",
            footer2: "Cultiver des idées pour une planète florissante et une IA alignée",
            choose_lang: "Choisir la langue",
            toggle_theme: "Basculer le thème clair/sombre",
            blog_heading: "Articles",
            blog_intro:
                "Réflexions sur la durabilité, la sécurité de l'IA et la culture d'un avenir technologique humain.",
            page_home: "Accueil",
            page_about: "À propos",
        },
    }
}

fn main() {
    let output_dir = Path::new("output");
    fs::create_dir_all(output_dir).expect("Failed to create output directory");
    fs::create_dir_all(output_dir.join("css")).expect("Failed to create css directory");

    // Generate CSS once (shared across all locales via absolute /css/ path)
    generate_css(output_dir);

    for locale in Locale::all() {
        let dir = output_dir.join(locale.prefix());
        fs::create_dir_all(&dir).expect("Failed to create locale directory");
        fs::create_dir_all(dir.join("posts")).expect("Failed to create posts directory");

        let posts = parse_posts(locale);

        for post in &posts {
            generate_post_page(locale, post, &dir);
        }

        generate_blog_index(locale, &posts, &dir);
        generate_home_page(locale, &dir);
        generate_about_page(locale, &dir);

        println!("✓ Generated locale '{}'", locale.code());
    }

    // Copy static content (standalone HTML pages, English only, verbatim)
    copy_static_content(output_dir);

    println!("✓ Site generated successfully in 'output/' directory");
    println!("  - / (English), /es/ (Español), /fr/ (Français)");
}

fn copy_static_content(output_dir: &Path) {
    let static_dir = Path::new("static");
    if !static_dir.exists() {
        return;
    }
    for entry in WalkDir::new(static_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let src = entry.path();
        let rel_path = src.strip_prefix(static_dir).expect("entry under static dir");
        let dest = output_dir.join(rel_path);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).expect("Failed to create output subdir");
        }
        fs::copy(src, &dest).expect(&format!("Failed to copy {}", rel_path.display()));
    }
}

fn generate_css(output_dir: &Path) {
    let css = r#"/* Nature & Humane Focused Modern Design */
@import url('https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,400;0,500;0,600;1,400&family=Inter:wght@300;400;500&display=swap');

:root {
    /* Nature-inspired palette - WCAG AA compliant contrast ratios */
    --bg-primary: #f8f6f3;
    --bg-secondary: #efeae4;
    --bg-accent: #e8e0d5;
    --text-primary: #2c3e2d;
    --text-secondary: #4a5c4b;  /* Darkened for better contrast - 4.5:1 ratio */
    --text-muted: #5f6f60;      /* Darkened from #8a9a8c for WCAG AA */
    --accent-primary: #3d6849;  /* Darkened for better contrast */
    --accent-secondary: #5a7c45;
    --accent-warm: #996b3d;     /* Darkened for better contrast */
    --accent-earth: #6b5740;    /* Darkened for better contrast */
    --border-light: rgba(74, 124, 89, 0.15);
    --border-medium: rgba(74, 124, 89, 0.25);
    --shadow-soft: rgba(44, 62, 45, 0.08);
    --shadow-medium: rgba(44, 62, 45, 0.12);

    /* Focus ring color */
    --focus-ring: #2c5e3f;

    /* Spacing scale */
    --space-xs: 0.5rem;
    --space-sm: 1rem;
    --space-md: 1.5rem;
    --space-lg: 2.5rem;
    --space-xl: 4rem;

    /* Typography */
    --font-serif: 'Cormorant Garamond', Georgia, serif;
    --font-sans: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;

    /* Transitions */
    --transition-fast: 0.2s ease;
    --transition-medium: 0.3s ease;
}

/* Dark theme palette - WCAG AA compliant, forest-night.
   Defined once, applied via explicit [data-theme="dark"] and via system
   preference when the user has not made an explicit choice. */
:root[data-theme="dark"] {
    --bg-primary: #161b16;
    --bg-secondary: #1f261f;
    --bg-accent: #2a322a;
    --text-primary: #e8ede8;
    --text-secondary: #c3cdc3;
    --text-muted: #9aa89a;
    --accent-primary: #7fae87;
    --accent-secondary: #93c07a;
    --accent-warm: #cd9d6c;
    --accent-earth: #cbb893;
    --border-light: rgba(160, 200, 170, 0.15);
    --border-medium: rgba(160, 200, 170, 0.28);
    --shadow-soft: rgba(0, 0, 0, 0.3);
    --shadow-medium: rgba(0, 0, 0, 0.45);
    --focus-ring: #9ccaa6;
}

@media (prefers-color-scheme: dark) {
    :root:not([data-theme="light"]) {
        --bg-primary: #161b16;
        --bg-secondary: #1f261f;
        --bg-accent: #2a322a;
        --text-primary: #e8ede8;
        --text-secondary: #c3cdc3;
        --text-muted: #9aa89a;
        --accent-primary: #7fae87;
        --accent-secondary: #93c07a;
        --accent-warm: #cd9d6c;
        --accent-earth: #cbb893;
        --border-light: rgba(160, 200, 170, 0.15);
        --border-medium: rgba(160, 200, 170, 0.28);
        --shadow-soft: rgba(0, 0, 0, 0.3);
        --shadow-medium: rgba(0, 0, 0, 0.45);
        --focus-ring: #9ccaa6;
    }
}

* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

/* Respect user preference for reduced motion */
@media (prefers-reduced-motion: reduce) {
    *,
    *::before,
    *::after {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
        scroll-behavior: auto !important;
    }
}

html {
    scroll-behavior: smooth;
}

/* Screen reader only utility - visually hidden but accessible */
.sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
}

/* Skip to content link - visible on focus for keyboard navigation */
.skip-link {
    position: absolute;
    top: -100%;
    left: 50%;
    transform: translateX(-50%);
    background: var(--text-primary);
    color: var(--bg-primary);
    padding: var(--space-sm) var(--space-md);
    border-radius: 0 0 8px 8px;
    z-index: 1000;
    text-decoration: none;
    font-weight: 500;
    transition: top var(--transition-fast);
}

.skip-link:focus {
    top: 0;
    outline: 3px solid var(--focus-ring);
    outline-offset: 2px;
}

body {
    font-family: var(--font-sans);
    font-weight: 400;
    line-height: 1.7;
    color: var(--text-primary);
    background: var(--bg-primary);
    background-image:
        radial-gradient(ellipse at 20% 0%, rgba(123, 160, 91, 0.05) 0%, transparent 50%),
        radial-gradient(ellipse at 80% 100%, rgba(196, 149, 106, 0.05) 0%, transparent 50%);
    min-height: 100vh;
    transition: background-color var(--transition-medium), color var(--transition-medium);
}

.container {
    max-width: 780px;
    margin: 0 auto;
    padding: var(--space-lg) var(--space-md);
}

/* Header */
header {
    margin-bottom: var(--space-xl);
    padding-bottom: var(--space-lg);
    position: relative;
}

header::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 2px;
    background: linear-gradient(90deg, var(--accent-primary), var(--accent-secondary), var(--accent-warm));
    opacity: 0.4;
    border-radius: 1px;
}

/* Header top row: title + language switcher */
.header-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: var(--space-sm);
    margin-bottom: var(--space-sm);
}

.site-title {
    font-family: var(--font-serif);
    font-size: 2rem;
    font-weight: 500;
    color: var(--text-primary);
    letter-spacing: -0.02em;
}

.site-title a {
    color: inherit;
    text-decoration: none;
}

.site-tagline {
    font-size: 0.9rem;
    color: var(--text-muted);
    font-weight: 300;
    margin-bottom: var(--space-md);
}

/* Header actions: theme toggle + language switcher */
.header-actions {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    flex-shrink: 0;
}

/* Light/dark theme toggle */
.theme-toggle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    padding: var(--space-xs) 0.7rem;
    font-size: 1.1rem;
    line-height: 1;
    color: var(--text-secondary);
    background: var(--bg-secondary);
    border: 1px solid var(--border-medium);
    border-radius: 8px;
    transition: background var(--transition-fast), border-color var(--transition-fast), color var(--transition-fast);
}

.theme-toggle:hover {
    background: var(--bg-accent);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
}

/* Show the icon for the theme you'd switch TO: moon in light, sun in dark */
.theme-toggle .icon-dark { display: inline; }
.theme-toggle .icon-light { display: none; }
:root[data-theme="dark"] .theme-toggle .icon-dark { display: none; }
:root[data-theme="dark"] .theme-toggle .icon-light { display: inline; }
@media (prefers-color-scheme: dark) {
    :root:not([data-theme="light"]) .theme-toggle .icon-dark { display: none; }
    :root:not([data-theme="light"]) .theme-toggle .icon-light { display: inline; }
}

/* Language switcher (zero-JS dropdown via <details>) */
.lang-switcher {
    position: relative;
    flex-shrink: 0;
}

.lang-switcher summary {
    list-style: none;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    padding: var(--space-xs) 0.75rem;
    font-size: 0.9rem;
    color: var(--text-secondary);
    background: var(--bg-secondary);
    border: 1px solid var(--border-medium);
    border-radius: 8px;
    transition: background var(--transition-fast), border-color var(--transition-fast), color var(--transition-fast);
}

.lang-switcher summary::-webkit-details-marker {
    display: none;
}

.lang-switcher summary:hover {
    background: var(--bg-accent);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
}

.lang-switcher summary:focus-visible {
    outline: 3px solid var(--focus-ring);
    outline-offset: 2px;
}

.lang-switcher .flag {
    font-size: 1.1rem;
    line-height: 1;
}

.lang-switcher .chev {
    font-size: 0.7rem;
    transition: transform var(--transition-fast);
}

.lang-switcher[open] .chev {
    transform: rotate(180deg);
}

.lang-menu {
    position: absolute;
    right: 0;
    top: calc(100% + 0.4rem);
    margin: 0;
    padding: 0.35rem;
    list-style: none;
    background: var(--bg-primary);
    border: 1px solid var(--border-medium);
    border-radius: 10px;
    box-shadow: 0 8px 30px var(--shadow-medium);
    min-width: 11rem;
    z-index: 100;
}

.lang-menu li {
    margin: 0;
}

.lang-menu a {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: var(--space-xs) 0.6rem;
    border-radius: 6px;
    color: var(--text-secondary);
    border-bottom: none;
    font-size: 0.95rem;
}

.lang-menu a:hover {
    background: var(--bg-secondary);
    color: var(--accent-primary);
    border-bottom: none;
}

.lang-menu a[aria-current="true"] {
    color: var(--accent-primary);
    font-weight: 500;
    background: var(--bg-accent);
}

nav {
    display: flex;
    gap: var(--space-md);
    flex-wrap: wrap;
}

nav a {
    color: var(--text-secondary);
    text-decoration: none;
    font-size: 0.95rem;
    font-weight: 400;
    padding: var(--space-xs) 0;
    position: relative;
    transition: color var(--transition-fast);
}

nav a::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    width: 0;
    height: 2px;
    background: var(--accent-primary);
    transition: width var(--transition-medium);
    border-radius: 1px;
}

nav a:hover {
    color: var(--accent-primary);
}

nav a:hover::after {
    width: 100%;
}

/* Active page indicator for current navigation */
nav a[aria-current="page"] {
    color: var(--accent-primary);
    font-weight: 500;
}

nav a[aria-current="page"]::after {
    width: 100%;
    background: var(--accent-primary);
}

/* Main content */
main {
    min-height: 50vh;
}

/* Focus outline when skipping to main */
main:focus {
    outline: none;
}

/* Typography */
h1, h2, h3, h4 {
    font-family: var(--font-serif);
    font-weight: 500;
    color: var(--text-primary);
    line-height: 1.3;
}

h1 {
    font-size: 2.5rem;
    margin-bottom: var(--space-md);
    letter-spacing: -0.02em;
}

h2 {
    font-size: 1.75rem;
    margin-top: var(--space-xl);
    margin-bottom: var(--space-sm);
}

h3 {
    font-size: 1.35rem;
    margin-top: var(--space-lg);
    margin-bottom: var(--space-sm);
}

p {
    margin-bottom: var(--space-sm);
    color: var(--text-secondary);
}

a {
    color: var(--accent-primary);
    text-decoration: none;
    transition: color var(--transition-fast);
    border-bottom: 1px solid transparent;
}

a:hover {
    color: var(--accent-secondary);
    border-bottom-color: var(--accent-secondary);
}

/* Intro section */
.intro {
    font-family: var(--font-serif);
    font-size: 1.35rem;
    line-height: 1.8;
    color: var(--text-primary);
    margin-bottom: var(--space-lg);
}

.intro strong {
    color: var(--accent-primary);
    font-weight: 500;
}

/* Feature cards - for home page emphasis */
.feature-highlight {
    background: var(--bg-secondary);
    border-radius: 12px;
    padding: var(--space-lg);
    margin: var(--space-lg) 0;
    border: 1px solid var(--border-light);
    box-shadow: 0 4px 20px var(--shadow-soft);
}

/* Blog post list */
.post-list {
    list-style: none;
}

.post-list li {
    margin-bottom: var(--space-lg);
    padding: var(--space-md);
    background: var(--bg-secondary);
    border-radius: 12px;
    border: 1px solid var(--border-light);
    transition: transform var(--transition-medium), box-shadow var(--transition-medium);
}

.post-list li:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 30px var(--shadow-medium);
}

.post-date {
    color: var(--text-muted);
    font-size: 0.85rem;
    font-weight: 400;
    display: flex;
    align-items: center;
    gap: var(--space-xs);
}

.post-date::before {
    content: '';
    display: inline-block;
    width: 6px;
    height: 6px;
    background: var(--accent-secondary);
    border-radius: 50%;
}

.post-title {
    font-family: var(--font-serif);
    font-size: 1.4rem;
    font-weight: 500;
    display: block;
    margin: var(--space-xs) 0;
    color: var(--text-primary);
    border-bottom: none;
    transition: color var(--transition-fast);
}

.post-title:hover {
    color: var(--accent-primary);
    border-bottom: none;
    text-decoration: underline;
    text-decoration-color: var(--accent-primary);
    text-underline-offset: 3px;
}

.post-description {
    color: var(--text-muted);
    font-size: 0.95rem;
    line-height: 1.6;
    margin-top: var(--space-xs);
}

/* Article styling */
article {
    line-height: 1.85;
}

article h1 {
    font-size: 2.75rem;
    margin-bottom: var(--space-sm);
}

article .post-date {
    margin-bottom: var(--space-lg);
    padding-bottom: var(--space-md);
    border-bottom: 1px solid var(--border-light);
}

article p {
    font-size: 1.05rem;
    margin-bottom: var(--space-md);
}

article img {
    max-width: 100%;
    height: auto;
    border-radius: 8px;
    margin: var(--space-lg) 0;
    box-shadow: 0 4px 20px var(--shadow-soft);
}

.post-author {
    font-family: var(--font-serif);
    font-style: italic;
    font-size: 1.1rem;
    color: var(--text-secondary);
    margin-top: var(--space-xl);
    padding-top: var(--space-md);
    border-top: 1px solid var(--border-light);
    text-align: right;
}

/* Blockquotes */
blockquote {
    border-left: 3px solid var(--accent-warm);
    padding: var(--space-md) var(--space-md);
    margin: var(--space-lg) 0;
    background: var(--bg-secondary);
    border-radius: 0 8px 8px 0;
    font-family: var(--font-serif);
    font-style: italic;
    font-size: 1.1rem;
    color: var(--text-secondary);
}

blockquote p:last-child {
    margin-bottom: 0;
}

/* Code blocks */
code {
    background: var(--bg-accent);
    padding: 0.2rem 0.5rem;
    border-radius: 4px;
    font-size: 0.9em;
    color: var(--accent-earth);
    font-family: 'SF Mono', Monaco, 'Cascadia Code', monospace;
}

pre {
    background: var(--text-primary);
    color: var(--bg-primary);
    padding: var(--space-md);
    border-radius: 10px;
    overflow-x: auto;
    margin: var(--space-lg) 0;
    box-shadow: 0 4px 20px var(--shadow-medium);
}

pre code {
    background: none;
    padding: 0;
    color: inherit;
}

/* Lists */
ul, ol {
    margin: var(--space-sm) 0 var(--space-md) var(--space-md);
}

li {
    margin-bottom: var(--space-xs);
    color: var(--text-secondary);
}

li::marker {
    color: var(--accent-primary);
}

/* Footer */
footer {
    margin-top: var(--space-xl);
    padding-top: var(--space-lg);
    position: relative;
    text-align: center;
}

footer::before {
    content: '';
    position: absolute;
    top: 0;
    left: 20%;
    right: 20%;
    height: 1px;
    background: linear-gradient(90deg, transparent, var(--border-medium), transparent);
}

footer p {
    color: var(--text-muted);
    font-size: 0.85rem;
}

.footer-nature {
    display: flex;
    justify-content: center;
    gap: var(--space-sm);
    margin-bottom: var(--space-sm);
    opacity: 0.6;
}

/* Decorative leaf elements */
.leaf-icon {
    display: inline-block;
    color: var(--accent-secondary);
}

/* About page specific */
.about-section {
    margin-bottom: var(--space-lg);
}

.about-section h2 {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
}

/* Responsive design */
@media (max-width: 640px) {
    :root {
        --space-lg: 2rem;
        --space-xl: 3rem;
    }

    h1 {
        font-size: 2rem;
    }

    article h1 {
        font-size: 2rem;
    }

    .intro {
        font-size: 1.15rem;
    }

    .site-title {
        font-size: 1.75rem;
    }

    nav {
        gap: var(--space-sm);
    }

    .lang-switcher .lang-name {
        display: none;
    }
}

/* Ensure sufficient touch target size on mobile */
@media (pointer: coarse) {
    nav a {
        min-height: 44px;
        display: inline-flex;
        align-items: center;
    }

    .post-list a {
        min-height: 44px;
        display: inline-flex;
        align-items: center;
    }
}

/* Focus states for accessibility - enhanced visibility */
a:focus-visible,
button:focus-visible,
input:focus-visible,
textarea:focus-visible,
select:focus-visible {
    outline: 3px solid var(--focus-ring);
    outline-offset: 2px;
    border-radius: 2px;
}

/* High contrast mode support */
@media (prefers-contrast: high) {
    :root {
        --text-secondary: #2c3e2d;
        --text-muted: #3a4a3b;
        --border-light: rgba(74, 124, 89, 0.4);
        --border-medium: rgba(74, 124, 89, 0.6);
    }

    a:focus-visible,
    button:focus-visible {
        outline-width: 4px;
    }
}

/* Selection styling */
::selection {
    background: rgba(74, 124, 89, 0.2);
    color: var(--text-primary);
}

/* Print styles */
@media print {
    body {
        background: white;
        color: black;
    }

    header::after,
    footer::before {
        display: none;
    }

    .skip-link,
    .lang-switcher {
        display: none;
    }
}
"#;
    fs::write(output_dir.join("css/style.css"), css).expect("Failed to write CSS");
}

fn parse_posts(locale: Locale) -> Vec<Post> {
    let mut posts = Vec::new();
    let posts_dir = Path::new(locale.content_dir());

    if !posts_dir.exists() {
        return posts;
    }

    for entry in WalkDir::new(posts_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let content = fs::read_to_string(entry.path()).expect("Failed to read post");

        if let Some(post) = parse_post(&content, entry.path()) {
            posts.push(post);
        }
    }

    // Sort by date descending
    posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
    posts
}

fn parse_post(content: &str, path: &Path) -> Option<Post> {
    // Parse YAML frontmatter
    if !content.starts_with("---") {
        return None;
    }

    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return None;
    }

    let yaml = parts[1].trim();
    let markdown = parts[2].trim();

    let meta: PostMeta = serde_yaml::from_str(yaml).ok()?;

    // Convert markdown to HTML
    let parser = Parser::new(markdown);
    let mut content_html = String::new();
    html::push_html(&mut content_html, parser);

    let slug = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("post")
        .to_string();

    Some(Post {
        meta,
        content_html,
        slug,
    })
}

/// Build the localized main navigation links (root-absolute hrefs).
fn nav_html(locale: Locale, current_page: &str) -> String {
    let u = ui(locale);
    let prefix = locale.prefix();

    let link = |rel: &str, label: &str, key: &str| {
        let aria = if current_page == key {
            r#" aria-current="page""#
        } else {
            ""
        };
        format!(r#"                <a href="/{prefix}{rel}"{aria}>{label}</a>"#)
    };

    format!(
        "{}\n{}\n{}",
        link("", u.nav_home, "home"),
        link("about.html", u.nav_about, "about"),
        link("blog.html", u.nav_writing, "blog"),
    )
}

/// Build the zero-JS flag dropdown that links to the equivalent page in each language.
/// `rel_path` is the page path relative to a locale root ("", "about.html",
/// "blog.html", "posts/<slug>.html").
fn lang_switcher_html(current: Locale, rel_path: &str) -> String {
    let mut items = String::new();
    for loc in Locale::all() {
        let aria = if loc == current {
            r#" aria-current="true""#
        } else {
            ""
        };
        let prefix = loc.prefix();
        let flag = loc.flag();
        let label = loc.label();
        items.push_str(&format!(
            r#"                    <li><a href="/{prefix}{rel_path}"{aria}><span class="flag">{flag}</span> {label}</a></li>
"#
        ));
    }

    let choose = ui(current).choose_lang;
    let cur_flag = current.flag();
    let cur_label = current.label();

    format!(
        r#"<details class="lang-switcher">
                <summary aria-label="{choose}"><span class="flag">{cur_flag}</span><span class="lang-name">{cur_label}</span><span class="chev" aria-hidden="true">▾</span></summary>
                <ul class="lang-menu">
{items}                </ul>
            </details>"#
    )
}

/// Blocking head script: applies the saved theme before first paint (no FOUC).
fn theme_head_script() -> &'static str {
    r#"<script>(function(){try{var t=localStorage.getItem('theme');if(t==='dark'||t==='light')document.documentElement.dataset.theme=t;}catch(e){}})();</script>"#
}

/// Theme toggle button. Shows moon (switch to dark) or sun (switch to light) via CSS.
fn theme_toggle_html(label: &str) -> String {
    format!(
        r#"<button type="button" class="theme-toggle" id="theme-toggle" aria-label="{label}" title="{label}"><span class="icon-dark" aria-hidden="true">🌙</span><span class="icon-light" aria-hidden="true">☀️</span></button>"#
    )
}

/// Toggle handler: flips the effective theme, persists it, runs at end of body.
fn theme_init_script() -> &'static str {
    r#"<script>(function(){var b=document.getElementById('theme-toggle');if(!b)return;b.addEventListener('click',function(){var r=document.documentElement,cur=r.dataset.theme;if(!cur)cur=window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches?'dark':'light';var next=cur==='dark'?'light':'dark';r.dataset.theme=next;try{localStorage.setItem('theme',next);}catch(e){}});})();</script>"#
}

fn html_template(
    locale: Locale,
    page_title: &str,
    content: &str,
    current_page: &str,
    rel_path: &str,
) -> String {
    let u = ui(locale);
    let lang = locale.code();
    let prefix = locale.prefix();
    let brand = u.site_title;
    let tagline = u.tagline;
    let skip = u.skip_link;
    let desc = u.meta_description;
    let nav_aria = u.nav_aria;
    let footer1 = u.footer1;
    let footer2 = u.footer2;
    let nav = nav_html(locale, current_page);
    let switcher = lang_switcher_html(locale, rel_path);
    let toggle = theme_toggle_html(u.toggle_theme);
    let head_script = theme_head_script();
    let init_script = theme_init_script();

    format!(
        r##"<!DOCTYPE html>
<html lang="{lang}">
<head>
    <meta charset="UTF-8">
    {head_script}
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="{desc}">
    <meta name="theme-color" content="#4a7c59">
    <title>{page_title} - {brand}</title>
    <link rel="stylesheet" href="/css/style.css">
    <link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🌿</text></svg>">
</head>
<body>
    <a href="#main-content" class="skip-link">{skip}</a>
    <div class="container">
        <header role="banner">
            <div class="header-top">
                <h1 class="site-title"><a href="/{prefix}">{brand}</a></h1>
                <div class="header-actions">
                    {toggle}
                    {switcher}
                </div>
            </div>
            <p class="site-tagline">{tagline}</p>
            <nav aria-label="{nav_aria}" role="navigation">
{nav}
            </nav>
        </header>
        <main id="main-content" role="main" tabindex="-1">
{content}
        </main>
        <footer role="contentinfo">
            <div class="footer-nature" aria-hidden="true">
                <span class="leaf-icon">🌱</span>
                <span class="leaf-icon">🍃</span>
                <span class="leaf-icon">🌿</span>
            </div>
            <p>{footer1}</p>
            <p>{footer2}</p>
        </footer>
    </div>
    {init_script}
</body>
</html>"##
    )
}

fn post_template(
    locale: Locale,
    title: &str,
    description: &str,
    content: &str,
    rel_path: &str,
) -> String {
    let u = ui(locale);
    let lang = locale.code();
    let prefix = locale.prefix();
    let brand = u.site_title;
    let tagline = u.tagline;
    let skip = u.skip_link;
    let nav_aria = u.nav_aria;
    let footer1 = u.footer1;
    let footer2 = u.footer2;
    let nav = nav_html(locale, "");
    let switcher = lang_switcher_html(locale, rel_path);
    let toggle = theme_toggle_html(u.toggle_theme);
    let head_script = theme_head_script();
    let init_script = theme_init_script();

    let meta_description = if description.is_empty() {
        u.meta_description
    } else {
        description
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="{lang}">
<head>
    <meta charset="UTF-8">
    {head_script}
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="{meta_description}">
    <meta name="theme-color" content="#4a7c59">
    <title>{title} - {brand}</title>
    <link rel="stylesheet" href="/css/style.css">
    <link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🌿</text></svg>">
</head>
<body>
    <a href="#main-content" class="skip-link">{skip}</a>
    <div class="container">
        <header role="banner">
            <div class="header-top">
                <h1 class="site-title"><a href="/{prefix}">{brand}</a></h1>
                <div class="header-actions">
                    {toggle}
                    {switcher}
                </div>
            </div>
            <p class="site-tagline">{tagline}</p>
            <nav aria-label="{nav_aria}" role="navigation">
{nav}
            </nav>
        </header>
        <main id="main-content" role="main" tabindex="-1">
            <article aria-labelledby="post-title">
{content}
            </article>
        </main>
        <footer role="contentinfo">
            <div class="footer-nature" aria-hidden="true">
                <span class="leaf-icon">🌱</span>
                <span class="leaf-icon">🍃</span>
                <span class="leaf-icon">🌿</span>
            </div>
            <p>{footer1}</p>
            <p>{footer2}</p>
        </footer>
    </div>
    {init_script}
</body>
</html>"##
    )
}

fn generate_post_page(locale: Locale, post: &Post, dir: &Path) {
    let author = ui(locale).author;
    let content = format!(
        r#"            <h1>{}</h1>
            <p class="post-date">{}</p>
            {}
            <p class="post-author">{}</p>"#,
        post.meta.title, post.meta.date, post.content_html, author
    );

    let desc = post.meta.description.as_deref().unwrap_or("");
    let rel_path = format!("posts/{}.html", post.slug);
    let html = post_template(locale, &post.meta.title, desc, &content, &rel_path);
    let path = dir.join("posts").join(format!("{}.html", post.slug));
    fs::write(path, html).expect("Failed to write post");
}

fn generate_blog_index(locale: Locale, posts: &[Post], dir: &Path) {
    let u = ui(locale);
    let prefix = locale.prefix();
    let mut list = format!(
        "        <h1>{}</h1>\n        <p class=\"intro\">{}</p>\n        <ul class=\"post-list\">\n",
        u.blog_heading, u.blog_intro
    );

    for post in posts {
        let desc = post
            .meta
            .description
            .as_ref()
            .map(|d| format!("<p class=\"post-description\">{}</p>", d))
            .unwrap_or_default();

        list.push_str(&format!(
            r#"            <li>
                <span class="post-date">{}</span>
                <a class="post-title" href="/{}posts/{}.html">{}</a>
                {}
            </li>
"#,
            post.meta.date, prefix, post.slug, post.meta.title, desc
        ));
    }

    list.push_str("        </ul>");

    let html = html_template(locale, u.blog_heading, &list, "blog", "blog.html");
    fs::write(dir.join("blog.html"), html).expect("Failed to write blog index");
}

fn generate_home_page(locale: Locale, dir: &Path) {
    let content = home_body(locale);
    let title = ui(locale).page_home;
    let html = html_template(locale, title, &content, "home", "");
    fs::write(dir.join("index.html"), html).expect("Failed to write home page");
}

fn home_body(locale: Locale) -> String {
    let p = locale.prefix();
    match locale {
        Locale::En => format!(
            r#"        <h1>Welcome</h1>
        <p class="intro">
            I explore the intersection of <strong>nature</strong>, <strong>technology</strong>, and our shared future.
            Like a garden, our technological landscape needs careful tending - nurturing what helps life flourish
            while being mindful of what we cultivate.
        </p>

        <div class="feature-highlight">
            <h2>The Wisdom of the Wheel</h2>
            <p>
                Consider the wheel and the bicycle - perhaps humanity's most elegant technologies.
                They don't demand more energy; they <strong>amplify what we already have</strong>.
                A cyclist moves four times faster than a walker while using the same effort.
                A cart with wheels lets one person move what would otherwise require many.
            </p>
            <p>
                These inventions didn't conquer nature - they worked with it. They extended our
                capacity to <strong>move</strong>, to <strong>build</strong>, and to <strong>connect</strong>
                without requiring ever-greater energy investment. The bicycle remains the most
                energy-efficient form of human transportation ever devised.
            </p>
            <p>
                This is the measure of truly great technology: not how much power it consumes,
                but how gracefully it multiplies human capability within natural limits.
            </p>
        </div>

        <div class="feature-highlight">
            <h2>Growing Thoughtful Technology</h2>
            <p>
                In an era of rapid change, I believe technology should serve humanity
                while honoring the living world that sustains us. My work focuses on two
                intertwined areas: <strong>sustainability</strong> and <strong>AI safety</strong>.
            </p>
            <p>
                Both require us to think beyond immediate gains - to consider the seeds we plant today
                and the forests they might become tomorrow. Like the wheel and bicycle, the best
                technologies expand what's possible without demanding more than we can sustainably give.
            </p>
        </div>

        <p>
            Explore my <a href="/{p}blog.html">writing</a> for reflections on these themes,
            or learn more <a href="/{p}about.html">about my journey</a>.
        </p>"#
        ),
        Locale::Es => format!(
            r#"        <h1>Bienvenido</h1>
        <p class="intro">
            Exploro la intersección entre la <strong>naturaleza</strong>, la <strong>tecnología</strong> y nuestro futuro compartido.
            Como un jardín, nuestro paisaje tecnológico necesita un cuidado atento: nutrir lo que ayuda a la vida a florecer
            siendo conscientes de lo que cultivamos.
        </p>

        <div class="feature-highlight">
            <h2>La sabiduría de la rueda</h2>
            <p>
                Piensa en la rueda y la bicicleta, quizá las tecnologías más elegantes de la humanidad.
                No exigen más energía; <strong>amplifican lo que ya tenemos</strong>.
                Un ciclista se mueve cuatro veces más rápido que quien camina usando el mismo esfuerzo.
                Una carreta con ruedas permite a una persona transportar lo que de otro modo requeriría a muchas.
            </p>
            <p>
                Estos inventos no conquistaron la naturaleza: trabajaron con ella. Ampliaron nuestra
                capacidad de <strong>movernos</strong>, <strong>construir</strong> y <strong>conectar</strong>
                sin requerir una inversión de energía cada vez mayor. La bicicleta sigue siendo la forma
                de transporte humano más eficiente energéticamente jamás concebida.
            </p>
            <p>
                Esta es la medida de la tecnología verdaderamente grande: no cuánta energía consume,
                sino con cuánta gracia multiplica la capacidad humana dentro de los límites naturales.
            </p>
        </div>

        <div class="feature-highlight">
            <h2>Cultivar una tecnología reflexiva</h2>
            <p>
                En una era de cambios rápidos, creo que la tecnología debería servir a la humanidad
                a la vez que honra el mundo vivo que nos sostiene. Mi trabajo se centra en dos
                áreas entrelazadas: la <strong>sostenibilidad</strong> y la <strong>seguridad de la IA</strong>.
            </p>
            <p>
                Ambas nos exigen pensar más allá de los beneficios inmediatos: considerar las semillas que plantamos hoy
                y los bosques en los que podrían convertirse mañana. Como la rueda y la bicicleta, las mejores
                tecnologías amplían lo posible sin exigir más de lo que podemos dar de forma sostenible.
            </p>
        </div>

        <p>
            Explora mis <a href="/{p}blog.html">escritos</a> para reflexiones sobre estos temas,
            o conoce más <a href="/{p}about.html">sobre mi trayectoria</a>.
        </p>"#
        ),
        Locale::Fr => format!(
            r#"        <h1>Bienvenue</h1>
        <p class="intro">
            J'explore l'intersection entre la <strong>nature</strong>, la <strong>technologie</strong> et notre avenir commun.
            Comme un jardin, notre paysage technologique demande un entretien attentif : nourrir ce qui aide la vie à s'épanouir
            tout en restant conscient de ce que nous cultivons.
        </p>

        <div class="feature-highlight">
            <h2>La sagesse de la roue</h2>
            <p>
                Pensez à la roue et à la bicyclette, peut-être les technologies les plus élégantes de l'humanité.
                Elles ne réclament pas plus d'énergie ; elles <strong>amplifient ce que nous avons déjà</strong>.
                Un cycliste avance quatre fois plus vite qu'un marcheur pour le même effort.
                Une charrette à roues permet à une personne de déplacer ce qui en exigerait autrement plusieurs.
            </p>
            <p>
                Ces inventions n'ont pas conquis la nature : elles ont travaillé avec elle. Elles ont étendu notre
                capacité à <strong>nous déplacer</strong>, à <strong>construire</strong> et à <strong>nous relier</strong>
                sans exiger un investissement énergétique toujours plus grand. La bicyclette demeure le mode
                de transport humain le plus efficace énergétiquement jamais conçu.
            </p>
            <p>
                Voilà la mesure d'une technologie vraiment grande : non pas l'énergie qu'elle consomme,
                mais la grâce avec laquelle elle multiplie la capacité humaine dans les limites naturelles.
            </p>
        </div>

        <div class="feature-highlight">
            <h2>Cultiver une technologie réfléchie</h2>
            <p>
                À une époque de changements rapides, je crois que la technologie devrait servir l'humanité
                tout en honorant le monde vivant qui nous soutient. Mon travail se concentre sur deux
                domaines entrelacés : la <strong>durabilité</strong> et la <strong>sécurité de l'IA</strong>.
            </p>
            <p>
                Tous deux nous demandent de penser au-delà des gains immédiats : de considérer les graines que nous plantons aujourd'hui
                et les forêts qu'elles pourraient devenir demain. Comme la roue et la bicyclette, les meilleures
                technologies élargissent le possible sans exiger plus que ce que nous pouvons durablement donner.
            </p>
        </div>

        <p>
            Explorez mes <a href="/{p}blog.html">articles</a> pour des réflexions sur ces thèmes,
            ou apprenez-en plus <a href="/{p}about.html">sur mon parcours</a>.
        </p>"#
        ),
    }
}

fn generate_about_page(locale: Locale, dir: &Path) {
    let content = about_body(locale);
    let title = ui(locale).page_about;
    let html = html_template(locale, title, content, "about", "about.html");
    fs::write(dir.join("about.html"), html).expect("Failed to write about page");
}

fn about_body(locale: Locale) -> &'static str {
    match locale {
        Locale::En => r#"        <h1>About Me</h1>
        <p class="intro">
            I'm Vincent Rizzo, a Machine Learning & Software engineer nurturing ideas at the intersection of technology and our living world.
        </p>

        <div class="about-section">
            <h2>What I Care About</h2>
            <p>
                <strong>Sustainability:</strong> Like understanding an ecosystem, I explore how our
                technological choices ripple through the environment. From energy-conscious computing
                to circular design principles, I'm drawn to approaches that let technology and nature
                coexist in harmony.
            </p>
            <p>
                <strong>AI Safety:</strong> As artificial intelligence grows more capable, ensuring it
                remains a beneficial presence - like a well-tended garden rather than an invasive species - becomes
                vital. I follow developments in alignment research, interpretability, and thoughtful governance.
            </p>
        </div>

        <div class="about-section">
            <h2>This Living Site</h2>
            <p>
                This website grows from a custom static site generator written in Rust.
                Posts bloom from simple Markdown files. The design itself reflects my values:
                minimal resource usage, gentle on the eyes, focused on what matters.
            </p>
            <p>
                Every byte saved is a small act of care for our digital commons.
            </p>
        </div>

        <div class="about-section">
            <h2>Let's Connect</h2>
            <p>
                I believe the best ideas emerge from conversation. Whether you're curious about
                sustainability, AI safety, or the places where they intertwine - I'd love to hear from you.
            </p>
            <p>
                Find me on GitHub or reach out through social channels.
            </p>
        </div>"#,
        Locale::Es => r#"        <h1>Sobre mí</h1>
        <p class="intro">
            Soy Vincent Rizzo, ingeniero de Machine Learning y Software que cultiva ideas en la intersección de la tecnología y nuestro mundo vivo.
        </p>

        <div class="about-section">
            <h2>Lo que me importa</h2>
            <p>
                <strong>Sostenibilidad:</strong> Como quien comprende un ecosistema, exploro cómo nuestras
                decisiones tecnológicas repercuten en el medio ambiente. Desde la computación consciente con la energía
                hasta los principios de diseño circular, me atraen los enfoques que permiten que la tecnología y la naturaleza
                coexistan en armonía.
            </p>
            <p>
                <strong>Seguridad de la IA:</strong> A medida que la inteligencia artificial se vuelve más capaz, garantizar que
                siga siendo una presencia beneficiosa - como un jardín bien cuidado y no como una especie invasora - se vuelve
                vital. Sigo los avances en investigación de alineación, interpretabilidad y gobernanza reflexiva.
            </p>
        </div>

        <div class="about-section">
            <h2>Este sitio vivo</h2>
            <p>
                Este sitio web crece a partir de un generador de sitios estáticos personalizado escrito en Rust.
                Las publicaciones brotan de simples archivos Markdown. El diseño en sí refleja mis valores:
                uso mínimo de recursos, suave para la vista, centrado en lo que importa.
            </p>
            <p>
                Cada byte ahorrado es un pequeño acto de cuidado hacia nuestros bienes digitales comunes.
            </p>
        </div>

        <div class="about-section">
            <h2>Conectemos</h2>
            <p>
                Creo que las mejores ideas surgen de la conversación. Tanto si sientes curiosidad por la
                sostenibilidad, la seguridad de la IA o los lugares donde se entrelazan, me encantaría saber de ti.
            </p>
            <p>
                Encuéntrame en GitHub o escríbeme a través de las redes sociales.
            </p>
        </div>"#,
        Locale::Fr => r#"        <h1>À propos de moi</h1>
        <p class="intro">
            Je suis Vincent Rizzo, ingénieur en Machine Learning et logiciel, qui cultive des idées à l'intersection de la technologie et de notre monde vivant.
        </p>

        <div class="about-section">
            <h2>Ce qui me tient à cœur</h2>
            <p>
                <strong>La durabilité :</strong> Comme on comprend un écosystème, j'explore comment nos
                choix technologiques se répercutent sur l'environnement. De l'informatique soucieuse de l'énergie
                aux principes de conception circulaire, je suis attiré par les approches qui laissent la technologie et la nature
                coexister en harmonie.
            </p>
            <p>
                <strong>La sécurité de l'IA :</strong> À mesure que l'intelligence artificielle gagne en capacité, garantir qu'elle
                reste une présence bénéfique - tel un jardin bien entretenu plutôt qu'une espèce invasive - devient
                vital. Je suis les avancées en recherche sur l'alignement, l'interprétabilité et une gouvernance réfléchie.
            </p>
        </div>

        <div class="about-section">
            <h2>Ce site vivant</h2>
            <p>
                Ce site web pousse à partir d'un générateur de site statique sur mesure écrit en Rust.
                Les articles éclosent de simples fichiers Markdown. Le design lui-même reflète mes valeurs :
                usage minimal des ressources, doux pour les yeux, centré sur l'essentiel.
            </p>
            <p>
                Chaque octet économisé est un petit geste de soin pour nos communs numériques.
            </p>
        </div>

        <div class="about-section">
            <h2>Restons en contact</h2>
            <p>
                Je crois que les meilleures idées naissent de la conversation. Que vous soyez curieux de
                durabilité, de sécurité de l'IA, ou des endroits où elles s'entremêlent, j'aimerais beaucoup avoir de vos nouvelles.
            </p>
            <p>
                Retrouvez-moi sur GitHub ou contactez-moi via les réseaux sociaux.
            </p>
        </div>"#,
    }
}
