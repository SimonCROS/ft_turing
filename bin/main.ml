open Yojson.Basic.Util

let get_name_value json =
  let name = member "name" json |> to_string in
  name

let () =
  (* TODO Check if param exists *)
  let filename = Sys.argv.(1) in
  try
    let ic = open_in filename in
    let json = Yojson.Basic.from_channel ic in
    close_in ic;

    Format.printf "\nFound value: %s\n" (get_name_value json);

    flush stdout;
  with e ->
    raise e;
