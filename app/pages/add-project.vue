<template>
  <div class="folder-picker">
    <button @click="selectFolder">Select Folder</button>
    <p v-if="folderPath">Selected Folder: {{ folderPath }}</p>
  </div>
</template>

<script setup>
import { open } from '@tauri-apps/plugin-dialog';

const folderPath = ref(null);

const selectFolder = async () => {
  try {
    const selectedFolder = await open({
      directory: true,
      multiple: false,
      title: "Select a Folder",
    });
    if (selectedFolder) {
      folderPath.value = selectedFolder;
    }
  } catch (error) {
    console.error('Error selecting folder:', error);
  }
};
</script>

<style scoped></style>
