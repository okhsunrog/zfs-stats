<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import TerminalDisplay from './components/terminal/TerminalDisplay.vue'
import { useTerminalStore } from './stores/terminalStore'
import ThemeToggle from './components/common/ThemeToggle.vue'

const greetMsg = ref('')
const name = ref('')
const terminalStore = useTerminalStore()

async function emitTestLogs() {
  try {
    await invoke('emit_test_logs')
  } catch (e) {
    console.error('emit_test_logs failed', e)
  }
}

async function greet() {
  greetMsg.value = await invoke('greet', { name: name.value })
}
</script>

<template>
  <main class="min-h-screen bg-base-200 p-8">
    <div class="container mx-auto max-w-5xl">
      <div class="flex justify-between items-center mb-6">
        <h1 class="text-3xl font-bold">Tauri + Vue Template</h1>
        <div class="flex gap-2 items-center">
          <ThemeToggle />
          <button class="btn btn-sm" @click="emitTestLogs">Emit Test Logs</button>
          <button class="btn btn-sm" @click="terminalStore.toggleVisibility()">
            {{ terminalStore.isVisible ? 'Hide Terminal' : 'Show Terminal' }}
          </button>
        </div>
      </div>

      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h2 class="card-title">Greet</h2>
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

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">Status</h2>
            <p>Use this card to show app state or device info.</p>
          </div>
        </div>
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">Actions</h2>
            <p>Place quick actions or links here.</p>
          </div>
        </div>
      </div>

      <div class="mt-6" v-show="terminalStore.isVisible">
        <TerminalDisplay />
      </div>
    </div>
  </main>
</template>
