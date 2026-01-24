import { Icons } from "../icons/Icons";

export function EmptyState() {
  return (
    <div className="h-full flex flex-col items-center justify-center text-brand-text-dim text-center opacity-60">
      <Icons.Empty className="w-12 h-12 mb-3 text-brand-text-muted/20" />
      <p className="text-sm select-none">Clipboard is empty</p>
      <p className="text-xs mt-1 select-none">Copy something to see it here</p>
    </div>
  );
}
