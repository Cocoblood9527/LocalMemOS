import { createRequire } from "node:module";
import type {
  ForgetInput,
  HistoryInput,
  ListInput,
  RecallInput,
  RecallResult,
  UpsertFactInput,
} from "./models.js";

const require = createRequire(import.meta.url);
const native = require("../memory-node.node") as {
  NativeMemoryClient: new (path: string) => {
    upsertFact(payloadJson: string): string;
    recall(payloadJson: string): string;
    list(payloadJson: string): string;
    forget(payloadJson: string): string;
    history(payloadJson: string): string;
  };
};

function mapFact(raw: any) {
  return {
    ...raw,
    scopeId: raw.scope_id,
    entity: raw.entity,
    attribute: raw.attribute,
    valueText: raw.value_text ?? null,
  };
}

export class MemoryClient {
  private readonly native: InstanceType<typeof native.NativeMemoryClient>;

  constructor(path: string) {
    this.native = new native.NativeMemoryClient(path);
  }

  async upsertFact(input: UpsertFactInput): Promise<Record<string, unknown>> {
    const payload = {
      namespace: input.namespace,
      scope_id: input.scopeId,
      entity: input.entity,
      attribute: input.attribute,
      value: input.value,
      confidence: input.confidence ?? 1.0,
      tags: input.tags ?? [],
      valid_from: input.validFrom ?? null,
      source_kind: input.sourceKind,
      source_ref: input.sourceRef ?? null,
      evidence_summary: input.evidenceSummary ?? null,
    };
    const raw = JSON.parse(this.native.upsertFact(JSON.stringify(payload)));
    return {
      ...raw,
      scopeId: raw.scope_id,
      valueText: raw.value_text ?? null,
    };
  }

  async recall(input: RecallInput): Promise<RecallResult> {
    const payload = {
      namespace: input.namespace,
      scope_id: input.scopeId,
      entity: input.entity ?? null,
      attribute: input.attribute ?? null,
      text_query: input.textQuery ?? null,
      include_history: input.includeHistory ?? false,
      as_of: input.asOf ?? null,
    };
    const raw = JSON.parse(this.native.recall(JSON.stringify(payload)));
    return {
      facts: (raw.facts ?? []).map(mapFact),
    };
  }

  async list(input: ListInput): Promise<RecallResult> {
    const payload = {
      namespace: input.namespace,
      scope_id: input.scopeId,
      entity: input.entity ?? null,
      attribute: input.attribute ?? null,
    };
    const raw = JSON.parse(this.native.list(JSON.stringify(payload)));
    return {
      facts: (raw.facts ?? []).map(mapFact),
    };
  }

  async forget(input: ForgetInput): Promise<{ ok: boolean }> {
    const payload = {
      namespace: input.namespace,
      scope_id: input.scopeId,
      entity: input.entity,
      attribute: input.attribute,
    };
    return JSON.parse(this.native.forget(JSON.stringify(payload)));
  }

  async history(input: HistoryInput): Promise<Array<Record<string, unknown>>> {
    const payload = {
      namespace: input.namespace,
      scope_id: input.scopeId,
      entity: input.entity,
      attribute: input.attribute,
    };
    return JSON.parse(this.native.history(JSON.stringify(payload)));
  }
}
