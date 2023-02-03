// Opcodes in Lua VM
pub enum Opcode {
    Move,       /* A B     R[A] := R[B]                                    */
    LoadI,      /* A sBx   R[A] := sBx                                     */
    LoadF,      /* A sBx   R[A] := (lua_Number)sBx                         */
    LoadK,      /* A Bx    R[A] := K[Bx]                                   */
    LoadKx,     /* A       R[A] := K[extra arg]                            */
    LoadFalse,  /* A       R[A] := false                                   */
    LFalseSkip, /* A       R[A] := false; pc++     (*)                     */
    LoadTrue,   /* A       R[A] := true                                    */
    LoadNil,    /* A B     R[A], R[A+1], ..., R[A+B] := nil                */
    GetUpval,   /* A B     R[A] := UpValue[B]                              */
    SetUpval,   /* A B     UpValue[B] := R[A]                              */
    GetTabup,   /* A B C   R[A] := UpValue[B][K[C]:string]                 */
    GetTable,   /* A B C   R[A] := R[B][R[C]]                              */
    GetI,       /* A B C   R[A] := R[B][C]                                 */
    GetField,   /* A B C   R[A] := R[B][K[C]:string]                       */
    SetTabup,   /* A B C   UpValue[A][K[B]:string] := RK(C)                */
    SetTable,   /* A B C   R[A][R[B]] := RK(C)                             */
    SetI,       /* A B C   R[A][B] := RK(C)                                */
    SetField,   /* A B C   R[A][K[B]:string] := RK(C)                      */
    NewTable,   /* A B C k R[A] := {}                                      */
    Self_,      /* A B C   R[A+1] := R[B]; R[A] := R[B][RK(C):string]      */
    AddI,       /* A B sC  R[A] := R[B] + sC                               */
    AddK,       /* A B C   R[A] := R[B] + K[C]:number                      */
    SubK,       /* A B C   R[A] := R[B] - K[C]:number                      */
    MulK,       /* A B C   R[A] := R[B] * K[C]:number                      */
    ModK,       /* A B C   R[A] := R[B] % K[C]:number                      */
    PowK,       /* A B C   R[A] := R[B] ^ K[C]:number                      */
    DivK,       /* A B C   R[A] := R[B] / K[C]:number                      */
    IDivK,      /* A B C   R[A] := R[B] // K[C]:number                     */
    BAndK,      /* A B C   R[A] := R[B] & K[C]:integer                     */
    BOrK,       /* A B C   R[A] := R[B] | K[C]:integer                     */
    BXork,      /* A B C   R[A] := R[B] ~ K[C]:integer                     */
    ShrI,       /* A B sC  R[A] := R[B] >> sC                              */
    ShlI,       /* A B sC  R[A] := sC << R[B]                              */
    Add,        /* A B C   R[A] := R[B] + R[C]                             */
    Sub,        /* A B C   R[A] := R[B] - R[C]                             */
    Mul,        /* A B C   R[A] := R[B] * R[C]                             */
    Mod,        /* A B C   R[A] := R[B] % R[C]                             */
    Pow,        /* A B C   R[A] := R[B] ^ R[C]                             */
    Div,        /* A B C   R[A] := R[B] / R[C]                             */
    IDiv,       /* A B C   R[A] := R[B] // R[C]                            */
    BAnd,       /* A B C   R[A] := R[B] & R[C]                             */
    BOr,        /* A B C   R[A] := R[B] | R[C]                             */
    BXor,       /* A B C   R[A] := R[B] ~ R[C]                             */
    Shl,        /* A B C   R[A] := R[B] << R[C]                            */
    Shr,        /* A B C   R[A] := R[B] >> R[C]                            */
    MmBin,      /* A B C   call C metamethod over R[A] and R[B]    (*)     */
    MmBinI,     /* A sB C k        call C metamethod over R[A] and sB      */
    MmBinK,     /* A B C k         call C metamethod over R[A] and K[B]    */
    Unm,        /* A B     R[A] := -R[B]                                   */
    BNot,       /* A B     R[A] := ~R[B]                                   */
    Not,        /* A B     R[A] := not R[B]                                */
    Len,        /* A B     R[A] := #R[B] (length operator)                 */
    Concat,     /* A B     R[A] := R[A].. ... ..R[A + B - 1]               */
    Close,      /* A       close all upvalues >= R[A]                      */
    Tbc,        /* A       mark variable A "to be closed"                  */
    Jmp,        /* A J     pc += sJ                                        */
    Eq,         /* A B k   if ((R[A] == R[B]) ~= k) then pc++              */
    Lt,         /* A B k   if ((R[A] <  R[B]) ~= k) then pc++              */
    Le,         /* A B k   if ((R[A] <= R[B]) ~= k) then pc++              */
    EqK,        /* A B k   if ((R[A] == K[B]) ~= k) then pc++              */
    EqI,        /* A sB k  if ((R[A] == sB) ~= k) then pc++                */
    LtI,        /* A sB k  if ((R[A] < sB) ~= k) then pc++                 */
    LeI,        /* A sB k  if ((R[A] <= sB) ~= k) then pc++                */
    GtI,        /* A sB k  if ((R[A] > sB) ~= k) then pc++                 */
    GeI,        /* A sB k  if ((R[A] >= sB) ~= k) then pc++                */
    Test,       /* A k     if (not R[A] == k) then pc++                    */
    TestSet,    /* A B k   if (not R[B] == k) then pc++ else R[A] := R[B] (*) */
    Call,       /* A B C   R[A], ... ,R[A+C-2] := R[A](R[A+1], ... ,R[A+B-1]) */
    TailCall,   /* A B C k return R[A](R[A+1], ... ,R[A+B-1])              */
    Return,     /* A B C k return R[A], ... ,R[A+B-2]      (see note)      */
    Return0,    /*         return                                          */
    Return1,    /* A       return R[A]                                     */
    ForLoop,    /* A Bx    update counters; if loop continues then pc-=Bx; */
    ForPrep,    /* A Bx    <check values and prepare counters>;
                           if not to run then pc+=Bx+1;                    */
    TForPrep,   /* A Bx    create upvalue for R[A + 3]; pc+=Bx             */
    TForCall,   /* A C     R[A+4], ... ,R[A+3+C] := R[A](R[A+1], R[A+2]);  */
    TForLoop,   /* A Bx    if R[A+2] ~= nil then { R[A]=R[A+2]; pc -= Bx } */
    SetList,    /* A B C k R[A][C+i] := R[A+i], 1 <= i <= B                */
    Closure,    /* A Bx    R[A] := closure(KPROTO[Bx])                     */
    Vararg,     /* A C     R[A], R[A+1], ..., R[A+C-2] = vararg            */
    VarargPrep, /*         (adjust vararg parameters)                      */
    Extraarg,   /* Ax      extra (larger) argument for previous opcode     */
}
