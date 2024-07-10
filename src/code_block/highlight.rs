use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};

pub fn highlight_code(code: &str, language: &str) -> String {
    let mut highlighted = String::new();
    let markdown = format!("```{}\n{}\n```", language, code);
    let parser = Parser::new(&markdown);

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                highlighted.push_str("<pre><code class=\"language-");
                highlighted.push_str(&lang);
                highlighted.push_str("\">");
            },
            Event::End(pulldown_cmark::TagEnd::CodeBlock) => {
                highlighted.push_str("</code></pre>");
            },
            Event::Text(text) => {
                // Basic syntax highlighting (you may want to expand this)
                let highlighted_text = text
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace('"',"&quot;")
                    .replace('\'', "&#39;");
                highlighted.push_str(&highlighted_text);
            },
            _ => {}
        }
    }

    highlighted
}