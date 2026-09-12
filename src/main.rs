fn parse_heading(markdown: &str) -> String {
    let mut level = 0;

    if markdown.starts_with("### ") {
        level = 3;
    } else if markdown.starts_with("## ") {
        level = 2;
    } else if markdown.starts_with("# ") {
        level = 1;
    }

    if level == 0 {
        return markdown.to_string();
    }

    let text = &markdown[(level + 1) as usize..];

    format!("<h{}>{}</h{}>", level, text, level)
}

fn parse_bold(markdown: &str) -> String {
    if markdown.len() >= 5 && markdown.starts_with("**") && markdown.ends_with("**") {
        let text = &markdown[2..markdown.len() - 2];
        return format!("<strong>{}</strong>", text);
    }

    markdown.to_string()
}

fn parse_italic(markdown: &str) -> String {
    if markdown.len() >= 3
        && markdown.starts_with("*")
        && markdown.ends_with("*")
        && !markdown.starts_with("**")
    {
        let text = &markdown[1..markdown.len() - 1];
        return format!("<em>{}</em>", text);
    }

    markdown.to_string()
}

fn parse_bold_italic(markdown: &str) -> String {
    if markdown.len() >= 7 && markdown.starts_with("***") && markdown.ends_with("***") {
        let text = &markdown[3..markdown.len() - 3];
        return format!("<strong><em>{}</em></strong>", text);
    }

    markdown.to_string()
}

fn main() {
    println!("{}", parse_heading("# Hello"));
    println!("{}", parse_heading("## Rust"));
    println!("{}", parse_heading("### Parser"));
    println!("{}", parse_bold("**bold**"));
    print!("{}", parse_italic("*Italic*"));
    println!("{}", parse_bold_italic("***Bold and italic***"));
}
