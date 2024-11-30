# https://adventofcode.com/2015/day/8

# Uiua 0.13.0
# Experimental!

# inputsträngen
# - summera längden av varje rad
# - summera längden efter behandling

# behandling
# - ta bort " i start och slut
# - ersätt \" med "
# - ersätt \\ med \
# - ersätt \xAA med B

# kan man istället för att ersätta bara räkna hur många av varje sort det finns per rad?

F ← [
  ⊃($ ^\"|\"$
    ×1⧻ $beginend regex
  | $ \\"
    ×1 $quote⧻ regex
  | $ \\\\[^\\]
    ×1 $slash⧻ regex
  | $ \\x[0-9a-f]{2}
    ×3 $hex⧻ regex
  )]

GetValueForRow ← - ⊃(/+F|⧻)

┌─╴test
  ⍤⤙≍ 0 GetValueForRow $ ""
  ⍤⤙≍ 3 GetValueForRow $ "abc"
  ⍤⤙≍ 7 GetValueForRow $ "aaa\"aaa"
  ⍤⤙≍ 1 GetValueForRow $ "\x27"
└─╴

GetValueForRowNew ← ⊃(/+F)

# &p"\""
# &p"\\\""

# $ "aaa\"bbb\\ccc"\x27"ddd
# .
# ⤚regex $ ^\"|\"$
# ⤚regex $ \\"
# ⤚regex $ \\\\
# ⤚regex $ \\x[0-9a-f]{2}

# 4844, too high
# 1358, too high
# 1354, too high
/+≡◇GetValueForRowNew ⊜□⊸≠@\n &fras "input.txt"

GetValueForRowNew $ "k\xd4cfixejvkicryipucwurq\x7eq"
GetValueForRowNew $ "d\\gkbqo\\fwukyxab\"u"

/+≡◇GetValueForRowNew ⊜□⊸≠@\n &fras "example.txt"

GetValueForRowNew $ "bnlxkjvl\x7docehufkj\\\"qoyhag"

$ "uwxhhzz\\mtmhghjn\\\\tnhzbejj"

$ \\\\
regex

# nåt tjall med upprepade \\
# men det är aldrig fler än 4 \ i rad
