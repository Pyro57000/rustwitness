# rustwitness
rust re-write of eyewitness

For now I'm only looking to get the screenshotting functionailty up and running. Generating the full eyewitness support is planned, but won't be implemented for some time. 

NOTE google chrome will need to be installed for this to work.

# Usage:

```
         ..............::::.....                                                        
    ...:-=====+==+++====--+:.....                                                       
  ..:-====+======+=+=---..=.............              ..........           ......       
...--==-==+**++*+**=--=+=....:---::----:....      .....:::----::......    ..:--..       
..-=-==+***###%%##*+*#+-+====-::::::::..::--......:::::::::::----------:-------..       
.:.-===+*###%#**#*##**##***++=----:::.....::::::-:...:::--=+==---==+====------.         
...:===+=-=+##%****++*+*****++==----::...::::::::.:::::======-=++-==*+===-----.         
.......-#-----+#***********:+===----:=-----:=---:--=--=:=---==-=--#*+++===---:.         
   ...:-+=======+####***#**+-=+==++-:----::--:===-===-=-=+=+==+=++=+=======--.          
   .-====-=++*****#*****#**+++++*++***==:-=+=-=:-+==-=+*+++++*++===+++=====-:.          
    .:----==+**###****#+***++===++++**#+-+=+==---+*+*++++=::.....:-=====-=-=..          
     ...===+++*#%%#***+=**=:........:=+#*+=--=-==+++++-......  .....-=*=+-=:.           
       ..=+++++**#****++==+=..      ...=*#*-:==--=+*=:..         ..--=-=-=-..           
       ......+++++******+==+..         .-#*=:-:-===-:.        ....-==+*+=-.             
             .-.=+++=**+*=+*..          .+*+--=====-.    ......:-=-=**===:.             
               .+*++*++*#*++===---==-----+*+-=====---===-===---=++-=++=+-.              
               .=*+=+*#*=*++*+==---=--=***+#*-===++===-:==*====+*+=+==+=..              
               .......=++++=+++==+==-=:-*+++**=+*=**===+-+-=:--=+==+++-..               
                    ....+++**=::===:+====++++*=++=+++=++===:+=+++++++:...               
                       ..=+++**++==+==+====+=+=*++++==+++=+===+++++:..                  
                        ...-=+====++=======+===***++++=======++=-....                   
                             ..:-=++=--:...          ........                           
Author: Kevin (Kage) Pyro 

Rust re-write of eyewitness! Currently only grabs screenshots, but the full report is on the to-do.

Usage: rustwitness --urls <URLS> --threads <THREADS> --output-dir <OUTPUT_DIR>

Options:
  -u, --urls <URLS>              path to the file containing urls, one per line
  -t, --threads <THREADS>        Threads to use, there is not upper limit so if you lock up your comptuer that's on you
  -o, --output-dir <OUTPUT_DIR>  path to a folder to save the screenshots to
  -h, --help                     Print help
  -V, --version                  Print version
```

# Install
1.) Download the latest release from github

2.) chmod +x the file you downloaded

3.) move that file to a folder on your path


# Build and install
1.) `git clone https://github.com/Pyro57000/rustwitness.git`

2.) `cd rustwitness`

3.) `cargo build --release`

4.) `sudo cp ./target/release/rustwitness /usr/bin/` (/usr/bin can be any folder on your $PATH variable)

# ToDo
1.) add proxy support for socks and http proxies

2.) figure out how to generate the full eyewitness reqort
