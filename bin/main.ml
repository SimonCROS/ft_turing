(* type transition =
{read : char;
 to_state : string;
 write : char;
 action : string} [@@deriving of_yojson] *)

(* type t_list = (string * (transition list))[@@deriving of_yojson] *)


type machine =
{name : string;
 alphabet : char list;
 blank : char;
 states : string list;
 initial : string;
 finals : string list;

 (* transitions: Assoc ( string * (transition list)) list; *)
 
 } [@@deriving of_yojson] [@@yojson.allow_extra_fields]
 
 (* 
 transitions is    assoc of (string) and (transition list)
 
 
 (* transitions: (string * string) list; *)
  (* transitions: t_list; *)
 (* transitions: (string * (transition list)) list; *)
 (* transitions: (string, transition list) ; *)
(* transitions: Assoc of [( string * transition list)]; *)
 (* transitions: Assoc (string , transition list) ; *)
(string, int) Hashtbl.t

let person = `Assoc [ ("name", `String "Anil") ];;
val person : [> `Assoc of (string * [> `String of string ]) list ] =
  `Assoc [("name", `String "Anil")]



*)


let () =
  if Array.length Sys.argv != 2 then begin
    failwith "Wrong amount of arguments. Required <filename>.";
  end;

  (* let filename = Sys.argv.(1) in *)
  try

      let json = Yojson.Safe.from_file Sys.argv.(1) in
      (* Format.printf "Parsed to %a" Yojson.Safe.pp json;
      Format.printf "\n"; *)
      
      
      let machine = machine_of_yojson json in
          Format.printf "Machine name : %s\n" machine.name;
          Format.printf "Machine alphabet :";
          List.iter (Format.printf " `%c`") machine.alphabet;
          Format.printf "\n";
          Format.printf "Blank : %c\n" machine.blank;
          Format.printf "Machine states :";
          List.iter (Format.printf " `%s`") machine.states;
          Format.printf "\n";
          Format.printf "Machine initial state : %s\n" machine.initial;
          Format.printf "Machine final states :";
          List.iter (Format.printf " `%s`") machine.finals;
          Format.printf "\n";


(* 
      Format.printf "transitions :";
      List.iter(Format.printf " `%s`") machine.transitions;
 *)





    (* let json = from_file filename in
          Format.printf "test "; *)

    (* let ic = open_in filename in
    let json = Yojson.Safe.from_channel ic in
    close_in ic;

    let () = Format.printf "Parsed to %a" Yojson.Safe.pp json in
             Format.printf "test ";

    let a = Yojson.Safe.pp json in 
            Format.printf "test "; *)
    (* let parsed = Yojson.Safe.pp machine_of_yojson json; *)

    (* let () = Format.printf "Parsed to %a" Yojson.Safe.pp json in

    let machine = machine_of_yojson json in
      Format.printf "test "; *)

    (* let machine = machine_of_yojson json in *)

    (* Format.printf "Machine name : %s\n" machine.name;
    Format.printf "Machine alphabet :";
    List.iter (Format.printf " `%c`") machine.alphabet;
    Format.printf "\n"; *)

    (* let () = List.iter (fun (l, r) -> Format.printf "%s -> %d" l (String.length r)) machine.transitions in *)

    flush stdout;
  with e ->
    raise e;