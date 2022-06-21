use crate::{Config, parser::{ParsedLine, CommandType, MemorySegment}};

pub fn codegen(config: &Config,
               parsed_line: &ParsedLine
) -> Result<String, &'static str> {

    let generated_hack_asm = match generate_hack_asm(&config, &parsed_line) {
        Ok(gha) => gha,
        Err(e) => return Err("Error during code generation: {e}"),
    };

    Ok(generated_hack_asm)
}

fn generate_hack_asm(config: &Config,
                     parsed_line: &ParsedLine
) -> Result<String, &'static str> {
    let mut generated_hack_asm = String::new();

    let command_type = match parsed_line.command_type {
        Some(str) => str,
        None => CommandType::Other,
    };

    match command_type {
        CommandType::Push => {
            let push_assembly = generate_push_assembly();
            generated_hack_asm
                .push_str(&push_assembly)
        },
        CommandType::Pop => {
            let pop_assembly = generate_pop_assembly();
            generated_hack_asm
                .push_str(&pop_assembly)
        },
        CommandType::Add => {
            let add_assembly = arithmetic_asm_gen(&CommandType::Add);
            generated_hack_asm
                .push_str(&add_assembly)
        },
        CommandType::Subtract => {
            let subtract_assembly = arithmetic_asm_gen(&CommandType::Subtract);
            generated_hack_asm
                .push_str(&subtract_assembly)
        },
        CommandType::Negate => {
            let negate_assembly = arithmetic_asm_gen(&CommandType::Negate);
            generated_hack_asm
                .push_str(&negate_assembly)
        },
        CommandType::Equals => {
            let equals_assembly = arithmetic_asm_gen(&CommandType::Equals);
            generated_hack_asm
                .push_str(&equals_assembly)
        },
        CommandType::GreaterThan => {
            let gt_assembly = arithmetic_asm_gen(&CommandType::GreaterThan);
            generated_hack_asm
                .push_str(&gt_assembly)
        },
        CommandType::LessThan => {
            let lt_assembly = arithmetic_asm_gen(&CommandType::LessThan);
            generated_hack_asm
                .push_str(&lt_assembly)
        },
        CommandType::And => {
            let and_assembly = arithmetic_asm_gen(&CommandType::And);
            generated_hack_asm
                .push_str(&and_assembly)
        },
        CommandType::Or => {
            let or_assembly = arithmetic_asm_gen(&CommandType::Or);
            generated_hack_asm
                .push_str(&or_assembly)
        },
        CommandType::Not => {
            let not_assembly = arithmetic_asm_gen(&CommandType::Not);
            generated_hack_asm
                .push_str(&not_assembly)
        },

        _ => generated_hack_asm.push_str(""),
    };

    Ok(generated_hack_asm)
}

// These should never fail so we are safe to just pass back Strings instead of
// results.
fn generate_push_assembly(parsed_line: &ParsedLine) -> String {
    let push_assembly = String::new();

    let memory_segment = match parsed_line.memory_segment {
        Some(seg) => seg,
        None => {
            panic!("There should be a memory segment here if you got this far!")
        },
    };

    if memory_segment != MemorySegment::Static {
        push_assembly.push_str(&format!("@ {} \n", 8));
    } else {
        push_assembly.push_str("@0\n");
    }

    push_assembly.push_str("D=A\n");

    match memory_segment {
        // I have no idea if the | does what I want it to do, but it looks right
        MemorySegment::Temp | MemorySegment::Static => {
            // FIXME: Add generate_memory_segment_label and proper struct for data.
            push_assembly.push_str();
            push_assembly.push_str("A=A+D\n");
            push_assembly.push_str("D=M\n");
        },
        MemorySegment::Pointer => {

        },
        _ => (),
    }

    push_assembly
}

fn generate_pop_assembly() -> String {

}

fn arithmetic_asm_gen(command_type: &CommandType) -> String {

}
