# rsdex &emsp; [![Latest Version]][crates.io]  [![Latest Release Banner]][Latest Release]


[Latest Release Banner]:https://img.shields.io/badge/Latest-release-blue
[Latest Version]: https://img.shields.io/crates/v/rsdex.svg
[Latest Release]:https://github.com/PurpleCraft999/rsdex/releases/latest/
[crates.io]: https://crates.io/crates/rsdex



A  little command line tool written in rust that acts like the pokedex


<h2>Installing</h2>

<h3>Rust</h3>
if you have rust installed you can use 
<code>cargo install rsdex</code>

<h3>Windows</h3>
go to the latest release and copy the power shell script into the command prompt and let it do its work


<h3>Linux</h3>
go to the latest release and copy the shell script into the terminal


<h2>Usage</h2>

<h3>Dex and Name</h3>

`rsdex 1` which will return bulbasaur's pokedex entry or
`rsdex bulbasaur` witch will also return bulbasaur pokedex info

<h3>Type and Color</h3>

`rsdex fire` to get all fire type pokemon or
`rsdex blue` to get all the blue pokemon

<h3>Stat</h3>

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



<h3>Egg Group</h3>

filtering by egg group is posible and for the egg groups that share a name with types add egg to the end

`rsdex field` for pokemon in the field egg group
`rsdex fairyegg` for the fairy egg group

<h2>Flags</h2>
if you add --detailed or -d to the end of a search you'll get a more detailed summary
Example
<code>rsdex 1 --detailed </code>
