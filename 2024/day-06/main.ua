# https://adventofcode.com/2024/day/6

# uiua v0.14.0-dev.7
# Experimental!

G‼! ← |3 (
  $delta^1
  $new⊸+
  # ⨬(+100?|∘) ?₂-:1/+⊸<₀
  ⨬(∘?|∘) ?₂-:1/+⊸<₀
  # ⍤"oob"
  ◡𝄐⊡?₃
  $coll∨⊃(=@#|=@O)
  ⨬(         # no collision
    ⊙⍜⊡(@X◌) # - mark visited
    ⍜⊡(^0◌)  # - set new pos
  |          # collision
    ⊙⍜⊡(^2◌) # - turn
    ◌        # - discard new pos
  )
)

F ← |1 (
  $matrix
  $curr♭⊸(⊚∈"><^v")
  $char◡⊡
  ⍣(°$"^"
    ⍩(G‼!@^[¯1 0]@>)
  | °$">"
    ⍩(G‼!@>[0 1]@v)
  | °$"v"
    ⍩(G‼!@v[1 0]@<)
  | °$"<"
    ⍩(G‼!@<[0 ¯1]@^)
  )
)

🔢 ← /+♭∈"X><^v"
🐪 ← 🔢 ⍥(⍣F)∞

┌─╴test
  # ⍤⤙≍ 41 🐪 ≡°□ °/$"_\n_" &fras$ example.txt
  # ⍤⤙≍ 5067 🐪 ≡°□ ↘¯1 °/$"_\n_" &fras$ input.txt
└─╴

Prep ← ≡°□ °/$"_\n_" &fras

TryVariant ← (
  map [] []
  ⍥(⍜:(
      ⟜insert:1
      ⍣(F              # matrix ? matrix
      | ⊙(map [] [] ◌) # matrix map ? matrix map
      )
      # ⊙(&s.)
      # &s.
      # &sl0.02
    )
  )∞
  ⧻
)

# TryVariant Prep "example_1.txt"
# TryVariant Prep "example_2a.txt"
# TryVariant Prep "example_2b.txt"
# TryVariant Prep "example_2c.txt"

# när en runda terminerat, antingen pga exception eller termination,
# kolla efter < m fl för att avgöra om det var loop eller ej

# F ger matrix map
# jag vänder på detta för att kunna avsluta ⍣

# Nu har jag en funktion som ger map matrix.
# Om den kraschat (kört utanför) är len map = 0.
# Annars har den ett större värde.

# Sedan iterera över alla . och prova att ersätta dem med O

Prep "example_1.txt"
⊚⊸⌕@.
⊙¤
≡⍜⊡(@O◌)
≡TryVariant
▽>0
# /+>₀

# Ibland kraschar det på 10_7, ibland fångas det som det ska. förstår inte varför!!

# Äh, det är ju ändå en kass lösning. Jag borde göra en som använder Do istället.
# Funktionen jag itererar med behöver returnera
# - mapen (så jag kan kolla efter loop)
# - om det var OOB
# alternativt bryta upp i två funktioner där jag kan kolla OOB innan jag ens försöker flytta
