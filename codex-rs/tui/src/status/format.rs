use ratatui::prelude::*;

#[derive(Debug, Clone)]
pub(crate) struct FieldFormatter;

impl FieldFormatter {
    pub(crate) fn line(&self, label: &str, value_spans: Vec<Span<'static>>) -> Line<'static> {
        let indent = if label.starts_with("  ") { "  " } else { "" };
        let label = escape_markdown(label.trim());
        let value = value_spans
            .into_iter()
            .map(|span| span.content.into_owned())
            .collect::<String>();
        Line::from(format!(
            "{indent}- **{label}:** {}",
            escape_markdown(&value)
        ))
    }

    pub(crate) fn continuation(&self, spans: Vec<Span<'static>>) -> Line<'static> {
        let value = spans
            .into_iter()
            .map(|span| span.content.into_owned())
            .collect::<String>();
        Line::from(format!("  {}", escape_markdown(&value)))
    }
}

fn escape_markdown(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '<' | '>' | '#' | '|' => {
                escaped.push('\\');
                escaped.push(character);
            }
            '\n' | '\r' => escaped.push(' '),
            _ => escaped.push(character),
        }
    }
    escaped
}
