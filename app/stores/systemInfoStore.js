import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export const useSystemInfoStore = defineStore("systemInfo", {
  state: () => ({
    os: "",
    architecture: "",
    isWindows: false,
    dockerInstalled: false,
    dockerRunning: false,
    ddevInstalled: false,
  }),
  actions: {
    // Fetch system info from Tauri and update the store
    async fetchSystemInfo() {
      try {
        const data = JSON.parse(await invoke("get_system_info"));
        this.setSystemInfo(data);
      } catch (error) {
        console.error("Failed to fetch system info:", error);
      }
    },

    // Update system info state
    setSystemInfo(systemInfo) {
      this.os = systemInfo.os;
      this.architecture = systemInfo.architecture;
      this.dockerInstalled = systemInfo.docker_installed;
      this.dockerRunning = systemInfo.docker_running;
      this.ddevInstalled = systemInfo.ddev_installed;
    },
  },
});
