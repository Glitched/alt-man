# alt-man

Mostly a joke project, this is a GPT-powered alternative to the `man` command. Summon the full power of Sam Altman to explain the syntax for obscure commands.

Written in Rust because aren't all the trendy command line tools? Really, I just wanted an excuse to play around with it.

It doens't work quite yet, but it can't be too much longer.

## Usage

Add an OpenAI API key in your environment as `OPENAI_API_KEY`

```bash
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