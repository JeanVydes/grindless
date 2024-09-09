pub static DEFAULT_SUMMARIZE_PRICE_PER_1000_TOKENS_IN_CREDITS: usize = 1;
pub static DEFAULT_SUMMARIZE_MAX_INPUT_TOKENS: usize = 32768;
pub static DEFAULT_SUMMARIZE_MAX_OUTPUT_TOKENS: usize = 1024;

#[derive(Clone, Debug, PartialEq)]
pub enum SummarizePromptKind {
    Plain,
    MarkdownTable,
    PDF,
    HTML,
    JSON,
}

impl SummarizePromptKind {
    pub fn from_string(kind: &str) -> Option<Self> {
        match kind {
            "plain" => Some(Self::Plain),
            "markdown_table" => Some(Self::MarkdownTable),
            "pdf" => Some(Self::PDF),
            "html" => Some(Self::HTML),
            "json" => Some(Self::JSON),
            _ => None,
        }
    }
}

pub static SUMMARIZE_SYSTEM_PROMPT: &str = "Respond with only the summarized content—concise, direct, and in the same language as the input text and in the indicated format.";
static SUMMARIZE_PROMPT_PLAIN: &str = "Bullet Point";
static SUMMARIZE_PROMPT_MARKDOWN_TABLE: &str = "Markdown Table";
static SUMMARIZE_PROMPT_PDF: &str = "PDF Document";
static SUMMARIZE_PROMPT_HTML: &str = "HTML Static Website";
static SUMMARIZE_PROMPT_JSON: &str = "JSON Object";

pub fn get_summarize_prompt(to_summarize: String, kind: SummarizePromptKind) -> String {
    let max_output = format!(
        "Maximum output tokens: {}\nMaximum output characters: {}",
        DEFAULT_SUMMARIZE_MAX_OUTPUT_TOKENS,
        DEFAULT_SUMMARIZE_MAX_OUTPUT_TOKENS * 4
    );

    let prompt_body = to_summarize.replace("\n", "<br>");

    match kind {
        SummarizePromptKind::Plain => format!(
            "<tokens-config>\n{max_output}\n</tokens-config> \n<format>\n{SUMMARIZE_PROMPT_PLAIN}\n</format> \n<input action=[summarize]>\n{prompt_body}\n</input>",
        ),
        SummarizePromptKind::MarkdownTable => format!(
            "<tokens-config>\n{max_output}\n</tokens-config> \n<format>\n{SUMMARIZE_PROMPT_MARKDOWN_TABLE}\n</format>\n \n<input action=[summarize]>\n{prompt_body}\n</input>",
        ),
        SummarizePromptKind::PDF => format!(
            "<tokens-config>\n{max_output}\n</tokens-config> \n<format>\n{SUMMARIZE_PROMPT_PDF}\n</format> \n<input action=[summarize]>\n{prompt_body}\n</input>",
        ),
        SummarizePromptKind::HTML => format!(
            "<tokens-config>\n{max_output}\n</tokens-config> \n<format>\n{SUMMARIZE_PROMPT_HTML}\n</format> \n<input action=[summarize]>\n{prompt_body}\n</input>",
        ),
        SummarizePromptKind::JSON => format!(
            "<tokens-config>\n{max_output}\n</tokens-config> \n<format>\n{SUMMARIZE_PROMPT_JSON}\n</format> \n<input action=[summarize]>\n{prompt_body}\n</input>",
        ),
    }
}
