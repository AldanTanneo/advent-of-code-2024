let input = open_in "input/day02.txt"

let lines =
  Stdlib.In_channel.input_lines input
  |> List.map (String.split_on_char ' ')
  |> List.map (List.map int_of_string)

type ordering = Increasing | Decreasing

let check ordering a b =
  match ordering with
  | Increasing -> a < b && b - a <= 3
  | Decreasing -> a > b && a - b <= 3

let rec is_safe lst last ordering saw_invalid =
  match lst with
  | [] | [ _ ] -> true
  | a :: b :: rest -> (
      match ordering with
      | Increasing ->
          if check ordering a b then
            is_safe (b :: rest) a Increasing saw_invalid
          else
            (not saw_invalid)
            && (is_safe (a :: rest) last Increasing true
               || check ordering last b
                  && is_safe (b :: rest) last Increasing true)
      | Decreasing ->
          if check ordering a b then
            is_safe (b :: rest) a Decreasing saw_invalid
          else
            (not saw_invalid)
            && (is_safe (a :: rest) last Decreasing true
               || check ordering last b
                  && is_safe (b :: rest) last Decreasing true))

let rec is_safe_init lst fst_invalid =
  match lst with
  | a :: b :: rest ->
      (if check Increasing a b then is_safe (b :: rest) a Increasing fst_invalid
       else if check Decreasing a b then
         is_safe (b :: rest) a Decreasing fst_invalid
       else false)
      || (not fst_invalid)
         && (is_safe_init (a :: rest) true || is_safe_init (b :: rest) true)
  | _ -> false
;;

print_string "p1 = ";
print_int
  (List.fold_left
     (fun acc lst -> if is_safe_init lst true then acc + 1 else acc)
     0 lines);
print_newline ()
;;

print_string "p2 = ";
print_int
  (List.fold_left
     (fun acc lst -> if is_safe_init lst false then acc + 1 else acc)
     0 lines);
print_newline ()
