use crate::common::compilation_state;
use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, translate::expression::parse_expression,
};
use syn::ExprReference;

pub fn handle_reference_expression(
    expr_reference: &ExprReference,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let reference_to_reference_expression: String = format!(
        "REFERENCE_EXPRESSION_{}",
        compilation_state.get_global_uuid()
    );

    let reference_expression = Instruction::new(
        compilation_state.get_global_uuid(),
        "reference".to_string(),
        vec![reference_to_reference_expression.clone()],
        compilation_state
            .next_assignment
            .clone()
            .unwrap_or_default(),
        compilation_state.scope(),
    );

    let mut rest: Vec<Instruction> = parse_expression(
        &expr_reference.expr,
        compilation_state.with_assignment(Some(reference_to_reference_expression.clone())),
    )?;

    let instructions = vec![reference_expression];

    rest.extend(instructions);

    Ok(rest)
}
