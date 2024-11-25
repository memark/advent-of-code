# https://adventofcode.com/2016/day/12

# Uiua 0.13.0
# Experimental!

# M ← map "abcd" 0_0_0_0
M ← map "abcd" 10_20_30_40

I  ← ⊜□⊸≠@\n &fras "example.txt"
II ← ⊜□⊸≠@\n &fras "input.txt"

# value ? src regs
GetCpyValue ← (
  ⍣(⊙◌⋕
  | get ⊢
  )
)

# ptrDiff regs ? instr regs
G ← (
  ⍣( # 
    °$"cpy _ _"
    ⍩(⊙⊢ : ⊙GetCpyValue : ⊙⊙.
      1 ⍜get◌ ⊙::
    )
  | °$"inc _"
    ⍩(1 ⍜get+₁ ⊢)
  | °$"dec _"
    ⍩(1 ⍜get-₁ ⊢)
  | °$"jnz _ _"
    ⍩(:⊙(⊸get) :⊓⊢⋕
      ⍥(1◌)=₀
    )
  | ⍤ "invalid instruction"
  )
)

# ptr instrs regs ? ptr instrs regs
F ← (
  ,,
  ⊙⊙(G°□⊡) # 2x by, with etc?
  +⊙:
)

get @a ◌◌ ⍢F(>,⧻, ⊙⊙⸮) 0 II M

# 9227663, too high
