![application](https://github.com/mortylen/openai-language-teacher/blob/main/img/teacher_02.png?raw=true)

# OpenAI Language Lector

## Corrects, explains, and enhances your conversations. Learn a language with conversation.

**[PROJECT ARTICLE](https://mortylen.hashnode.dev/openai-language-lector)**
**|**
**[LICENSE](https://github.com/mortylen/openai-language-teacher/blob/main/LICENSE)**
**|**
**[DONATE](https://mortylen.hashnode.dev/contact)**

## Project Layout
```
joule-heat/
|--docs                    # Folder for other documents
|--|--MANUAL.md              # Manual for last release     
|--src                     # All source code
|  |--main.rs                # Source code
|  |--model_conversation.rs  # Model for conversation
|  |--model_words.rs  	     # Model for learning words
|  |--openai_executor.rs     # OpenAI API wrapper
|--CODE_OF_CONDUCT.md      # Code of conduct for this project
|--Cargo.toml              # Manifest file for Rust's package manager
|--LICENCE                 # Licence file for this project
|--NOTICE                  # Notice for the licence file
|--README                  # Readme file
```

## Introduction
I am not a native English speaker, but I am eager to improve my language skills. I have considered enrolling in English courses, hiring tutors, or seeking help from my English-speaking friends. However, it occurred to me that I am a programmer, so why not explore the realm of artificial intelligence? That's when I decided to use OpenAI's API. The API is well-documented and easy to use. For my project, I will be using Rust to create a simple console application that allows me to practice English conversations. The application will not only correct my English responses but also engage in extended conversations. Of course, this application is not limited to being just an English language tutor. OpenAI supports over 80 different languages, providing the flexibility to switch to any supported language and enhance proficiency in that language. By the way, this project is not meant for serious foreign language learning; it's more of a fun endeavor during my free time to explore the OpenAI API and enhance my programming skills. :)

**Disclaimer:** *Before I start describing the details, I need to point out that the application utilizes the OpenAI API. OpenAI is a paid service, and I believe that for the first three months after registration, you can enjoy a $5 credit for free. The app requires your API key for authentication. You can see your API keys in your OpanAI account in the **'View API Keys'** tab. You can find more information about OpenAI accounts, APIs, and pricing on their official website at OpenAI.*

## How to run it?
Download the latest Release, extract it to your local computer and run it.
Enter your OpenAI API key, enter the language you want to learn, enter your native language, choose your learning model and have fun.

### Requirements for compile
To compile the source code on your system, you must have Rust installed to run the application. See [Install Rust](https://rust-lang.org/tools/install) 

### Manual
For detailed instructions see the file ***[MANUAL.md](https://github.com/mortylen/openai-language-teacher/blob/main/docs/MANUAL.md)*** or ***MANUAL.txt*** in the ***docs*** folder for the latest release version.

## License
This project is licensed under the [Apache License 2.0](https://github.com/mortylen/openai-language-teacher/blob/main/LICENSE) license.
