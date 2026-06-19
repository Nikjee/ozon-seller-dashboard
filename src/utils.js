function formatRub(value) {
  const n = Math.round(Math.abs(value))
  const sign = value < 0 ? '−' : ''
  const formatted = n.toLocaleString('ru-RU')
  return `${sign}${formatted} ₽`
}

function formatRubCompact(value) {
  const abs = Math.abs(value)
  const sign = value < 0 ? '−' : ''
  if (abs >= 1_000_000) {
    return `${sign}${(abs / 1_000_000).toFixed(1)}M ₽`
  }
  if (abs >= 10_000) {
    return `${sign}${(abs / 1_000).toFixed(1)}K ₽`
  }
  const formatted = Math.round(abs).toLocaleString('ru-RU')
  return `${sign}${formatted} ₽`
}

function formatInt(value) {
  if (value === undefined || value === null) return '—'
  return Number(value).toLocaleString('ru-RU')
}

export { formatRub, formatRubCompact, formatInt }
