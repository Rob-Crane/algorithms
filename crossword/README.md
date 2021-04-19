# Crossword Filler

Given a grid of +/- which represents an empty crossword and a list of words to fill in.  Compute a configuration that fits all of the words.

## Approach
A function scans linearly over the crossword until it finds a blank.  At the blank, it tries to fill it with every remaining word.  If a word fits, the scanning function is called recursively with the remaining words and the updated gameboard.  If no word fits, the function returns.
