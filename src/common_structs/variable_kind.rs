
#[derive(FromPrimitive, PartialEq, Debug)]
// Kinds of variables
pub enum VariableKind {
    Regular,
    Constant,
    ToBeClosed,
    CompileTimeConstant
}