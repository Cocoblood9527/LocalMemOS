import { expect, it } from "vitest";
import { MemoryClient } from "../src/index.js";

it("node sdk preserves core recall and list fields", async () => {
  const client = new MemoryClient(":memory:");
  const firstFact = await client.upsertFact({
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
    value: "npm",
    sourceKind: "manual",
    confidence: 0.6,
  });

  const betweenVersions = String(firstFact.updated_at);
  await new Promise((resolve) => setTimeout(resolve, 10));

  await client.upsertFact({
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
    value: "bun",
    sourceKind: "manual",
    confidence: 0.8,
  });

  const result = await client.recall({
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
  });

  expect(result.facts[0].value_json).toBe("bun");
  expect(result.facts[0].value_text).toBe("bun");
  expect(result.facts[0].confidence).toBe(0.8);
  expect(result.facts[0].valid_from).toBeTruthy();
  expect(result.facts[0]).toHaveProperty("valid_to");
  expect(result.facts[0].updated_at).toBeTruthy();

  const asOfResult = await client.recall({
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
    asOf: betweenVersions,
  });
  expect(asOfResult.facts[0].value_json).toBe("npm");

  const listed = await client.list({
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
  });
  expect(listed.facts[0].value_json).toBe("bun");
  expect(listed.facts[0].updated_at).toBeTruthy();

  const history = await client.history({
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
  });
  expect(history.length).toBe(2);

  const forgetResult = await client.forget({
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
  });
  expect(forgetResult.ok).toBe(true);

  const afterForget = await client.recall({
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
  });
  expect(afterForget.facts).toEqual([]);
});
