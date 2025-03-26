use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        ResponsesCompletionResponseStream, CreateResponsesCompletionRequest, CreateResponsesCompletionResponse,
    },
    Client,
};

/// Given a list of messages comprising a conversation, the model will return a response.
///
/// Related guide: [Chat completions](https://platform.openai.com//docs/guides/text-generation)
pub struct Responses<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Responses<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Creates a model response. 
    /// 
    /// Provide [text](https://platform.openai.com/docs/guides/text) or [image](https://platform.openai.com/docs/guides/images) inputs to generate [text](https://platform.openai.com/docs/guides/text) or [JSON](https://platform.openai.com/docs/guides/structured-outputs) outputs. 
    /// 
    /// Have the model call your own [custom code](https://platform.openai.com/docs/guides/function-calling) or use built-in [tools](https://platform.openai.com/docs/guides/tools) like [web search](https://platform.openai.com/docs/guides/tools-web-search) or [file search](https://platform.openai.com/docs/guides/tools-file-search) to use your own data as input for the model's response.
    pub async fn create(
        &self,
        request: CreateResponsesCompletionRequest,
    ) -> Result<CreateResponsesCompletionResponse, OpenAIError> {
        if request.stream.is_some() && request.stream.unwrap() {
            return Err(OpenAIError::InvalidArgument(
                "When stream is true, use Responses::create_stream".into(),
            ));
        }
        self.client.post("/responses", request).await
    }

    /// Creates a streamed mode responses.
    ///
    /// Partial message deltas will be sent, like in ChatGPT.
    /// 
    /// When you [create a Response](https://platform.openai.com/docs/api-reference/responses/create) with stream set to true, the server will emit server-sent events to the client as the Response is generated. This section contains the events that are emitted by the server.
    ///
    /// [Learn more about streaming responses](https://platform.openai.com/docs/guides/streaming-responses?api-mode=responses).
    pub async fn create_stream(
        &self,
        mut request: CreateResponsesCompletionRequest,
    ) -> Result<ResponsesCompletionResponseStream, OpenAIError> {
        if request.stream.is_some() && !request.stream.unwrap() {
            return Err(OpenAIError::InvalidArgument(
                "When stream is false, use Responses::create".into(),
            ));
        }

        request.stream = Some(true);

        Ok(self.client.post_stream("/responses", request).await)
    }
}
