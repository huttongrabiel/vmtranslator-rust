use crate::{Config, parser::{ParsedLine, CommandType}};

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

fn generate_push_assembly() -> String {

}

fn generate_pop_assembly() -> String {

}

fn arithmetic_asm_gen(command_type: &CommandType) -> String {

}
