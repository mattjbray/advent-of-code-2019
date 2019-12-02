open Aoc.Day_02

let part = ref 1

let data = ref "data/day_02.txt"

let () =
  Arg.parse
    (Arg.align
       [ ("--part", Set_int part, "Part to execute (1 or 2)")
       ; ("--data", Set_string data, "Data file") ])
    ignore ""

let () =
  match !part with
  | 1 ->
      Part_1.(
        let program =
          CCIO.with_in !data CCIO.read_all |> program_of_string_exn
        in
        let result = Part_2.run_with_values program 12 02 in
        CCFormat.(
          printf "%a@." (pp_print_result ~ok:int ~error:pp_error) result))
  | 2 ->
      let program =
        CCIO.with_in !data CCIO.read_all |> Part_1.program_of_string_exn
      in
      let next noun verb =
        if verb < 99 then (noun, verb + 1) else (noun + 1, 0)
      in
      let rec attempt noun verb =
        let p = Array.copy program in
        match Part_2.run_with_values p noun verb with
        | Ok 19690720 ->
            Some ((100 * noun) + verb)
        | _ ->
            let noun, verb = next noun verb in
            attempt noun verb
      in
      let result = attempt 0 0 in
      CCFormat.(printf "%a@." (some int) result)
  | _ ->
      ()
