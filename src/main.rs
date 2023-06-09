use bpaf::*;
use std::{error::Error, process::Command};

use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Options {
    /// Include the man page contents with the query
    #[bpaf(long, short)]
    include_man: bool,

    /// Use GPT-4 for a better, but more expensive response
    #[bpaf(long, short)]
    gpt_4: bool,

    /// Specify a model by string. Takes precedence over -g
    #[bpaf(long, short)]
    model: Option<String>,

    /// Max tokens used in the response. Default: 512
    #[bpaf(long, short)]
    answer_max_tokens: Option<u16>,

    /// Command in question
    #[bpaf(positional("COMMAND"))]
    command: String,

    /// Your question for alt-man!
    #[bpaf(positional("QUERY"))]
    query: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = options().run();

    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(opts.answer_max_tokens.unwrap_or(512u16))
        .model(opts.model.unwrap_or(String::from(match opts.gpt_4 {
            true => "gpt-4",
            false => "gpt-3.5-turbo",
        })))
        .messages(build_request(
            &opts.command,
            &opts.query.join(" "),
            opts.include_man,
        )?)
        .build()?;

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        println!("{}", choice.message.content,);
    }

    Ok(())
}

/// build_request constructs the messages used to prompt the model.
fn build_request(
    command: &str,
    query: &str,
    include_man: bool,
) -> Result<Vec<ChatCompletionRequestMessage>, Box<dyn Error>> {
    let mut messages = vec![ChatCompletionRequestMessageArgs::default()
        .role(Role::System)
        .content(String::from("Answer the following question about the console command ") + command)
        .build()?];

    if include_man {
        messages.push(
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(
                    String::from("Base your answer on the following content from the man page and do not invent new features or arguments: ")
                        + &read_man_page(command)?,
                )
                .build()?
        );
    }

    messages.push(
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(query)
            .build()?,
    );

    Ok(messages)
}

/// read_man_page will look up the man page for a given command.
fn read_man_page(command_name: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("man")
        .arg(command_name)
        .output()
        .expect("Failed to read man page.");

    let parsed = std::str::from_utf8(&output.stdout)?;

    Ok(String::from(parsed))
}
