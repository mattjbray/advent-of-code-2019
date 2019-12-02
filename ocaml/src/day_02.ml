module Part_1 = struct
  type program = Program of int array

  module Program = struct
    type t = program

    let pp fmt (Program p) =
      CCFormat.(fprintf fmt "[%a]" (array ~sep:(return ",@,") int) p)

    let of_array xs = Program xs

    let p = CCParse.(sep ~by:(char ',') U.int >|= Array.of_list >|= of_array)

    let of_string_exn str = CCParse.parse_string_exn p str


    let copy (Program p) = Program (Array.copy p)
  end

  type address = Address of int

  module Address = struct
    type t = address

    let pp fmt (Address a) = CCFormat.fprintf fmt "%i" a

    let of_int i = Address i

    let ( + ) (Address a1) step = Address (a1 + step)
  end

  type value = Value of int

  module Value = struct
    type t = value

    let pp fmt (Value a) = CCFormat.fprintf fmt "%i" a

    let of_int i = Value i

    let ( + ) (Value a1) (Value a2) = Value (a1 + a2)

    let ( * ) (Value a1) (Value a2) = Value (a1 * a2)
  end

  type error = [ `Invalid_address of address | `Invalid_op_code of int ]

  let pp_error fmt =
    CCFormat.(
      function
      | `Invalid_address a -> fprintf fmt "invalid address %a" Address.pp a
      | `Invalid_op_code i -> fprintf fmt "invalid op code %i" i)

  let int_of_address (Program program) (Address address) =
    CCArray.get_safe program address
    |> CCOpt.to_result (`Invalid_address (Address address))

  let value_of_address program address =
    CCResult.Infix.(int_of_address program address >|= fun v -> Value v)

  let address_of_address program address =
    CCResult.Infix.(int_of_address program address >|= fun v -> Address v)

  let set (Program p) (Address a) (Value v) =
    try Ok (CCArray.set p a v)
    with Invalid_argument _ -> Error (`Invalid_address (Address a))

  type code = Add | Mul | Stop

  let code_of_address program pc =
    CCResult.(
      int_of_address program pc >>= function
      | 1 -> return Add
      | 2 -> return Mul
      | 99 -> return Stop
      | i -> fail (`Invalid_op_code i))

  let args_3 program pc =
    CCResult.Infix.(
      address_of_address program Address.(pc + 1) >>= value_of_address program
      >>= fun arg1 ->
      address_of_address program Address.(pc + 2) >>= value_of_address program
      >>= fun arg2 ->
      address_of_address program Address.(pc + 3) >|= fun store ->
      (arg1, arg2, store))

  type state = Terminated | Pc of address

  module State = struct
    type t = state

    let pp fmt =
      CCFormat.(
        function
        | Terminated -> fprintf fmt "terminated"
        | Pc pc -> fprintf fmt "pc: %a" Address.pp pc)
  end

  let step program state =
    match state with
    | Terminated -> Ok state
    | Pc pc -> (
        CCResult.Infix.(
          code_of_address program pc >>= function
          | Stop -> Ok Terminated
          | Add ->
              args_3 program pc >>= fun (arg1, arg2, store) ->
              set program store Value.(arg1 + arg2) >>= fun () ->
              Ok (Pc Address.(pc + 4))
          | Mul ->
              args_3 program pc >>= fun (arg1, arg2, store) ->
              set program store Value.(arg1 * arg2) >>= fun () ->
              Ok (Pc Address.(pc + 4))) )

  let%expect_test "step" =
    let pp_result =
      CCFormat.(
        hbox (pair (pp_print_result ~ok:State.pp ~error:pp_error) Program.pp))
    in

    let program =
      Program.of_array [| 1; 9; 10; 3; 2; 3; 11; 0; 99; 30; 40; 50 |]
    in
    let result = step program (Pc (Address.of_int 0)) in
    CCFormat.(printf "%a@." pp_result (result, program));
    [%expect {|
        pc: 4, [1,9,10,70,2,3,11,0,99,30,40,50]
      |}];

    let result = step program (Pc (Address.of_int 4)) in
    CCFormat.(printf "%a@." pp_result (result, program));
    [%expect {|
        pc: 8, [3500,9,10,70,2,3,11,0,99,30,40,50]
      |}];

    let result = step program (Pc (Address.of_int 8)) in
    CCFormat.(printf "%a@." pp_result (result, program));
    [%expect
      {|
        terminated, [3500,9,10,70,2,3,11,0,99,30,40,50]
      |}]

  let run program =
    let rec go pc =
      CCResult.Infix.(
        step program pc >>= function Terminated -> Ok () | pc -> go pc)
    in
    go (Pc (Address.of_int 0))

  let pp_result =
    CCFormat.(
      hbox (pair (pp_print_result ~ok:unit ~error:pp_error) (hbox Program.pp)))

  let%expect_test "run" =
    let program = Program.of_string_exn "1,0,0,0,99" in
    let result = run program in
    CCFormat.(printf "%a@." pp_result (result, program));
    [%expect {| (), [2,0,0,0,99] |}];

    let program = Program.of_string_exn "2,3,0,3,99" in
    let result = run program in
    CCFormat.(printf "%a@." pp_result (result, program));
    [%expect {| (), [2,3,0,6,99] |}];

    let program = Program.of_string_exn "2,4,4,5,99,0" in
    let result = run program in
    CCFormat.(printf "%a@." pp_result (result, program));
    [%expect {| (), [2,4,4,5,99,9801] |}];

    let program = Program.of_string_exn "1,1,1,4,99,5,6,0,99" in
    let result = run program in
    CCFormat.(printf "%a@." pp_result (result, program));
    [%expect {| (), [30,1,1,4,2,5,6,0,99] |}]
end


module Part_2 = struct
  let run_with_values program v1 v2 =
    let open Part_1 in
    CCResult.(
      set program (Address 1) (Value v1) >>= fun () ->
      set program (Address 2) (Value v2) >>= fun () ->
      run program >>= fun () -> value_of_address program (Address 0)
    )
end
