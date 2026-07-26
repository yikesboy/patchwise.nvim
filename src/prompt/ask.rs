use crate::{feature::ask::AskRequest, prompt::Prompt};

pub fn build(request: AskRequest) -> Prompt {
    let current_file = request.current_file.display();
    let question = request.question;

    let ask_prompt_template: String = format!(
        r#"Answer the following question about the codebase in your current working directory. 
        Before answering: 
            - Inspect the relevant files in the repository. 
            - Base the answer on the actual implementation. 
            - Mention relevant file paths and symbols. 
            - Do not modify any files. 
            - Do not propose edits unless the question asks for recommendations. 

        <current_file>
        {} 
        </current_file> 

        <question>
        {}
        </question> 

        Return a clear Markdown answer.
    "#,
        current_file, question
    );

    Prompt::new(ask_prompt_template)
}
