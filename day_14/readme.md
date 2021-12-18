# Advent Day 14: Extended Polymerization
The incredible pressures at this depth are starting to put a strain on your submarine. The submarine has polymerization equipment that would produce suitable materials to reinforce the submarine, and the nearby volcanically-active caves should even have the necessary input elements in sufficient quantities.

The submarine manual contains instructions for finding the optimal polymer formula; specifically, it offers a polymer template and a list of pair insertion rules (your puzzle input). You just need to work out what polymer would result after repeating the pair insertion process a few times.

## Puzzle 1
This wasn't too bad, I treated everything as a character and read the template in as `Vec<char>` and the rules in as a `HashMap<(char, char), char)>`. Then I simply created a 'polymer' `Vec<char>` that was double the length of the template - 1. From there I iterated over the template, inserted the characters in to the new polymer. Rinse and repeate 10 times and volia. 

## Puzzle 2
And, here's where my solution fell apart. Increasing the iterations from 10 to 40 caused the program to exit with a memory allocation error on iteration 28: `memory allocation of 40802189316 bytes failed` (that's 40GB it was trying to write). My PC has 32 GB and straight up couldn't handle it. I thought about trying this on an EC2 server just to see, but at that growth there is no way I could find an EC2 instance (that I could afford) to run for the amount of time it might take. 

I need to rewrite this entirely. There is probably a pattern I can establish, a way to predict how many letters there are, or a different way of couting characters and positions. 

### Solution
I tracked by the total number of Polymer Pairs created, and since each time a polymer pair 'splits' `n` number of new characters are inserted into the string. `n` characters is simply the new polymer character * the total of the previous polymer pair. 

## Summary
Solution 1 was pretty easy and straightforward, the second one required a complete re-write. That took me quite a while to figure out and required a whiteboard. 