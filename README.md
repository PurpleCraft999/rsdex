<h1>rsdex</h1>

A  little command line tool written in rust that acts like the pokedex


<h2>Installing</h2>



<h3>Windows</h3>
paste <code>powershell -ExecutionPolicy Bypass -c "irm https://github.com/PurpleCraft999/rsdex/releases/download/v0.1.110/rsdex-installer.ps1 | iex"</code>
into the Command Promt and it will install it automaticly

<h3>Linux</h3>

paste `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/PurpleCraft999/rsdex/releases/download/v0.1.110/rsdex-installer.sh | sh`
into the terminal to install
<h3>Rust</h3>
if you have rust installed you can use 
<code>cargo install rsdex</code>

<h2>Usage</h2>

after installing rsdex you can use rsdex like this

`rsdex 1` witch will return bulbasaur's pokedex entry or
`rsdex "bulbasaur"` witch will also return bulbasaur pokedex info

You can also filter by type or color such as 

`rsdex "fire"` to get all fire type pokemon or
`rsdex "blue"` to get all the blue pokemon
