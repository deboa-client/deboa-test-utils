use caramelo::{expect, matchers::eq};
use deboa::{
    Client, InnerClient, TestResult,
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    request::DeboaRequest,
};
use easyhttpmock::{
    EasyHttpMock,
    matchers::{method, path},
    mock::{AsyncMatcherExt, Mock, StatusCodeExt, given},
    server::ServerAdapter,
};
use http::{Method, StatusCode};

pub async fn test_delete<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method(Method::DELETE).and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let response = DeboaRequest::delete(server.url("/posts/1"))?
        .version(protocol_version)
        .send_with(client)
        .await?;

    expect(response.status()).to_be(eq(StatusCode::OK));

    server
        .stop()
        .await?;

    Ok(())
}
