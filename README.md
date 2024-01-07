# Type trainer

A program that provides two exercises to help you become a faster at typing:

1. Quicktype: a random word from the file is chosen and appears on the screen, the user should type it as fast as possible, the test ends when a time limit is reached
2. Copy: each line of the text is shown to the user in the order they appear in the file, the user should copy the entire line, the test ends when the whole text is copied or when an optional time limit is reached

Note: This program is only intended to train the user to type faster and more accurate, it doesn't help the user learn the correct ways to hit the keys.

## Operation

Because this is a type trainer, it is fitting that the program is operated by typing commands into the console.
To start an exercise, the user should type the name of the exercise, followed by the path to the file that is to be used for training.
For a full list of possible commands and their arguments, the 'help' command can be used.

## Statistics

Whenever a test is completed, the following results are written to a file:

- the title of the text
- the total amount of characters in the text
- the amount of correct characters submitted
- the amount of mistakes in the submitted lines compared to the original text
- the average words per minute (calculated with the amount of correct characters)
- the amount of times the backspace was pressed
