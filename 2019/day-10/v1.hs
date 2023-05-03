import qualified Data.Char as Char
import qualified Data.List as List
import qualified Data.Map.Strict as Map
import qualified Data.Set as Set
import qualified Control.Exception as Ex
import qualified Control.Monad
import Text.Printf
import Data.Tuple.Select

sample_1a = "\
\.#..#\n\
\.....\n\
\#####\n\
\....#\n\
\...##"

sample_1b = "\
\......#.#.\n\
\#..#.#....\n\
\..#######.\n\
\.#.#.###..\n\
\.#..#.....\n\
\..#....#.#\n\
\#..#....#.\n\
\.##.#..###\n\
\##...#..#.\n\
\.#....####"

sample_1c = "\
\.#..##.###...#######\n\
\##.############..##.\n\
\.#.######.########.#\n\
\.###.#######.####.#.\n\
\#####.##.#.##.###.##\n\
\..#####..#.#########\n\
\####################\n\
\#.####....###.#.#.##\n\
\##.#################\n\
\#####.##.###..####..\n\
\..######..##.#######\n\
\####.##.####...##..#\n\
\.#####..#.######.###\n\
\##...#.##########...\n\
\#.##########.#######\n\
\.####.#.###.###.#.##\n\
\....##.##.###..#####\n\
\.#.#.###########.###\n\
\#.#.#.#####.####.###\n\
\###.##.####.##.#..##"

sample_2a = "\
\.#....#####...#..\n\
\##...##.#####..##\n\
\##...#...#.#####.\n\
\..#.....X...###..\n\
\..#.#.....#....##\n\
\"


type Coord = (Int,Int)
type Direction = (Int,Int)


main = do
  input <- readFile "input.txt"

  let d = lines . trim $ input --sample_1c
  let asts = asteroids d
  
  -- Part One -- 221
  let visAstsFromLoc = Set.map (\loc -> (loc, length $ visibleDirs asts loc)) asts
  let best = List.maximumBy compareSel2 visAstsFromLoc
  -- putStrLn (show (fst best) ++ " -> " ++ show (snd best))

  -- Part Two -- 806
  let loc = fst best
  let oasts = loc `Set.delete` asts
  let astsDirDist = List.map (\ ast -> (ast, dir' ast loc, dist' ast loc)) $ Set.toList oasts

  let sorted = List.sortBy compareDirDist astsDirDist
  -- putStrLn $ unlines $ map show sorted

  prettyPrint $ fire (-1) sorted
  let the200th = fire (-1) sorted !! (200-1)
  print the200th
  let ((x,y),_,_) = the200th
  let res = x*100+y
  print res

  putStr ""


dir' :: Coord -> Coord -> Float
dir' ast@(x1,y1) loc@(x2,y2) = posDeg
  where posDeg = if deg < 0 then deg + 360 else deg
        deg = rad * 180 / pi
        rad = atan2 dx (-dy)
        dx = fromIntegral (x1-x2)
        dy = fromIntegral (y1-y2)
  

prettyPrint :: [((Int,Int),Float,Double)] -> IO ()
prettyPrint = putStrLn . unlines . map (\ a -> show (sel1 a) ++ ":  " ++ show (sel2 a) ++ "  " ++ show (sel3 a))

fire _ [] = []
fire lastDir list = ast : fire (sel2 ast) (List.delete ast list)
  where ast = if null a then head b else head a
        a = filter (\ a -> sel2 a > lastDir) list
        b = filter (\ a -> sel2 a > lastDir - 360) list


compareDirDist a b = (sel2 a, sel3 a) `compare` (sel2 b, sel3 b)

compareSel2 x y = sel2 x `compare` sel2 y

visibleDirs :: Set.Set Coord -> Coord -> Set.Set Direction
visibleDirs asts loc = Set.map (dir loc) $ loc `Set.delete` asts


dir ast loc = (xo `div` g, yo `div` g)
  where xo = fst ast - fst loc
        yo = snd ast - snd loc
        g = max 1 $ gcd xo yo


dist' ast@(x1,y1) loc@(x2,y2) = sqrt . fromIntegral $ (x1-x2)^2 + (y1-y2)^2

dist = manDist

asteroids d = Set.fromList [(x,y) | y <- yr, x <- xr, d!!y!!x == '#']
  where
    yl = length d
    xl = length (head d)
    yr = [0..yl-1]
    xr = [0..xl-1]


manDist (x1,y1) (x2,y2) = abs (x1-x2) + abs (y1-y2)


update n x a = take n a ++ [x] ++ drop (n + 1) a


trim = List.dropWhileEnd Char.isSpace . dropWhile Char.isSpace
