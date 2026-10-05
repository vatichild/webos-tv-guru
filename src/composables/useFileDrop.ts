import { onMounted, onUnmounted, ref } from "vue";
import { getCurrentWebview } from "@tauri-apps/api/webview";

/**
 * Files dropped on the window. Tauri gives full paths, which the browser drop event does not.
 */
export function useFileDrop(onDrop: (paths: string[]) => void) {
  const dragging = ref(false);
  let stop: (() => void) | undefined;

  onMounted(async () => {
    stop = await getCurrentWebview().onDragDropEvent(({ payload }) => {
      if (payload.type === "drop") {
        dragging.value = false;
        onDrop(payload.paths);
      } else {
        dragging.value = payload.type !== "leave";
      }
    });
  });

  onUnmounted(() => stop?.());

  return { dragging };
}
