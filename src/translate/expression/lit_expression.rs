use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_lit;

pub fn handle_lit_expression(
    lit: &syn::Lit,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    // TODO: fix this so it actually returns the right type
    // TODO: this is also super hacky for strings, please fix it
    let literal_value: String = parse_lit(lit)?;

    Ok(vec![Instruction::new(
        compilation_state.get_global_uuid(),
        "assign".to_string(),
        vec![literal_value],
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or_default(),
        compilation_state.scope(),
    )])
}

#[cfg(test)]
mod tests {
    use crate::common::compilation_state;
    use syn;

    mod lit_expression {
        use super::*;
        use compilation_state::CompilationState;
        use syn::{Lit, LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt};

        use crate::{instruction::Instruction, translate::expression::parse_expression};

        #[test]
        fn test_lit_expression_bool() {
            let parsed_lit_bool: LitBool = syn::parse_str("true").unwrap();
            let result = parse_expression(
                &syn::Expr::Lit(syn::ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Bool(parsed_lit_bool),
                }),
                &mut CompilationState::new(),
            );
            let expected: Vec<Instruction> = vec![Instruction::new(
                0,
                "assign".to_string(),
                vec!["true".to_string()],
                "".to_string(),
                0,
            )];

            assert_eq!(result, Ok(expected));
        }

        #[test]
        fn test_lit_expression_byte() {
            let parsed_lit_byte: LitByte = syn::parse_str("b'1'").unwrap();
            let result = parse_expression(
                &syn::Expr::Lit(syn::ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Byte(parsed_lit_byte),
                }),
                &mut CompilationState::new(),
            );
            let expected: Vec<Instruction> = vec![Instruction::new(
                0,
                "assign".to_string(),
                vec!["49".to_string()],
                "".to_string(),
                0,
            )];

            assert_eq!(result, Ok(expected));
        }

        #[test]
        fn test_lit_expression_byte_str() {
            let parsed_lit_byte_str: LitByteStr = syn::parse_str("b\"hello\"").unwrap();
            let result = parse_expression(
                &syn::Expr::Lit(syn::ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::ByteStr(parsed_lit_byte_str),
                }),
                &mut CompilationState::new(),
            );
            let expected: Vec<Instruction> = vec![Instruction::new(
                0,
                "assign".to_string(),
                vec!["\"hello\"".to_string()],
                "".to_string(),
                0,
            )];

            assert_eq!(result, Ok(expected));
        }

        #[test]
        fn test_lit_expression_char() {
            let parsed_lit_char: LitChar = syn::parse_str("'a'").unwrap();
            let result = parse_expression(
                &syn::Expr::Lit(syn::ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Char(parsed_lit_char),
                }),
                &mut CompilationState::new(),
            );

            let expected: Vec<Instruction> = vec![Instruction::new(
                0,
                "assign".to_string(),
                vec!["\'a\'".to_string()],
                "".to_string(),
                0,
            )];

            assert_eq!(result, Ok(expected));
        }

        #[test]
        fn test_lit_expression_float() {
            let parsed_lit_float: LitFloat = syn::parse_str("3.14").unwrap();
            let result = parse_expression(
                &syn::Expr::Lit(syn::ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Float(parsed_lit_float),
                }),
                &mut CompilationState::new(),
            );

            let expected: Vec<Instruction> = vec![Instruction::new(
                0,
                "assign".to_string(),
                vec!["3.14".to_string()],
                "".to_string(),
                0,
            )];

            assert_eq!(result, Ok(expected));
        }

        #[test]
        fn test_lit_expression_int() {
            let parsed_lit_int: LitInt = syn::parse_str("42").unwrap();
            let result = parse_expression(
                &syn::Expr::Lit(syn::ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Int(parsed_lit_int),
                }),
                &mut CompilationState::new(),
            );

            let expected: Vec<Instruction> = vec![Instruction::new(
                0,
                "assign".to_string(),
                vec!["42".to_string()],
                "".to_string(),
                0,
            )];

            assert_eq!(result, Ok(expected));
        }
    }
}
