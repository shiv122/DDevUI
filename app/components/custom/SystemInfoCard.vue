<template>
  <Card>
    <div class="flex items-center space-x-4 p-2">
      <span
        class="inline-flex items-center justify-center font-normal text-foreground select-none shrink-0 bg-secondary overflow-hidden h-8 w-8 text-xs rounded-full"
      >
        <component :is="icon" class="size-4" />
      </span>
      <div>
        <p class="text-xs font-medium leading-none">{{ title }}</p>
        <p class="text-xs" :class="valueClass">{{ value }}</p>
      </div>
    </div>
  </Card>
</template>

<script setup>
import { defineProps } from "vue";

defineProps({
  title: {
    type: String,
    required: true,
  },
  value: {
    type: String,
    required: true,
  },
  valueClass: {
    type: String,
    default: "",
  },
  icon: {
    type: [String, Object],
    required: true, // Accept any valid Vue component or tag
  },
});
</script>
