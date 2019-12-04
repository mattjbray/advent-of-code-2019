open Aoc.Day_04

let part = ref 1

let () =
  Arg.parse
    (Arg.align [("--part", Set_int part, "Part to execute (1 or 2)")])
    ignore ""

let range = CCList.(265275 -- 781584)

let () =
  match !part with
  | 1 ->
      Part_1.(
        let result = range |> CCList.filter meets_criteria |> CCList.length in
        CCFormat.(printf "%a@." int result))
  | 2 ->
      Part_2.(
        let result = range |> CCList.filter meets_criteria |> CCList.length in
        CCFormat.(printf "%a@." int result))
  | _ ->
      ()
