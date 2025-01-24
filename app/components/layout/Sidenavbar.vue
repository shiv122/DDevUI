<template>
  <div class="hidden border-r bg-muted/40 md:block">
    <div class="flex h-full max-h-screen flex-col gap-2">
      <div class="flex h-14 items-center border-b px-4 lg:h-[60px] lg:px-6">
        <a href="/" class="flex items-center gap-2 font-semibold">
          <Package2 class="h-6 w-6" />
          <span class="">Acme Inc</span>
        </a>
        <Button variant="outline" size="icon" class="ml-auto h-8 w-8">
          <Bell class="h-4 w-4" />
          <span class="sr-only">Toggle notifications</span>
        </Button>
      </div>
      <div class="flex-1">
        <nav class="grid items-start px-2 gap-1 text-sm font-medium lg:px-4">
          <a
            v-for="menu in menus"
            :key="menu.label"
            :href="menu.href"
            @click="setActive(menu.href)"
            :class="[
              'flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary hover:bg-muted',
              menu.href === activeMenu
                ? 'bg-muted text-primary'
                : 'text-muted-foreground hover:text-primary',
            ]"
          >
            <component :is="menu.icon" class="h-4 w-4" />
            {{ menu.label }}
            <Badge
              v-if="menu.badge"
              class="ml-auto flex h-5 w-5 shrink-0 items-center justify-center rounded-full"
            >
              {{ menu.badge }}
            </Badge>
          </a>
        </nav>
      </div>
      <!-- <div class="mt-auto p-4">
        <Card>
          <CardHeader class="p-2 pt-0 md:p-4">
            <CardTitle>Upgrade to Pro</CardTitle>
            <CardDescription>
              Unlock all features and get unlimited access to our support team.
            </CardDescription>
          </CardHeader>
          <CardContent class="p-2 pt-0 md:p-4 md:pt-0">
            <Button size="sm" class="w-full"> Upgrade </Button>
          </CardContent>
        </Card>
      </div> -->
    </div>
  </div>
</template>

<script setup>
import { Package2, Bell } from "lucide-vue-next";
import { useSidebarStore } from "@/stores/sidebar";
import { computed } from "vue";

const sidebarStore = useSidebarStore();

const menus = computed(() => sidebarStore.menus);
const activeMenu = computed(() => sidebarStore.activeMenu);

const setActive = (menuHref) => {
  sidebarStore.setActiveMenu(menuHref);
};
</script>

<style></style>
