# alt-man

Mostly a joke project, this is a GPT-powered alternative to the `man` command. Summon the full power of Sam Altman to explain the syntax for obscure commands.

Written in Rust because aren't all the trendy command line tools? Really, I just wanted an excuse to play around with it.

It doens't work quite yet, but it can't be too much longer.

## Usage

Add an OpenAI API key in your environment as `OPENAI_API_KEY`

```
❯ alt-man -h

Usage: [-i] [-g] [-m ARG] [-a ARG] <COMMAND> [<QUERY>]...

Available positional items:
    <COMMAND>  Command in question
    <QUERY>    Your question for alt-man!

Available options:
    -i, --include-man  Include the man page contents with the query
    -g, --gpt-4        Use GPT-4 for a better, but more expensive response
    -m, --model <ARG>  Specify a model by string. Takes precedence over -g
    -a, --answer-max-tokens <ARG>  Max tokens used in the response. Default: 512
    -h, --help         Prints help information
```

Example Request:

```
❯ alt-man git point a branch at a specific hash without checking it out

To point a branch at a specific hash without checking it out, you can use the `git branch` command with the `-f` or `--force` option.

For example, if you want to point the branch `my-branch` at the commit hash `abc123`, you can run the following command:

`git branch -f my-branch abc123`

Note that this will move the branch pointer to the specified commit hash and should be used with caution, as it can potentially overwrite changes.
```