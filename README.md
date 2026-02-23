# rsdex &emsp; [![Latest Version]][crates.io]  [![Latest Release Banner]][Latest Release]
[Latest Release Banner]:https://img.shields.io/badge/Latest-release-blue
[Latest Version]: https://img.shields.io/crates/v/rsdex.svg
[Latest Release]:https://github.com/PurpleCraft999/rsdex/releases/latest/
[crates.io]: https://crates.io/crates/rsdex


A little command line tool written in rust that acts like the pokedex 

Thank you to PokéAPI for providing the data for this project

## Installing

### Rust
if you have rust installed you can use <code>cargo install rsdex</code>

### Windows
go to the latest release and copy the power shell script into the command prompt


### Linux
go to the latest release and copy the shell script into the terminal


## Usage

### General Tips
results will always be in pokedex order
everything is case insensitive 


### Dex and Name
`rsdex 1` which will return bulbasaur's pokedex entry or
`rsdex bulbasaur` witch will also return bulbasaur pokedex info
if the pokemon has a space in the name replace it with a `-` 


### Type and Color
`rsdex fire` to get all fire type pokemon or
`rsdex blue` to get all the blue pokemon

### Stat
To filter by stat you append the stat you want to the end such as 
`rsdex 20hp` for Health
`rsdex 20a` for Attack
`rsdex 20d` for Defence
`rsdex 20sa` for Special Attack
`rsdex 20sd` for Special Defence
`rsdex 20s` for Speed

you can also do get stats with ≤ or ≥ the stat value by adding `l` or `g` to the begaining

`rsdex g100hp` returns all pokemon with ≥ 100 hp

`rsdex l50s` returns all pokemon with ≤ 50 speed



### Egg Group
for the egg groups that share a name with types add egg to the end

`rsdex field` for pokemon in the field egg group
`rsdex fairyegg` for the fairy egg group

### Range
if you need the pokemon between say 50 and 100 you can do that with
<code>rsdex 50..100</code>
it accepts numbers  1 ≤ n ≤ max pokedex number


## Compound Searches

### Key Words
* and 
* or

### Use of Key Words
`and` between arguments will return pokemon with both properties
`or` between arguments will return pokemon with either property 

### Examples
<code>rsdex flying and bug</code> returning all the bug and flying types
<code>rsdex red or green</code> returns all pokemon that have the color of red or the color of green

## Flags
if you add --detailed or -d to the end of a search along with a number 0-5 and you'll get a more detailed summary
Example
<code>rsdex 1 -d 5</code>


### Writing to file

when you do `rsdex red --fp "red_pokemon.json"` it will save the reasults to the specified file path
you can also add the --write_mode write mode to the end of it to change the write mode to another type
it also takes into acount the level of detail set 
the pretty flag is set the write will be pretty if available
currently json,jsonl,and csv is supported
