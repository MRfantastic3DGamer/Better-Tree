# About
This will help you better visualize the folder structure of your app

## Instalation

### Prebuilt
just download the executabl file from executables/better-tree


### build from source code

with rust installed


```bash
git clone git@github.com:MRfantastic3DGamer/Better-Tree.git
cd Better-Tree/
cargo build
```

## Use
when in the folder of installed executabl better-tree
```bash
            [req]     [optional]          [optional]      [optional]           [optional]         [optional]
better-tree <folder> <Documentation file> <-n / no-files> <-s / stack-folders> <-H / show-hidden> <-i <relativ path> ignored-locations[0..]>
folder                          : the folder whose structure needs to be built
Documentation file              : the structure will be prented in the codeblock under <!---BETTER_FILES_TREE--> in the specified file
                                to see an example of how you should prepair the documentation file check this README.md
no-files          -n            : if added then no files will be rendered
stack-folders     -s            : if added then folders will be stacked whenever possible
show-hidden       -H            : if added then hidden files and folders will also be rendered
ignored-locations -i <location> : these locations are ignored while rendering

```

## Example of folder structure on this repo
it was bilt with this cummand
```
target/debug/better-tree ./ -i ./target/debug
```
```
target/debug/better-tree ./ -i ./target/debug
```
```
┏ better-tree━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┣output.txt                              ┃
┣Cargo.toml                              ┃
┣Cargo.lock                              ┃
┣┏ target━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃
┃┣┏ release━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ ┃
┃┃┣better-tree                       ┃ ┃ ┃
┃┃┣┏ deps━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ ┃ ┃
┃┃┃┣better_tree-b2c105fd1fc1bef3.d ┃ ┃ ┃ ┃
┃┃┃┣better_tree-b2c105fd1fc1bef3   ┃ ┃ ┃ ┃
┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ ┃ ┃
┃┃┣better-tree.d                     ┃ ┃ ┃
┃┃┣┏ build━┓                         ┃ ┃ ┃
┃┃┃┗━━━━━━━┛                         ┃ ┃ ┃
┃┃┣┏ incremental━┓                   ┃ ┃ ┃
┃┃┃┗━━━━━━━━━━━━━┛                   ┃ ┃ ┃
┃┃┣┏ examples━┓                      ┃ ┃ ┃
┃┃┃┗━━━━━━━━━━┛                      ┃ ┃ ┃
┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ ┃
┃┣CACHEDIR.TAG                         ┃ ┃
┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃
┣README.md                               ┃
┣┏ src━━━━━┓                             ┃
┃┣main.rs  ┃                             ┃
┃┣style.rs ┃                             ┃
┃┗━━━━━━━━━┛                             ┃
┣┏ executables━┓                         ┃
┃┣better-tree  ┃                         ┃
┃┗━━━━━━━━━━━━━┛                         ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```


what are you trying to find scrolling this far down.


while you are here, let me explain why I built this.


This was created as the folder structure created by lenux command tree looks too generic without any options for customizability


# Hope it helps with what ever it is you are doing . Bye
