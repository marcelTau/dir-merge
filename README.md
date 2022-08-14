# dir-merge

Dir-merge is a simple tool to compare two directories and get information about duplicated files.  
The're options provided to e.g merge two directories into one and remove all the duplicates.
```
dir-merge -A Directory1 -B Directory2 --merge Directory3
```
This example merges `Directory1` and `Directory2` together and stores the files in `Directory3`.<br>Duplicated files are removed.

## How to install
```
git clone https://github.com/marcelTau/dir-merge.git
cd dir-merge
cargo build --release
sudo mv build/release/merge-tool /usr/bin/dir-merge
```

After installing, open another terminal and you should be able to run it.

