from dataclasses import dataclass
from typing import Any, Optional


@dataclass
class UpsertFactInput:
    namespace: str
    scope_id: str
    entity: str
    attribute: str
    value: Any
    source_kind: str
    confidence: Optional[float] = None
    source_ref: Optional[str] = None
    evidence_summary: Optional[str] = None
