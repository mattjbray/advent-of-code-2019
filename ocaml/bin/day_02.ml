open Aoc.Day_02

let part = ref 1

let data = ref "data/day_02.txt"

let () =
  Arg.parse
    (Arg.align
       [
         ("--part", Set_int part, "Part to execute (1 or 2)");
         ("--data", Set_string data, "Data file");
       ])
    ignore ""

let () =
  match !part with
  | 1 ->
      Part_1.(
        let program =
          CCIO.with_in !data CCIO.read_all |> Program.of_string_exn
        in
        let result =
          CCResult.(
            set program (Address 1) (Value 12) >>= fun () ->
            set program (Address 2) (Value 2) >>= fun () ->
            run program >>= fun () -> value_of_address program (Address 0))
        in
        CCFormat.(
          printf "%a@." (pp_print_result ~ok:Value.pp ~error:pp_error) result))
  | _ -> ()
