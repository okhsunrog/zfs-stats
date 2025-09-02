<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import ZfsDashboard from './components/zfs/ZfsDashboard.vue'
import ThemeToggle from './components/common/ThemeToggle.vue'

const greetMsg = ref('')
const name = ref('')

async function greet() {
  greetMsg.value = await invoke('greet', { name: name.value })
}
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

      <!-- Development/Debug section - collapsible -->
      <div class="collapse collapse-arrow bg-base-100 shadow-xl mt-8">
        <input type="checkbox" />
        <div class="collapse-title text-xl font-medium">Development Tools</div>
        <div class="collapse-content">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="card bg-base-200">
              <div class="card-body">
                <h2 class="card-title">Test Greet Command</h2>
                <form @submit.prevent="greet" class="flex flex-col gap-4">
                  <div class="form-control">
                    <input
                      type="text"
                      v-model="name"
                      placeholder="Enter a name..."
                      class="input input-bordered" />
                  </div>
                  <button type="submit" class="btn btn-primary">Greet</button>
                </form>

                <div v-if="greetMsg" class="mt-4 alert alert-success">
                  {{ greetMsg }}
                </div>
              </div>
            </div>

            <div class="card bg-base-200">
              <div class="card-body">
                <h2 class="card-title">System Info</h2>
                <p>Debug information and system diagnostics can be shown here.</p>
                <div class="mt-4">
                  <div class="badge badge-outline">ZFS Stats App</div>
                  <div class="badge badge-outline ml-2">v0.1.0</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </main>
</template>
