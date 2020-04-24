# Favdir
Tired of having to constantly spam ```cd``` back and forth when dealing with complex project structures? 
**No more!** 

*Favdir* is a tiny program designed to allow adding favorite directory locations. It is mostly written in Rust, but also leverages a Bash function to circumvent the default functionality of launching a new terminal session when running programs. 

## Installation
To install the latest version, simply run:

```
curl -s https://raw.githubusercontent.com/avjves/Favdir/master/install.sh | bash
```

and you're done!
The install script loads both the binary and bash function files to ~/.fav folder, adds a source command to .bashrc to automatically load the bash function on terminal startup and finally, for your convenience, sources the .bashrc, so you can instantly start using the software!
## Usage
Favdir provides 4 different options: saving/deleting a favorite, list all of them and changing your directory to one.
##### 1. Adding a new favorite
The location is set to current $PWD and the identifier is saved as fav1. If fav1 is already present, it is overwritten.
```
fav s fav1
```
##### 2. Deleting a favorite
Deletes a favorite with the identifier fav2.
```
fav d fav2
```

##### 3. Listing all favorites
Lists all favorites in the format identifier:location, one format per line.
```
fav ls
```

##### 4. Going to a favorite directory
Changes the current terminal location to a favorite with identifier fav3.
```
fav cd fav3
```

