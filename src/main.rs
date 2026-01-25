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

fn main() {
    let output_dir = Path::new("output");
    fs::create_dir_all(output_dir).expect("Failed to create output directory");
    fs::create_dir_all(output_dir.join("posts")).expect("Failed to create posts directory");
    fs::create_dir_all(output_dir.join("css")).expect("Failed to create css directory");

    // Generate CSS
    generate_css(output_dir);

    // Parse and generate blog posts
    let posts = parse_posts();

    // Generate individual post pages
    for post in &posts {
        generate_post_page(post, output_dir);
    }

    // Generate blog index
    generate_blog_index(&posts, output_dir);

    // Generate home page
    generate_home_page(output_dir);

    // Generate about page
    generate_about_page(output_dir);

    println!("✓ Site generated successfully in 'output/' directory");
    println!("  - index.html (home)");
    println!("  - about.html");
    println!("  - blog.html");
    println!("  - posts/*.html");
}

fn generate_css(output_dir: &Path) {
    let css = r#"/* Nature & Humane Focused Modern Design */
@import url('https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,400;0,500;0,600;1,400&family=Inter:wght@300;400;500&display=swap');

:root {
    /* Nature-inspired palette */
    --bg-primary: #f8f6f3;
    --bg-secondary: #efeae4;
    --bg-accent: #e8e0d5;
    --text-primary: #2c3e2d;
    --text-secondary: #5a6b5c;
    --text-muted: #8a9a8c;
    --accent-primary: #4a7c59;
    --accent-secondary: #7ba05b;
    --accent-warm: #c4956a;
    --accent-earth: #8b7355;
    --border-light: rgba(74, 124, 89, 0.15);
    --border-medium: rgba(74, 124, 89, 0.25);
    --shadow-soft: rgba(44, 62, 45, 0.08);
    --shadow-medium: rgba(44, 62, 45, 0.12);

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

* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

html {
    scroll-behavior: smooth;
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

.site-title {
    font-family: var(--font-serif);
    font-size: 2rem;
    font-weight: 500;
    color: var(--text-primary);
    margin-bottom: var(--space-sm);
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

/* Main content */
main {
    min-height: 50vh;
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
}

/* Focus states for accessibility */
a:focus-visible,
button:focus-visible {
    outline: 2px solid var(--accent-primary);
    outline-offset: 2px;
    border-radius: 2px;
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
}
"#;
    fs::write(output_dir.join("css/style.css"), css).expect("Failed to write CSS");
}

fn parse_posts() -> Vec<Post> {
    let mut posts = Vec::new();
    let posts_dir = Path::new("content/posts");

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

fn html_template(title: &str, content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="Exploring the intersection of sustainability, AI safety, and our collective future.">
    <title>{title} · Nature & Technology</title>
    <link rel="stylesheet" href="css/style.css">
    <link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🌿</text></svg>">
</head>
<body>
    <div class="container">
        <header>
            <h1 class="site-title"><a href="index.html">Nature & Technology</a></h1>
            <p class="site-tagline">Exploring sustainability and AI safety for a humane future</p>
            <nav aria-label="Main navigation">
                <a href="index.html">Home</a>
                <a href="about.html">About</a>
                <a href="blog.html">Writing</a>
            </nav>
        </header>
        <main>
{content}
        </main>
        <footer>
            <div class="footer-nature">
                <span class="leaf-icon">🌱</span>
                <span class="leaf-icon">🍃</span>
                <span class="leaf-icon">🌿</span>
            </div>
            <p>Crafted with care using Rust · Designed for sustainability</p>
            <p>Growing ideas for a thriving planet and aligned AI</p>
        </footer>
    </div>
</body>
</html>"#
    )
}

fn post_template(title: &str, content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="Exploring the intersection of sustainability, AI safety, and our collective future.">
    <title>{title} · Nature & Technology</title>
    <link rel="stylesheet" href="../css/style.css">
    <link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🌿</text></svg>">
</head>
<body>
    <div class="container">
        <header>
            <h1 class="site-title"><a href="../index.html">Nature & Technology</a></h1>
            <p class="site-tagline">Exploring sustainability and AI safety for a humane future</p>
            <nav aria-label="Main navigation">
                <a href="../index.html">Home</a>
                <a href="../about.html">About</a>
                <a href="../blog.html">Writing</a>
            </nav>
        </header>
        <main>
            <article>
{content}
            </article>
        </main>
        <footer>
            <div class="footer-nature">
                <span class="leaf-icon">🌱</span>
                <span class="leaf-icon">🍃</span>
                <span class="leaf-icon">🌿</span>
            </div>
            <p>Crafted with care using Rust · Designed for sustainability</p>
            <p>Growing ideas for a thriving planet and aligned AI</p>
        </footer>
    </div>
</body>
</html>"#
    )
}

fn generate_post_page(post: &Post, output_dir: &Path) {
    let content = format!(
        r#"            <h1>{}</h1>
            <p class="post-date">{}</p>
            {}
            <p class="post-author">Vincent Rizzo</p>"#,
        post.meta.title, post.meta.date, post.content_html
    );

    let html = post_template(&post.meta.title, &content);
    let path = output_dir.join("posts").join(format!("{}.html", post.slug));
    fs::write(path, html).expect("Failed to write post");
}

fn generate_blog_index(posts: &[Post], output_dir: &Path) {
    let mut list = String::from("        <h1>Writing</h1>\n        <p class=\"intro\">Reflections on sustainability, AI safety, and cultivating a humane technological future.</p>\n        <ul class=\"post-list\">\n");

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
                <a class="post-title" href="posts/{}.html">{}</a>
                {}
            </li>
"#,
            post.meta.date, post.slug, post.meta.title, desc
        ));
    }

    list.push_str("        </ul>");

    let html = html_template("Writing", &list);
    fs::write(output_dir.join("blog.html"), html).expect("Failed to write blog index");
}

fn generate_home_page(output_dir: &Path) {
    let content = r#"        <h1>Welcome</h1>
        <p class="intro">
            I explore the intersection of <strong>nature</strong>, <strong>technology</strong>, and our shared future.
            Like a garden, our technological landscape needs careful tending—nurturing what helps life flourish
            while being mindful of what we cultivate.
        </p>

        <div class="feature-highlight">
            <h2>The Wisdom of the Wheel</h2>
            <p>
                Consider the wheel and the bicycle—perhaps humanity's most elegant technologies.
                They don't demand more energy; they <strong>amplify what we already have</strong>.
                A cyclist moves four times faster than a walker while using the same effort.
                A cart with wheels lets one person move what would otherwise require many.
            </p>
            <p>
                These inventions didn't conquer nature—they worked with it. They extended our
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
                Both require us to think beyond immediate gains—to consider the seeds we plant today
                and the forests they might become tomorrow. Like the wheel and bicycle, the best
                technologies expand what's possible without demanding more than we can sustainably give.
            </p>
        </div>

        <p>
            Explore my <a href="blog.html">writing</a> for reflections on these themes,
            or learn more <a href="about.html">about my journey</a>.
        </p>"#;

    let html = html_template("Home", content);
    fs::write(output_dir.join("index.html"), html).expect("Failed to write home page");
}

fn generate_about_page(output_dir: &Path) {
    let content = r#"        <h1>About Me</h1>
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
                remains a beneficial presence—like a well-tended garden rather than an invasive species—becomes
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
                sustainability, AI safety, or the places where they intertwine—I'd love to hear from you.
            </p>
            <p>
                Find me on GitHub or reach out through social channels.
            </p>
        </div>"#;

    let html = html_template("About", content);
    fs::write(output_dir.join("about.html"), html).expect("Failed to write about page");
}
