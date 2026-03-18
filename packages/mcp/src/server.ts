import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { MemoryClient } from "../../node/src/index.js";
import {
  registerMemoryTools,
  type RegisteredTool,
  type RegisteredToolName,
} from "./tools.js";

export type LocalMemoryMcpServer = {
  server: McpServer;
  listTools(): RegisteredToolName[];
  callTool(name: string, args: any): Promise<any>;
};

export async function createServer(path: string): Promise<LocalMemoryMcpServer> {
  const client = new MemoryClient(path);
  const server = new McpServer({ name: "local-agent-memory", version: "0.1.0" });
  const tools = registerMemoryTools(server, client);
  const handlers = new Map<string, RegisteredTool>(
    tools.map((tool: RegisteredTool) => [tool.name, tool] as const),
  );

  return {
    server,
    listTools() {
      return tools.map((tool: RegisteredToolName & RegisteredTool) => ({
        name: tool.name,
      }));
    },
    async callTool(name: string, args: any) {
      const tool = handlers.get(name);
      if (!tool) {
        throw new Error(`unknown tool: ${name}`);
      }
      return tool.handler(args);
    },
  };
}
