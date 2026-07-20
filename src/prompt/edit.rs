use crate::feature::edit::EditRequest;
use crate::prompt::Prompt;

pub fn build(request: &EditRequest) -> Prompt {
    let file_path = request.file_path.to_string_lossy();

    let edit_prompt_text: String = format!(
        r#"
            Replace the selected source text according to the given instruction.

            <instruction>
            {}
            </instruction>

            <file_path>
            {}{}
            </file_path>

            <selected_text>
            {}
            </selected_text>

            <file_context>
            {}
            </file_context>

            Return only the exact replacement text.
            Preserve indentation appropriate for the selected location and context. 
        "#,
        request.instruction, &file_path, request.file_type, request.selection, request.context,
    );

    Prompt::new(edit_prompt_text)
}
