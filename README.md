# pdf2webp

Convert PDF files to optimized WebP images — one image per page, preserving your folder structure.

## Install

```bash
curl -fsSL https://github.com/manithd/PDF2WebP/releases/latest/download/install.sh | sh
```

## Usage

```bash
# Interactive wizard (no flags needed)
pdf2webp

# One-shot for scripting
pdf2webp -s ./pdfs -o ./output -y

# All options
pdf2webp --help
```

### What it does

- Scans a folder recursively for PDFs
- Renders each page as a WebP image using poppler (pdftoppm)
- Mirrors your folder structure in the output
- Uses SSIMULACRA2 adaptive encoding to find the optimal quality per page
- Skips already-converted files by default

### Requirements

- **macOS**: [poppler](https://poppler.freedesktop.org/) (`brew install poppler`)
- **Windows**: poppler for Windows (included in [poppler-binaries](https://github.com/oschwartz10612/poppler-windows/releases/))
