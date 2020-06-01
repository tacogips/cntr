## A simple word counter
Give the word and count the appearance.

The first rust project of mine.

```
> wcounter -dest-file="path/to/file" add some_word
> wcounter -dest-file="path/to/file" show

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
