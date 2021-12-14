# Advent Day 12: Passage Pathing
With your submarine's subterranean subsystems subsisting suboptimally, the only way you're getting out of this cave anytime soon is by finding a path yourself. Not just a path - the only way to know if you've found the best path is to find all of them.

## Puzzle 1
This one was not too tricky. Given what I learned from Day 11 I created a hasmap of caves, each cave has a series of neighbors so use the cave as the key and Vec of Strings as the caves they are connected to. 

Then, create a VecDequeu to get queueing, push the 'start' node to the end, pop it off and start iterating through all the paths available from there. End when you hit and 'end' node and if there is more than one small cave in the list end the path there as well. 

## Puzzle 2
Another puzzle that broke me. Building off the first problem all I wanted was to check each time a small cave was encountered if there existed a small cave that had been visited twice. I ended up doing this with a LOT of loops... my total execution time went from 92ms to 12s, so the effiency loss was intense. An input that was too much larger may have run for minutes or longer. 

## Summary
Using what I learned from Day 11 I was able to build a concise solution for day 12 part 1. Part 2 required sound outside the box thinking and I ended up using some more Rust knowledge to do my compares, although very inefficently.