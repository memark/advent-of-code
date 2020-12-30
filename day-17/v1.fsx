let data = """
.#.
..#
###
"""

type Coord = { x: int; y: int; z: int; w: int }

let d =
    data.Trim().Split "\n"
    |> Array.toList
    |> List.map (fun x -> x.ToCharArray() |> Array.toList)

let actives =
    [ 0 .. (d.Length - 1) ]
    |> List.collect
        (fun y ->
            ([ 0 .. (d.[y].Length - 1) ]
             |> List.filter (fun x -> d.[x].[y] = '#')
             |> List.map (fun x -> { x = x; y = y; z = 0; w = 0 })))
    |> Set.ofList

let getNeighbours (coord: Coord) =
    [ for w in (coord.w - 1) .. (coord.w + 1) do
        for z in (coord.z - 1) .. (coord.z + 1) do
            for y in (coord.y - 1) .. (coord.y + 1) do
                for x in (coord.x - 1) .. (coord.x + 1) do
                    let c = { x = x; y = y; z = z; w = w }
                    if (c <> coord) then yield c ]
    |> Set.ofList

let rec recurse (maxCycles: int) (cycle: int) (actives: Coord Set) =
    if cycle > maxCycles then
        actives
    else
        let getNewState (coord: Coord) =
            let flip f x y = f y x

            let activeNeighbours =
                getNeighbours coord
                |> Set.filter (flip Set.contains actives)
                |> Set.count

            if Set.contains coord actives then
                activeNeighbours = 2 || activeNeighbours = 3
            else
                activeNeighbours = 3

        let newActives =
            actives
            |> Seq.collect (fun a -> (getNeighbours a).Add(a))
            |> Seq.distinct
            |> (fun seq ->
                (printfn $"cycle={cycle}: checking {seq |> Seq.length} relevant coords"
                 seq))
            |> Seq.filter getNewState
            |> Set.ofSeq
            |> (fun set ->
                (printfn $"cycle={cycle}: going from {actives |> Set.count} to {set |> Set.count} active cubes"
                 set))

        recurse maxCycles (cycle + 1) newActives

recurse 6 1 actives |> Set.count
