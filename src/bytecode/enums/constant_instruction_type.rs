pub enum ConstantInstructionType {
    OpArgR,
    OpArgN,
    OpArgK,
    OpArgU,
}

pub static CONSTANT_INSTRUCTION_MAP: [(ConstantInstructionType, ConstantInstructionType); 38] = [
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgU,
        ConstantInstructionType::OpArgU,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgU,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgU,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgU,
        ConstantInstructionType::OpArgU,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgR,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgK,
        ConstantInstructionType::OpArgK,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgU,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgU,
    ),
    (
        ConstantInstructionType::OpArgU,
        ConstantInstructionType::OpArgU,
    ),
    (
        ConstantInstructionType::OpArgU,
        ConstantInstructionType::OpArgU,
    ),
    (
        ConstantInstructionType::OpArgU,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgR,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgN,
        ConstantInstructionType::OpArgU,
    ),
    (
        ConstantInstructionType::OpArgU,
        ConstantInstructionType::OpArgU,
    ),
    (
        ConstantInstructionType::OpArgN,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgU,
        ConstantInstructionType::OpArgN,
    ),
    (
        ConstantInstructionType::OpArgU,
        ConstantInstructionType::OpArgN,
    ),
];