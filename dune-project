(lang dune 3.7)

(name ft_turing)

(generate_opam_files true)

(source
 (github SimonCROS/ft_turing))

(authors "Simon Cros" "Karim Zennoune")

(maintainers "Simon Cros" "Karim Zennoune")

(license LICENSE)

(documentation https://github.com/SimonCROS/ft_turing)

(package
 (name ft_turing)
 (synopsis "Introduction to the Turing model.")
 (description "ft_turing will introduce you to the Turing model at the base of the imperative paradigm through an implementation of this model in OCaml.")
 (depends ocaml dune)
 (tags
  ("Theoretical model of the Turing machine" "State Machines" "Awareness of computability and complexity")))
