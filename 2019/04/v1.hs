import qualified Data.List as List

main = do
  let input = "178416-676461"
  let [m1,m2] = map toInt . splitBy '-' $ input

  -- Part One -- 1650
  let a1 = findAnswerWithAlgorithms m1 m2 [conditionAdjacentSame (>= 2), neverDecrease]
  print a1

  -- Part Two -- 1129
  let a2 = findAnswerWithAlgorithms m1 m2 [conditionAdjacentSame (== 2), neverDecrease]
  print a2


findAnswerWithAlgorithms :: Int -> Int -> [String -> Bool] -> Int
findAnswerWithAlgorithms m1 m2 algs =
  length . filterAll algs . map intToString $ [m1..m2]

-- Förstå hur detta är samma sak som
-- filterAll fs col = foldr filter col fs
filterAll [] col = col
filterAll (f:fs) col = filter f $ (filterAll fs col)

conditionAdjacentSame :: Eq a => (Int -> Bool) -> [a] -> Bool
conditionAdjacentSame cond = any (cond . length) . List.group

-- Skriv om som foldl
neverDecrease :: String -> Bool
neverDecrease [p] = True
neverDecrease (p:ps)
  | head ps < p = False
  | otherwise = neverDecrease ps


toInt :: String -> Int
toInt = read

intToChar :: Int -> Char
intToChar = head . filter (/= '"') . show

intToString :: Int -> String
intToString = filter (/= '"') . show


splitBy _ [] = []
splitBy c s = as : splitBy c (if null bs then [] else tail bs)
  where
    i = (length . takeWhile (/= c)) s
    (as, bs) = splitAt i s
