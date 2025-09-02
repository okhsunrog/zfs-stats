<template>
  <div class="space-y-6">
    <!-- Header with refresh button -->
    <div class="flex justify-between items-center">
      <div>
        <h1 class="text-3xl font-bold">ZFS Statistics</h1>
        <p v-if="zfsStore.lastUpdated" class="text-sm text-base-content/70">
          Last updated: {{ formatTime(zfsStore.lastUpdated) }}
        </p>
      </div>
      <button
        class="btn btn-primary"
        :class="{ loading: zfsStore.loading }"
        @click="zfsStore.refreshStats()"
        :disabled="zfsStore.loading">
        <svg
          v-if="!zfsStore.loading"
          class="w-4 h-4 mr-2"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
        {{ zfsStore.loading ? 'Loading...' : 'Refresh' }}
      </button>
    </div>

    <!-- Error display -->
    <div v-if="zfsStore.error" class="alert alert-error">
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>{{ zfsStore.error }}</span>
      <button class="btn btn-sm btn-outline" @click="zfsStore.refreshStats()">Retry</button>
    </div>

    <!-- Loading state -->
    <div v-if="zfsStore.loading && !zfsStore.stats" class="flex justify-center py-12">
      <div class="loading loading-spinner loading-lg"></div>
    </div>

    <!-- Main content -->
    <div v-if="zfsStore.stats" class="space-y-6">
      <!-- Overview cards -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <div class="stat bg-base-100 rounded-box shadow">
          <div class="stat-title">Total Pools</div>
          <div class="stat-value text-primary">{{ zfsStore.stats.pools.length }}</div>
          <div class="stat-desc">{{ zfsStore.stats.pools.join(', ') }}</div>
        </div>

        <div class="stat bg-base-100 rounded-box shadow">
          <div class="stat-title">Filesystems</div>
          <div class="stat-value text-secondary">{{ zfsStore.stats.filesystems.length }}</div>
          <div class="stat-desc">Active datasets</div>
        </div>

        <div class="stat bg-base-100 rounded-box shadow">
          <div class="stat-title">Snapshots</div>
          <div class="stat-value text-accent">{{ zfsStore.stats.snapshots.length }}</div>
          <div class="stat-desc">Point-in-time backups</div>
        </div>

        <div class="stat bg-base-100 rounded-box shadow">
          <div class="stat-title">Bookmarks</div>
          <div class="stat-value">{{ zfsStore.stats.bookmarks.length }}</div>
          <div class="stat-desc">Reference markers</div>
        </div>
      </div>

      <!-- Storage overview -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h2 class="card-title">Storage Overview</h2>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div class="space-y-2">
              <div class="flex justify-between">
                <span class="font-medium">Total Used:</span>
                <span class="font-mono">{{ zfsStore.stats.total_used }}</span>
              </div>
              <div class="flex justify-between">
                <span class="font-medium">Total Available:</span>
                <span class="font-mono">{{ zfsStore.stats.total_available }}</span>
              </div>
              <div class="space-y-2">
                <div class="text-sm font-medium">Usage Distribution</div>
                <div class="w-full bg-base-200 rounded-full h-4" :title="`${usagePercent}% used`">
                  <div
                    class="bg-gradient-to-r from-primary to-secondary h-4 rounded-full transition-all duration-300"
                    :style="{ width: `${Math.min(usagePercent, 100)}%` }"></div>
                </div>
                <div class="text-sm text-base-content/70">{{ usagePercent }}% used</div>
              </div>
            </div>
            <div class="md:col-span-2">
              <SpaceOverview />
            </div>
          </div>
        </div>
      </div>

      <!-- Pool tabs -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <div class="tabs tabs-bordered">
            <button
              v-for="pool in zfsStore.stats.pools"
              :key="pool"
              class="tab"
              :class="{ 'tab-active': selectedPool === pool }"
              @click="selectedPool = pool">
              {{ pool }}
            </button>
          </div>

          <div v-if="selectedPool" class="mt-4 space-y-6">
            <DatasetUsageBar :pool-name="selectedPool" :top-n="10" />
            <PoolDetails :pool-name="selectedPool" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useZfsStore } from '@/stores/zfsStore'
import PoolDetails from './PoolDetails.vue'
import SpaceOverview from './charts/SpaceOverview.vue'
import DatasetUsageBar from './charts/DatasetUsageBar.vue'

const zfsStore = useZfsStore()
const selectedPool = ref<string>('')

const usagePercent = computed(() => {
  if (!zfsStore.stats) return 0
  return zfsStore.getUsagePercentage(zfsStore.stats.total_used, zfsStore.stats.total_available)
})

const formatTime = (date: Date): string => {
  return date.toLocaleString()
}

onMounted(async () => {
  await zfsStore.fetchStats()
  // Select first pool by default
  if (zfsStore.stats && zfsStore.stats.pools.length > 0) {
    selectedPool.value = zfsStore.stats.pools[0]
  }
})
</script>
