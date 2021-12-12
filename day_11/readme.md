# Advent Day 11: Dumbo Octopus
You enter a large cavern full of rare bioluminescent dumbo octopuses! They seem to not like the Christmas lights on your submarine, so you turn them off for now.

There are 100 octopuses arranged neatly in a 10 by 10 grid. Each octopus slowly gains energy over time and flashes brightly for a moment when its energy is full. Although your lights are off, maybe you could navigate through the cave without disturbing the octopuses if you could predict when the flashes of light will happen.

## Puzzle 1
This one was tricky, because I was thinking of the last few wrong I watched a Twitch Stream where a couple developers talked through their thought process. I wanted to learn more about how Rust can manipulate objects, better memory management, etc.. as well as how to break my thought process down into easier to steps (e.g. not a million loops and checks). 

Basically, we iterate through the grid, determine if the value is set to flash. If it does, we push onto a Vec of tuples the point that is supposed to flash. Once we have iterated over the entire Grid we process all the flash points. Here we check each neighbor, see if it is in bounds of the grid, if it is and it flashes, add it to the flash list. Continue to process the flash list until we have flashed every item. 

## Puzzle 2
This one was easy, but as a starting point I just copied all my code. Basically, we were looking for a count of 100 flashes. Continue looping (learned withat `loop{ }` does in Rust) till we hit that number and count the number of times we looped

## Summary
Watching the Twitch stream was clutch for me. It taught me a different way to think and walked me through some of the finer syntax of Rust code. While I 'wrote' this myself, it is a follow along to make sure I understood what I was doing and how I should be thinking about problems moving forward. 

[Twitch Stream](https://www.youtube.com/watch?v=xxpoUY4a_-A)

