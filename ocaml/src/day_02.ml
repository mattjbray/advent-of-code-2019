module Part_1 = struct
  let pp_program fmt p =
    CCFormat.(fprintf fmt "[%a]" (array ~sep:(return ",@,") int) p)

  let p = CCParse.(sep ~by:(char ',') U.int >|= Array.of_list)

  let program_of_string_exn str = CCParse.parse_string_exn p str

  type error = [`Invalid_address of int | `Invalid_op_code of int]

  let pp_error fmt =
    CCFormat.(
      function
      | `Invalid_address a ->
          fprintf fmt "invalid address %i" a
      | `Invalid_op_code i ->
          fprintf fmt "invalid op code %i" i)

  let get program address =
    CCArray.get_safe program address
    |> CCOpt.to_result (`Invalid_address address)

  let set p ~address ~value =
    try Ok (CCArray.set p address value)
    with Invalid_argument _ -> Error (`Invalid_address address)

  let op_3 f program pc =
    let g = get program in
    CCResult.Infix.(
      let* arg1 = g (pc + 1) >>= g in
      let* arg2 = g (pc + 2) >>= g in
      let* result_addr = g (pc + 3) in
      let+ () = set program ~address:result_addr ~value:(f arg1 arg2) in
      pc + 4)

  type state = Terminated | Running of int

  let pp_state fmt =
    CCFormat.(
      function
      | Terminated ->
          fprintf fmt "terminated"
      | Running pc ->
          fprintf fmt "pc: %i" pc)

  let step program state =
    match state with
    | Terminated ->
        Ok state
    | Running pc -> (
        CCResult.Infix.(
          get program pc
          >>= function
          | 99 ->
              Ok Terminated
          | 1 ->
              let+ pc = op_3 ( + ) program pc in
              Running pc
          | 2 ->
              let+ pc = op_3 ( * ) program pc in
              Running pc
          | i ->
              Error (`Invalid_op_code i)) )

  let%expect_test "step" =
    let pp_result =
      CCFormat.(
        hbox (pair (pp_print_result ~ok:pp_state ~error:pp_error) pp_program))
    in
    let program = [|1; 9; 10; 3; 2; 3; 11; 0; 99; 30; 40; 50|] in
    let result = step program (Running 0) in
    CCFormat.(printf "%a@." pp_result (result, program)) ;
    [%expect {|
        pc: 4, [1,9,10,70,2,3,11,0,99,30,40,50]
      |}] ;
    let result = step program (Running 4) in
    CCFormat.(printf "%a@." pp_result (result, program)) ;
    [%expect {|
        pc: 8, [3500,9,10,70,2,3,11,0,99,30,40,50]
      |}] ;
    let result = step program (Running 8) in
    CCFormat.(printf "%a@." pp_result (result, program)) ;
    [%expect
      {|
        terminated, [3500,9,10,70,2,3,11,0,99,30,40,50]
      |}]

  let run program =
    let rec go pc =
      CCResult.Infix.(
        step program pc >>= function Terminated -> Ok () | pc -> go pc)
    in
    go (Running 0)

  let pp_result =
    CCFormat.(
      hbox (pair (pp_print_result ~ok:unit ~error:pp_error) (hbox pp_program)))

  let%expect_test "run" =
    let program = program_of_string_exn "1,0,0,0,99" in
    let result = run program in
    CCFormat.(printf "%a@." pp_result (result, program)) ;
    [%expect {| (), [2,0,0,0,99] |}] ;
    let program = program_of_string_exn "2,3,0,3,99" in
    let result = run program in
    CCFormat.(printf "%a@." pp_result (result, program)) ;
    [%expect {| (), [2,3,0,6,99] |}] ;
    let program = program_of_string_exn "2,4,4,5,99,0" in
    let result = run program in
    CCFormat.(printf "%a@." pp_result (result, program)) ;
    [%expect {| (), [2,4,4,5,99,9801] |}] ;
    let program = program_of_string_exn "1,1,1,4,99,5,6,0,99" in
    let result = run program in
    CCFormat.(printf "%a@." pp_result (result, program)) ;
    [%expect {| (), [30,1,1,4,2,5,6,0,99] |}]
end

module Part_2 = struct
  let run_with_values program v1 v2 =
    let open Part_1 in
    CCResult.Infix.(
      let* () = set program ~address:1 ~value:v1
      and* () = set program ~address:2 ~value:v2 in
      let* () = run program in
      get program 0)
end
