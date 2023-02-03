use nom::{error::ErrorKind, IResult, Needed};

use super::{instruction_encodings::InstructionEncoding, opcodes::Opcode};

// All VM instructions
pub enum Instruction {
    Move(u8, u8),             /* A B     R[A] := R[B]                                    */
    LoadI(u8, i32),           /* A sBx   R[A] := sBx                                     */
    LoadF(u8, f32),           /* A sBx   R[A] := (lua_Number)sBx                         */
    LoadK(u8, u32),           /* A Bx    R[A] := K[Bx]                                   */
    LoadKx(u8, u64),          /* A       R[A] := K[extra arg]                            */
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
    BXork(u8, u8, u8),        /* A B C   R[A] := R[B] ~ K[C]:integer                     */
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
                              if not to run then pc+=Bx+1;                    */
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
            _ => Err(nom::Err::Failure(nom::error::Error {
                input,
                code: ErrorKind::Fail,
            })),
        }
    }
}
