<template>
  <div class="hidden border-r bg-muted/40 md:block">
    <div class="flex h-full max-h-screen flex-col gap-2">
      <div class="flex h-14 items-center border-b px-4 lg:h-[60px] lg:px-6">
        <NuxtLink to="/" class="flex items-center gap-2 font-semibold">
          <Package2 class="h-6 w-6" />
          <span class="">DDevUI</span>
        </NuxtLink>
        <Button variant="outline" size="icon" class="ml-auto h-8 w-8">
          <Bell class="h-4 w-4" />
          <span class="sr-only">Toggle notifications</span>
        </Button>
      </div>
      <div class="flex-1">
        <nav class="grid items-start px-2 gap-1 text-sm font-medium lg:px-4">
          <NuxtLink v-for="menu in menus" :key="menu.label" :to="menu.href"
            class="flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary hover:bg-muted text-muted-foreground"
            exact-active-class="bg-muted text-primary">
            <component :is="menu.icon" class="h-4 w-4" />
            {{ menu.label }}
            <Badge v-if="menu.badge" class="ml-auto flex h-5 w-5 shrink-0 items-center justify-center rounded-full">
              {{ menu.badge }}
            </Badge>
          </NuxtLink>
        </nav>
      </div>
      <div class="mt-auto p-4">
        <div class="grid gap-1">
          <SystemInfoCard title="Operating System" :value="systemInfoStore.os" :icon="Computer" />
          <SystemInfoCard title="Cpu" :value="systemInfoStore.architecture" :icon="Cpu" />
          <SystemInfoCard title="Docker" :value="systemInfoStore.dockerInstalled ? 'Installed' : 'Not Installed'
            " :valueClass="systemInfoStore.dockerInstalled
              ? 'text-green-300'
              : 'text-red-300'
              " :icon="Container" />
          <SystemInfoCard title="Docker" :value="systemInfoStore.dockerRunning ? 'Running' : 'Not Running'" :valueClass="systemInfoStore.dockerRunning ? 'text-green-300' : 'text-red-300'
            " :icon="Power" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import {
  Package2,
  Bell,
  Computer,
  Cpu,
  Container,
  Power,
} from "lucide-vue-next";
import { useSidebarStore } from "@/stores/sidebar";
import { useSystemInfoStore } from "@/stores/systemInfoStore";
import { computed } from "vue";

const sidebarStore = useSidebarStore();
const systemInfoStore = useSystemInfoStore();

const menus = computed(() => sidebarStore.menus);

onMounted(() => {
  const fetchSystemInfoRecursive = () => {
    console.log("fetching system info");
    systemInfoStore.fetchSystemInfo();
    setTimeout(fetchSystemInfoRecursive, 10000);
  };
  fetchSystemInfoRecursive();
});
</script>

<style></style>
