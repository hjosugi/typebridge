declare module "@tauri-apps/api/core" {
  export function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}

declare function request<T>(cmd: string, args?: unknown): Promise<T>;
