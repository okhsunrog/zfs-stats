import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { ZfsStats, Dataset } from '@/bindings'
import { getZfsStats } from '@/lib/api'

export const useZfsStore = defineStore('zfs', () => {
  const stats = ref<ZfsStats | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const lastUpdated = ref<Date | null>(null)

  const fetchStats = async () => {
    loading.value = true
    error.value = null

    try {
      const data = await getZfsStats()
      stats.value = data
      lastUpdated.value = new Date()
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch ZFS stats'
      console.error('Failed to fetch ZFS stats:', err)
    } finally {
      loading.value = false
    }
  }

  const refreshStats = () => {
    return fetchStats()
  }

  // Helper functions for data processing
  const getFilesystemsByPool = (poolName: string): Dataset[] => {
    if (!stats.value) return []
    return stats.value.filesystems.filter((fs) => fs.pool === poolName)
  }

  const getSnapshotsByDataset = (datasetName: string): Dataset[] => {
    if (!stats.value) return []
    return stats.value.snapshots.filter((snap) => snap.dataset === datasetName)
  }

  const parseSize = (sizeStr: string): number => {
    if (sizeStr === '-' || sizeStr === '0B') return 0

    const match = sizeStr.match(/^([\d.]+)([KMGTPE]?)B?$/i)
    if (!match) return 0

    const value = parseFloat(match[1])
    const unit = match[2]?.toUpperCase() || ''

    const multipliers: Record<string, number> = {
      '': 1,
      K: 1024,
      M: 1024 ** 2,
      G: 1024 ** 3,
      T: 1024 ** 4,
      P: 1024 ** 5,
      E: 1024 ** 6,
    }

    return Math.round(value * (multipliers[unit] || 1))
  }

  const formatSize = (bytes: number): string => {
    if (bytes === 0) return '0B'

    const units = ['B', 'K', 'M', 'G', 'T', 'P', 'E']
    const threshold = 1024

    let unitIndex = 0
    let size = bytes

    while (size >= threshold && unitIndex < units.length - 1) {
      size /= threshold
      unitIndex++
    }

    if (unitIndex === 0) {
      return `${size}B`
    } else if (size >= 100) {
      return `${size.toFixed(0)}${units[unitIndex]}`
    } else if (size >= 10) {
      return `${size.toFixed(1)}${units[unitIndex]}`
    } else {
      return `${size.toFixed(2)}${units[unitIndex]}`
    }
  }

  const getUsagePercentage = (used: string, available: string): number => {
    const usedBytes = parseSize(used)
    const availableBytes = parseSize(available)
    const total = usedBytes + availableBytes

    if (total === 0) return 0
    return Math.round((usedBytes / total) * 100)
  }

  return {
    stats,
    loading,
    error,
    lastUpdated,
    fetchStats,
    refreshStats,
    getFilesystemsByPool,
    getSnapshotsByDataset,
    parseSize,
    formatSize,
    getUsagePercentage,
  }
})
