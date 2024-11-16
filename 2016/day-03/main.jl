println(
    readlines("input.txt") |>
    l -> split.(l, " ", keepempty=false) .|>
    (l -> parse.(Int, l)) .|>
    sort |>
    l -> count(x -> x[1] + x[2] > x[3], l)
)
