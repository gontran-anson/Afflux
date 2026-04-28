<script setup lang="ts">
import { onMounted, ref } from 'vue'

import HelloWorld from './components/HelloWorld.vue'
import TheWelcome from './components/TheWelcome.vue'
import { invoke } from '@tauri-apps/api/core'
import type { AudioDevice } from '../../shared-types/bindings/AudioDevice'

const devices = ref<AudioDevice[]>([]) 

const isStreaming = ref(false)

async function toggleStream(deviceName: string) {
  if (isStreaming.value) {
    invoke('stop_stream')
    isStreaming.value = false
  } else {
    invoke('start_stream', { deviceName })
    isStreaming.value = true
  }
}

const loadDevices = async () => {
  try {
    devices.value = await invoke<AudioDevice[]>('get_input_devices')
    console.log("Mocros detectés: ", devices.value);
  } catch (error) {
    console.error("Error:", error);
  }
  
}

onMounted(() => {
  loadDevices()
})

</script>

<template>
  <header>
    <img alt="Vue logo" class="logo" src="./assets/logo.svg" width="125" height="125" />

    <div class="p-4">
      <h1 class="text-xl font-bold">Sources Audio</h1>
      <ul>
        <li v-for="device in devices" :key="device.name">
          🎤 {{ device.name }}
        </li>
      </ul>
    </div>

    <div class="wrapper">
      <HelloWorld msg="You did it!" />
    </div>
  </header>

  <main>
    <TheWelcome />
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }
}
</style>
