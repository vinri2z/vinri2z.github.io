# Personal Website

A simple, elegant static site generator written in Rust. Designed for GitHub Pages hosting.

## Features

- Static HTML generation from Markdown blog posts
- YAML frontmatter for post metadata
- Clean, minimal CSS design
- No JavaScript required
- Focus on sustainability and AI safety topics

## Usage

### Building the site

```bash
cargo build --release
cargo run --release
```

The generated site will be in the `output/` directory.

### Adding blog posts

Create Markdown files in `content/posts/` with YAML frontmatter:

```markdown
---
title: "Your Post Title"
date: "2025-01-20"
description: "A brief description of your post."
---

Your post content here in Markdown...
```

### Local preview

You can preview the site locally using any static file server:

```bash
cd output
python -m http.server 8000
```

Then open http://localhost:8000

### Deploying to GitHub Pages

1. Push this repository to GitHub
2. Go to Settings > Pages
3. Set Source to "GitHub Actions"
4. The site will auto-deploy on each push to main

## Structure

```
personal-website/
├── src/main.rs           # Site generator
├── content/posts/        # Markdown blog posts
├── output/              # Generated HTML (gitignored)
├── .github/workflows/   # GitHub Actions for deployment
└── Cargo.toml
```

## License

MIT
