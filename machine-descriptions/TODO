Things the machine itself should check:
(from subject)
-Your program must detect and reject ill formated or invalid machine descriptions
and inputs, with a relevant error message. This means that your program must
never crash for any reason.
-blank: The blank character, must be part of the alphabet, must NOT be part of the
input.

(more parsing check stuff)
-make sure read/write statements use only stuff from "alphabet"
-make sure "to_state" statements use only stuff from "states"
-action can only be RIGHT or LEFT
-at least one HALT statement must be present in a to_state statement
-initial and finals states must be included in states
-blank must be included in alphabet
-cannot have 2 read statement for the same char in the same state
-make sure every state (finals excluded) has transitions


(error raising)
-if transition does not exists

################################################################################
################################################################################

1. A machine able to compute an unary addition.

(starts at the first 1 like in the video of unary sub)
-> (step 1) go right until you meet a 1 or = 
		if 1
			change it to blank
			go right until blank
			write 1
			go back left until blank
			(go back to step 1)
		if =
			we're done



bug:
program is not protected against 11+11=1, it would produce false result 11111
protection would require making a separate set of state to handle first pass (remove anything after the = )
check for this before running machine description ? how

################################################################################
################################################################################

2. A machine able to decide whether its input is a palindrome or not. Before halting,
write the result on the tape as a ’n’ or a ’y’ at the right of the rightmost character
of the tape.

TL;DR: "grignoter chaque extremite tant que la symetrie est valide"
->  2 state per letter per side (4 total)
	states exemples
		(LR means L or R, need both versions  for both going left and right)
		(X means any alphabet letter, that's the letter we're checking for)
		"aquire_new_letter_LR", get new letter to check for
		"go_end_LR_X", go to end of string (one of each letter to "store" that data to check against)
		"check_last_LR_X", check the letter that was before end of string (this is the one that can trigger false)

	start on first letter of word or empty at it's left ? 
	start state should have transitions for every letter
	go to end of string, go left 1 with again a letter specific transition
	check if letter is same as the state specific letter 
		(move right 1 and write N if not)
	start again but to the left (need mirror version of everything)
		(difference with going to the right: go to end of word before printing answer)
	(how it ends as true)
	the string will end up empty if you remove both ends enought times without getting a false condition
		for odd length words
			after removing the first letter of each "duo",
			check must be in the letter specific transitions after the turn of end of string
			exemple:
			.[o]. -> read o, print . ,transition to go_end_right_o, go right(get a letter to check, in this case o )
			..[.] -> read ., print ., transition to check_last_right_o, go left (go back 1 after reaching end of string)
			.[.]. -> read ., print y, transition to HALT, go left (last char before end is blank, meaning string is blank)
		for even length words
			after removing second letter of each "duo",
			check must be in the acquire sequence (both left and right)
			if unable to acquire a new letter to check, it means string is empty
			exemple:
			.[l]l. -> read l, print ., transition to go_end_right_l, go right(get a letter to check, in this case l )
			..[l]. -> read l, print l, transition to go_end_right_l, go right (keep going until end of string)
			..l[.] -> read ., print ., transition to check_last_right_l, go left (go back 1 after reaching end of string)
			..[l]. -> read l, print ., transition to aquire_new_letter_L, go left (look for a letter that would have been in between the 2 l )
			.[.].. -> read ., print y, transition to HALT, go left (string is empty)
-> make the answer is in the right spot going both left and right as last move
-> make sure it works for sentences ? (ignore the space character that must be different from blank)
-> not sure where to print y when we deleted everything and lost where the word ended
	(put a special char at the end of string ?)
	does it even matter on an infinite roll of tape ?

################################################################################
################################################################################

3. A machine able to decide if the input is a word of the language 0n1n, for instance
the words 000111 or 0000011111. Before halting, write the result on the tape as a
’n’ or a ’y’ at the right of the rightmost character of the tape.

-> like palindrome but checks 0 and 1 opposite mirroring

################################################################################
################################################################################


4. A machine able to decide if the input is a word of the language 0 2n
, for instance
the words 00 or 0000, but not the words 000 or 00000. Before halting, write the
result on the tape as a ’n’ or a ’y’ at the right of the rightmost character of the
tape.

-> like palindrome but remove 0 by pair,
	if string empty when attempting to remove first of pair, true
	if string empty when attempting to remove second of pair, false
since there is only one char to test for, it can be done in one pass rather than
having to get to each end like palindrome


################################################################################
################################################################################

5. A machine able to run the first machine of this list, the one computing an unary
addition. The machine alphabet, states, transitions and input ARE the input of
the machine you are writing, encoded as you see fit.

-> je sais pas lol ¯\_(ツ)_/¯