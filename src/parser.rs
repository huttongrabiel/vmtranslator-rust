#[derive(Debug, PartialEq)]
pub enum CommandType {
    Push,
    Pop,
    Add,
    Subtract,
    Negate,
    Equals,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
    Other,
}

#[derive(Debug, PartialEq)]
pub enum MemorySegment {
    Constant,
    Local,
    Static,
    This,
    That,
    Argument,
    Temp,
    Pointer,
}

#[derive(Debug, PartialEq)]
pub enum ControlFlow {
    Goto,
    IfGoto,
    Label,
    Function,
    Call,
    Return,
}

#[derive(Debug, PartialEq)]
pub struct ParsedLine {
    pub command_type: Option<CommandType>,
    pub memory_segment: Option<MemorySegment>,
    pub control_flow: Option<ControlFlow>,
}

impl ParsedLine {
    pub fn parse(
        tokens: Vec<&str>
    ) -> Result<Self, &'static str> {
        let mut command_type: Option<CommandType> = None;
        let mut memory_segment: Option<MemorySegment> = None;
        let mut control_flow: Option<ControlFlow> = None;

        for &token in tokens[0..2].iter() {
            match token {
                "push" => command_type = Some(CommandType::Push),
                "pop"  => command_type = Some(CommandType::Pop),
                "add"  => command_type = Some(CommandType::Add),
                "sub"  => command_type = Some(CommandType::Subtract),
                "neg"  => command_type = Some(CommandType::Negate),
                "eq"   => command_type = Some(CommandType::Equals),
                "gt"   => command_type = Some(CommandType::GreaterThan),
                "lt"   => command_type = Some(CommandType::LessThan),
                "and"  => command_type = Some(CommandType::And),
                "or"   => command_type = Some(CommandType::Or),
                "not"  => command_type = Some(CommandType::Not),

                "constant" => memory_segment = Some(MemorySegment::Constant),
                "local"    => memory_segment = Some(MemorySegment::Local),
                "static"   => memory_segment = Some(MemorySegment::Static),
                "this"     => memory_segment = Some(MemorySegment::This),
                "that"     => memory_segment = Some(MemorySegment::That),
                "argument" => memory_segment = Some(MemorySegment::Argument),
                "temp"     => memory_segment = Some(MemorySegment::Temp),
                "pointer"  => memory_segment = Some(MemorySegment::Pointer),

                "goto"     => control_flow = Some(ControlFlow::Goto),
                "if-goto"  => control_flow = Some(ControlFlow::IfGoto),
                "label"    => control_flow = Some(ControlFlow::Label),
                "call"     => control_flow = Some(ControlFlow::Call),
                "return"   => control_flow = Some(ControlFlow::Return),

                _ => continue,
            }
        }

        Ok(ParsedLine {
            command_type,
            memory_segment,
            control_flow,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add test that checks each enum combination

    #[test]
    fn basic_parse() {
        let pl1 = ParsedLine {
            command_type: Some(CommandType::Push),
            memory_segment: Some(MemorySegment::Local),
            control_flow: None,
        };

        let instruction = vec!["push", "local", "8"];
        let parsed_line = match ParsedLine::parse(instruction) {
            Ok(pl) => pl,
            Err(e) => panic!("{e}"),
        };

        assert_eq!(pl1, parsed_line);
    }

    #[test]
    fn parse_goto() {
        let pl1 = ParsedLine {
            command_type: None,
            memory_segment: None,
            control_flow: Some(ControlFlow::Goto),
        };

        let instruction = vec!["goto", "IF_FALSE"];
        let parsed_line = match ParsedLine::parse(instruction) {
            Ok(pl) => pl,
            Err(e) => panic!("{e}"),
        };

        assert_eq!(pl1, parsed_line);
    }
}
