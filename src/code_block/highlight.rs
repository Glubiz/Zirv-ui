//! Code Highlighting Module
//!
//! This module provides functionality for basic syntax highlighting of code snippets.
//! It uses the pulldown_cmark library to parse markdown and generate HTML with syntax highlighting.

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};

/// Highlights code with basic syntax highlighting
///
/// This function takes a code snippet and a language identifier, wraps it in markdown-style
/// code fences, and then parses it to generate HTML with basic syntax highlighting.
///
/// # Arguments
///
/// * `code` - A string slice containing the code to be highlighted
/// * `language` - A string slice specifying the programming language of the code
///
/// # Returns
///
/// A `String` containing the HTML representation of the highlighted code
///
/// # Example
///
/// ```
/// let code = "fn main() {\n    println!(\"Hello, World!\");\n}";
/// let language = "rust";
/// let highlighted = highlight_code(code, language);
/// assert!(highlighted.contains("<pre><code class=\"language-rust\">"));
/// ```
pub fn highlight_code(code: &str, language: &str) -> String {
    let mut highlighted = String::new();
    // Wrap the code in markdown-style code fences
    let markdown = format!("```{}\n{}\n```", language, code);
    let parser = Parser::new(&markdown);

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                // Start of a code block - add opening tags with language class
                highlighted.push_str("<pre><code class=\"language-");
                highlighted.push_str(&lang);
                highlighted.push_str("\">");
            }
            Event::End(pulldown_cmark::TagEnd::CodeBlock) => {
                // End of a code block - add closing tags
                highlighted.push_str("</code></pre>");
            }
            Event::Text(text) => {
                // Basic syntax highlighting
                // This performs HTML entity escaping for special characters
                let highlighted_text = text
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace('"', "&quot;")
                    .replace('\'', "&#39;");
                highlighted.push_str(&highlighted_text);
            },
            _ => {}  // Ignore other markdown events
        }
    }

    highlighted
}
