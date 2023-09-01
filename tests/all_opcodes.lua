---- This conatins all of the opcodes ----
local r = 0 -- LOADI
local f = 1. -- LOADF
local k = 1.2 -- LOADK
-- LOADKX too hard to trigger (need more than 2**18 constants)
r = f -- MOVE
local f = false -- LOADFALSE
local f = not k or true -- LFALSESKIP and LOADTRUE
local n = nil -- LOADNIL
local x = _ENV -- GETUPVAL
_ENV = 2 -- SETUPVAL
local d = hello -- GETTABUP
hello = '' -- SETTABUP
local r = {} -- NEWTABLE and EXTRAARG
local i = r[x] -- GETTABLE
r[x] = '' -- SETTABLE
local i = r[1] -- GETI
r[1] = '' -- SETI
local i = r['hello'] -- GETFIELD
r['hello'] = 1 -- SETFIELD
r:a() -- SELF and CALL
local i = i + 1 -- ADDI and MMBINI
a = s + 1.2 -- ADDK
a = s - 1.2 -- SUBK
a = s * 3 -- MULK
a = s % 5 -- MODK
a = s ^ 5 -- POWK
a = s / 5 -- DIVK
a = s // 5 -- IDIVK
a = s & 5 -- BANDK
a = s | 5 -- BORK
a = s ~ 5 -- BXORK and MMBINK
a = s >> 1 -- SHRI
a = s << 1 -- SHLI (well it almost always compiles with SHRI -1)
a = s + a -- ADD
a = s - a -- SUB
a = s * a -- MUL
a = s % a -- MOD
a = s ^ a -- POW
a = s / a -- DIV
a = s // a -- IDIV
a = s & a -- BAND
a = s | a -- BOR
a = s ~ a -- BXOR
a = s >> a -- SHR
a = s << a -- SHL and MMBIN
a = -a -- UNM
a = ~a -- BNOT
a = not a -- NOT
a = #a -- LEN
:: label ::
local y = 1
goto label -- CLOSE and JMP
if (x == y) then -- EQ
end 
if (x < y) then -- LT
end
if (x <= y) then -- LE
end
if (x == 'afaf') then -- EQK
end
if (x == 1) then -- EQI
end
if (x < 1) then -- LTI
end
if (x <= 1) then -- LEI
end
if (x > 1) then -- GTI
end
if (x >= 1) then -- GEI
end
if (x) then -- TEST
end
function a()
	return x(1, 2, 3) -- TAILCALL and RETURN and RETURN0
end
function a()
	return x -- RETURN1
end
for i = 1, 2, 3 do -- FORPREP and FORLOOP
end
for x in a do -- TFORPREP and TFORCALL and TFORLOOP
end
function r(...) 
	for i in ipairs {...} do end -- VARARG and SETLIST
end
local af = r -- CLOSURE