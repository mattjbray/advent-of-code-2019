module Part_1 = struct
  let is_six_digits s = String.length s = 6

  let two_adjacent_digits_the_same s =
    let rec go = function
      | a :: b :: _ when a = b ->
          true
      | _ :: cs ->
          go cs
      | [] ->
          false
    in
    go (CCString.to_list s)

  let digits_do_not_decrease s =
    let rec go = function
      | a :: b :: _ when a > b ->
          false
      | _ :: cs ->
          go cs
      | [] ->
          true
    in
    go (CCString.to_list s)

  let meets_criteria i =
    let s = string_of_int i in
    is_six_digits s
    && two_adjacent_digits_the_same s
    && digits_do_not_decrease s

  let%expect_test _ =
    Format.printf "%b@." (meets_criteria 111111) ;
    [%expect "true"] ;
    Format.printf "%b@." (meets_criteria 223450) ;
    [%expect "false"] ;
    Format.printf "%b@." (meets_criteria 123789) ;
    [%expect "false"]
end

module Part_2 = struct
  let exactly_two_adjacent_digits_the_same s =
    let rec go last group = function
      | c :: cs ->
          if Some c = last then go last (group + 1) cs
          else if group = 2 then true
          else go (Some c) 1 cs
      | [] ->
          group = 2
    in
    go None 0 (CCString.to_list s)

  let meets_criteria i =
    let s = string_of_int i in
    Part_1.meets_criteria i && exactly_two_adjacent_digits_the_same s

  let%expect_test _ =
    Format.printf "%b@." (meets_criteria 112233) ;
    [%expect "true"] ;
    Format.printf "%b@." (meets_criteria 123444) ;
    [%expect "false"] ;
    Format.printf "%b@." (meets_criteria 111122) ;
    [%expect "true"]
end
