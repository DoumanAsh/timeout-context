use timeout_context::{time, parse_timeout, try_parse_timeout, ParseError};

#[test]
fn should_parse_valid_timestamp() {
    let data = [
        (time::Duration::from_secs(13), "13S"),
        (time::Duration::from_millis(13), "13m"),
        (time::Duration::from_secs(13 * 60), "13M"),
        (time::Duration::from_secs(13 * 60 * 60), "13H"),
        (time::Duration::from_nanos(13), "13n"),
        (time::Duration::from_micros(13), "13u"),
    ];

    for (expected, input) in data {
        let result = parse_timeout(input.as_bytes());
        assert_eq!(result, expected, "Expected {:?} but got {:?}", expected, result);
    }
}

#[test]
fn should_not_parse_invalid_timestamp() {
    let data = [
        (ParseError::MissingValue, ""),
        (ParseError::MissingValue, "1"),
        (ParseError::InvalidUnit('g'), "1g"),
        (ParseError::InvalidValue("-1"), "-1M"),
    ];

    for (expected, input) in data {
        let result = try_parse_timeout(input.as_bytes()).expect_err("not to parse");
        assert_eq!(result, expected, "Expected {:?} but got {:?}", expected, result);
    }
}

#[cfg(feature = "http")]
#[test]
fn should_pass_timeout_via_http1_headers() {
    use http::Request;
    use timeout_context::TimeoutPropagation;

    use core::time;

    const HEADER: &str = "x-request-deadline";
    const TIMEOUT: time::Duration = time::Duration::from_millis(150);

    let mut request = Request::new(());
    let result = request.get_header_value(HEADER);
    assert!(result.is_none(), "No header should be availalbe yet");
    request.set_timeout_ctx(HEADER, TIMEOUT);

    let result = request.get_timeout_ctx(HEADER).expect("to extract set timeout");
    assert_eq!(result, TIMEOUT);
}

#[cfg(feature = "tonic014")]
#[test]
fn should_pass_timeout_via_tonic014_headers() {
    use tonic014::Request;
    use timeout_context::TimeoutPropagation;

    use core::time;

    const HEADER: &str = "x-request-deadline";
    const TIMEOUT: time::Duration = time::Duration::from_millis(150);

    let mut request = Request::new(());
    let result = request.get_header_value(HEADER);
    assert!(result.is_none(), "No header should be availalbe yet");
    request.set_timeout_ctx(HEADER, TIMEOUT);

    let result = request.get_timeout_ctx(HEADER).expect("to extract set timeout");
    assert_eq!(result, TIMEOUT);
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn should_expire_on_time_tokio() {
    const TIMEOUT: time::Duration = time::Duration::from_millis(150);

    tokio::time::pause();

    let timeout = timeout_context::Timeout::new(tokio::time::Instant::now(), TIMEOUT);
    let fut = timeout.run_tokio(core::future::ready(()));

    fut.await.expect("not expired to success");

    let fut = timeout.run_tokio(core::future::pending::<()>());
    tokio::time::advance(TIMEOUT).await;

    fut.await.expect_err("should expire");
}
