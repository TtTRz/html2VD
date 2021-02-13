use regex::Regex;

pub fn html_filter(html_source: String) -> String {
    let rg = Regex::new(r"[\n]+[\s]*").unwrap();
    let html_trim = html_source.trim();
    let target_html = rg.replace_all(html_trim, "");
    return target_html.into();
}
