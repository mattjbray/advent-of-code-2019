module Part_1 = struct
  type dir = U | R | D | L

  type move = {dir: dir; steps: int}

  type pos = int * int

  type segment =
    { start: pos
    ; move: move
    ; steps_to_start: int
          (** number of steps taken so far to the start of this segment *) }

  type path = segment list

  let dir_of_string str =
    match str with
    | "U" ->
        U
    | "R" ->
        R
    | "D" ->
        D
    | "L" ->
        L
    | _ ->
        raise (Invalid_argument ("dir_of_string: " ^ str))

  let move_of_string str : move =
    let dir, steps = CCString.take_drop 1 str in
    let dir = dir_of_string dir in
    let steps = int_of_string steps in
    {dir; steps}

  let moves_of_string str : move list =
    str |> CCString.split_on_char ',' |> CCList.map move_of_string

  let move_from ((x, y) : pos) ({dir; steps} : move) : pos =
    match dir with
    | U ->
        (x, y - steps)
    | R ->
        (x + steps, y)
    | D ->
        (x, y + steps)
    | L ->
        (x - steps, y)

  let path moves : path =
    let _, _, path =
      moves
      |> CCList.fold_left
           (fun (pos, steps, path) move ->
             let segment = {start= pos; move; steps_to_start= steps} in
             let pos' = move_from pos move in
             let steps' = steps + move.steps in
             (pos', steps', segment :: path))
           ((0, 0), 0, [])
    in
    CCList.rev path

  let ends ({start= (x, y) as start; move; _} : segment) : pos * pos =
    let x', y' = move_from start move in
    ((min x x', min y y'), (max x x', max y y'))

  type intersection = {pos: pos; steps: int}

  (** Find the intersection of [h] and [v], assuming [h] is horizontal and [v]
      is vertical. *)
  let intersection_perpendicular ~(h : segment) ~(v : segment) :
      intersection option =
    let (hx1, hy), (hx2, _) = ends h in
    let (vx, vy1), (_, vy2) = ends v in
    if (hx1 <= vx && vx <= hx2) && vy1 <= hy && hy <= vy2 then
      let h_steps =
        let x, _ = h.start in
        h.steps_to_start + abs (vx - x)
      in
      let v_steps =
        let _, y = v.start in
        v.steps_to_start + abs (hy - y)
      in
      Some {pos= (vx, hy); steps= h_steps + v_steps}
    else None

  let is_vertical = function U | D -> true | L | R -> false

  let intersection (seg1 : segment) (seg2 : segment) : intersection option =
    match (is_vertical seg1.move.dir, is_vertical seg2.move.dir) with
    | true, true | false, false ->
        None
    | true, false ->
        intersection_perpendicular ~h:seg2 ~v:seg1
    | false, true ->
        intersection_perpendicular ~h:seg1 ~v:seg2

  let intersections (path_1 : path) (path_2 : path) =
    CCList.product intersection path_1 path_2 |> CCList.filter_map CCFun.id

  let manhattan_dist (x1, y1) (x2, y2) = abs (x2 - x1) + abs (y2 - y1)

  let central_port = (0, 0)

  let closest_to_central_port (is : intersection list) : int option =
    is
    |> CCList.filter_map (function
         | {pos= 0, 0; _} ->
             None
         | {pos; _} ->
             Some (manhattan_dist central_port pos))
    |> CCList.sort Int.compare |> CCList.head_opt

  let solve moves_1 moves_2 =
    intersections (path moves_1) (path moves_2) |> closest_to_central_port

  let%expect_test "wires_1" =
    let moves_1 = moves_of_string "R8,U5,L5,D3" in
    let moves_2 = moves_of_string "U7,R6,D4,L4" in
    let result = solve moves_1 moves_2 in
    CCFormat.(printf "%a@." (some int) result) ;
    [%expect "6"]

  let%expect_test "wires_2" =
    let moves_1 = moves_of_string "R75,D30,R83,U83,L12,D49,R71,U7,L72" in
    let moves_2 = moves_of_string "U62,R66,U55,R34,D71,R55,D58,R83" in
    let result = solve moves_1 moves_2 in
    CCFormat.(printf "%a@." (some int) result) ;
    [%expect "159"]

  let%expect_test "wires_3" =
    let moves_1 =
      moves_of_string "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
    in
    let moves_2 = moves_of_string "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7" in
    let result = solve moves_1 moves_2 in
    CCFormat.(printf "%a@." (some int) result) ;
    [%expect "135"]
end

module Part_2 = struct
  include Part_1

  let least_steps (is : intersection list) : int option =
    is
    |> CCList.filter_map (function
         | {pos= 0, 0; _} ->
             None
         | {steps; _} ->
             Some steps)
    |> CCList.sort Int.compare |> CCList.head_opt

  let solve moves_1 moves_2 : int option =
    intersections (path moves_1) (path moves_2) |> least_steps

  let%expect_test "wires_1" =
    let moves_1 = moves_of_string "R8,U5,L5,D3" in
    let moves_2 = moves_of_string "U7,R6,D4,L4" in
    let result = solve moves_1 moves_2 in
    CCFormat.(printf "%a@." (some int) result) ;
    [%expect "30"]

  let%expect_test "wires_2" =
    let moves_1 = moves_of_string "R75,D30,R83,U83,L12,D49,R71,U7,L72" in
    let moves_2 = moves_of_string "U62,R66,U55,R34,D71,R55,D58,R83" in
    let result = solve moves_1 moves_2 in
    CCFormat.(printf "%a@." (some int) result) ;
    [%expect "610"]

  let%expect_test "wires_3" =
    let moves_1 =
      moves_of_string "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
    in
    let moves_2 = moves_of_string "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7" in
    let result = solve moves_1 moves_2 in
    CCFormat.(printf "%a@." (some int) result) ;
    [%expect "410"]
end
