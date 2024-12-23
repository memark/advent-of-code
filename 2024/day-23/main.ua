# https://adventofcode.com/2024/day/23

# uiua v0.15.0-dev.1

🗄 ← ↘1[□"ab"]

&fras"example.txt"
regex"(..)-(..)"
# ↙5

▽⊸≡(/↥⌕@t°□⊢)

≡↘₁
∵°□
# ⊂⊸≡⇌
⍆
↙10

# shit vad krångligt att få till. pilla inte på denna mer nu.
# det funkar för två, men inte tre just nu...
∧(⍜(⬚🗄get⊙:)(□≡(?◇♭?)?⊂□:) °⊟) :[]

# ok, det finns så klart dublettnycklar i tabellen.
# (och ännu mer så om man även vänder på den)
# så hashtabellen funkar inte med mindre än att låt value vara en array
# ja, så får det bli. börjar med att prova det.

# det jag behöver är en map
# key   - str
# value - box av array av str

# borde förstås bryta ut mer till egen funktion där jag kan verifiera input och output,
# samt enhetstesta fallen med 1, 2, 3 delar
