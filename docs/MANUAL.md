# OpenAI Language Teacher v0.1.0

## How to run it?
Download the latest Release, extract it to your local computer and run it.

*Windows:*

`openai-language-teacher.exe`

*Linux:*

`openai-language`

*or:*

`./openai-language-teacher`

*or, if you don't have permission:*

`sudo chmod +x ./openai-language-teacher`

*To use the application it is necessary to have an account on OpenAI and to have a generated API Key.*

- First the application asks for your OpenAI API key. You can see your API keys in your OpanAI account in the 'View API Keys' tab. You can find more information about OpenAI accounts, APIs, and pricing on their official website at [OpenAI](https://openai.com/).
- Next, enter the language you want to learn.
- Then enter your native language.
- And finally, choose a language learning model.

Now you just proceed according to the chosen model.
- For the Conversation model, you just start a conversation with your tutor.
- For the Learning words model, you will translate the words that your tutor generates for you.

### Requirements for compile
To compile the source code on your system, you must have Rust installed to run the application. See [Install Rust](https://rust-lang.org/tools/install) 

*Remove the `openssl` dependency from `Cargo.toml` if you will compile the code under windows.*

## License
This project is licensed under the [Apache License 2.0](https://github.com/mortylen/openai-language-teacher/blob/main/LICENSE) license.
