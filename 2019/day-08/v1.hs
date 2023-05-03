import qualified Data.Char as Char
import qualified Data.List as List

main = do
  input <- readFile "input.txt"
  let sz = (25,6)
  let parsed = splitEvery (uncurry (*) sz) . trim $ input

  -- Part One -- 1596
  let layer = List.minimumBy (\ x y -> c0 x `compare` c0 y) parsed
  let res = c1 layer * c2 layer
  print res

  -- Part Two -- LBRCE
  let visiblePixels = map (head . filter (/= '2')) . List.transpose $ parsed
  let img = getImage $ splitEvery (fst sz) visiblePixels
  putStrLn img

c0 = countNs '0'
c1 = countNs '1'
c2 = countNs '2'

countNs n = length . filter (== n)

getImage = replace '0' ' ' . replace '1' '■' . List.intercalate "\n"

replace a b = map $ \c -> if c == a then b else c

trim = List.dropWhileEnd Char.isSpace . dropWhile Char.isSpace

splitEvery n = takeWhile (not . null) . map (take n) . iterate (drop n)
