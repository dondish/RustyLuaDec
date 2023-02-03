// Opcodes in Lua VM
pub enum Opcode {
    Move,       /*  B     R[A] := R[B]                                    */
    LoadI,      /*  sBx   R[A] := sBx                                     */
    LoadF,      /*  sBx   R[A] := (lua_Number)sBx                         */
    LoadK,      /*  Bx    R[A] := K[Bx]                                   */
    LoadKx,     /*        R[A] := K[extra arg]                            */
    LoadFalse,  /*        R[A] := false                                   */
    LFalseSkip, /*        R[A] := false; pc++     (*)                     */
    LoadTrue,   /*        R[A] := true                                    */
    LoadNil,    /*  B     R[A], R[A+1], ..., R[A+B] := nil                */
    GetUpval,   /*  B     R[A] := UpValue[B]                              */
    SetUpval,   /*  B     UpValue[B] := R[A]                              */
    GetTabup,   /*  B C   R[A] := UpValue[B][K[C]:string]                 */
    GetTable,   /*  B C   R[A] := R[B][R[C]]                              */
    GetI,       /*  B C   R[A] := R[B][C]                                 */
    GetField,   /*  B C   R[A] := R[B][K[C]:string]                       */
    SetTabup,   /*  B C   UpValue[A][K[B]:string] := RK(C)                */
    SetTable,   /*  B C   R[A][R[B]] := RK(C)                             */
    SetI,       /*  B C   R[A][B] := RK(C)                                */
    SetField,   /*  B C   R[A][K[B]:string] := RK(C)                      */
    NewTable,   /*  B C k R[A] := {}                                      */
    Self_,      /*  B C   R[A+1] := R[B]; R[A] := R[B][RK(C):string]      */
    AddI,       /*  B sC  R[A] := R[B] + sC                               */
    AddK,       /*  B C   R[A] := R[B] + K[C]:number                      */
    SubK,       /*  B C   R[A] := R[B] - K[C]:number                      */
    MulK,       /*  B C   R[A] := R[B] * K[C]:number                      */
    ModK,       /*  B C   R[A] := R[B] % K[C]:number                      */
    PowK,       /*  B C   R[A] := R[B] ^ K[C]:number                      */
    DivK,       /*  B C   R[A] := R[B] / K[C]:number                      */
    IDivK,      /*  B C   R[A] := R[B] // K[C]:number                     */
    BAndK,      /*  B C   R[A] := R[B] & K[C]:integer                     */
    BOrK,       /*  B C   R[A] := R[B] | K[C]:integer                     */
    BXork,      /*  B C   R[A] := R[B] ~ K[C]:integer                     */
    ShrI,       /*  B sC  R[A] := R[B] >> sC                              */
    ShlI,       /*  B sC  R[A] := sC << R[B]                              */
    Add,        /*  B C   R[A] := R[B] + R[C]                             */
    Sub,        /*  B C   R[A] := R[B] - R[C]                             */
    Mul,        /*  B C   R[A] := R[B] * R[C]                             */
    Mod,        /*  B C   R[A] := R[B] % R[C]                             */
    Pow,        /*  B C   R[A] := R[B] ^ R[C]                             */
    Div,        /*  B C   R[A] := R[B] / R[C]                             */
    IDiv,       /*  B C   R[A] := R[B] // R[C]                            */
    BAnd,       /*  B C   R[A] := R[B] & R[C]                             */
    BOr,        /*  B C   R[A] := R[B] | R[C]                             */
    BXor,       /*  B C   R[A] := R[B] ~ R[C]                             */
    Shl,        /*  B C   R[A] := R[B] << R[C]                            */
    Shr,        /*  B C   R[A] := R[B] >> R[C]                            */
    MmBin,      /*  B C   call C metamethod over R[A] and R[B]    (*)     */
    MmBinI,     /*  sB C k        call C metamethod over R[A] and sB      */
    MmBiNK,     /*  B C k         call C metamethod over R[A] and K[B]    */
    Unm,        /*  B     R[A] := -R[B]                                   */
    BNot,       /*  B     R[A] := ~R[B]                                   */
    Not,        /*  B     R[A] := not R[B]                                */
    Len,        /*  B     R[A] := #R[B] (length operator)                 */
    Concat,     /*  B     R[A] := R[A].. ... ..R[A + B - 1]               */
    Close,      /*        close all upvalues >= R[A]                      */
    Tbc,        /*        mark variable A "to be closed"                  */
    Jmp,        /*  J     pc += sJ                                        */
    Eq,         /*  B k   if ((R[A] == R[B]) ~= k) then pc++              */
    Lt,         /*  B k   if ((R[A] <  R[B]) ~= k) then pc++              */
    Le,         /*  B k   if ((R[A] <= R[B]) ~= k) then pc++              */
    EqK,        /*  B k   if ((R[A] == K[B]) ~= k) then pc++              */
    EqI,        /*  sB k  if ((R[A] == sB) ~= k) then pc++                */
    LtI,        /*  sB k  if ((R[A] < sB) ~= k) then pc++                 */
    LeI,        /*  sB k  if ((R[A] <= sB) ~= k) then pc++                */
    GtI,        /*  sB k  if ((R[A] > sB) ~= k) then pc++                 */
    GeI,        /*  sB k  if ((R[A] >= sB) ~= k) then pc++                */
    Test,       /*  k     if (not R[A] == k) then pc++                    */
    TestSet,    /*  B k   if (not R[B] == k) then pc++ else R[A] := R[B] (*) */
    Call,       /*  B C   R[A], ... ,R[A+C-2] := R[A](R[A+1], ... ,R[A+B-1]) */
    TailCall,   /*  B C k return R[A](R[A+1], ... ,R[A+B-1])              */
    Return,     /*  B C k return R[A], ... ,R[A+B-2]      (see note)      */
    Return0,    /*        return                                          */
    Return1,    /*        return R[A]                                     */
    ForLoop,    /*  Bx    update counters; if loop continues then pc-=Bx; */
    ForPrep,    /*  Bx    <check values and prepare counters>;
                          if not to run then pc+=Bx+1;                    */
    TForPrep,   /*  Bx    create upvalue for R[A + 3]; pc+=Bx             */
    TForCall,   /*  C     R[A+4], ... ,R[A+3+C] := R[A](R[A+1], R[A+2]);  */
    TForLoop,   /*  Bx    if R[A+2] ~= nil then { R[A]=R[A+2]; pc -= Bx } */
    SetList,    /*  B C k R[A][C+i] := R[A+i], 1 <= i <= B                */
    Closure,    /*  Bx    R[A] := closure(KPROTO[Bx])                     */
    Vararg,     /*  C     R[A], R[A+1], ..., R[A+C-2] = vararg            */
    VarargPrep, /*        (adjust vararg parameters)                      */
    Extraarg,   /* x      extra (larger) argument for previous opcode     */
}
