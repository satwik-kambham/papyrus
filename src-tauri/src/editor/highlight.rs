#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
/// Types of highlighted tokens
pub enum HighlightType {
    None,
    Variable,
    Constant,
    Literal,
    Function,
    Bracket,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
/// Syntax highlighted text
pub struct HighlightedText {
    pub text: Vec<Vec<(HighlightType, String)>>,
}
