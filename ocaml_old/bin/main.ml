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
 transitions : Yojson.Safe.t} [@@deriving of_yojson]


let test = Parse.allo;


let () =
  if Array.length Sys.argv != 2 then begin
    failwith "Wrong amount of arguments. Required <filename>.";
  end;

  let filename = Sys.argv.(1) in
  try
    let ic = open_in filename in
    let json = Yojson.Safe.from_channel ic in
    close_in ic;

    let machine = match machine_of_yojson json with
    | Ok data -> data;
    | Error err -> failwith ("Invalid machine : parsing failed for " ^ err);
    in


    Format.printf "Machine name : %s\n" machine.name;
    Format.printf "Machine alphabet :";
    List.iter (Format.printf " `%c`") machine.alphabet;
    Format.printf "\n";
    Format.printf "Machine blank char : %c\n" machine.blank;
    
    Format.printf "Machine states :";
    List.iter (Format.printf " `%s`") machine.states;
    
    Format.printf "Machine initial state : %s\n" machine.initial;


    Format.printf "Machine finals :";
    List.iter (Format.printf " `%s`") machine.finals;


    Format.printf "Machine transitions : %a" Yojson.Safe.pp machine.transitions;

    flush stdout;
  with e ->
    raise e;