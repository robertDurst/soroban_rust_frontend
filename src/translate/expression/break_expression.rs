use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use syn::ExprBreak;

pub fn handle_break_expression(
    _expr: &ExprBreak,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let mut instructions_to_return = vec![];

    instructions_to_return.push(Instruction::new(
        compilation_state.get_global_uuid(),
        "break".to_string(),
        vec![],
        "".to_string(),
        compilation_state.scope(),
    ));

    Ok(instructions_to_return)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_handle_break_expression() {
        let expr: ExprBreak = syn::parse_str("break").unwrap();
        let mut compilation_state = compilation_state::CompilationState::new();

        compilation_state.enter_new_scope(true);

        let instructions = handle_break_expression(&expr, &mut compilation_state).unwrap();
        assert_eq!(
            instructions,
            vec![Instruction::new(
                1,
                "break".to_string(),
                vec![],
                "".to_string(),
                compilation_state.scope(),
            ),]
        );
    }
}
