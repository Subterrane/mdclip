use pulldown_cmark::{html, Options, Parser};

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
    html::push_html(&mut body, parser);

    let mut out = String::with_capacity(STYLE.len() + body.len() + FOOTER.len());
    out.push_str(STYLE);
    out.push_str(&body);
    out.push_str(FOOTER);
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

    #[test]
    fn no_smart_quotes() {
        let html = to_styled_html(r#"He said "hello" -- world"#);
        // pulldown-cmark does not convert to smart quotes by default
        assert!(!html.contains('\u{201c}')); // no left double quote
        assert!(!html.contains('\u{2014}')); // no em dash
    }
}
