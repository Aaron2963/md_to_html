use std::fs;

struct HtmlContent {
    title: String,
    body: String,
    style: String,
    script: String,
}

pub fn merge(title: &String, body: &String) -> String {
    let html = fs::read_to_string("asset/index.html").unwrap();
    let style = fs::read_to_string("asset/style.css").unwrap();
    let script = fs::read_to_string("asset/script.js").unwrap();
    let content = HtmlContent {
        title: title.clone(),
        body: body.clone(),
        style: style,
        script: script,
    };

    html.replace("{{title}}", content.title.as_str())
        .replace("{{style}}", content.style.as_str())
        .replace("{{body}}", content.body.as_str())
        .replace("{{script}}", content.script.as_str())
}
