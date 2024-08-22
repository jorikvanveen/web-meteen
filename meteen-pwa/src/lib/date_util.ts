export function overwriteDate(datetime: Date, date: Date) {
  datetime.setFullYear(date.getFullYear());
  datetime.setMonth(date.getMonth());
  datetime.setDate(date.getDate());
}

export function overwriteTime(datetime: Date, time: Date) {
  datetime.setHours(time.getHours());
  datetime.setMinutes(time.getMinutes());
}

function prefixZero(x: number): string {
  return x.toString().length < 2 ? `0${x}` : x.toString();
}
export function serializeToHtmlDate(datetime: Date): string {
  return `${datetime.getFullYear()}-${prefixZero(datetime.getMonth() + 1)}-${datetime.getDate()}`;
}

export function serializeToHtmlTime(datetime: Date): string {
  return `${prefixZero(datetime.getHours())}:${prefixZero(datetime.getMinutes())}`;
}
