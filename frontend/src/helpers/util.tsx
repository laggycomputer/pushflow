export function pluralize (count: number, plural: string, singular: string) {
  if (count === 1) return `${count} ${singular}`
  else return `${count} ${plural}`
}
