<script setup lang="ts">
import ZfsDashboard from './components/zfs/ZfsDashboard.vue'
import ThemeToggle from './components/common/ThemeToggle.vue'
</script>

<template>
  <main class="min-h-screen bg-base-200 p-8">
    <div class="container mx-auto max-w-7xl">
      <div class="flex justify-between items-center mb-6">
        <div class="flex gap-2 items-center">
          <ThemeToggle />
        </div>
      </div>

      <!-- Main ZFS Dashboard -->
      <ZfsDashboard />

      
    </div>
  </main>
</template>
