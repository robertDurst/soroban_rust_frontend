use crate::{common::compilation_state::CompilationState, Instruction};

mod and_assign_elimination;
mod constant_propagation;
mod dead_code_elimination;
mod unreachable_code_elimination;

pub fn apply(
    instructions: Vec<Instruction>,
    compilation_state: CompilationState,
) -> Vec<Instruction> {
    let mut phase_0_optimized: Vec<Instruction> = vec![];

    let mut last_was_reference = false;
    let mut last_reference_instruction: Option<Instruction> = None;
    instructions.clone().into_iter().for_each(|instruction| {
        println!("Reference instruction: {:?}", instruction);

        if instruction.name == "reference" {
            if let Some(lri) = last_reference_instruction.clone() {
                let mut lri_to_push = lri.clone();
                lri_to_push.input.insert(0, format!("&"));
                lri_to_push.assign = instruction.assign.clone();

                phase_0_optimized.push(lri_to_push);
            } else {
                panic!(
                    "Reference instruction should be preceded by a reference assign instruction"
                );
            }

            last_was_reference = false;
            last_reference_instruction = None;
        } else if instruction.assign.starts_with("REFERENCE_EXPRESSION_") {
            last_was_reference = true;
            last_reference_instruction = Some(instruction);
        } else {
            last_was_reference = false;
            last_reference_instruction = None;

            phase_0_optimized.push(instruction);
        }
    });

    let phase_1_optimized =
        constant_propagation::apply(phase_0_optimized, compilation_state.scope_tree_root.clone());
    let phase_2_optimized: Vec<Instruction> =
        dead_code_elimination::apply(phase_1_optimized, compilation_state.scope_tree_root.clone());
    let phase_3_optimized = unreachable_code_elimination::apply(phase_2_optimized);
    let phase_4_optimized = and_assign_elimination::apply(phase_3_optimized);

    phase_4_optimized
        .into_iter()
        .map(move |mut instruction| {
            instruction.input = instruction
                .input
                .into_iter()
                .map(|input| input.replace("&.", "&"))
                .collect();

            instruction
        })
        .collect()
}

pub fn create_instruction(name: &str, input: Vec<&str>, assign: &str) -> Instruction {
    Instruction::new(
        0,
        name.to_string(),
        input.into_iter().map(|s| s.to_string()).collect(),
        assign.to_string(),
        0, // does not really matter here yet... but it might in the future
    )
}

pub fn create_instruction_with_scope(
    name: &str,
    input: Vec<&str>,
    assign: &str,
    scope: u128,
) -> Instruction {
    Instruction::new(
        0,
        name.to_string(),
        input.into_iter().map(|s| s.to_string()).collect(),
        assign.to_string(),
        scope, // does not really matter here yet... but it might in the future
    )
}

pub fn create_instruction_with_scope_and_id(
    name: &str,
    input: Vec<&str>,
    assign: &str,
    scope: u128,
    id: u128,
) -> Instruction {
    Instruction::new(
        id,
        name.to_string(),
        input.into_iter().map(|s| s.to_string()).collect(),
        assign.to_string(),
        scope, // does not really matter here yet... but it might in the future
    )
}
