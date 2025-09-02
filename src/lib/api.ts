// HTTP API client for Axum backend
import type { ZfsStats } from '@/bindings'

export async function getZfsStats(): Promise<ZfsStats> {
  // Always use HTTP API (Tauri removed on this branch)

  const resp = await fetch('/api/zfs', { headers: { Accept: 'application/json' } })
  if (!resp.ok) {
    const text = await resp.text().catch(() => '')
    throw new Error(text || `HTTP ${resp.status}`)
  }
  return (await resp.json()) as ZfsStats
}

export const runtime = { isTauri: false, isWeb: true }
