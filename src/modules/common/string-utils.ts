export function isBlank(value: string | undefined | null) {
  return value === undefined || value === null || value === '';
}

export function isNotBlank(value: string | undefined | null) {
  return !isBlank(value);
}
