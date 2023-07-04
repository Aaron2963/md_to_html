struct HtmlContent {
    title: String,
    body: String,
    style: String,
    script: String,
}

pub fn merge(title: &String, body: &String) -> String {
    let html = include_str!("../asset/index.html").to_string();
    let style = include_str!("../asset/style.css").to_string();
    let script = include_str!("../asset/script.js").to_string();
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
