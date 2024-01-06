#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
/// Types of highlighted tokens
pub enum HighlightType {
    None,
    White,
    Red,
    Orange,
    Blue,
    Green,
    Purple,
    Yellow,
    Gray,
    Turquoise,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
/// Syntax highlighted text
pub struct HighlightedText {
    pub text: Vec<Vec<(HighlightType, String)>>,
}

pub trait LanguageHighlightTypeMapping {
    fn get_highlight_type(&self, token_kind: &str) -> HighlightType;
}

pub struct PythonMapping<'a> {
    mapping: Vec<(&'a str, HighlightType)>,
}

impl<'a> PythonMapping<'a> {
    pub fn new() -> Self {
        Self {
            mapping: vec![
                (".class_definition.argument_list.identifier", HighlightType::Yellow),
                (".function_definition.identifier", HighlightType::Blue),
                (".call.identifier", HighlightType::Blue),
                (".identifier", HighlightType::Red),
                (".class", HighlightType::Yellow),
                (".comment", HighlightType::Gray),
                (".import", HighlightType::Purple),
                (".string_content", HighlightType::Green),
            ],
        }
    }
}

impl<'a> LanguageHighlightTypeMapping for PythonMapping<'a> {
    fn get_highlight_type(&self, token_kind: &str) -> HighlightType {
        for (kind, highlight_type) in &self.mapping {
            if token_kind.ends_with(kind) {
                return highlight_type.to_owned();
            }
        }
        HighlightType::None
    }
}
