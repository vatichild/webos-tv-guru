import { useDialog } from "naive-ui";

export interface ConfirmOptions {
  title: string;
  content: string;
  confirmText: string;
}

export function useConfirm() {
  const dialog = useDialog();

  return function confirm(options: ConfirmOptions): Promise<boolean> {
    return new Promise((resolve) => {
      dialog.warning({
        title: options.title,
        content: options.content,
        positiveText: options.confirmText,
        negativeText: "Cancel",
        onPositiveClick: () => resolve(true),
        onNegativeClick: () => resolve(false),
        onClose: () => resolve(false),
        onMaskClick: () => resolve(false),
      });
    });
  };
}
