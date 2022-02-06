# This repo is DEPRECATED
I'm using https://github.com/tacogips/path-frecency instead.

## A simple word counter
Give the word and count the appearance.

The first rust project of mine.



##  Install
```
cargo install wcounter
```

##  Usage

Add the words to wcounter, And It show the word list in order of appearances.

```
> wcounter --dest-file="path/to/file" add some_word
> wcounter --dest-file="path/to/file" add some_word2
> wcounter --dest-file="path/to/file" add some_word3
> wcounter --dest-file="path/to/file" add some_word2

# shows by ascending
> wcounter --dest-file="path/to/file" show

some_word
some_word3
some_word2

# shows by descending
> wcounter --dest-file="path/to/file" show --reverse

some_word2
some_word3
some_word
...
```

### Integrated with Zsh and FZF
Inspired by [the entry](http://blog.naichilab.com/entry/zsh-percol)

```
typeset -U chpwd_functions
## save the current directory after cd
CD_HISTORY_FILE=${HOME}/.cd_history_file # cd history filcd history file
function chpwd_record_history() {
    echo $PWD | xargs wcounter --dest-file=${CD_HISTORY_FILE} add
}
chpwd_functions=($chpwd_functions chpwd_record_history)

## load the cd history
function fd(){
    dest=$(wcounter --dest-file=${CD_HISTORY_FILE} show --reverse |  fzf +m --query "$LBUFFER" --prompt="cd > ")
    cd $dest
}


```


### File Locking

To avoid conflict of the output file by written simultaneously by multiple process, wcounter uses [cluFlock](https://crates.io/crates/cluFlock).The lock file will be created at `{dest_file_path}.lock`, and will be reused by another process, so the lock file not deleted automatically.

