<template>
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <div class="card bg-base-100 shadow">
      <div class="card-body">
        <h3 class="card-title">Space Overview</h3>
        <div class="h-56">
          <Doughnut :data="spaceOverviewData" :options="doughnutOpts" />
        </div>
        <p class="text-xs text-base-content/60">
          Note: dataset and snapshot usage can overlap in ZFS (CoW).
        </p>
      </div>
    </div>
    <div class="card bg-base-100 shadow">
      <div class="card-body">
        <h3 class="card-title">Datasets Usage Breakdown</h3>
        <div class="h-56">
          <Doughnut :data="datasetsBreakdownData" :options="doughnutOpts" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Doughnut } from 'vue-chartjs'
import { Chart, ArcElement, Tooltip, Legend } from 'chart.js'
import { useZfsStore } from '@/stores/zfsStore'

Chart.register(ArcElement, Tooltip, Legend)

const zfs = useZfsStore()

const cssColor = (name: string, fallback: string) => {
  const v = getComputedStyle(document.documentElement).getPropertyValue(name).trim()
  if (!v) return fallback
  if (v.startsWith('#') || v.startsWith('rgb')) return v
  return `hsl(${v})`
}

const totals = computed(() => {
  if (!zfs.stats) return { used: 0, free: 0 }
  const used = zfs.parseSize(zfs.stats.total_used)
  const free = zfs.parseSize(zfs.stats.total_available)
  return { used, free }
})

const datasetsUsed = computed(() => {
  if (!zfs.stats) return 0
  return zfs.stats.filesystems.reduce(
    (a, d) => a + zfs.parseSize(d.properties.used.value),
    0,
  )
})

const snapshotsUsed = computed(() => {
  if (!zfs.stats) return 0
  return zfs.stats.snapshots.reduce(
    (a, s) => a + zfs.parseSize(s.properties.used.value),
    0,
  )
})

const spaceOverviewData = computed(() => ({
  labels: ['Free', 'Datasets', 'Snapshots'],
  datasets: [
    {
      data: [totals.value.free, datasetsUsed.value, snapshotsUsed.value],
      backgroundColor: [
        cssColor('--b3', '#94a3b8'),
        cssColor('--p', '#3b82f6'),
        cssColor('--a', '#22c55e'),
      ],
      borderWidth: 0,
    },
  ],
}))

const genPalette = (n: number) => {
  const colors: string[] = []
  for (let i = 0; i < n; i++) {
    const hue = Math.round((360 * i) / Math.max(1, n))
    colors.push(`hsl(${hue} 70% 55%)`)
  }
  return colors
}

const datasetsBreakdownData = computed(() => {
  if (!zfs.stats || zfs.stats.filesystems.length === 0) {
    return { labels: [], datasets: [] }
  }
  const labels = zfs.stats.filesystems.map((f) => f.name)
  const data = zfs.stats.filesystems.map((f) => zfs.parseSize(f.properties.used.value))
  const colors = genPalette(labels.length)
  return {
    labels,
    datasets: [
      {
        data,
        backgroundColor: colors,
        borderWidth: 0,
      },
    ],
  }
})

const doughnutOpts = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: { legend: { position: 'bottom' as const } },
}
</script>
