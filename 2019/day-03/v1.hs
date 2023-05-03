import qualified Data.Char as Char
import qualified Data.List as List
import qualified Data.Set as Set

sample_1 = "R8,U5,L5,D3\n\
\U7,R6,D4,L4"

sample_2 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
\U62,R66,U55,R34,D71,R55,D58,R83"

sample_3 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
\U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"

main = do
    input <- readFile "input.txt"
    let [r0,r1] = map (filter (/= (0,0)) . foldl getRoute [(0,0)]) . parse $ input
    let intersections = Set.intersection (Set.fromList r0) (Set.fromList r1)

    -- Part One -- 266
    print (getBestWithAlgorithm manDistFromC intersections)

    -- Part Two -- 19242
    print (getBestWithAlgorithm (stepsForIntersection r0 r1) intersections)

getRoute xys (_, 0) = xys
getRoute ((x,y):xys) (d, l)
  | d == 'R' = rec (x+1, y)
  | d == 'L' = rec (x-1, y)
  | d == 'U' = rec (x,   y-1)
  | d == 'D' = rec (x,   y+1)
  where rec a = getRoute (a:(x,y):xys) (d, l-1)
    
getBestWithAlgorithm f = minimum . Set.filter (>0) . Set.map f

manDistFromC = manDist (0,0)
               where manDist (x1,y1) (x2,y2) = abs (x1-x2) + abs (y1-y2)

stepsForIntersection r0 r1 p =
  case s of Just n -> length r0 + length r1 - n
  where s = sumMaybe [p `List.elemIndex` r0, p `List.elemIndex` r1]
        sumMaybe = fmap sum . sequence

parse = map parseLine . lines
        where parseLine = map parseMove . splitBy ','
              parseMove m = (head m, toInt . tail $ m)
              toInt x = read x :: Int

splitBy _ [] = []
splitBy c s = as : splitBy c (if null bs then [] else tail bs)
  where
    i = (length . takeWhile (/= c)) s
    (as, bs) = splitAt i s

trim = dropWhile Char.isSpace . List.dropWhileEnd Char.isSpace
