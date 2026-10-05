import { ref } from "vue";
import { appsApi } from "@/lib/api";
import { fileName, isIpk } from "@/lib/paths";
import type { InstallProgress, InstallTask } from "@/types";
import { useNotify } from "./useNotify";

export function useInstaller() {
  const notify = useNotify();
  const task = ref<InstallTask | null>(null);

  function track(progress: InstallProgress) {
    if (!task.value) return;
    if (progress.stage === "uploading") {
      task.value.percent = progress.total ? Math.round((progress.sent / progress.total) * 100) : 0;
    } else if (progress.stage === "verifying") {
      task.value.status = "Checking the upload";
    } else if (progress.stage === "installing") {
      task.value.status = `Installing ${progress.status}`;
    }
  }

  async function install(device: string, paths: string[]) {
    for (const path of paths.filter(isIpk)) {
      const file = fileName(path);
      task.value = { file, percent: 0, status: "Uploading" };
      try {
        const id = await appsApi.install(device, path, track);
        notify.success(`Installed ${id || file}`);
      } catch (cause) {
        notify.error(cause, file);
      }
    }
    task.value = null;
  }

  return { task, install };
}
