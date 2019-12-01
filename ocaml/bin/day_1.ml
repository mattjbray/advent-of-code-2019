let part = ref 1

let data = ref "data/day_1.txt"

let () =
  Arg.parse
    (Arg.align
       [ ("--part", Set_int part, "Part to execute (1 or 2)")
       ; ("--data", Set_string data, "Data file") ])
    ignore ""

let masses = CCIO.with_in !data CCIO.read_lines_l |> CCList.map int_of_string

let sum xs = CCList.fold_left ( + ) 0 xs

let () =
  match !part with
  | 1 ->
      let total_fuel = masses |> CCList.map Aoc.Day_1.Part_1.fuel |> sum in
      CCFormat.printf "%i@." total_fuel
  | 2 ->
      let total_fuel = masses |> CCList.map Aoc.Day_1.Part_2.fuel |> sum in
      CCFormat.printf "%i@." total_fuel
  | _ ->
      ()
