import { Icons } from "../icons/Icons";

interface HeaderProps {
  count: number;
  onClear: () => void;
}

export function Header({ count, onClear }: HeaderProps) {
  return (
    <header className="flex-none mb-4 px-2 pt-2">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold text-white tracking-tight flex items-center gap-2">
          <span className="w-8 h-8 rounded-lg bg-gradient-to-tr from-brand-primary-strong to-brand-accent flex items-center justify-center shadow-lg shadow-brand-primary/20">
            <Icons.Logo className="w-4 h-4 text-white" />
          </span>
          ClipLumina
        </h1>

        <div className="flex items-center gap-2">
          <span className="text-xs font-medium px-2 py-1 rounded-full bg-white/5 border border-white/5 text-brand-text-muted">
            {count} items
          </span>
          {count > 0 && (
            <button
              onClick={onClear}
              className="p-1.5 rounded-lg bg-white/5 hover:bg-white/10 hover:text-red-400 text-brand-text-dim transition-colors border border-white/5"
              title="Clear All (Keep Pinned)"
            >
              <Icons.Trash className="w-4 h-4" />
            </button>
          )}
        </div>
      </div>
    </header>
  );
}
