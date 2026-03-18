import json
from typing import Any

from . import _memory_sdk


class MemoryClient:
    def __init__(self, path: str) -> None:
        self._path = path
        self._core = _memory_sdk.NativeMemoryClient(path)

    def upsert_fact(self, **kwargs: Any) -> dict[str, Any]:
        payload = {
            "namespace": kwargs["namespace"],
            "scope_id": kwargs["scope_id"],
            "entity": kwargs["entity"],
            "attribute": kwargs["attribute"],
            "value": kwargs["value"],
            "confidence": kwargs.get("confidence", 1.0),
            "tags": kwargs.get("tags", []),
            "valid_from": kwargs.get("valid_from"),
            "source_kind": kwargs["source_kind"],
            "source_ref": kwargs.get("source_ref"),
            "evidence_summary": kwargs.get("evidence_summary"),
        }
        return json.loads(self._core.upsert_fact(json.dumps(payload)))

    def recall(self, **kwargs: Any) -> dict[str, Any]:
        payload = {
            "namespace": kwargs["namespace"],
            "scope_id": kwargs["scope_id"],
            "entity": kwargs.get("entity"),
            "attribute": kwargs.get("attribute"),
            "text_query": kwargs.get("text_query"),
            "include_history": kwargs.get("include_history", False),
        }
        return json.loads(self._core.recall(json.dumps(payload)))

    def list(self, **kwargs: Any) -> dict[str, Any]:
        payload = {
            "namespace": kwargs["namespace"],
            "scope_id": kwargs["scope_id"],
            "entity": kwargs.get("entity"),
            "attribute": kwargs.get("attribute"),
        }
        return json.loads(self._core.list(json.dumps(payload)))

    def forget(self, **kwargs: Any) -> dict[str, Any]:
        payload = {
            "namespace": kwargs["namespace"],
            "scope_id": kwargs["scope_id"],
            "entity": kwargs["entity"],
            "attribute": kwargs["attribute"],
        }
        return json.loads(self._core.forget(json.dumps(payload)))

    def history(self, **kwargs: Any) -> "list[dict[str, Any]]":
        payload = {
            "namespace": kwargs["namespace"],
            "scope_id": kwargs["scope_id"],
            "entity": kwargs["entity"],
            "attribute": kwargs["attribute"],
        }
        return json.loads(self._core.history(json.dumps(payload)))
