module Part_1 = struct
  let fuel mass = (mass / 3) - 2

  let%expect_test "fuel" =
    CCFormat.(
      printf "%a@." (hbox (list int)) (List.map fuel [12; 14; 1969; 100756])) ;
    [%expect "2, 2, 654, 33583"]
end

module Part_2 = struct
  let rec fuel mass =
    let f = Part_1.fuel mass in
    if f < 0 then 0 else f + fuel f

  let%expect_test "fuel" =
    CCFormat.(
      printf "%a@." (hbox (list int)) (List.map fuel [14; 1969; 100756])) ;
    [%expect "2, 966, 50346"]
end
