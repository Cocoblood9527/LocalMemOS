import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { MemoryClient } from "../../node/src/index.js";

export type RegisteredToolName = { name: string };
export type RegisteredTool = {
  name: string;
  handler: (args: any) => Promise<any>;
};

function asTextResult(payload: unknown) {
  return {
    content: [{ type: "text" as const, text: JSON.stringify(payload) }],
  };
}

export function registerMemoryTools(
  server: McpServer,
  client: MemoryClient,
): RegisteredTool[] {
  const tools: RegisteredTool[] = [];

  const upsert = async (args: any) => {
    return asTextResult(await client.upsertFact(args));
  };
  server.tool("memory_upsert_fact", upsert as any);
  tools.push({ name: "memory_upsert_fact", handler: upsert });

  const recall = async (args: any) => {
    return asTextResult(await client.recall(args));
  };
  server.tool("memory_recall", recall as any);
  tools.push({ name: "memory_recall", handler: recall });

  const list = async (args: any) => {
    return asTextResult(await client.list(args));
  };
  server.tool("memory_list", list as any);
  tools.push({ name: "memory_list", handler: list });

  const forget = async (args: any) => {
    return asTextResult(await client.forget(args));
  };
  server.tool("memory_forget", forget as any);
  tools.push({ name: "memory_forget", handler: forget });

  const history = async (args: any) => {
    return asTextResult(await client.history(args));
  };
  server.tool("memory_history", history as any);
  tools.push({ name: "memory_history", handler: history });

  return tools;
}
