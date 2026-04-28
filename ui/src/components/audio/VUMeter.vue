<script setup lang="ts">

import { onMounted, onUnmounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import type { AudioLevel } from '@shared/AudioLevel';


const SEGMENTS = 20;
const levelLeft = ref(0); // Valeur de 0 à 1

onMounted(async () => {
	// On écoute les mises à jour venant de Rust
	const unlisten = await listen<AudioLevel>('vumeter-update', (event) => {
		// On récupère le RMS ou le Peak
		levelLeft.value = event.payload.rms;
	});

	onUnmounted(() => unlisten());
});

// Calcule si un segment doit être allumé
const isActive = (index: number) => {
	const threshold = (SEGMENTS - index) / SEGMENTS;
	return levelLeft.value >= threshold;
};
</script>

<template>
	<div class="vumeter-container bg-black p-2 rounded-lg w-12 flex justify-center">
		<div class="flex flex-col gap-1">
			<div v-for="i in SEGMENTS" :key="i" class="w-6 h-2 rounded-sm transition-colors duration-75" :class="[
				isActive(i)
					? (i < 4 ? 'bg-red-500' : i < 8 ? 'bg-yellow-400' : 'bg-green-500')
					: 'bg-gray-800'
			]"></div>
		</div>
	</div>
</template>