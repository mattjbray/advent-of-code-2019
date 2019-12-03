open Aoc.Day_03

let part = ref 1

let data = ref "data/day_03.txt"

let () =
  Arg.parse
    (Arg.align
       [ ("--part", Set_int part, "Part to execute (1 or 2)")
       ; ("--data", Set_string data, "Data file") ])
    ignore ""

let read_data () =
  match CCIO.with_in !data CCIO.read_lines_l with
  | [str1; str2] ->
      Part_1.(moves_of_string str1, moves_of_string str2)
  | _ ->
      raise (Invalid_argument "bad data")

let () =
  match !part with
  | 1 ->
      Part_1.(
        let moves_1, moves_2 = read_data () in
        let result = solve moves_1 moves_2 in
        CCFormat.(printf "%a@." (some int) result))
  | 2 ->
      Part_2.(
        let moves_1, moves_2 = read_data () in
        let result = solve moves_1 moves_2 in
        CCFormat.(printf "%a@." (some int) result))
  | _ ->
      ()
