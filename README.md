# Advent of code 2022

This is my repo for my answers to [the advent of code 2022](https://adventofcode.com/2022).

My participation this year is in Rust.

It should get updated daily, Summary of each exercices will be bellow.

## Day 01
The input file contains groups of integers to add up, separated by a blank line.

### *part one*
We need to find out which group has the highest total and display it.

### *part two*
We now need to find out the three highest totals, add them up and display it.

## Day 02
We have a list of inputs detailing a strategy at a game of Rock, Paper, Scissors.

The first element of each line indicates the oponnent's choice:

|Input|oponnent choice|
|:---:|:-------------:|
|  A  |      Rock     |
|  B  |     Paper     |
|  C  |     Scissors  |

The second element's meaning changes between the two parts.

We need to calculate a score based on these principles:

|Event|     Score     |
|:---:|:-------------:|
|We played Rock|  +1  |
|We played Paper| +2  |
|We played Scissors|+3|
|We won|+6|
|Draw|+3|
|We lost|+0|


### *part one*

During this part we assume that the second part of the input represents which move we're supposed to made like so:

|Input|oponnent choice|
|:---:|:-------------:|
|  X  |      Rock     |
|  Y  |     Paper     |
|  Z  |     Scissors  |

### *part two*

During this part we assume that the second part of the input indicates if we should win, lose or draw like so:

|Input|oponnent choice|
|:---:|:-------------:|
|  X  |      Lose     |
|  Y  |     Draw     |
|  Z  |     Win  |