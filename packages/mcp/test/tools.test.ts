import { expect, it } from "vitest";
import { createServer } from "../src/server.js";

it("registers all v1 memory tools", async () => {
  const server = await createServer(":memory:");
  const tools = server.listTools();
  expect(tools.map((tool: { name: string }) => tool.name)).toEqual([
    "memory_upsert_fact",
    "memory_recall",
    "memory_list",
    "memory_forget",
    "memory_history",
  ]);
});

it("mcp tools match core write and recall semantics", async () => {
  const server = await createServer(":memory:");

  const firstUpsert = await server.callTool("memory_upsert_fact", {
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
    value: "npm",
    sourceKind: "manual",
    confidence: 0.6,
  });

  const betweenVersions = JSON.parse(firstUpsert.content[0].text).updated_at;
  await new Promise((resolve) => setTimeout(resolve, 10));

  await server.callTool("memory_upsert_fact", {
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
    value: "bun",
    sourceKind: "manual",
    confidence: 0.8,
  });

  const recallResult = await server.callTool("memory_recall", {
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
  });
  const recallParsed = JSON.parse(recallResult.content[0].text);
  expect(recallParsed.facts[0].value_json).toBe("bun");
  expect(recallParsed.facts[0].confidence).toBe(0.8);
  expect(recallParsed.facts[0].valid_from).toBeTruthy();
  expect(recallParsed.facts[0]).toHaveProperty("valid_to");
  expect(recallParsed.facts[0].updated_at).toBeTruthy();

  const asOfResult = await server.callTool("memory_recall", {
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
    asOf: betweenVersions,
  });
  const asOfParsed = JSON.parse(asOfResult.content[0].text);
  expect(asOfParsed.facts[0].value_json).toBe("npm");

  const listResult = await server.callTool("memory_list", {
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
  });
  const listParsed = JSON.parse(listResult.content[0].text);
  expect(listParsed.facts[0].value_json).toBe("bun");
  expect(listParsed.facts[0].updated_at).toBeTruthy();

  const historyResult = await server.callTool("memory_history", {
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
  });
  const historyParsed = JSON.parse(historyResult.content[0].text);
  expect(historyParsed.length).toBe(2);
});
