// stores/sidebar.js
import { defineStore } from 'pinia';
import { Home,Blocks,Server,Layers,Settings } from 'lucide-vue-next';

export const useSidebarStore = defineStore('sidebar', {
  state: () => ({
    menus: [
      { label: 'Home', href: '/', icon: Home, badge: null },
      { label: 'Services', href: '/services', icon: Blocks, badge: 6 },
      { label: 'Containers', href: '/containers', icon: Server, badge: null },
      { label: 'Projects', href: '/projects', icon: Layers, badge: null },
      { label: 'Setting', href: '/setting', icon: Settings, badge: null },
    ],
  }),
  actions: {
   
    addMenu(menu) {
      this.menus.push(menu);
    },
    removeMenu(menuHref) {
      this.menus = this.menus.filter((menu) => menu.href !== menuHref);
    },
    updateBadge(menuHref, badge) {
      const menu = this.menus.find((menu) => menu.href === menuHref);
      if (menu) menu.badge = badge;
    },
  },
});
