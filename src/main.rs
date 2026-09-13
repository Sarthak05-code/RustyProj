fn parse_heading(markdown: &str) -> Option<String> {
    let mut level = 0;

    if markdown.starts_with("###### ") {
        level = 6;
    } else if markdown.starts_with("##### ") {
        level = 5;
    } else if markdown.starts_with("#### ") {
        level = 4;
    } else if markdown.starts_with("### ") {
        level = 3;
    } else if markdown.starts_with("## ") {
        level = 2;
    } else if markdown.starts_with("# ") {
        level = 1;
    }

    if level == 0 {
        return None;
    }

    let text = markdown[level..].trim_start();
    let parsed_text = parse_inline(text);

    Some(format!("<h{}>{}</h{}>", level, parsed_text, level))
}

fn parse_paragraph(markdown: &str) -> String {
    let parsed_text = parse_inline(markdown.trim());
    format!("<p>{}</p>", parsed_text)
}

fn parse_inline(text: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    let mut is_bold_italic = false;
    let mut is_bold = false;
    let mut is_italic = false;

    while i < chars.len() {
        // ---------------- LINK ----------------
        if chars[i] == '[' {
            let mut close_bracket = i;

            while close_bracket < chars.len() && chars[close_bracket] != ']' {
                close_bracket += 1;
            }

            if close_bracket < chars.len()
                && close_bracket + 1 < chars.len()
                && chars[close_bracket + 1] == '('
            {
                let mut close_parenthesis = close_bracket + 2;

                while close_parenthesis < chars.len() && chars[close_parenthesis] != ')' {
                    close_parenthesis += 1;
                }

                if close_parenthesis < chars.len() {
                    let link_text: String = chars[i + 1..close_bracket].iter().collect();

                    let url: String = chars[close_bracket + 2..close_parenthesis].iter().collect();

                    let parsed_link_text = parse_inline(&link_text);

                    result.push_str(&format!("<a href=\"{}\">{}</a>", url, parsed_link_text));

                    i = close_parenthesis + 1;
                    continue;
                }
            }
        }

        // ---------------- BOLD + ITALIC ----------------
        if i + 2 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' && chars[i + 2] == '*' {
            if is_bold_italic {
                result.push_str("</em></strong>");
                is_bold_italic = false;
            } else {
                result.push_str("<strong><em>");
                is_bold_italic = true;
            }

            i += 3;

        // ---------------- BOLD ----------------
        } else if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            if is_bold {
                result.push_str("</strong>");
                is_bold = false;
            } else {
                result.push_str("<strong>");
                is_bold = true;
            }

            i += 2;

        // ---------------- ITALIC ----------------
        } else if chars[i] == '*' {
            if is_italic {
                result.push_str("</em>");
                is_italic = false;
            } else {
                result.push_str("<em>");
                is_italic = true;
            }

            i += 1;

        // ---------------- NORMAL CHARACTER ----------------
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}
fn parse_markdown(markdown: &str) -> String {
    let mut html = String::new();

    for line in markdown.lines() {
        if let Some(parsed_line) = parse_heading(line) {
            html.push_str(&parsed_line);
            html.push('\n');
        } else if line.trim().is_empty() {
            continue;
        } else {
            let parsed_line = parse_paragraph(line);
            html.push_str(&parsed_line);
            html.push('\n');
        }
    }

    html
}

fn main() {
    let input = r#"# Rust Markdown Test

This is **bold**, this is *italic*, and this is ***both***.

Visit [Rust](https://www.rust-lang.org) to learn **systems programming**.

## Another Heading

You can write [**bold links**](https://example.com) and *italic text* together.

This line has **bold [a link](https://example.com) inside it**.

### Final Test
"#;

    println!("{}", parse_markdown(input));
}
