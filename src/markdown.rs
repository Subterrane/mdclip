use pulldown_cmark::{html, CowStr, Event, Options, Parser};

const STYLE: &str = r#"<html><head><style>
body { font-family: -apple-system, sans-serif; font-size: 13px; }
h1 { margin-top: 20px; margin-bottom: 10px; }
h2 { margin-top: 16px; margin-bottom: 8px; }
h3 { margin-top: 14px; margin-bottom: 6px; }
p { margin-bottom: 8px; }
table { border-collapse: collapse; margin-bottom: 12px; }
td, th { border: 1px solid #ccc; padding: 4px 8px; }
code { background: #f4f4f4; padding: 1px 4px; border-radius: 3px; font-size: 12px; }
pre { background: #f4f4f4; padding: 8px; border-radius: 4px; margin-bottom: 8px; }
li { margin-bottom: 4px; }
</style></head><body>
"#;

const FOOTER: &str = "\n</body></html>";

pub fn to_styled_html(markdown: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(markdown, options);

    let mut body = String::new();
    html::push_html(&mut body, space_between_blocks(parser).into_iter());

    let mut out = String::with_capacity(STYLE.len() + body.len() + FOOTER.len());
    out.push_str(STYLE);
    out.push_str(&body);
    out.push_str(FOOTER);
    out
}

// Teams' RTF paste drops paragraph spacing (CSS margins become \sa/\sb,
// which it discards) on every block, so consecutive paragraphs, headings,
// lists and tables run together. Put a non-breaking-space paragraph between
// each pair of top-level blocks to force a visible gap. Nested blocks (list
// items, paragraphs inside a quote) are left alone.
fn space_between_blocks<'a>(events: impl Iterator<Item = Event<'a>>) -> Vec<Event<'a>> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut seen_block = false;
    for event in events {
        let starts_top_level_block = depth == 0 && matches!(event, Event::Start(_) | Event::Rule);
        if starts_top_level_block {
            if seen_block {
                out.push(Event::Html(CowStr::Borrowed("<p>&nbsp;</p>\n")));
            }
            seen_block = true;
        }
        match event {
            Event::Start(_) => depth += 1,
            Event::End(_) => depth = depth.saturating_sub(1),
            _ => {}
        }
        out.push(event);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_formatting() {
        let html = to_styled_html("**bold** and *italic*");
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
        assert!(html.contains("<style>"));
    }

    #[test]
    fn headings() {
        let html = to_styled_html("# H1\n## H2\n### H3");
        assert!(html.contains("<h1>H1</h1>"));
        assert!(html.contains("<h2>H2</h2>"));
        assert!(html.contains("<h3>H3</h3>"));
    }

    #[test]
    fn table() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = to_styled_html(md);
        assert!(html.contains("<table>"));
        assert!(html.contains("<td>1</td>"));
    }

    const SPACER: &str = "<p>&nbsp;</p>";

    #[test]
    fn spacer_between_paragraphs() {
        let html = to_styled_html("one\n\ntwo\n\nthree");
        assert_eq!(html.matches(SPACER).count(), 2);
        assert!(html.contains("<p>one</p>\n<p>&nbsp;</p>\n<p>two</p>"));
    }

    #[test]
    fn spacer_between_headings_and_text() {
        let html = to_styled_html("intro\n\n## A\n\ntext\n\n## B\n");
        assert_eq!(html.matches(SPACER).count(), 3);
    }

    #[test]
    fn no_leading_or_trailing_spacer() {
        let html = to_styled_html("# First\n\nbody\n");
        assert!(!html.contains(&format!("{SPACER}\n<h1")));
        assert!(!html.trim_end_matches(FOOTER).trim_end().ends_with(SPACER));
        assert_eq!(html.matches(SPACER).count(), 1);
    }

    #[test]
    fn single_block_has_no_spacer() {
        assert_eq!(to_styled_html("just one paragraph").matches(SPACER).count(), 0);
    }

    #[test]
    fn nested_blocks_not_spaced() {
        // list items and paragraphs inside a quote stay together
        assert_eq!(to_styled_html("- a\n- b\n- c").matches(SPACER).count(), 0);
        assert_eq!(to_styled_html("> one\n>\n> two").matches(SPACER).count(), 0);
        assert_eq!(to_styled_html("- a\n\n  more a\n\n- b").matches(SPACER).count(), 0);
    }

    #[test]
    fn spacer_around_tables_lists_code_and_rules() {
        let md = "intro\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n- x\n- y\n\n```\ncode\n```\n\n---\n\nend";
        // intro, table, list, code, rule, end = 6 top-level blocks
        assert_eq!(to_styled_html(md).matches(SPACER).count(), 5);
    }

    #[test]
    fn utf8_preserved() {
        let html = to_styled_html("café — naïve\n\n## Über\n");
        assert!(html.contains("café — naïve"));
        assert!(html.contains("Über"));
    }

    #[test]
    fn no_smart_quotes() {
        let html = to_styled_html(r#"He said "hello" -- world"#);
        // pulldown-cmark does not convert to smart quotes by default
        assert!(!html.contains('\u{201c}')); // no left double quote
        assert!(!html.contains('\u{2014}')); // no em dash
    }
}
