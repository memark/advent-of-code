# https://adventofcode.com/2024/day/5

# uiua 0.15.0-dev.1
# Experimental!

UpdateMap ← (
  ⊙⊃(⬚(□[])get|⊙∘):
  insert:⍜(°□:)⊂
)

# map $ rules
🗺 ← ∧(UpdateMap°⊟₂) :[]

# updates rules $ data
🗄 ← (
  °/$"_\n\n_"
  ⍚°/$"_\n_"
  ∩°□:°⊟
  ⊓(≡(⍚⊜⋕⊸≠@,) ↘¯1|🗺≡(◇⊜⋕⊸≠@|))
)

# bool $ update rules
IsCorrectlyOrdered ← (
  # för varje update
  # - try get i rules
  # - för varje i rules
  #   - jämför mot kvarvarande updates
  ≡⌟◇(/∨◇∊: ⬚[]get ⊙: °⊂ ⇌) ⧅□
  ¬/∨
)

# answer $ updates rules
🐪 ← (
  ▽⤚≡⌟◇IsCorrectlyOrdered
  ≡◇(⊡⌊÷2⊸⧻) # pick out the middle number
  /+
)
┌─╴🧪
  ⍤⤙≍ 143 🐪 🗄 &fras "example.txt"
  # ⍤⤙≍ 6498 🐪 🗄 &fras "input.txt"
└─╴

? 🗄 &fras "example.txt"
? $incorrect ▽¬⤚≡⌟◇IsCorrectlyOrdered ⊙.
# ⊸≡&p
# ¤⊡₁
# ≡⌟(⊙◌?⇌°□)

# kan man göra nåt slags sortalgo?
# så fort jag hittar ett par som ingår i en regel,
# och har fel ordning,
# byta plats på dem?

# byta plats
# ⍜(▽0_1_0_1)⇌ 1_2_3_4

# man kan antingen hitta första paret som inte funkar,
# eller hitta alla par och sedan bara ta det första

# det finns inga dubletter inom en update,
# dvs har jag värdet kan jag hitta dess index

# 61, 13, 29 becomes 61, 29, 13

# ≡⌟◇(◇∊?: ⬚[]get ? °□ ⊙: °⊂ ⇌ ? °□ ?) ? ⧅□ ⊢

# candidates $ update rules
GetCandidates ← (
  ?
  °□
  ≡⌟◇(∩□ ⊙(▽⊸∊) ⊙°□ ⊙⊙: ⊙: ◠⬚[]get ⊙: °⊂ ⇌ °□) ⧅□
  ?
  ⊙⊙◌
  ?
  ⊟
  ⍉
  ?
  ▽⊸≡(¬≍□[] ⊣)
  ?
)

≡⌟GetCandidates ¤⊡₁

# ?
# $candidates ≡⌟(
#   ?
#   °□
#   ⇌
#   °⊂
#   ◡𝄐get

#   °□
#   ◡𝄐∊
#   ⊙◌
#   𝄐⌟▽

#   # kan inte ta first rakt av utifall den är tom
#   ⨬(□[]|□⊢) >₀⊸⧻
#   ⊙□
#   ⊂
# )

# ▽⊸≡(¬≍□[] ⊢) # är det tom array eller en match?
# ≡°□⊢ # ta första
# => 29 13, dessa två ska byta plats, sedan göra om det igen

# hitta alla par som inte är i rätt ordning är det jag vill göra
# det borde ju vara ungefär samma som det jag redan gör
#   ≡⌟◇(/∨◇∊: ⬚[]get ⊙: °⊂ ⇌) ⧅□

# kanske är det bättre att jobba med 0 istf box[]?
# det finns inga nollor i inputen, så det kan funka.
