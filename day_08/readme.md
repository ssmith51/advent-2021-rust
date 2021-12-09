# Advent Day 08: Seven Segment Search
You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.

As your submarine slowly makes its way through the cave system, you notice that the four-digit seven-segment displays in your submarine are malfunctioning; they must have been damaged during the escape. You'll be in a lot of trouble without them, so you'd better figure out what's wrong.

## Puzzle 1
This was very straight forward. Iterate over all the 'readings' after the | icon and add up the total of known numbers found.  

## Puzzle 2
This one took me a bit to figure out. I knew we were working with sets and had to determine what was included in each number and use the possiblities to further refine them. Ultimately I came up with the following logic: 

>* 1, 4, 7 , 8 are known
>* Top Line = difference between 7-1
>* Top Left & Middle Lines = difference 4 - 1
>* Bottom Left & Bottom Lines = difference of 8-4 - Top Line
>* 2 = len(5) w/Top Line, Bottom Left & Bottom Line
>* 5 = len(5) w/Top Line, Top Left & Middle
>* 3 = Last len(5) left
>* 0 = len(6) w/Top Line, Right Lines, Top Line, Bottom Left & Bottom Lines, 1 of Top Right and Middle
>* 6 = len(6) w/Top Line, Top Left & Middle, Bottom Left & Bottom, 1 of Right Lines
>* 9 = len(6) left

With that logic I spent wayyyyy too long trying to write this in Rust. I ran into the following challenges: 

1. How to do a `String` compare. All the `.contains()` function take `&str` not `String`, or `char` or `slice of chars`. I finally solved this, but it wasn't as simple as that. 
2. Letters were not guaranted to be in the same order! I brute forced this by converting `Strings` to `Vec<char`> to hold each indivual character, iterate over it and see if the readings value `String` contained each `char`. 
3. My lack of Rust skill lead me to creating a LOT of variables and conversions back and forth. I'm 100% positive that my solution has a more straight forward coding approach. 

## Summary
My first Rust solution I wrote before writing it in Golang was rough. I learned a lot (which was the intent) but also learned a lot of what not to do. 
