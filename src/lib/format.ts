export function formatCompactDateTime(
  value: string | null | undefined,
  emptyText = "",
) {
  if (!value) {
    return emptyText;
  }

  const date = new Date(value);
  const year = date.getFullYear();
  const month = date.getMonth() + 1;
  const day = date.getDate();
  const hours = padTimePart(date.getHours());
  const minutes = padTimePart(date.getMinutes());
  const seconds = padTimePart(date.getSeconds());

  return `${year}.${month}.${day} ${hours}:${minutes}:${seconds}`;
}

function padTimePart(value: number) {
  return value.toString().padStart(2, "0");
}
