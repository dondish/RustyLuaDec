use nom::{error::ErrorKind, IResult, Needed};

use super::{instruction_encodings::InstructionEncoding, opcodes::Opcode};

// All VM instructions
pub enum Instruction {
    Move(u8, u8),             /* A B     R[A] := R[B]                                    */
    LoadI(u8, i32),           /* A sBx   R[A] := sBx                                     */
    LoadF(u8, f64),           /* A sBx   R[A] := (lua_Number)sBx                         */
    LoadK(u8, u32),           /* A Bx    R[A] := K[Bx]                                   */
    LoadKx(u8),               /* A       R[A] := K[extra arg]                            */
    LoadFalse(u8),            /* A       R[A] := false                                   */
    LFalseSkip(u8),           /* A       R[A] := false; pc++     (*)                     */
    LoadTrue(u8),             /* A       R[A] := true                                    */
    LoadNil(u8, u8),          /* A B     R[A], R[A+1], ..., R[A+B] := nil                */
    GetUpval(u8, u8),         /* A B     R[A] := UpValue[B]                              */
    SetUpval(u8, u8),         /* A B     UpValue[B] := R[A]                              */
    GetTabup(u8, u8, u8),     /* A B C   R[A] := UpValue[B][K[C]:string]                 */
    GetTable(u8, u8, u8),     /* A B C   R[A] := R[B][R[C]]                              */
    GetI(u8, u8, u8),         /* A B C   R[A] := R[B][C]                                 */
    GetField(u8, u8, u8),     /* A B C   R[A] := R[B][K[C]:string]                       */
    SetTabup(u8, u8, u8),     /* A B C   UpValue[A][K[B]:string] := RK(C)                */
    SetTable(u8, u8, u8),     /* A B C   R[A][R[B]] := RK(C)                             */
    SetI(u8, u8, u8),         /* A B C   R[A][B] := RK(C)                                */
    SetField(u8, u8, u8),     /* A B C   R[A][K[B]:string] := RK(C)                      */
    NewTable(u8, u8, u8, u8), /* A B C k R[A] := {}                                      */
    Self_(u8, u8, u8),        /* A B C   R[A+1] := R[B]; R[A] := R[B][RK(C):string]      */
    AddI(u8, u8, i8),         /* A B sC  R[A] := R[B] + sC                               */
    AddK(u8, u8, u8),         /* A B C   R[A] := R[B] + K[C]:number                      */
    SubK(u8, u8, u8),         /* A B C   R[A] := R[B] - K[C]:number                      */
    MulK(u8, u8, u8),         /* A B C   R[A] := R[B] * K[C]:number                      */
    ModK(u8, u8, u8),         /* A B C   R[A] := R[B] % K[C]:number                      */
    PowK(u8, u8, u8),         /* A B C   R[A] := R[B] ^ K[C]:number                      */
    DivK(u8, u8, u8),         /* A B C   R[A] := R[B] / K[C]:number                      */
    IDivK(u8, u8, u8),        /* A B C   R[A] := R[B] // K[C]:number                     */
    BAndK(u8, u8, u8),        /* A B C   R[A] := R[B] & K[C]:integer                     */
    BOrK(u8, u8, u8),         /* A B C   R[A] := R[B] | K[C]:integer                     */
    BXorK(u8, u8, u8),        /* A B C   R[A] := R[B] ~ K[C]:integer                     */
    ShrI(u8, u8, i8),         /* A B sC  R[A] := R[B] >> sC                              */
    ShlI(u8, u8, i8),         /* A B sC  R[A] := sC << R[B]                              */
    Add(u8, u8, u8),          /* A B C   R[A] := R[B] + R[C]                             */
    Sub(u8, u8, u8),          /* A B C   R[A] := R[B] - R[C]                             */
    Mul(u8, u8, u8),          /* A B C   R[A] := R[B] * R[C]                             */
    Mod(u8, u8, u8),          /* A B C   R[A] := R[B] % R[C]                             */
    Pow(u8, u8, u8),          /* A B C   R[A] := R[B] ^ R[C]                             */
    Div(u8, u8, u8),          /* A B C   R[A] := R[B] / R[C]                             */
    IDiv(u8, u8, u8),         /* A B C   R[A] := R[B] // R[C]                            */
    BAnd(u8, u8, u8),         /* A B C   R[A] := R[B] & R[C]                             */
    BOr(u8, u8, u8),          /* A B C   R[A] := R[B] | R[C]                             */
    BXor(u8, u8, u8),         /* A B C   R[A] := R[B] ~ R[C]                             */
    Shl(u8, u8, u8),          /* A B C   R[A] := R[B] << R[C]                            */
    Shr(u8, u8, u8),          /* A B C   R[A] := R[B] >> R[C]                            */
    MmBin(u8, u8, u8),        /* A B C   call C metamethod over R[A] and R[B]    (*)     */
    MmBinI(u8, i8, u8, u8),   /* A sB C k        call C metamethod over R[A] and sB      */
    MmBinK(u8, u8, u8, u8),   /* A B C k         call C metamethod over R[A] and K[B]    */
    Unm(u8, u8, u8),          /* A B     R[A] := -R[B]                                   */
    BNot(u8, u8),             /* A B     R[A] := ~R[B]                                   */
    Not(u8, u8),              /* A B     R[A] := not R[B]                                */
    Len(u8, u8),              /* A B     R[A] := #R[B] (length operator)                 */
    Concat(u8, u8),           /* A B     R[A] := R[A].. ... ..R[A + B - 1]               */
    Close(u8),                /* A       close all upvalues >= R[A]                      */
    Tbc(u8),                  /* A       mark variable A "to be closed"                  */
    Jmp(u8, i32),             /* A J     pc += sJ                                        */
    Eq(u8, u8, u8),           /* A B k   if ((R[A] == R[B]) ~= k) then pc++              */
    Lt(u8, u8, u8),           /* A B k   if ((R[A] <  R[B]) ~= k) then pc++              */
    Le(u8, u8, u8),           /* A B k   if ((R[A] <= R[B]) ~= k) then pc++              */
    EqK(u8, u8, u8),          /* A B k   if ((R[A] == K[B]) ~= k) then pc++              */
    EqI(u8, i8, u8),          /* A sB k  if ((R[A] == sB) ~= k) then pc++                */
    LtI(u8, i8, u8),          /* A sB k  if ((R[A] < sB) ~= k) then pc++                 */
    LeI(u8, i8, u8),          /* A sB k  if ((R[A] <= sB) ~= k) then pc++                */
    GtI(u8, i8, u8),          /* A sB k  if ((R[A] > sB) ~= k) then pc++                 */
    GeI(u8, i8, u8),          /* A sB k  if ((R[A] >= sB) ~= k) then pc++                */
    Test(u8, u8),             /* A k     if (not R[A] == k) then pc++                    */
    TestSet(u8, u8, u8),      /* A B k   if (not R[B] == k) then pc++ else R[A] := R[B] (*) */
    Call(u8, u8, u8),         /* A B C   R[A], ... ,R[A+C-2] := R[A](R[A+1], ... ,R[A+B-1]) */
    TailCall(u8, u8, u8, u8), /* A B C k return R[A](R[A+1], ... ,R[A+B-1])              */
    Return(u8, u8, u8, u8),   /* A B C k return R[A], ... ,R[A+B-2]      (see note)      */
    Return0(),                /*         return                                          */
    Return1(u8),              /* A       return R[A]                                     */
    ForLoop(u8, u32),         /* A Bx    update counters; if loop continues then pc-=Bx; */
    ForPrep(u8, u32),         /* A Bx    <check values and prepare counters>;
                              if not to run then pc+=Bx+1;                        */
    TForPrep(u8, u32), /* A Bx    create upvalue for R[A + 3]; pc+=Bx             */
    TForCall(u8, u8),  /* A C     R[A+4], ... ,R[A+3+C] := R[A](R[A+1], R[A+2]);  */
    TForLoop(u8, u32), /* A Bx    if R[A+2] ~= nil then { R[A]=R[A+2]; pc -= Bx } */
    SetList(u8, u8, u8, u8), /* A B C k R[A][C+i] := R[A+i], 1 <= i <= B                */
    Closure(u8, u32),  /* A Bx    R[A] := closure(KPROTO[Bx])                     */
    Vararg(u8, u8),    /* A C     R[A], R[A+1], ..., R[A+C-2] = vararg            */
    VarargPrep(u8),    /* A        (adjust vararg parameters)                      */
    Extraarg(u32),     /* Ax      extra (larger) argument for previous opcode     */
}

/// A utility function to parse IABC encoded instructions
fn handle_iabc(
    input: &[u8],
    f: impl Fn(&[u8], u8, u8, u8, u8) -> IResult<&[u8], Instruction>,
) -> IResult<&[u8], Instruction> {
    let (next_input, instruction) = InstructionEncoding::parse_iabc(input)?;
    if let InstructionEncoding::IABC {
        c,
        b,
        k,
        a,
        opcode: _,
    } = instruction
    {
        f(next_input, c, b, k, a)
    } else {
        Err(nom::Err::Failure(nom::error::Error {
            input,
            code: ErrorKind::Fail,
        }))
    }
}

/// A utility function to parse IABx encoded instructions
fn handle_iabx(
    input: &[u8],
    f: impl Fn(&[u8], u32, u8) -> IResult<&[u8], Instruction>,
) -> IResult<&[u8], Instruction> {
    let (next_input, instruction) = InstructionEncoding::parse_iabx(input)?;
    if let InstructionEncoding::IABx { bx, a, opcode: _ } = instruction {
        f(next_input, bx, a)
    } else {
        Err(nom::Err::Failure(nom::error::Error {
            input,
            code: ErrorKind::Fail,
        }))
    }
}

/// A utility function to parse IAsBx encoded instructions
fn handle_iasbx(
    input: &[u8],
    f: impl Fn(&[u8], i32, u8) -> IResult<&[u8], Instruction>,
) -> IResult<&[u8], Instruction> {
    let (next_input, instruction) = InstructionEncoding::parse_iasbx(input)?;
    if let InstructionEncoding::IAsBx { sbx, a, opcode: _ } = instruction {
        f(next_input, sbx, a)
    } else {
        Err(nom::Err::Failure(nom::error::Error {
            input,
            code: ErrorKind::Fail,
        }))
    }
}

impl Instruction {
    /// Parses an instruction
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        if input.len() < 4 {
            return Err(nom::Err::Incomplete(Needed::new(4)));
        }
        let opc = num::FromPrimitive::from_u8(input[0] & 0x7f);
        match opc {
            Some(Opcode::Move) => handle_iabc(input, move |next_input, _, b, _, a| {
                Ok((next_input, Self::Move(a, b)))
            }),
            Some(Opcode::LoadI) => handle_iasbx(input, move |next_input, sbx, a| {
                Ok((next_input, Self::LoadI(a, sbx)))
            }),
            Some(Opcode::LoadF) => handle_iasbx(input, |next_input, sbx, a| {
                Ok((next_input, Self::LoadF(a, sbx as f64))) // LoadF is only for floats that are integers
            }),
            Some(Opcode::LoadK) => handle_iabx(input, |next_input, bx, a| {
                Ok((next_input, Self::LoadK(a, bx)))
            }),
            Some(Opcode::LoadKx) => {
                handle_iabx(input, |next_input, _, a| Ok((next_input, Self::LoadKx(a))))
            }
            Some(Opcode::LoadFalse) => handle_iabc(input, |next_input, _, _, _, a| {
                Ok((next_input, Self::LoadFalse(a)))
            }),
            Some(Opcode::LFalseSkip) => handle_iabc(input, |next_input, _, _, _, a| {
                Ok((next_input, Self::LFalseSkip(a)))
            }),
            Some(Opcode::LoadTrue) => handle_iabc(input, |next_input, _, _, _, a| {
                Ok((next_input, Self::LoadTrue(a)))
            }),
            Some(Opcode::LoadNil) => handle_iabc(input, |next_input, _, b, _, a| {
                Ok((next_input, Self::LoadNil(a, b)))
            }),
            Some(Opcode::GetUpval) => handle_iabc(input, |next_input, _, b, _, a| {
                Ok((next_input, Self::GetUpval(a, b)))
            }),
            Some(Opcode::SetUpval) => handle_iabc(input, |next_input, _, b, _, a| {
                Ok((next_input, Self::SetUpval(a, b)))
            }),
            Some(Opcode::GetTabup) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::GetTabup(a, b, c)))
            }),
            Some(Opcode::GetTable) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::GetTable(a, b, c)))
            }),
            Some(Opcode::GetI) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::GetI(a, b, c)))
            }),
            Some(Opcode::GetField) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::GetField(a, b, c)))
            }),
            Some(Opcode::SetTabup) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::SetTabup(a, b, c)))
            }),
            Some(Opcode::SetTable) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::SetTable(a, b, c)))
            }),
            Some(Opcode::SetI) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::SetI(a, b, c)))
            }),
            Some(Opcode::SetField) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::SetField(a, b, c)))
            }),
            Some(Opcode::NewTable) => handle_iabc(input, |next_input, c, b, k, a| {
                Ok((next_input, Self::NewTable(a, b, c, k)))
            }),
            Some(Opcode::Self_) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::Self_(a, b, c)))
            }),
            Some(Opcode::AddI) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::AddI(a, b, c as i8)))
            }),
            Some(Opcode::AddK) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::AddK(a, b, c)))
            }),
            Some(Opcode::SubK) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::SubK(a, b, c)))
            }),
            Some(Opcode::MulK) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::MulK(a, b, c)))
            }),
            Some(Opcode::ModK) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::ModK(a, b, c)))
            }),
            Some(Opcode::PowK) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::PowK(a, b, c)))
            }),
            Some(Opcode::DivK) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::DivK(a, b, c)))
            }),
            Some(Opcode::IDivK) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::IDivK(a, b, c)))
            }),
            Some(Opcode::BAndK) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::BAndK(a, b, c)))
            }),
            Some(Opcode::BOrK) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::BOrK(a, b, c)))
            }),
            Some(Opcode::BXorK) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::BXorK(a, b, c)))
            }),
            Some(Opcode::ShrI) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::ShrI(a, b, c as i8)))
            }),
            Some(Opcode::ShlI) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::ShlI(a, b, c as i8)))
            }),
            Some(Opcode::Add) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::Add(a, b, c)))
            }),
            Some(Opcode::Sub) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::Sub(a, b, c)))
            }),
            Some(Opcode::Mul) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::Mul(a, b, c)))
            }),
            Some(Opcode::Mod) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::Mod(a, b, c)))
            }),
            Some(Opcode::Pow) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::Pow(a, b, c)))
            }),
            Some(Opcode::Div) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::Div(a, b, c)))
            }),
            Some(Opcode::IDiv) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::IDiv(a, b, c)))
            }),
            Some(Opcode::BAnd) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::BAnd(a, b, c)))
            }),
            Some(Opcode::BOr) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::BOr(a, b, c)))
            }),
            Some(Opcode::BXor) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::BXor(a, b, c)))
            }),
            Some(Opcode::Shr) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::Shr(a, b, c)))
            }),
            Some(Opcode::Shl) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::Shl(a, b, c)))
            }),
            Some(Opcode::MmBin) => handle_iabc(input, |next_input, c, b, _, a| {
                Ok((next_input, Self::MmBin(a, b, c)))
            }),
            Some(Opcode::MmBinI) => handle_iabc(input, |next_input, c, b, k, a| {
                Ok((next_input, Self::MmBinI(a, b as i8, c, k)))
            }),
            Some(Opcode::MmBinK) => handle_iabc(input, |next_input, c, b, k, a| {
                Ok((next_input, Self::MmBinK(a, b, c, k)))
            }),
            _ => Err(nom::Err::Failure(nom::error::Error {
                input,
                code: ErrorKind::Fail,
            })),
        }
    }
}
