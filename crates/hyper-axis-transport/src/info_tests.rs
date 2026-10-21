use serde_json::{Value, json};

use super::{
    InfoQuery, OrderStatusSelector, parse_perp_dexs_response,
    parse_perp_meta_and_asset_ctxs_response, parse_perp_meta_response, prepare_info_query,
};
use crate::CandleInterval;

const USER: &str = "0x000000000000000000000000000000000000beef";

#[test]
fn serializes_exact_numeric_wire_values() {
    let query = serde_json::from_value::<InfoQuery>(json!({
        "type": "userFillsByTime",
        "user": USER,
        "startTime": "9007199254740993",
        "endTime": "9007199254741993",
        "aggregateByTime": true,
    }))
    .unwrap();
    let prepared = prepare_info_query(query).unwrap();
    assert_eq!(prepared.query_type, "userFillsByTime");
    assert_eq!(
        serde_json::from_str::<Value>(&prepared.body).unwrap(),
        json!({
            "type": "userFillsByTime",
            "user": USER,
            "startTime": 9_007_199_254_740_993_u64,
            "endTime": 9_007_199_254_741_993_u64,
            "aggregateByTime": true,
        })
    );
}

#[test]
fn covers_nested_and_polymorphic_queries() {
    let candle = prepare_info_query(InfoQuery::CandleSnapshot {
        coin: "BTC".to_owned(),
        interval: CandleInterval::FifteenMinutes,
        start_time: 100,
        end_time: 200,
    })
    .unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&candle.body).unwrap(),
        json!({
            "type": "candleSnapshot",
            "req": {"coin": "BTC", "interval": "15m", "startTime": 100, "endTime": 200},
        })
    );

    for (selector, expected) in [
        (OrderStatusSelector::Oid { oid: 42 }, json!(42)),
        (
            OrderStatusSelector::Cloid {
                cloid: "0x1234567890abcdef1234567890abcdef".to_owned(),
            },
            json!("0x1234567890abcdef1234567890abcdef"),
        ),
    ] {
        let prepared = prepare_info_query(InfoQuery::OrderStatus {
            user: USER.to_owned(),
            selector,
        })
        .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&prepared.body).unwrap()["oid"],
            expected
        );
    }
}

#[test]
fn rejects_invalid_ranges_addresses_and_l2_aggregation() {
    assert!(
        prepare_info_query(InfoQuery::FundingHistory {
            coin: "BTC".to_owned(),
            start_time: 2,
            end_time: Some(1),
        })
        .is_err()
    );
    assert!(
        prepare_info_query(InfoQuery::UserFills {
            user: "invalid".to_owned(),
        })
        .is_err()
    );
    assert!(
        prepare_info_query(InfoQuery::L2Book {
            coin: "BTC".to_owned(),
            n_sig_figs: Some(4),
            mantissa: Some(2),
        })
        .is_err()
    );
}

#[test]
fn scopes_meta_and_asset_contexts_without_breaking_default_wire_shape() {
    let default_query = serde_json::from_value::<InfoQuery>(json!({
        "type": "metaAndAssetCtxs"
    }))
    .unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&prepare_info_query(default_query).unwrap().body).unwrap(),
        json!({"type": "metaAndAssetCtxs"})
    );

    let hip3_query = serde_json::from_value::<InfoQuery>(json!({
        "type": "metaAndAssetCtxs",
        "dex": "xyz"
    }))
    .unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&prepare_info_query(hip3_query).unwrap().body).unwrap(),
        json!({"type": "metaAndAssetCtxs", "dex": "xyz"})
    );

    assert!(
        prepare_info_query(InfoQuery::MetaAndAssetCtxs {
            dex: Some("  ".to_owned()),
        })
        .is_err()
    );
}

#[test]
fn parses_typed_hip3_market_responses() {
    let dexes = parse_perp_dexs_response(
        r#"[null,{"name":"xyz","fullName":"XYZ","deployer":"0x01","oracleUpdater":null,"feeRecipient":"0x02","assetToStreamingOiCap":[["xyz:TSLA","200000000.0"]],"subDeployers":[],"assetToFundingMultiplier":[["xyz:TSLA","0.5"]],"assetToFundingInterestRate":[],"assetToFundingClamp":[]}]"#,
    )
    .unwrap();
    assert!(dexes[0].is_none());
    assert_eq!(dexes[1].as_ref().unwrap().name, "xyz");

    let meta_json = r#"{"universe":[{"name":"xyz:TSLA","szDecimals":3,"maxLeverage":20,"marginTableId":20,"growthMode":"enabled","deployerFeeScale":"1.0"}],"collateralToken":0,"marginTables":[[20,{"description":"","marginTiers":[{"lowerBound":"0.0","maxLeverage":20}]}]]}"#;
    let meta = parse_perp_meta_response(meta_json).unwrap();
    assert_eq!(meta.universe[0].hip3_parts(), Some(("xyz", "TSLA")));

    let response = parse_perp_meta_and_asset_ctxs_response(&format!(
        r#"[{meta_json},[{{"funding":"0.0001","openInterest":"10.5","prevDayPx":"300.0","dayNtlVlm":"1000.0","oraclePx":"301.0","markPx":"301.1","midPx":"301.2","impactPxs":["301.0","301.3"]}}]]"#
    ))
    .unwrap();
    let (market, context) = response.markets().next().unwrap();
    assert_eq!(market.name, "xyz:TSLA");
    assert_eq!(context.mark_px, "301.1");

    let mismatched = format!(r"[{meta_json},[]]");
    assert!(parse_perp_meta_and_asset_ctxs_response(&mismatched).is_err());
}

#[test]
#[allow(clippy::too_many_lines)]
fn prepares_every_official_info_wire_type() {
    let user = || USER.to_owned();
    let queries = vec![
        InfoQuery::ClearinghouseState {
            user: user(),
            dex: String::new(),
        },
        InfoQuery::SpotClearinghouseState { user: user() },
        InfoQuery::OpenOrders {
            user: user(),
            dex: String::new(),
        },
        InfoQuery::FrontendOpenOrders {
            user: user(),
            dex: String::new(),
        },
        InfoQuery::AllMids { dex: String::new() },
        InfoQuery::UserFills { user: user() },
        InfoQuery::UserFillsByTime {
            user: user(),
            start_time: 1,
            end_time: Some(2),
            aggregate_by_time: false,
        },
        InfoQuery::Meta { dex: String::new() },
        InfoQuery::MetaAndAssetCtxs { dex: None },
        InfoQuery::PerpDexs,
        InfoQuery::SpotMeta,
        InfoQuery::SpotMetaAndAssetCtxs,
        InfoQuery::FundingHistory {
            coin: "BTC".to_owned(),
            start_time: 1,
            end_time: None,
        },
        InfoQuery::UserFunding {
            user: user(),
            start_time: 1,
            end_time: None,
        },
        InfoQuery::L2Book {
            coin: "BTC".to_owned(),
            n_sig_figs: Some(5),
            mantissa: Some(2),
        },
        InfoQuery::CandleSnapshot {
            coin: "BTC".to_owned(),
            interval: CandleInterval::OneHour,
            start_time: 1,
            end_time: 2,
        },
        InfoQuery::UserFees { user: user() },
        InfoQuery::DelegatorSummary { user: user() },
        InfoQuery::Delegations { user: user() },
        InfoQuery::DelegatorRewards { user: user() },
        InfoQuery::DelegatorHistory { user: user() },
        InfoQuery::OrderStatus {
            user: user(),
            selector: OrderStatusSelector::Oid { oid: 42 },
        },
        InfoQuery::Referral { user: user() },
        InfoQuery::SubAccounts { user: user() },
        InfoQuery::UserToMultiSigSigners { user: user() },
        InfoQuery::PerpDeployAuctionStatus,
        InfoQuery::UserDexAbstraction { user: user() },
        InfoQuery::UserAbstraction { user: user() },
        InfoQuery::HistoricalOrders { user: user() },
        InfoQuery::UserNonFundingLedgerUpdates {
            user: user(),
            start_time: 1,
            end_time: Some(2),
        },
        InfoQuery::Portfolio { user: user() },
        InfoQuery::UserTwapSliceFills { user: user() },
        InfoQuery::UserVaultEquities { user: user() },
        InfoQuery::UserRole { user: user() },
        InfoQuery::UserRateLimit { user: user() },
        InfoQuery::SpotDeployState { user: user() },
        InfoQuery::ExtraAgents { user: user() },
    ];
    assert_eq!(queries.len(), 37);
    for query in queries {
        let expected_type = query.query_type();
        let prepared = prepare_info_query(query).unwrap();
        let body = serde_json::from_str::<Value>(&prepared.body).unwrap();
        assert_eq!(body["type"], expected_type);
        assert_eq!(prepared.query_type, expected_type);
    }
}
