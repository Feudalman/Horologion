import * as React from "react";
import { createPortal } from "react-dom";

import { cn } from "@/lib/utils";

function Table({ className, ...props }: React.ComponentProps<"table">) {
  return (
    <div className="w-full overflow-auto">
      <table className={cn("w-full caption-bottom text-sm", className)} {...props} />
    </div>
  );
}

function TableHeader({ className, ...props }: React.ComponentProps<"thead">) {
  return <thead className={cn("[&_tr]:border-b", className)} {...props} />;
}

function TableBody({ className, ...props }: React.ComponentProps<"tbody">) {
  return (
    <tbody className={cn("[&_tr:last-child]:border-0", className)} {...props} />
  );
}

function TableRow({ className, ...props }: React.ComponentProps<"tr">) {
  return (
    <tr
      className={cn(
        "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted",
        className,
      )}
      {...props}
    />
  );
}

function TableHead({ className, ...props }: React.ComponentProps<"th">) {
  return (
    <th
      className={cn(
        "h-10 whitespace-nowrap px-3 text-left align-middle text-xs font-medium uppercase tracking-normal text-muted-foreground",
        className,
      )}
      {...props}
    />
  );
}

function TableCell({ className, ...props }: React.ComponentProps<"td">) {
  return <td className={cn("px-3 py-3 align-middle", className)} {...props} />;
}

type TableCellTextProps = {
  children: React.ReactNode;
  className?: string;
  tooltip: React.ReactNode;
};

function TableCellText({ children, className, tooltip }: TableCellTextProps) {
  const ref = React.useRef<HTMLSpanElement>(null);
  const [position, setPosition] = React.useState<React.CSSProperties | null>(null);
  const tooltipText =
    typeof tooltip === "string" || typeof tooltip === "number"
      ? String(tooltip)
      : undefined;

  function showTooltip() {
    const element = ref.current;
    if (!element || typeof window === "undefined") {
      return;
    }

    const rect = element.getBoundingClientRect();
    const maxWidth = Math.min(384, window.innerWidth - 24);
    const left = Math.min(
      Math.max(12, rect.left),
      Math.max(12, window.innerWidth - maxWidth - 12),
    );
    const top = Math.min(rect.bottom + 6, window.innerHeight - 48);

    setPosition({
      left,
      maxWidth,
      top,
    });
  }

  function hideTooltip() {
    setPosition(null);
  }

  return (
    <>
      <span
        className={cn("block min-w-0 max-w-full truncate", className)}
        onMouseEnter={showTooltip}
        onMouseLeave={hideTooltip}
        aria-label={tooltipText}
        ref={ref}
      >
        {children}
      </span>
      {position && typeof document !== "undefined"
        ? createPortal(
            <span
              className="pointer-events-none fixed z-50 max-h-40 overflow-auto whitespace-pre-wrap break-words rounded-md border bg-popover px-2.5 py-1.5 text-xs font-normal normal-case tracking-normal text-popover-foreground shadow-md"
              style={position}
            >
              {tooltip}
            </span>,
            document.body,
          )
        : null}
    </>
  );
}

export {
  Table,
  TableBody,
  TableCell,
  TableCellText,
  TableHead,
  TableHeader,
  TableRow,
};
