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
                (
                    ".class_definition.argument_list.identifier",
                    HighlightType::Yellow,
                ),
                (".function_definition.identifier", HighlightType::Blue),
                (".call.identifier", HighlightType::Blue),
                (".identifier", HighlightType::White),
                (".;", HighlightType::White),
                (".import", HighlightType::Purple),
                ("..", HighlightType::White),
                (".from", HighlightType::Purple),
                (".__future__", HighlightType::White),
                (".(", HighlightType::Yellow),
                (".)", HighlightType::Yellow),
                (".,", HighlightType::White),
                (".as", HighlightType::Purple),
                (".*", HighlightType::Purple),
                (".print", HighlightType::Turquoise),
                (".>>", HighlightType::Purple),
                (".assert", HighlightType::Purple),
                (".:=", HighlightType::Purple),
                (".return", HighlightType::Purple),
                (".del", HighlightType::Purple),
                (".raise", HighlightType::Purple),
                (".pass", HighlightType::Purple),
                (".break", HighlightType::Purple),
                (".continue", HighlightType::Purple),
                (".if", HighlightType::Purple),
                (".:", HighlightType::Purple),
                (".elif", HighlightType::Purple),
                (".else", HighlightType::Purple),
                (".match", HighlightType::Purple),
                (".case", HighlightType::Purple),
                (".async", HighlightType::Purple),
                (".for", HighlightType::Purple),
                (".in", HighlightType::Purple),
                (".while", HighlightType::Purple),
                (".try", HighlightType::Purple),
                (".except", HighlightType::Purple),
                (".except*", HighlightType::Purple),
                (".finally", HighlightType::Purple),
                (".with", HighlightType::Purple),
                (".def", HighlightType::Purple),
                (".->", HighlightType::Purple),
                (".**", HighlightType::Purple),
                (".global", HighlightType::Purple),
                (".nonlocal", HighlightType::Purple),
                (".exec", HighlightType::Purple),
                (".type", HighlightType::Purple),
                (".=", HighlightType::Purple),
                (".class", HighlightType::Purple),
                (".[", HighlightType::Yellow),
                (".]", HighlightType::Yellow),
                (".@", HighlightType::Purple),
                (".-", HighlightType::Purple),
                ("._", HighlightType::White),
                (".|", HighlightType::Purple),
                (".{", HighlightType::Yellow),
                (".}", HighlightType::Yellow),
                (".+", HighlightType::Purple),
                (".not", HighlightType::Purple),
                (".and", HighlightType::Purple),
                (".or", HighlightType::Purple),
                ("./", HighlightType::Purple),
                (".%", HighlightType::Purple),
                (".//", HighlightType::Purple),
                (".&", HighlightType::Purple),
                (".^", HighlightType::Purple),
                (".<<", HighlightType::Purple),
                (".~", HighlightType::Purple),
                (".<", HighlightType::Purple),
                (".<=", HighlightType::Purple),
                (".==", HighlightType::Purple),
                (".!=", HighlightType::Purple),
                (".>=", HighlightType::Purple),
                (".>", HighlightType::Purple),
                (".<>", HighlightType::Purple),
                (".is", HighlightType::Purple),
                (".lambda", HighlightType::Purple),
                (".+=", HighlightType::Purple),
                (".-=", HighlightType::Purple),
                (".*=", HighlightType::Purple),
                ("./=", HighlightType::Purple),
                (".@=", HighlightType::Purple),
                (".//=", HighlightType::Purple),
                (".%=", HighlightType::Purple),
                (".**=", HighlightType::Purple),
                (".>>=", HighlightType::Purple),
                (".<<=", HighlightType::Purple),
                (".&=", HighlightType::Purple),
                (".^=", HighlightType::Purple),
                (".|=", HighlightType::Purple),
                (".yield", HighlightType::Purple),
                (".ellipsis", HighlightType::White),
                (".escape_sequence", HighlightType::Turquoise),
                ("._not_escape_sequence", HighlightType::Turquoise),
                (".format_specifier_token1", HighlightType::Turquoise),
                (".type_conversion", HighlightType::Turquoise),
                (".integer", HighlightType::Orange),
                (".float", HighlightType::Orange),
                (".await", HighlightType::Purple),
                (".true", HighlightType::Orange),
                (".false", HighlightType::Orange),
                (".none", HighlightType::Orange),
                (".comment", HighlightType::Gray),
                (".line_continuation", HighlightType::White),
                ("._newline", HighlightType::White),
                ("._indent", HighlightType::White),
                ("._dedent", HighlightType::White),
                (".string_start", HighlightType::Green),
                ("._string_content", HighlightType::Green),
                (".escape_interpolation", HighlightType::Turquoise),
                (".string_end", HighlightType::Green),
                (".module", HighlightType::White),
                ("._statement", HighlightType::White),
                ("._simple_statements", HighlightType::White),
                (".import_statement", HighlightType::White),
                (".import_prefix", HighlightType::White),
                (".relative_import", HighlightType::White),
                (".future_import_statement", HighlightType::White),
                (".import_from_statement", HighlightType::White),
                ("._import_list", HighlightType::White),
                (".aliased_import", HighlightType::White),
                (".wildcard_import", HighlightType::White),
                (".print_statement", HighlightType::White),
                (".chevron", HighlightType::Purple),
                (".assert_statement", HighlightType::White),
                (".expression_statement", HighlightType::White),
                (".named_expression", HighlightType::White),
                ("._named_expression_lhs", HighlightType::White),
                (".return_statement", HighlightType::White),
                (".delete_statement", HighlightType::White),
                (".raise_statement", HighlightType::White),
                (".pass_statement", HighlightType::White),
                (".break_statement", HighlightType::White),
                (".continue_statement", HighlightType::White),
                (".if_statement", HighlightType::White),
                (".elif_clause", HighlightType::White),
                (".else_clause", HighlightType::White),
                (".match_statement", HighlightType::White),
                (".block", HighlightType::White),
                (".case_clause", HighlightType::White),
                (".for_statement", HighlightType::White),
                (".while_statement", HighlightType::White),
                (".try_statement", HighlightType::White),
                (".except_clause", HighlightType::White),
                (".except_group_clause", HighlightType::White),
                (".finally_clause", HighlightType::White),
                (".with_statement", HighlightType::White),
                (".with_clause", HighlightType::White),
                (".with_item", HighlightType::White),
                (".function_definition", HighlightType::White),
                (".parameters", HighlightType::White),
                (".lambda_parameters", HighlightType::White),
                (".list_splat", HighlightType::White),
                (".dictionary_splat", HighlightType::White),
                (".global_statement", HighlightType::White),
                (".nonlocal_statement", HighlightType::White),
                (".exec_statement", HighlightType::White),
                (".type_alias_statement", HighlightType::White),
                (".class_definition", HighlightType::White),
                (".type_parameter", HighlightType::White),
                (".parenthesized_list_splat", HighlightType::White),
                (".argument_list", HighlightType::White),
                (".decorated_definition", HighlightType::White),
                (".decorator", HighlightType::White),
                (".block", HighlightType::White),
                (".expression_list", HighlightType::White),
                (".dotted_name", HighlightType::White),
                (".case_pattern", HighlightType::White),
                ("._simple_pattern", HighlightType::White),
                (".as_pattern", HighlightType::White),
                (".union_pattern", HighlightType::White),
                (".list_pattern", HighlightType::White),
                (".tuple_pattern", HighlightType::White),
                (".dict_pattern", HighlightType::White),
                ("._key_value_pattern", HighlightType::White),
                (".keyword_pattern", HighlightType::White),
                (".splat_pattern", HighlightType::White),
                (".class_pattern", HighlightType::White),
                (".complex_pattern", HighlightType::White),
                ("._parameters", HighlightType::White),
                ("._patterns", HighlightType::White),
                (".parameter", HighlightType::Red),
                (".pattern", HighlightType::Turquoise),
                (".tuple_pattern", HighlightType::White),
                (".list_pattern", HighlightType::White),
                (".default_parameter", HighlightType::White),
                (".typed_default_parameter", HighlightType::White),
                (".list_splat_pattern", HighlightType::White),
                (".dictionary_splat_pattern", HighlightType::White),
                (".as_pattern", HighlightType::White),
                ("._expression_within_for_in_clause", HighlightType::White),
                (".expression", HighlightType::White),
                (".primary_expression", HighlightType::White),
                (".not_operator", HighlightType::White),
                (".boolean_operator", HighlightType::White),
                (".binary_operator", HighlightType::White),
                (".unary_operator", HighlightType::White),
                (".comparison_operator", HighlightType::White),
                (".lambda", HighlightType::White),
                (".lambda", HighlightType::White),
                (".assignment", HighlightType::White),
                (".augmented_assignment", HighlightType::White),
                (".pattern_list", HighlightType::White),
                ("._right_hand_side", HighlightType::White),
                (".yield", HighlightType::Purple),
                (".attribute", HighlightType::Red),
                (".subscript", HighlightType::White),
                (".slice", HighlightType::White),
                (".call", HighlightType::Blue),
                (".typed_parameter", HighlightType::Red),
                (".type", HighlightType::Turquoise),
                (".splat_type", HighlightType::White),
                (".generic_type", HighlightType::White),
                (".union_type", HighlightType::White),
                (".constrained_typeconstrained_type", HighlightType::White),
                (".member_type", HighlightType::White),
                (".keyword_argument", HighlightType::Red),
                (".list", HighlightType::White),
                (".set", HighlightType::White),
                (".tuple", HighlightType::White),
                (".dictionary", HighlightType::White),
                (".pair", HighlightType::White),
                (".list_comprehension", HighlightType::White),
                (".dictionary_comprehension", HighlightType::White),
                (".set_comprehension", HighlightType::White),
                (".generator_expression", HighlightType::White),
                ("._comprehension_clauses", HighlightType::White),
                (".parenthesized_expression", HighlightType::White),
                ("._collection_elements", HighlightType::White),
                (".for_in_clause", HighlightType::White),
                (".if_clause", HighlightType::White),
                (".conditional_expression", HighlightType::White),
                (".concatenated_string", HighlightType::Green),
                (".string", HighlightType::Green),
                (".string_content", HighlightType::Green),
                (".interpolation", HighlightType::White),
                ("._f_expression", HighlightType::White),
                (".format_specifier", HighlightType::White),
                (".await", HighlightType::Purple),
                (".positional_separator", HighlightType::White),
                (".keyword_separator", HighlightType::White),
                (".module_repeat1", HighlightType::White),
                ("._simple_statements_repeat1", HighlightType::White),
                (".import_prefix_repeat1", HighlightType::White),
                ("._import_list_repeat1", HighlightType::White),
                (".print_statement_repeat1", HighlightType::White),
                (".assert_statement_repeat1", HighlightType::White),
                (".if_statement_repeat1", HighlightType::White),
                (".match_statement_repeat1", HighlightType::White),
                ("._match_block_repeat1", HighlightType::White),
                (".case_clause_repeat1", HighlightType::White),
                (".try_statement_repeat1", HighlightType::White),
                (".try_statement_repeat2", HighlightType::White),
                (".with_clause_repeat1", HighlightType::White),
                (".global_statement_repeat1", HighlightType::White),
                (".type_parameter_repeat1", HighlightType::White),
                (".argument_list_repeat1", HighlightType::White),
                (".decorated_definition_repeat1", HighlightType::White),
                (".dotted_name_repeat1", HighlightType::White),
                (".union_pattern_repeat1", HighlightType::White),
                (".dict_pattern_repeat1", HighlightType::White),
                ("._parameters_repeat1", HighlightType::White),
                ("._patterns_repeat1", HighlightType::White),
                (".comparison_operator_repeat1", HighlightType::White),
                (".subscript_repeat1", HighlightType::White),
                (".dictionary_repeat1", HighlightType::White),
                ("._comprehension_clauses_repeat1", HighlightType::White),
                ("._collection_elements_repeat1", HighlightType::White),
                (".for_in_clause_repeat1", HighlightType::White),
                (".concatenated_string_repeat1", HighlightType::Green),
                (".string_repeat1", HighlightType::Green),
                (".string_content_repeat1", HighlightType::Green),
                (".format_specifier_repeat1", HighlightType::White),
                (".as_pattern_target", HighlightType::White),
                (".format_expression", HighlightType::White),
                (".is not", HighlightType::Purple),
                (".not in", HighlightType::Purple),
                (".Unknown", HighlightType::White),
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
