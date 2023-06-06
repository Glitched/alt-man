use bpaf::*;
use std::error::Error;

use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Options {
    // #[bpaf(long, short, argument("query"))]
    // query: Vec<String>,
    /// Include the man page contents with the query
    #[bpaf(long, short)]
    include_man: bool,
    /// Use GPT-4 for a better, but more expensive response
    #[bpaf(long, short)]
    gpt_4: bool,
    #[bpaf(long, short)]
    model: Option<String>,
    // Command in question
    #[bpaf(positional("COMMAND"))]
    command: String,
    #[bpaf(positional("QUERY"))]
    query: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = options().run();
    println!("{:#?}", opts);
    let model = select_model(opts.model, opts.gpt_4);
    println!("Model: {:#?}", model);

    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model(model)
        .messages(build_request(&opts.command, &opts.query.join(" "))?)
        .build()?;

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    Ok(())
}

/// select_model returns the OpenAI model to use based on the users input.
/// The model string overrides the use_gpt_4 flag if both are set.
fn select_model(user_specified_model: Option<String>, use_gpt_4: bool) -> String {
    return match user_specified_model {
        Some(s) => s,
        None => match use_gpt_4 {
            false => String::from("gpt-3.5-turbo"),
            true => String::from("gpt-4"),
        },
    };
}

/// build_request constructs the messages used to prompt the model.
fn build_request(
    command: &str,
    query: &str,
) -> Result<Vec<ChatCompletionRequestMessage>, Box<dyn Error>> {
    return Ok(vec![
        ChatCompletionRequestMessageArgs::default()
            .role(Role::System)
            .content(
                String::from("Answer the following question about the console command ") + command,
            )
            .build()?,
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(query)
            .build()?,
    ]);
}
