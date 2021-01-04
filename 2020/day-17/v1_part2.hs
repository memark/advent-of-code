import qualified Data.List as List
import qualified Data.Set as Set

main = do
  let actives = getActives . lines $ sample_1
  print actives
  -- let endActives = recurse 1 1 actives
  -- print endActives
  print (getNeighbours (2,2,2,2))

sample_1 = "\
  \.#.\n\
  \..#\n\
  \###\
  \"

type Coord = (Int, Int, Int, Int)
type State = Set.Set Coord

getActives :: [[Char]] -> State
getActives d = Set.fromList [(x,y,0,0) |
                             y <- [0..length d - 1],
                             x <- [0..length (d !! y) - 1],
                             d !! x !! y == '#']

recurse :: Int -> Int -> State -> State
recurse maxCycles cycle actives
  | cycle > maxCycles = actives
  | otherwise = recurse maxCycles (cycle+1) newActives
  where
    newActives = Set.filter getNewState . Set.fromList . concatMap (\a -> a : getNeighbours a) $ actives

getNeighbours :: Coord -> [Coord]
getNeighbours c = [cc |
                   w <- pmo c3,
                   z <- pmo c2,
                   y <- pmo c1,
                   x <- pmo c0,
                   let cc = (x,y,z,w),
                   cc /= c]
  where (c0,c1,c2,c3) = c
        pmo n = [n-1,n,n+1]

getNewState c = True