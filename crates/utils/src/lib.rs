use std::ops::Not;

use models::ReadOnlyInstruction;

pub fn find_all_instructions_by_program_id(
    instructions: Vec<ReadOnlyInstruction>,
    program_id: &str,
) -> Vec<ReadOnlyInstruction> {
    let mut result = Vec::new();

    for instruction in instructions {
        let found = search(instruction, program_id);
        if let Some(found) = found {
            result.push(found);
        }
    }

    result
}

fn search(instruction: ReadOnlyInstruction, program_id: &str) -> Option<ReadOnlyInstruction> {
    if instruction.program_id.eq(program_id) {
        return Some(instruction);
    }

    if instruction.inner_instructions.is_empty().not() {
        instruction
            .inner_instructions
            .iter()
            .for_each(|inner_instruction| {
                search(inner_instruction.clone(), program_id);
            });
    }

    None
}
