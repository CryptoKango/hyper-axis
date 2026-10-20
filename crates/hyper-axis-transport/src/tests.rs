    use std::str::FromStr;

    use serde_json::{Value, json};

    use super::{
        CandleInterval, ExchangeStatus, HostFailureKind, HttpMethod, HttpResponse, PostRequestId,
        RequestKind, ResponseKind, RetryAdvice, Subscription, WsCommand, WsMessageKind,
        WsPostResponseType, classify_host_failure, info_http_request, parse_http_response,
        parse_ws_message,
    };
    use hyper_axis_types::{EvmAddress, Network};

    fn user() -> EvmAddress {
        EvmAddress::from_str("0x7e5f4552091a69125d5dfcb7b8c2659029395bdf").unwrap()
    }

    #[test]
    fn builds_info_request_without_reserializing_body() {
        let body = r#"{ "type": "meta" }"#;
        let request = info_http_request(Network::Testnet, body).unwrap();
        assert_eq!(request.kind, RequestKind::Info);
        assert_eq!(request.method, HttpMethod::Post);
        assert_eq!(request.url, "https://api.hyperliquid-testnet.xyz/info");
        assert_eq!(request.body, body);
        assert_eq!(request.headers["content-type"], "application/json");
    }

    #[test]
    fn preserves_exchange_batch_statuses() {
        let response = HttpResponse {
            status_code: 200,
            body: json!({
                "status": "ok",
                "response": {
                    "type": "order",
                    "data": {"statuses": [{"resting": {"oid": 77_738_308}}, {"error": "Bad price"}]}
                }
            })
            .to_string(),
        };
        let parsed = parse_http_response(RequestKind::SignedAction, &response);
        assert_eq!(parsed.kind, ResponseKind::Success);
        let exchange: super::ExchangeResponse =
            serde_json::from_value(parsed.payload.unwrap()).unwrap();
        assert_eq!(exchange.status, ExchangeStatus::Ok);
        assert_eq!(exchange.response_type(), Some("order"));
        assert_eq!(exchange.statuses().unwrap().len(), 2);
    }

    #[test]
    fn explicit_exchange_rejection_is_never_retried() {
        let response = HttpResponse {
            status_code: 200,
            body: r#"{"status":"err","response":"L1 error: invalid nonce"}"#.to_owned(),
        };
        let parsed = parse_http_response(RequestKind::SignedAction, &response);
        assert_eq!(parsed.kind, ResponseKind::ApiRejected);
        assert_eq!(parsed.retry, RetryAdvice::Never);
        assert_eq!(parsed.message.as_deref(), Some("L1 error: invalid nonce"));
    }

    #[test]
    fn retry_classification_never_blindly_replays_signed_actions() {
        let rate_limited = HttpResponse {
            status_code: 429,
            body: r#"{"code":"rate_limit","msg":"slow down"}"#.to_owned(),
        };
        assert_eq!(
            parse_http_response(RequestKind::Info, &rate_limited).retry,
            RetryAdvice::SafeAfterBackoff
        );
        assert_eq!(
            parse_http_response(RequestKind::SignedAction, &rate_limited).retry,
            RetryAdvice::ReconcileBeforeRetry
        );
        assert_eq!(
            classify_host_failure(RequestKind::SignedAction, HostFailureKind::Timeout),
            RetryAdvice::ReconcileBeforeRetry
        );
        assert_eq!(
            classify_host_failure(RequestKind::Info, HostFailureKind::Cancelled),
            RetryAdvice::Never
        );
        assert_eq!(
            classify_host_failure(RequestKind::SignedAction, HostFailureKind::Cancelled),
            RetryAdvice::ReconcileBeforeRetry
        );
    }

    #[test]
    fn serializes_and_keys_subscriptions_exactly() {
        let subscription = Subscription::UserFills {
            user: user(),
            aggregate_by_time: Some(true),
        };
        let command = WsCommand::Subscribe {
            subscription: subscription.clone(),
        };
        assert_eq!(
            command.to_json().unwrap(),
            r#"{"method":"subscribe","subscription":{"type":"userFills","user":"0x7e5f4552091a69125d5dfcb7b8c2659029395bdf","aggregateByTime":true}}"#
        );
        assert_eq!(
            subscription.key().unwrap(),
            r#"{"type":"userFills","user":"0x7e5f4552091a69125d5dfcb7b8c2659029395bdf","aggregateByTime":true}"#
        );
    }

    #[test]
    fn validates_l2_aggregation_and_candle_interval() {
        let invalid = Subscription::L2Book {
            coin: "BTC".to_owned(),
            n_sig_figs: Some(4),
            mantissa: Some(2),
            fast: Some(true),
        };
        assert!(invalid.validate().is_err());
        let candle = Subscription::Candle {
            coin: "ETH".to_owned(),
            interval: CandleInterval::OneMonth,
        };
        assert_eq!(
            serde_json::to_value(candle).unwrap(),
            json!({"type": "candle", "coin": "ETH", "interval": "1M"})
        );
    }

    #[test]
    fn builds_ping_and_websocket_posts() {
        assert_eq!(WsCommand::Ping.to_json().unwrap(), r#"{"method":"ping"}"#);
        let post =
            WsCommand::info_post(PostRequestId::new(123), r#"{"type":"l2Book","coin":"ETH"}"#)
                .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&post.to_json().unwrap()).unwrap(),
            json!({
                "method": "post",
                "id": 123,
                "request": {"type": "info", "payload": {"type": "l2Book", "coin": "ETH"}}
            })
        );
    }

    #[test]
    fn parses_pong_snapshot_and_post_response() {
        let pong = parse_ws_message(r#"{"channel":"pong"}"#).unwrap();
        assert_eq!(pong.kind(), WsMessageKind::Pong);

        let snapshot =
            parse_ws_message(r#"{"channel":"userFills","data":{"isSnapshot":true,"fills":[]}}"#)
                .unwrap();
        assert!(snapshot.is_snapshot());

        let post = parse_ws_message(
            r#"{"channel":"post","data":{"id":7,"response":{"type":"error","payload":"429: rate limited"}}}"#,
        )
        .unwrap();
        let response = post.post_response().unwrap();
        assert_eq!(response.id.get(), 7);
        assert_eq!(response.response.response_type, WsPostResponseType::Error);
    }
