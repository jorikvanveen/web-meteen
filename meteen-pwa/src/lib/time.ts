export function formatTime(datetime: Date): string {
  return Intl.DateTimeFormat(navigator.language, {
    dateStyle: undefined,
    timeStyle: "short",
  }).format(datetime);
}

export function formatDate(datetime: Date): string {
  const now = new Date();

  let year: "numeric" | undefined = undefined;
  if (datetime.valueOf() - now.valueOf() > 330 * 24 * 60 * 60 * 1000) {
    year = "numeric";
  }

  return Intl.DateTimeFormat(navigator.language, {
    weekday: "short",
    month: "short",
    day: "numeric",
    year,
  }).format(datetime);
}
