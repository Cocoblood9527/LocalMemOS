export interface UpsertFactInput {
  namespace: string;
  scopeId: string;
  entity: string;
  attribute: string;
  value: unknown;
  sourceKind: string;
  confidence?: number;
  tags?: string[];
  validFrom?: string | null;
  sourceRef?: string | null;
  evidenceSummary?: string | null;
}

export interface RecallInput {
  namespace: string;
  scopeId: string;
  entity?: string;
  attribute?: string;
  textQuery?: string;
  // Reserved in v1; forwarded for compatibility but currently ignored by core recall semantics.
  includeHistory?: boolean;
  asOf?: string | null;
}

export interface Fact {
  id: string;
  namespace: string;
  scope_id: string;
  scopeId: string;
  entity: string;
  attribute: string;
  value_json: unknown;
  value_text: string | null;
  valueText: string | null;
  confidence: number | null;
  valid_from: string;
  valid_to: string | null;
  updated_at: string;
}

export interface RecallResult {
  facts: Fact[];
}

export interface ListInput {
  namespace: string;
  scopeId: string;
  entity?: string;
  attribute?: string;
}

export interface ForgetInput {
  namespace: string;
  scopeId: string;
  entity: string;
  attribute: string;
}

export interface HistoryInput {
  namespace: string;
  scopeId: string;
  entity: string;
  attribute: string;
}
