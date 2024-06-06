use bytes::Bytes;
use futures::{Stream, StreamExt};
use leptos::server_fn::error::{
    ServerFnError, ServerFnErrorErr, ServerFnErrorSerde, SERVER_FN_ERROR_HEADER,
};
use leptos::server_fn::response::Res;
use pavex::response::Response;
use pavex::http::{HeaderValue, header::SERVER};
use std::pin::Pin;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use typed_builder::TypedBuilder;
/// This is here because the orphan rule does not allow us to implement it on IncomingRequest with
/// the generic error. So we have to wrap it to make it happy
pub struct PavexResponse(pub PavexResponseParts);

#[derive(TypedBuilder)]
pub struct PavexResponseParts {
    pub status_code: u16,
    pub headers: Headers,
    pub body: PavexBody,
}

/// We either can return a fairly simple Box type for normal bodies or a Stream for Streaming
/// server functions
pub enum PavexBody {
    Plain(Vec<u8>),
    Streaming(Pin<Box<dyn Stream<Item = Result<Bytes, Box<dyn std::error::Error>>> + Send>>),
}
impl<CustErr> Res<CustErr> for PavexResponse
where
    CustErr: Send + Sync + Debug + FromStr + Display + 'static,
{
    fn try_from_string(content_type: &str, data: String) -> Result<Self, ServerFnError<CustErr>> {
        let headers =
            Headers::from_list(&[("Content-Type".to_string(), content_type.as_bytes().to_vec())])
                .expect("Failed to create Headers from String Response Input");
        let parts = PavexResponseParts::builder()
            .status_code(200)
            .headers(headers)
            .body(PavexBody::Plain(data.into()))
            .build();
        Ok(PavexResponse(parts))
    }

    fn try_from_bytes(content_type: &str, data: Bytes) -> Result<Self, ServerFnError<CustErr>> {
        let headers = Headers::from_list(&[("Content-Type".to_string(), content_type.into())])
            .expect("Failed to create Headers from Bytes Response Input");
        let parts = PavexResponseParts::builder()
            .status_code(200)
            .headers(headers)
            .body(PavexBody::Plain(data.into()))
            .build();
        Ok(PavexResponse(parts))
    }

    fn try_from_stream(
        content_type: &str,
        data: impl Stream<Item = Result<Bytes, ServerFnError<CustErr>>> + Send + 'static,
    ) -> Result<Self, ServerFnError<CustErr>> {
        let body = data.map(|n| {
            n.map_err(ServerFnErrorErr::from)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        });

        let headers = Headers::from_list(&[("Content-Type".to_string(), content_type.into())])
            .expect("Failed to create Headers from Stream Response Input");
        let parts = PavexResponseParts::builder()
            .status_code(200)
            .headers(headers)
            .body(PavexBody::Streaming(Box::pin(body)))
            .build();
        Ok(PavexResponse(parts))
    }

    fn error_response(path: &str, err: &ServerFnError<CustErr>) -> Self {
        let headers = Headers::from_list(&[(SERVER_FN_ERROR_HEADER.to_string(), path.into())])
            .expect("Failed to create Error Response. This should be impossible");
        let parts = PavexResponseParts::builder()
            .status_code(500)
            .headers(headers)
            .body(PavexBody::Plain(
                err.ser().unwrap_or_else(|_| err.to_string()).into(),
            ))
            .build();
        PavexResponse(parts)
    }

    fn redirect(&mut self, _path: &str) {
        //TODO: Enabling these seems to override location header
        // not sure what's causing that
        //let res_options = expect_context::<ResponseOptions>();
        //res_options.insert_header("Location", path);
        //res_options.set_status(302);
    }
}