import { ClipItem } from "../../hooks/useClipboardHistory";
import { Icons } from "../icons/Icons";
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

interface ClipItemProps {
  item: ClipItem;
  onCopy: (content: string) => void;
  onDelete: (id: string, e: React.MouseEvent) => void;
  onPin: (id: string, e: React.MouseEvent) => void;
}

export function ClipItemCard({ item, onCopy, onDelete, onPin }: ClipItemProps) {
  const isImage = item.kind === 'image' || item.content.startsWith('data:image');
  const isLink = !isImage && (item.content.startsWith('http://') || item.content.startsWith('https://'));
  // Simple heuristic for code: explicit multiline with typical code chars, or just assume "Code" if it looks structured?
  // Let's rely on the previous heuristic for now but make it smarter?
  // Actually, syntax highlighter works on anything. Let's strictly detect "Code Snippet" via length + chars.
  const isCode = !isImage && !isLink && (
      item.content.includes('function') || 
      item.content.includes('const') || 
      item.content.includes('import') || 
      item.content.includes('class ') ||
      item.content.includes('=>') ||
      (item.content.includes('{') && item.content.includes('}'))
  );
  
  const type = isImage ? 'IMAGE' : isLink ? 'LINK' : isCode ? 'CODE' : 'TEXT';

  // Extract domain for links
  const getDomain = (url: string) => {
      try {
          return new URL(url).hostname;
      } catch {
          return 'Link';
      }
  };

  return (
    <div
      onClick={() => onCopy(item.content)}
      className={`group relative border rounded-xl p-3 transition-all duration-200 cursor-pointer ${
        item.pinned
          ? "bg-brand-primary-strong/10 border-brand-primary/20 shadow-[0_0_15px_-5px_var(--color-primary)]"
          : "bg-white/5 hover:bg-white/10 border-white/5 hover:border-brand-primary/30"
      }`}
      title="Click to copy"
    >
      <div className="flex items-start gap-3">
        <div
          className={`mt-1 w-1 h-1 rounded-full transition-colors ${
            item.pinned
              ? "bg-brand-accent shadow-[0_0_8px_var(--color-accent)] w-1.5 h-1.5"
              : "bg-brand-primary/50 group-hover:bg-brand-primary"
          }`}
        />

        <div className="flex-1 min-w-0 pr-8">
          <div className="flex items-center justify-between mb-1">
            <span
              className={`text-[10px] font-bold uppercase tracking-wider ${
                item.pinned
                  ? "text-brand-accent"
                  : "text-brand-text-dim group-hover:text-brand-text-muted"
              }`}
            >
              {type}
            </span>
            <span className="text-[10px] text-brand-text-dim/50 font-mono">
              {new Date(item.timestamp).toLocaleTimeString([], {
                hour: "2-digit",
                minute: "2-digit",
              })}
            </span>
          </div>
          
          {isImage ? (
              <div className="relative rounded-lg overflow-hidden border border-white/5 bg-black/20 w-fit max-w-full">
                 <img src={item.content} alt="Copied Image" className="max-h-32 object-contain" />
              </div>
          ) : isLink ? (
              <div className="flex items-center gap-3 p-2 rounded-lg bg-black/20 border border-white/5">
                  <div className="w-8 h-8 rounded bg-white/10 flex items-center justify-center overflow-hidden flex-shrink-0">
                      <img 
                        src={`https://www.google.com/s2/favicons?domain=${item.content}&sz=64`} 
                        alt="Favicon" 
                        className="w-5 h-5"
                        onError={(e) => {
                            (e.target as HTMLImageElement).style.display = 'none';
                        }}
                      />
                  </div>
                  <div className="overflow-hidden">
                      <p className="text-sm font-medium text-brand-text truncate">{getDomain(item.content)}</p>
                      <p className="text-xs text-brand-text-dim truncate">{item.content}</p>
                  </div>
              </div>
          ) : isCode ? (
              <div className="text-xs rounded-lg overflow-hidden border border-white/5 select-none" onClick={(e) => e.stopPropagation()}>
                  {/* Stop propagation on code block click? No, we still want copy behavior. But selection might be tricky. */}
                  {/* Actually, let's keep click-to-copy behavior on the card itself. */}
                  <SyntaxHighlighter 
                    language="javascript" 
                    style={vscDarkPlus} 
                    customStyle={{ margin: 0, padding: '0.75rem', background: 'rgba(0,0,0,0.3)' }}
                    wrapLongLines={false}
                  >
                    {item.content.length > 300 ? item.content.substring(0, 300) + '\n...' : item.content}
                  </SyntaxHighlighter>
              </div>
          ) : (
              <pre
                className={`text-xs font-mono rounded p-2 overflow-hidden text-ellipsis whitespace-nowrap border select-none ${
                  item.pinned
                    ? "bg-black/30 text-white border-brand-primary/10"
                    : "bg-black/20 text-brand-text border-white/5 group-hover:border-white/10"
                }`}
              >
                {item.content.trim()}
              </pre>
          )}
        </div>

        {/* Actions */}
        <div className="opacity-0 group-hover:opacity-100 transition-opacity absolute right-2 top-2 flex flex-col gap-1 bg-brand-bg-deep/80 backdrop-blur-sm rounded-lg border border-white/5 p-0.5 shadow-xl translate-x-1 group-hover:translate-x-0">
          <button
            onClick={(e) => onPin(item.id, e)}
            className={`p-1.5 rounded-md transition-colors cursor-pointer ${
              item.pinned
                ? "text-brand-accent bg-brand-accent/10 hover:bg-brand-accent/20"
                : "text-brand-text-dim hover:text-brand-accent hover:bg-brand-accent/5"
            }`}
            title={item.pinned ? "Unpin" : "Pin"}
          >
            <Icons.Pin className="w-3.5 h-3.5" fill={item.pinned ? "currentColor" : "none"} />
          </button>
          <button
            onClick={(e) => onDelete(item.id, e)}
            className="p-1.5 rounded-md cursor-pointer hover:bg-red-500/20 text-brand-text-dim hover:text-red-400 transition-colors"
            title="Delete"
          >
            <Icons.Trash className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>
    </div>
  );
}
