# mdclip

Convert Markdown to rich text and copy it to the macOS clipboard.

Paste into Teams, Slack, Notes, Outlook — anywhere that accepts rich text.

## Install

```
cargo install --path .
```

Requires macOS (uses `textutil` and `NSPasteboard`).

## Usage

```
mdclip file.md              # from a file
echo "**bold**" | mdclip    # from stdin
```

## How it works

```
Markdown  →  pulldown-cmark (HTML)  →  textutil (RTF)  →  NSPasteboard (clipboard)
```

1. Parses Markdown to styled HTML using [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) with a `<style>` block for spacing and formatting
2. Converts HTML to RTF via macOS `textutil` (a system binary backed by WebKit)
3. Places RTF on the clipboard via direct `NSPasteboard` access

## Supported formatting

- **Bold**, *italic*, ~~strikethrough~~
- Headings (h1–h3)
- Bullet and numbered lists, task lists
- Tables with borders
- Inline `code` and fenced code blocks
- Links
