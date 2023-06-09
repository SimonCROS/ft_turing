type transition =
{read : char;
 to_state : string;
 write : char;
 action : string} [@@deriving of_yojson]

type machine =
{name : string;
 alphabet : char list;
 blank : char;
 states : string list;
 initial : string;
 finals : string list;
 transitions : (string * [`Null]) list } [@@deriving of_yojson]

let () =
  if Array.length Sys.argv != 2 then begin
    failwith "Wrong amount of arguments. Required <filename>.";
  end;

  let filename = Sys.argv.(1) in
  try
    let ic = open_in filename in
    let json = Yojson.Safe.from_channel ic in
    close_in ic;

    let () = Format.printf "Parsed to %a" Yojson.Safe.pp json in

    let machine = machine_of_yojson json in

    Format.printf "Machine name : %s\n" machine.name;
    Format.printf "Machine alphabet :";
    List.iter (Format.printf " `%c`") machine.alphabet;
    Format.printf "\n";

    let () = List.iter (fun (l, r) -> Format.printf "%s -> %d" l (String.length r)) machine.transitions in

    flush stdout;
  with e ->
    raise e;