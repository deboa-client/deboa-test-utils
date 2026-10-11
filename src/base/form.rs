use caramelo::{expect, matchers::eq};
use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    form::EncodedForm,
    request::DeboaRequest,
    Client, InnerClient, TestResult,
};
use easyhttpmock::{
    matchers::{header_value, method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::{header::CONTENT_TYPE, Method, StatusCode};

pub fn test_encoded_form() -> TestResult<()> {
    let form = EncodedForm::builder()
        .field("name", "deboa")
        .field("version", "0.0.1");

    let form = form.build();

    assert_eq!(form.to_vec(), b"name=deboa&version=0.0.1");

    Ok(())
}

pub async fn test_submit_encoded_form<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(
            method(Method::POST)
                .and(header_value(CONTENT_TYPE, "application/x-www-form-urlencoded"))
                .and(path("/posts")),
        )
        .will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let form = EncodedForm::builder()
        .field("name", "deboa")
        .field("version", "0.0.1");

    let response = DeboaRequest::post(server.url("/posts"))?
        .header(CONTENT_TYPE, form.content_type())?
        .bytes(&form.build())
        .send_with(client)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::OK));

    server
        .stop()
        .await?;

    Ok(())
}
