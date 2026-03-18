use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn upsert_and_recall_work_over_http() {
    let app = memory_http::app_for_test(":memory:").await;

    let upsert_payload_npm = serde_json::json!({
        "namespace": "workspace",
        "scope_id": "localmemos",
        "entity": "project",
        "attribute": "preferred_package_manager",
        "value": "npm",
        "confidence": 1.0,
        "tags": [],
        "valid_from": null,
        "source_kind": "manual",
        "source_ref": "test",
        "evidence_summary": null
    });
    let upsert_req = Request::builder()
        .method("POST")
        .uri("/facts:upsert")
        .header("content-type", "application/json")
        .body(Body::from(upsert_payload_npm.to_string()))
        .unwrap();
    let upsert_resp = app.clone().oneshot(upsert_req).await.unwrap();
    assert_eq!(upsert_resp.status(), StatusCode::OK);

    let upsert_payload_bun = serde_json::json!({
        "namespace": "workspace",
        "scope_id": "localmemos",
        "entity": "project",
        "attribute": "preferred_package_manager",
        "value": "bun",
        "confidence": 1.0,
        "tags": [],
        "valid_from": null,
        "source_kind": "manual",
        "source_ref": "test",
        "evidence_summary": null
    });
    let upsert_req = Request::builder()
        .method("POST")
        .uri("/facts:upsert")
        .header("content-type", "application/json")
        .body(Body::from(upsert_payload_bun.to_string()))
        .unwrap();
    let upsert_resp = app.clone().oneshot(upsert_req).await.unwrap();
    assert_eq!(upsert_resp.status(), StatusCode::OK);
    let upsert_body = to_bytes(upsert_resp.into_body(), usize::MAX).await.unwrap();
    let upsert_parsed: serde_json::Value = serde_json::from_slice(&upsert_body).unwrap();
    let current_fact_id = upsert_parsed["id"].as_str().unwrap().to_string();

    let recall_payload = serde_json::json!({
        "namespace": "workspace",
        "scope_id": "localmemos",
        "entity": "project",
        "attribute": "preferred_package_manager",
        "text_query": null,
        "include_history": false
    });
    let recall_req = Request::builder()
        .method("POST")
        .uri("/facts:recall")
        .header("content-type", "application/json")
        .body(Body::from(recall_payload.to_string()))
        .unwrap();
    let recall_resp = app.clone().oneshot(recall_req).await.unwrap();
    assert_eq!(recall_resp.status(), StatusCode::OK);

    let body = to_bytes(recall_resp.into_body(), usize::MAX).await.unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(parsed["facts"][0]["value_text"], "bun");

    let history_req = Request::builder()
        .method("GET")
        .uri(format!("/facts/{current_fact_id}/history"))
        .body(Body::empty())
        .unwrap();
    let history_resp = app.oneshot(history_req).await.unwrap();
    assert_eq!(history_resp.status(), StatusCode::OK);

    let history_body = to_bytes(history_resp.into_body(), usize::MAX).await.unwrap();
    let history_parsed: serde_json::Value = serde_json::from_slice(&history_body).unwrap();
    assert_eq!(history_parsed.as_array().unwrap().len(), 2);
    assert_eq!(history_parsed[0]["value_text"], "npm");
    assert_eq!(history_parsed[1]["value_text"], "bun");
}

#[tokio::test]
async fn history_returns_404_for_unknown_fact_id() {
    let app = memory_http::app_for_test(":memory:").await;
    let req = Request::builder()
        .method("GET")
        .uri("/facts/fact-missing/history")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
