#[derive(Debug, PartialEq, Clone)]
pub enum Inline {
    Text(String),
    Bold(String),
    Italic(String),
    BoldItalic(String),
    Code(String),
    Link { text: String, url: String },
    Image { alt: String, src: String },
}

#[derive(Debug, PartialEq)]
pub enum Block {
    Heading { level: usize, content: Vec<Inline> },
    Paragraph(Vec<Inline>),
    UnorderedList(Vec<Vec<Inline>>),
    OrderedList(Vec<Vec<Inline>>),
    CodeBlock(String),
    Blockquote(Vec<Inline>),
    HorizontalRule,
}

fn parse_inline_ast(text: &str) -> Vec<Inline> {
    let mut nodes = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // ---------------- IMAGES ![alt](url) ----------------
        if i + 1 < chars.len() && chars[i] == '!' && chars[i + 1] == '[' {
            if let Some(close_bracket) = text[i..].find(']') {
                let close_bracket_idx = i + close_bracket;

                if close_bracket_idx + 1 < chars.len() && chars[close_bracket_idx + 1] == '(' {
                    if let Some(close_paren) = text[close_bracket_idx + 1..].find(')') {
                        let close_paren_idx = close_bracket_idx + 1 + close_paren;

                        let alt = text[i + 2..close_bracket_idx].to_string();
                        let src = text[close_bracket_idx + 2..close_paren_idx].to_string();

                        nodes.push(Inline::Image { alt, src });
                        i = close_paren_idx + 1;
                        continue;
                    }
                }
            }
        }

        // ---------------- LINK [text](url) ----------------
        if chars[i] == '[' {
            if let Some(close_bracket) = text[i..].find(']') {
                let close_bracket_idx = i + close_bracket;

                if close_bracket_idx + 1 < chars.len() && chars[close_bracket_idx + 1] == '(' {
                    if let Some(close_paren) = text[close_bracket_idx + 1..].find(')') {
                        let close_paren_idx = close_bracket_idx + 1 + close_paren;

                        let link_text = text[i + 1..close_bracket_idx].to_string();
                        let url = text[close_bracket_idx + 2..close_paren_idx].to_string();

                        nodes.push(Inline::Link {
                            text: link_text,
                            url,
                        });
                        i = close_paren_idx + 1;
                        continue;
                    }
                }
            }
        }

        // ---------------- INLINE CODE `code` ----------------
        if chars[i] == '`' {
            if let Some(close_backtick) = text[i + 1..].find('`') {
                let code_content = text[i + 1..i + 1 + close_backtick].to_string();
                nodes.push(Inline::Code(code_content));
                i = i + 2 + close_backtick;
                continue;
            }
        }

        // ---------------- BOLD + ITALIC ----------------
        if i + 2 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' && chars[i + 2] == '*' {
            // Simple slice scanner to find end of ***
            let rest = &text[i + 3..];
            if let Some(end) = rest.find("***") {
                nodes.push(Inline::BoldItalic(rest[..end].to_string()));
                i += 6 + end;
                continue;
            }
        }

        // ---------------- BOLD ----------------
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            let rest = &text[i + 2..];
            if let Some(end) = rest.find("**") {
                nodes.push(Inline::Bold(rest[..end].to_string()));
                i += 4 + end;
                continue;
            }
        }

        // ---------------- ITALIC ----------------
        if chars[i] == '*' {
            let rest = &text[i + 1..];
            if let Some(end) = rest.find('*') {
                nodes.push(Inline::Italic(rest[..end].to_string()));
                i += 2 + end;
                continue;
            }
        }

        // ---------------- PLAIN TEXT ----------------
        // Accumulate contiguous normal characters into a single Text node
        let mut text_buf = String::new();
        while i < chars.len()
            && chars[i] != '['
            && chars[i] != '!'
            && chars[i] != '`'
            && chars[i] != '*'
        {
            text_buf.push(chars[i]);
            i += 1;
        }
        if !text_buf.is_empty() {
            nodes.push(Inline::Text(text_buf));
        }
    }

    nodes
}

fn parse_heading_block(line: &str) -> Option<Block> {
    let mut level = 0;
    if line.starts_with("###### ") {
        level = 6;
    } else if line.starts_with("##### ") {
        level = 5;
    } else if line.starts_with("#### ") {
        level = 4;
    } else if line.starts_with("### ") {
        level = 3;
    } else if line.starts_with("## ") {
        level = 2;
    } else if line.starts_with("# ") {
        level = 1;
    }

    if level == 0 {
        return None;
    }

    let text = line[level..].trim_start();
    Some(Block::Heading {
        level,
        content: parse_inline_ast(text),
    })
}

pub fn parse_markdown_to_ast(markdown: &str) -> Vec<Block> {
    let mut ast = Vec::new();
    let mut current_unordered: Vec<Vec<Inline>> = Vec::new();
    let mut current_ordered: Vec<Vec<Inline>> = Vec::new();
    let mut in_code_block = false;
    let mut code_buffer = String::new();

    for line in markdown.lines() {
        let trimmed = line.trim();

        // 1. Code block toggling
        if trimmed.starts_with("```") {
            if in_code_block {
                ast.push(Block::CodeBlock(code_buffer.clone()));
                code_buffer.clear();
                in_code_block = false;
            } else {
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            code_buffer.push_str(line);
            code_buffer.push('\n');
            continue;
        }

        // Check list items
        let unordered_item = if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            Some(&trimmed[2..])
        } else {
            None
        };

        let ordered_item = parse_ordered_list_item(line);

        // 2. Flush active lists if non-list item is reached
        if unordered_item.is_none() && !current_unordered.is_empty() {
            ast.push(Block::UnorderedList(current_unordered.clone()));
            current_unordered.clear();
        }
        if ordered_item.is_none() && !current_ordered.is_empty() {
            ast.push(Block::OrderedList(current_ordered.clone()));
            current_ordered.clear();
        }

        // 3. Process line types into AST Blocks
        if trimmed == "---" || trimmed == "***" {
            ast.push(Block::HorizontalRule);
        } else if trimmed.starts_with("> ") {
            ast.push(Block::Blockquote(parse_inline_ast(&trimmed[2..])));
        } else if let Some(item_text) = unordered_item {
            current_unordered.push(parse_inline_ast(item_text));
        } else if let Some(item_text) = ordered_item {
            current_ordered.push(parse_inline_ast(item_text));
        } else if let Some(heading_block) = parse_heading_block(line) {
            ast.push(heading_block);
        } else if trimmed.is_empty() {
            continue;
        } else {
            ast.push(Block::Paragraph(parse_inline_ast(trimmed)));
        }
    }

    // Flush remaining open lists at document end
    if !current_unordered.is_empty() {
        ast.push(Block::UnorderedList(current_unordered));
    }
    if !current_ordered.is_empty() {
        ast.push(Block::OrderedList(current_ordered));
    }

    ast
}

fn parse_ordered_list_item(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    let mut number_end = 0;
    for ch in trimmed.chars() {
        if ch.is_ascii_digit() {
            number_end += 1;
        } else {
            break;
        }
    }
    if number_end > 0 && trimmed[number_end..].starts_with(". ") {
        Some(&trimmed[number_end + 2..])
    } else {
        None
    }
}

fn render_inline_to_html(nodes: &[Inline]) -> String {
    let mut html = String::new();
    for node in nodes {
        match node {
            Inline::Text(t) => html.push_str(t),
            Inline::Bold(t) => html.push_str(&format!("<strong>{}</strong>", t)),
            Inline::Italic(t) => html.push_str(&format!("<em>{}</em>", t)),
            Inline::BoldItalic(t) => html.push_str(&format!("<strong><em>{}</em></strong>", t)),
            Inline::Code(t) => html.push_str(&format!("<code>{}</code>", t)),
            Inline::Link { text, url } => {
                html.push_str(&format!("<a href=\"{}\">{}</a>", url, text))
            }
            Inline::Image { alt, src } => {
                html.push_str(&format!("<img src=\"{}\" alt=\"{}\">", src, alt))
            }
        }
    }
    html
}

pub fn render_ast_to_html(ast: &[Block]) -> String {
    let mut html = String::new();

    for block in ast {
        match block {
            Block::Heading { level, content } => {
                let inner = render_inline_to_html(content);
                html.push_str(&format!("<h{}>{}</h{}>\n", level, inner, level));
            }
            Block::Paragraph(content) => {
                let inner = render_inline_to_html(content);
                html.push_str(&format!("<p>{}</p>\n", inner));
            }
            Block::UnorderedList(items) => {
                html.push_str("<ul>\n");
                for item in items {
                    let inner = render_inline_to_html(item);
                    html.push_str(&format!("  <li>{}</li>\n", inner));
                }
                html.push_str("</ul>\n");
            }
            Block::OrderedList(items) => {
                html.push_str("<ol>\n");
                for item in items {
                    let inner = render_inline_to_html(item);
                    html.push_str(&format!("  <li>{}</li>\n", inner));
                }
                html.push_str("</ol>\n");
            }
            Block::CodeBlock(code) => {
                html.push_str(&format!("<pre><code>{}</code></pre>\n", code));
            }
            Block::Blockquote(content) => {
                let inner = render_inline_to_html(content);
                html.push_str(&format!("<blockquote>{}</blockquote>\n", inner));
            }
            Block::HorizontalRule => {
                html.push_str("<hr>\n");
            }
        }
    }

    html
}

fn main() {
    let markdown_input = r#"# AST Markdown Parser

This is a **bold** statement using an `AST`.

- Item 1 with *italics*
- Item 2 with a [Link](https://rust-lang.org)

> Quotation block
"#;

    // 1. Build AST
    let ast = parse_markdown_to_ast(markdown_input);
    println!("--- GENERATED AST ---");
    println!("{:#?}\n", ast);

    // 2. Render to HTML
    let html_output = render_ast_to_html(&ast);
    println!("--- GENERATED HTML ---");
    println!("{}", html_output);
}
