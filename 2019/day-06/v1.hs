import qualified Data.Char as Char
import qualified Data.List as List
import qualified Data.Map.Strict as Map
import qualified Data.Set as Set

sample_1 = "COM)B\n\
\B)C\n\
\C)D\n\
\D)E\n\
\E)F\n\
\B)G\n\
\G)H\n\
\D)I\n\
\E)J\n\
\J)K\n\
\K)L"

sample_1b = "COM)B\n\
\B)C\n\
\C)D\n\
\D)E\n\
\E)F\n\
\B)G\n\
\G)H\n\
\D)I\n\
\E)J\n\
\J)K\n\
\K)L\n\
\K)YOU\n\
\I)SAN"


type Planet = String

main = do
  input <- readFile "input.txt"

  let m = getMap $ lines . trim $ input

  -- print m

  -- Part One -- 315757
  print (sum . cao $ m)

  -- Part Two -- 481
  let pathYou = getPathToCom "YOU" m
  let pathSan = getPathToCom "SAN" m
  let p = findParent pathYou pathSan
  -- print pathYou

  print (countHops p pathYou pathSan)

  putStr ""

countHops p l1 l2 = elemIndex' p l1 + elemIndex' p l2

elemIndex' a as = case List.elemIndex a as of Just b -> b

findParent :: [Planet] -> [Planet] -> Planet
findParent as bs = head [a | a <- as, b <- bs, a == b ]

getPathToCom :: Planet -> Map.Map Planet Planet -> [Planet]
getPathToCom a b = tail (rec a b)
  where 
    rec "COM" _ = []
    rec start m = start : rec (m Map.! start) m

co :: Planet -> Map.Map Planet Planet -> Int
co "COM" _ = 0
co o     m = 1 + co newo m
  where newo = m Map.! o

cao :: Map.Map Planet Planet -> Map.Map Planet Int
cao m = Map.mapWithKey f m
  where f k x = co k m

getMap :: [String] -> Map.Map Planet Planet
getMap [] = Map.empty
getMap (x:xs) = Map.insert k v $ getMap xs
  where [v,k] = splitBy ')' x


splitBy _ [] = []
splitBy c s =
  let i = (length . takeWhile (/= c)) s
      (as, bs) = splitAt i s
   in as : splitBy c (if null bs then [] else tail bs)

trim = List.dropWhileEnd Char.isSpace . dropWhile Char.isSpace
