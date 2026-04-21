import * as React from "react";
import { ChevronLeft, ChevronRight } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

const pageSizeOptions = [25, 50, 100];

export function PaginationControls({
  page,
  pages,
  total,
  size,
  onPageChange,
  onSizeChange,
}: {
  page: number;
  pages: number;
  total: number;
  size: number;
  onPageChange: (page: number) => void;
  onSizeChange: (size: number) => void;
}) {
  const { t } = useTranslation();
  const safePages = Math.max(pages, 1);
  const [draftPage, setDraftPage] = React.useState(String(page));

  React.useEffect(() => {
    setDraftPage(String(page));
  }, [page]);

  function jumpToPage(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const nextPage = Number(draftPage);
    if (!Number.isFinite(nextPage)) {
      setDraftPage(String(page));
      return;
    }

    const clampedPage = Math.min(Math.max(Math.trunc(nextPage), 1), safePages);
    setDraftPage(String(clampedPage));
    onPageChange(clampedPage);
  }

  return (
    <div className="flex flex-col gap-3 border-t pt-4 sm:flex-row sm:items-center sm:justify-between">
      <div className="text-sm text-muted-foreground">
        {t("pagination.summary", {
          page,
          pages: safePages,
          total,
        })}
      </div>

      <div className="flex flex-wrap items-center gap-3">
        <form className="flex items-center gap-2" onSubmit={jumpToPage}>
          <label
            className="text-sm text-muted-foreground"
            htmlFor="pagination-page"
          >
            {t("pagination.jumpTo")}
          </label>
          <Input
            className="h-9 w-20"
            id="pagination-page"
            min={1}
            max={safePages}
            onChange={(event) => setDraftPage(event.target.value)}
            type="number"
            value={draftPage}
          />
          <Button size="sm" type="submit" variant="outline">
            {t("pagination.go")}
          </Button>
        </form>

        <label className="flex items-center gap-2 text-sm text-muted-foreground">
          <span>{t("pagination.rowsPerPage")}</span>
          <select
            className="h-9 rounded-md border border-input bg-background px-2 text-sm text-foreground shadow-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            onChange={(event) => onSizeChange(Number(event.target.value))}
            value={size}
          >
            {pageSizeOptions.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        </label>

        <div className="flex items-center gap-2">
          <Button
            disabled={page <= 1}
            onClick={() => onPageChange(page - 1)}
            size="sm"
            type="button"
            variant="outline"
          >
            <ChevronLeft />
            {t("pagination.previous")}
          </Button>
          <Button
            disabled={page >= safePages}
            onClick={() => onPageChange(page + 1)}
            size="sm"
            type="button"
            variant="outline"
          >
            {t("pagination.next")}
            <ChevronRight />
          </Button>
        </div>
      </div>
    </div>
  );
}
