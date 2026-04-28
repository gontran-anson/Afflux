<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { AudioDevice } from '@shared/AudioDevice';

// UI state
const devices = ref<AudioDevice[]>([]);
const selectedDevice = ref<string>("");
const isStreaming = ref(false);
const errorMsg = ref<string | null>(null);

const refreshDevices = async () => {
  try {
    devices.value = await invoke<AudioDevice[]>('get_input_devices');
    if (devices.value.length > 0 && !selectedDevice.value) {
      if(devices.value[0]) {
        selectedDevice.value = devices.value[0].name;
      }
    }
  } catch (e) {
    errorMsg.value = "Erreur de détection : " + e;
  }
};

const toggleStream = async () => {
  try {
    errorMsg.value = null;
    if (isStreaming.value) {
      await invoke('stop_stream');
      isStreaming.value = false;
    } else {
      await invoke('start_stream', { deviceName: selectedDevice.value });
      isStreaming.value = true;
    }
  } catch (e) {
    errorMsg.value = "Erreur de flux : " + e;
    isStreaming.value = false;
  }
};

onMounted(refreshDevices);
</script>

<template>
  <div class="p-6 bg-zinc-900 border border-zinc-800 rounded-xl shadow-xl w-80 text-white">
    <h2 class="text-sm font-semibold uppercase tracking-wider mb-4 text-zinc-400">Configuration Audio</h2>

    <div class="mb-6">
      <label class="block text-xs mb-2 text-zinc-500">Entrée Audio</label>
      <select 
        v-model="selectedDevice"
        :disabled="isStreaming"
        class="w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
      >
        <option v-for="device in devices" :key="device.name" :value="device.name">
          {{ device.name }}
        </option>
      </select>
    </div>

    <button 
      @click="toggleStream"
      :class="[
        'w-full py-3 rounded-lg font-bold transition-all active:scale-95',
        isStreaming ? 'bg-red-600 hover:bg-red-500' : 'bg-blue-600 hover:bg-blue-500'
      ]"
    >
      {{ isStreaming ? 'ARRÊTER LA CAPTURE' : 'DÉMARRER LA CAPTURE' }}
    </button>

    <p v-if="errorMsg" class="mt-4 text-xs text-red-400 bg-red-900/20 p-2 rounded border border-red-900/50">
      {{ errorMsg }}
    </p>
  </div>
</template>