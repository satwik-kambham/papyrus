#[derive(serde::Serialize, serde::Deserialize, Debug)]
/// Types of highlighted tokens
pub enum HighlightType {
    None,
    Variable,
    Constant,
    Literal,
    Function,
    Bracket,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
/// Syntax highlighted code
pub struct HighlightedCode {
    pub code: Vec<Vec<(HighlightType, String)>>,
}
