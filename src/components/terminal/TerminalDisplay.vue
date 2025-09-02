<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { Terminal } from '@xterm/xterm'
import { FitAddon } from '@xterm/addon-fit'
import { useTerminalStore } from '../../stores/terminalStore'
import { THEMES } from '../../constants/themes'
import '@xterm/xterm/css/xterm.css'
import type { ITheme } from '@xterm/xterm'

const terminal = ref<Terminal | null>(null)
const terminalElement = ref<HTMLElement | null>(null)
let fitAddon: FitAddon | null = null
const terminalStore = useTerminalStore()

// Track component mount state to prevent duplicate initialization
const isMounted = ref(false)

// --- Theme Logic ---
const isLightTheme = ref(false)

// Helper: Get CSS Variable Value
const getCssVarValue = (varName: string): string => {
  if (typeof window === 'undefined' || !document?.documentElement) return ''
  const value = getComputedStyle(document.documentElement).getPropertyValue(varName).trim()
  return value
}

// Helper: Apply Opacity to OKLCH colors
const applyOpacity = (baseColorVarName: string, alpha: number): string => {
  const baseColor = getCssVarValue(baseColorVarName)

  if (!baseColor || !baseColor.startsWith('oklch')) {
    return `rgba(128, 128, 128, ${alpha})`
  }

  // Extract the OKLCH components (lightness, chroma, hue)
  const match = baseColor.match(/oklch\(([\d.]+)%\s+([\d.]+)\s+([\d.]+)(?:\s+\/\s+[\d.]+)?\)/)
  if (match) {
    const [, l, c, h] = match
    return `oklch(${l}% ${c} ${h} / ${alpha})`
  }

  return `rgba(128, 128, 128, ${alpha})`
}

// Theme Detection
const isCurrentThemeLight = (): boolean => {
  if (typeof document === 'undefined') return false
  const currentTheme = document.documentElement.getAttribute('data-theme')
  return currentTheme === THEMES.LIGHT
}

// Get theme colors
const getThemeColors = (): ITheme => {
  const isLight = isCurrentThemeLight()
  isLightTheme.value = isLight

  const baseVars = {
    background: '--color-base-200',
    foreground: '--color-base-content',
    cursor: '--color-primary',
    selection: '--color-primary',
    neutral: '--color-neutral',
    base100: '--color-base-100',
    base300: '--color-base-300',
  }

  let ansiColors: Partial<ITheme> = {}

  if (isLight) {
    // Light theme colors
    ansiColors = {
      // Standard colors
      red: getCssVarValue('--color-red-600'),
      green: getCssVarValue('--color-green-600'),
      yellow: getCssVarValue('--color-amber-500'),
      blue: getCssVarValue('--color-blue-600'),
      magenta: getCssVarValue('--color-purple-500'),
      cyan: getCssVarValue('--color-cyan-500'),

      // Bright variants
      brightRed: getCssVarValue('--color-red-500'),
      brightGreen: getCssVarValue('--color-green-500'),
      brightYellow: getCssVarValue('--color-amber-400'),
      brightBlue: getCssVarValue('--color-blue-500'),
      brightMagenta: getCssVarValue('--color-purple-400'),
      brightCyan: getCssVarValue('--color-cyan-400'),
    }
  } else {
    // Dark theme colors
    ansiColors = {
      red: getCssVarValue('--color-error'),
      green: getCssVarValue('--color-success'),
      yellow: getCssVarValue('--color-warning'),
      blue: getCssVarValue('--color-info'),
      magenta: getCssVarValue('--color-accent'),
      cyan: getCssVarValue('--color-secondary'),
      brightRed: getCssVarValue('--color-error'),
      brightGreen: getCssVarValue('--color-success'),
      brightYellow: getCssVarValue('--color-warning'),
      brightBlue: getCssVarValue('--color-info'),
      brightMagenta: getCssVarValue('--color-accent'),
      brightCyan: getCssVarValue('--color-secondary'),
    }
  }

  return {
    background: getCssVarValue(baseVars.background),
    foreground: getCssVarValue(baseVars.foreground),
    cursor: getCssVarValue(baseVars.cursor),
    selectionBackground: applyOpacity(baseVars.selection, 0.4),
    selectionForeground: undefined,
    black: getCssVarValue(baseVars.neutral),
    white: getCssVarValue(baseVars.foreground),
    brightBlack: getCssVarValue(baseVars.base300),
    brightWhite: getCssVarValue(baseVars.base100),
    ...ansiColors,
  }
}

// --- Terminal Logic ---
const dimCodeRegex = /\x1b\[2m/g

// (optional) track state if deduplication is needed in future

// Update terminal theme
function updateTerminalTheme() {
  if (!terminal.value) return
  terminal.value.options = {
    ...terminal.value.options,
    theme: getThemeColors(),
  }
}

// Watch for theme changes and apply theme accordingly
const observer = new MutationObserver(() => updateTerminalTheme())

onMounted(async () => {
  if (isMounted.value) return
  isMounted.value = true

  // Initialize xterm
  terminal.value = new Terminal({
    convertEol: true,
    fontFamily:
      'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
    fontSize: 12,
    cursorBlink: false,
    disableStdin: true,
    scrollback: 2000,
    theme: getThemeColors(),
  })
  fitAddon = new FitAddon()
  terminal.value.loadAddon(fitAddon)
  terminal.value.open(terminalElement.value!)
  fitAddon.fit()

  // Observe changes in the data-theme attribute
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] })

  // Initialize log listener
  await terminalStore.initLogListener()

  // Write any existing buffered messages
  await nextTick()
  for (const m of terminalStore.messages) {
    let content = m.content
    // Convert dim ANSI code to bright
    content = content.replace(dimCodeRegex, '\x1b[1m')
    terminal.value.write(content.endsWith('\n') ? content : content + '\n')
  }

  // Handle window resizing
  window.addEventListener('resize', () => fitAddon?.fit())
})

onUnmounted(() => {
  observer.disconnect()
  terminalStore.cleanup()
  terminal.value?.dispose()
  terminal.value = null
})

// Watch for new messages to append
watch(
  () => terminalStore.messages.length,
  (curr, prev) => {
    if (!terminal.value) return
    // Process only new messages
    const newMessages = terminalStore.messages.slice(prev)
    for (const m of newMessages) {
      let content = m.content
      // Convert dim ANSI code to bright
      content = content.replace(dimCodeRegex, '\x1b[1m')
      terminal.value.write(content.endsWith('\n') ? content : content + '\n')
    }
    fitAddon?.fit()
  },
)
</script>

<template>
  <div class="rounded-lg border border-base-300 bg-base-100">
    <div ref="terminalElement" class="p-2" style="height: 250px; width: 100%"></div>
  </div>
  <div class="mt-2 flex gap-2">
    <button class="btn btn-xs" @click="terminalStore.clear()">Clear</button>
  </div>
</template>
