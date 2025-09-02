<template>
  <div class="card bg-base-100 shadow">
    <div class="card-body">
      <h3 class="card-title">Top Datasets by Used</h3>
      <div class="h-80">
        <Bar :data="data" :options="options" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Bar } from 'vue-chartjs'
import { Chart, BarElement, CategoryScale, LinearScale, Tooltip, Legend } from 'chart.js'
import { useZfsStore } from '@/stores/zfsStore'

Chart.register(BarElement, CategoryScale, LinearScale, Tooltip, Legend)

interface Props {
  poolName: string
  topN?: number
}
const props = defineProps<Props>()
const zfs = useZfsStore()

const cssColor = (name: string, fallback: string) => {
  const v = getComputedStyle(document.documentElement).getPropertyValue(name).trim()
  if (!v) return fallback
  if (v.startsWith('#') || v.startsWith('rgb')) return v
  return `hsl(${v})`
}

const ranked = computed(() => {
  const fs = zfs.getFilesystemsByPool(props.poolName)
  const rows = fs.map((f) => ({
    name: f.name,
    used: zfs.parseSize(f.properties.used.value),
  }))
  rows.sort((a, b) => b.used - a.used)
  return rows.slice(0, props.topN ?? 10)
})

const data = computed(() => ({
  labels: ranked.value.map((r) => r.name),
  datasets: [
    {
      label: 'Used',
      data: ranked.value.map((r) => r.used),
      backgroundColor: cssColor('--p', '#3b82f6'),
      borderWidth: 0,
    },
  ],
}))

const options = {
  responsive: true,
  indexAxis: 'y' as const,
  maintainAspectRatio: false,
  scales: {
    x: {
      ticks: {
        callback: (v: unknown) => zfs.formatSize(Number(v)),
      },
      grid: { color: 'rgba(127,127,127,0.2)' },
    },
    y: {
      ticks: {
        autoSkip: false,
        callback: (val: unknown, idx: number) => {
          const label = (data.value.labels?.[idx] as string) || ''
          if (label.length > 40) return label.slice(0, 37) + '…'
          return label
        },
      },
      grid: { display: false },
    },
  },
  plugins: { legend: { display: false } },
}
</script>

