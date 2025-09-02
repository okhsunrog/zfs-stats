<template>
  <div class="space-y-6">
    <!-- Filesystems table -->
    <div>
      <h3 class="text-lg font-semibold mb-4">Filesystems</h3>
      <div class="overflow-x-auto">
        <table class="table table-zebra w-full">
          <thead>
            <tr>
              <th>Name</th>
              <th>Used</th>
              <th>Available</th>
              <th>Referenced</th>
              <th>Mountpoint</th>
              <th>Usage</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="filesystem in filesystems" :key="filesystem.name">
              <td>
                <div class="font-mono text-sm">{{ filesystem.name }}</div>
              </td>
              <td class="font-mono">{{ filesystem.properties.used.value }}</td>
              <td class="font-mono">{{ filesystem.properties.available.value }}</td>
              <td class="font-mono">{{ filesystem.properties.referenced.value }}</td>
              <td>
                <div
                  class="badge badge-outline"
                  v-if="filesystem.properties.mountpoint.value === 'none'">
                  not mounted
                </div>
                <div
                  v-else-if="filesystem.properties.mountpoint.value === '-'"
                  class="text-base-content/50">
                  N/A
                </div>
                <div v-else class="font-mono">{{ filesystem.properties.mountpoint.value }}</div>
              </td>
              <td>
                <div class="w-full max-w-xs">
                  <div
                    class="w-full bg-base-200 rounded-full h-2"
                    :title="`${getUsagePercent(filesystem)}% used`">
                    <div
                      class="h-2 rounded-full transition-all duration-300"
                      :class="getUsageColorClass(getUsagePercent(filesystem))"
                      :style="{ width: `${Math.min(getUsagePercent(filesystem), 100)}%` }"></div>
                  </div>
                  <div class="text-xs text-center mt-1">{{ getUsagePercent(filesystem) }}%</div>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Snapshots section -->
    <div v-if="recentSnapshots.length > 0">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-lg font-semibold">Recent Snapshots</h3>
        <div class="flex gap-2">
          <select v-model="selectedDataset" class="select select-sm select-bordered">
            <option value="">All datasets</option>
            <option v-for="dataset in datasetsWithSnapshots" :key="dataset" :value="dataset">
              {{ dataset }}
            </option>
          </select>
          <button class="btn btn-sm btn-outline" @click="showAllSnapshots = !showAllSnapshots">
            {{ showAllSnapshots ? 'Show Recent' : 'Show All' }}
          </button>
        </div>
      </div>

      <div class="overflow-x-auto">
        <table class="table table-zebra w-full">
          <thead>
            <tr>
              <th>Dataset</th>
              <th>Snapshot Name</th>
              <th>Used</th>
              <th>Referenced</th>
              <th>Created</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="snapshot in displayedSnapshots" :key="snapshot.name">
              <td class="font-mono text-sm">{{ snapshot.dataset }}</td>
              <td>
                <div class="font-mono text-sm">{{ snapshot.snapshot_name }}</div>
              </td>
              <td class="font-mono">{{ snapshot.properties.used.value }}</td>
              <td class="font-mono">{{ snapshot.properties.referenced.value }}</td>
              <td class="text-sm">{{ formatCreateTime(snapshot.createtxg) }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Bookmarks section -->
    <div v-if="bookmarks.length > 0">
      <h3 class="text-lg font-semibold mb-4">Bookmarks</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div v-for="bookmark in bookmarks" :key="bookmark.name" class="card bg-base-200 shadow">
          <div class="card-body p-4 break-words">
            <h4 class="card-title text-sm font-mono whitespace-normal break-words">
              {{ bookmark.name.split('#')[1] || bookmark.name }}
            </h4>
            <div class="text-xs space-y-1">
              <div class="whitespace-normal break-all">
                <span class="font-medium">Dataset:</span>
                {{ bookmark.name.split('#')[0] }}
              </div>
              <div>
                <span class="font-medium">Referenced:</span>
                {{ bookmark.properties.referenced.value }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useZfsStore } from '@/stores/zfsStore'
import type { Dataset } from '@/bindings'

interface Props {
  poolName: string
}

const props = defineProps<Props>()
const zfsStore = useZfsStore()

const selectedDataset = ref('')
const showAllSnapshots = ref(false)

const filesystems = computed(() => {
  return zfsStore.getFilesystemsByPool(props.poolName)
})

const snapshots = computed(() => {
  if (!zfsStore.stats) return []
  return zfsStore.stats.snapshots
    .filter((snap) => snap.pool === props.poolName)
    .sort((a, b) => parseInt(b.createtxg) - parseInt(a.createtxg))
})

const recentSnapshots = computed(() => {
  return snapshots.value.slice(0, 10)
})

const displayedSnapshots = computed(() => {
  let filtered = showAllSnapshots.value ? snapshots.value : recentSnapshots.value

  if (selectedDataset.value) {
    filtered = filtered.filter((snap) => snap.dataset === selectedDataset.value)
  }

  return filtered
})

const datasetsWithSnapshots = computed(() => {
  const datasets = new Set(
    snapshots.value
      .map((snap) => snap.dataset)
      .filter((dataset): dataset is string => Boolean(dataset)),
  )
  return Array.from(datasets).sort()
})

const bookmarks = computed(() => {
  if (!zfsStore.stats) return []
  return zfsStore.stats.bookmarks.filter((bookmark) => bookmark.pool === props.poolName)
})

const getUsagePercent = (filesystem: Dataset): number => {
  return zfsStore.getUsagePercentage(
    filesystem.properties.used.value,
    filesystem.properties.available.value,
  )
}

const getUsageColorClass = (percent: number): string => {
  if (percent >= 90) return 'bg-error'
  if (percent >= 75) return 'bg-warning'
  if (percent >= 50) return 'bg-info'
  return 'bg-success'
}

const formatCreateTime = (createtxg: string): string => {
  // This is a simplified format - in reality you might want to parse the transaction group
  // or use a different timestamp if available
  return `TXG ${createtxg}`
}
</script>
