# vinri2z.github.io

A minimalist static site generator written in Rust, designed with sustainability in mind. This project powers a personal blog focused on AI safety and environmental sustainability.

## Overview

This static site generator converts Markdown posts into a lightweight, accessible website. It embodies the principles it discusses: efficient code, minimal resource usage, and thoughtful design.

**Live site:** [vinri2z.github.io](https://vinri2z.github.io)

## Features

- Markdown to HTML conversion with YAML frontmatter support
- Nature-inspired, responsive design
- Automatic deployment to GitHub Pages
- Zero JavaScript - pure HTML and CSS
- Accessible and print-friendly

## Project Structure

```
.
├── content/
│   └── posts/           # Markdown blog posts
├── src/
│   └── main.rs          # Site generator source
├── output/              # Generated site (gitignored)
├── .github/
│   └── workflows/
│       └── deploy.yml   # GitHub Actions deployment
├── Cargo.toml           # Rust dependencies
└── README.md
```

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)

## Usage

### Building the Site

```bash
# Build and run the generator
cargo run --release

# The site is generated in the output/ directory
```

### Local Development

```bash
# Build the site
cargo run --release

# Serve locally (using Python's built-in server)
cd output && python3 -m http.server 8000

# Visit http://localhost:8000
```

### Creating a New Post

Create a new Markdown file in `content/posts/` with YAML frontmatter:

```markdown
---
title: "Your Post Title"
date: "2025-01-25"
description: "A brief description for the post list."
---

Your post content here. Supports standard Markdown:

## Headings

**Bold**, *italic*, and `code` formatting.

- Bullet lists
- Work as expected

> Blockquotes too
```

The filename becomes the URL slug (e.g., `my-post.md` → `/posts/my-post.html`).

### Post Frontmatter

| Field | Required | Description |
|-------|----------|-------------|
| `title` | Yes | The post title displayed on the page |
| `date` | Yes | Publication date (YYYY-MM-DD format) |
| `description` | No | Short description shown in the post list |

## Deployment

The site automatically deploys to GitHub Pages when you push to the `main` branch.

### How It Works

1. GitHub Actions triggers on push to `main`
2. The workflow builds the Rust generator
3. Runs the generator to produce the `output/` directory
4. Deploys `output/` to GitHub Pages

### Manual Deployment

If you prefer manual deployment:

```bash
# Build the site
cargo run --release

# The output/ directory contains the complete static site
# Upload its contents to any static hosting service
```

## Extending the Generator

### Customizing the Design

The CSS is embedded in `src/main.rs` in the `generate_css()` function. Key design tokens are defined as CSS custom properties:

```css
:root {
    --bg-primary: #f8f6f3;      /* Background color */
    --accent-primary: #4a7c59;  /* Primary accent (green) */
    --font-serif: 'Cormorant Garamond', Georgia, serif;
    --font-sans: 'Inter', sans-serif;
    /* ... more variables */
}
```

### Adding New Pages

To add a static page:

1. Create a new function following the pattern of `generate_about_page()`:

```rust
fn generate_custom_page(output_dir: &Path) {
    let content = r#"        <h1>Page Title</h1>
        <p class="intro">Page introduction.</p>
        <!-- Your content -->"#;

    let html = html_template("Page Title", content);
    fs::write(output_dir.join("custom.html"), html)
        .expect("Failed to write custom page");
}
```

2. Call it from `main()`:

```rust
generate_custom_page(output_dir);
```

3. Add navigation in `html_template()`:

```rust
<nav>
    <a href="index.html">Home</a>
    <a href="custom.html">Custom</a>
    <!-- ... -->
</nav>
```

### Modifying Post Metadata

To add new frontmatter fields, update the `PostMeta` struct:

```rust
#[derive(Debug, Deserialize)]
struct PostMeta {
    title: String,
    date: String,
    description: Option<String>,
    tags: Option<Vec<String>>,  // New field
}
```

Then use the new field in `generate_blog_index()` or `generate_post_page()`.

### Adding Features

Common extensions you might want to add:

- **RSS feed**: Generate an XML file in `main()` after parsing posts
- **Tags/categories**: Add to `PostMeta` and create tag index pages
- **Syntax highlighting**: Integrate a library like `syntect`
- **Image optimization**: Process images during build

## Dependencies

| Crate | Purpose |
|-------|---------|
| `pulldown-cmark` | Markdown to HTML conversion |
| `walkdir` | Recursive directory traversal |
| `chrono` | Date handling |
| `serde` | Serialization framework |
| `serde_yaml` | YAML frontmatter parsing |

## Design Philosophy

This generator follows several principles:

1. **Simplicity**: Single Rust file, no complex build systems
2. **Efficiency**: Static HTML output, no client-side JavaScript
3. **Sustainability**: Minimal resource usage, lightweight pages
4. **Accessibility**: Semantic HTML, responsive design, print styles

## License

This project is open source. Feel free to use it as a starting point for your own site.

---

*Crafted with care using Rust*
