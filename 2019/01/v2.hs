main = do
    text <- readFile "input.txt"
    let masses = map toInt (lines text)
    print (sum (map fuel masses))
    print (sum (map fuel' masses))
    print (sum (map fuel'' masses))
    
toInt x = read x :: Int

fuel m = m `div` 3 - 2    

fuel' m
    | f > 0 = f + fuel' f
    | otherwise = 0
    where f = fuel m

fuel'' = sum . takeWhile (>0) . tail . iterate fuel