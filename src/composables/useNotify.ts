import { useMessage } from "naive-ui";
import { errorText } from "@/lib/errors";

export function useNotify() {
  const message = useMessage();

  function success(text: string) {
    message.success(text);
  }

  function warning(text: string) {
    message.warning(text);
  }

  function error(cause: unknown, prefix?: string) {
    const text = errorText(cause);
    message.error(prefix ? `${prefix}: ${text}` : text, { duration: 8000 });
  }

  return { success, warning, error };
}
