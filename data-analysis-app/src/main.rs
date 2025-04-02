use std::error::Error;
use std::fs::File;
use std::io::{ self, Write };

use csv::Reader;
// use dotenvy::dotenv;
use llm_chain::{ executor, parameters, prompt, step::Step };

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /*
    We create an executor instance using the executor! macro from the llm_chain crate. The macro makes it easy for us to create a new executor for a specific model without having to directly call the constructor functions of the respective executor structs. In short; it allows you to call an LLM with a pre-defined input and output, using multiple steps to refine the output.
    */
    let exec = executor!()?;

    /*
    The snippet below opens a CSV file named "data.csv" in the root folder and reads its contents into a string variable csv_data, where each row is represented as a comma-separated string with a newline character at the end. It also uses the csv crate to handle the CSV parsing. In short, it makes sure that we can the csv data for further actions.
    */
    let file = File::open("data.csv")?;
    let mut reader = Reader::from_reader(file);

    let mut csv_data = String::new();
    for result in reader.records() {
        let record = result?;
        csv_data.push_str(&record.iter().collect::<Vec<_>>().join(","));
        csv_data.push('\n');
    }

    /*
    The user input loop is the loop which the user uses to continuosly ask questions to our helper. The next couple of sections are all happening within this loop -- prompting, executing, outputting the result, etcetera.
    
    To start thing off; we'll be asking the user to enter their question and when the user is done with asking questions, they can type in quit to exit the helper.
    */
    loop {
        println!("Enter your prompt (or 'quit' to exit):");
        io::stdout().flush()?;

        let mut user_prompt = String::new();
        io::stdin().read_line(&mut user_prompt)?;
        user_prompt = user_prompt.trim().to_string();

        if user_prompt.to_lowercase() == "quit" {
            break;
        }

        /*
        Now, we'll create a prompt string that includes the user's question and the CSV data. This prompt will be used by the llm_chain crate to generate a response.

        TIP: When defining prompts, be clear and concise about the task you want the language model to perform. Provide any necessary context or input data (like the CSV example) and be specific about the desired output (eg, a summary, analysis, code, or text generation).
        */
        let prompt_string = format!(
            "You are a data analyst tasked with analyzing a CSV file containing information about individuals, including their name, age, occupation, city, favorite sport, and annual income. Your goal is to provide clear and concise answers to the given questions based on the data provided.

        Question: {}\n\nCSV Data:\n{}",
            user_prompt,
            csv_data
        );

        /*
        We create a Step instance from the llm_chain crate, passing in the prompt string we created earlier.
        
        Steps are individual LLM invocations in a chain. They are a combination of a prompt and a configuration and we use them to set the per-invocation setting for a prompt. This comes in very handy when we want to change the settings for a specific prompt in a chain.
        */
        let step = Step::for_prompt_template(prompt!("{}", &prompt_string));

        // We run the analysis by calling the run method on the Step instance, passing in the parameters and the executor we created earlier.
        let res = step.run(&parameters!(), &exec).await?;

        // print the result
        println!("{}", res.to_immediate().await?.as_content());
    }

    Ok(())
}
