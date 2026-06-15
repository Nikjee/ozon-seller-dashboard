import { ref, watch } from 'vue'

const LOCALE_KEY = 'ozon-dashboard-locale'

const locale = ref(localStorage.getItem(LOCALE_KEY) || 'ru')

const messages = {
  ru: {
    title: 'Ozon Dashboard',
    update: 'Обновить',
    lightMode: 'Светлая тема',
    darkMode: 'Тёмная тема',
    productsSold: 'Товаров продано',
    positions: 'позиций',
    revenue: 'Выручка',
    expenses: 'Расходы',
    netProfit: 'Чистая прибыль',
    product: 'Товар',
    sold: 'Продано',
    commission: 'Комиссия',
    delivery: 'Доставка',
    returns: 'Возвраты',
    services: 'Сервисы',
    profit: 'Прибыль',
    entries: 'записей',
    posting: 'Отправление',
    date: 'Дата',
    price: 'Цена',
    net: 'Чистая',
    noData: 'Нет данных за этот период',
    noDataHint: 'Продажи за выбранный месяц не найдены. Попробуйте другой период.',
    loading: 'Загрузка данных...',
    loadError: 'Ошибка загрузки',
    retry: 'Повторить',
    dataSource: 'Источник данных',
    previousMonth: 'Предыдущий месяц',
    nextMonth: 'Следующий месяц',
    total: 'Всего',
    expenseBreakdown: 'Расходы по категориям',
    commissionLabel: 'Комиссия Ozon',
    deliveryLabel: 'Доставка',
    returnLabel: 'Возвраты',
    adExpense: 'Реклама',
    storageFees: 'Хранение',
    logisticsFees: 'Логистика',
    otherServices: 'Прочие услуги',
    compensation: 'Компенсации',
    acquiring: 'Эквайринг',
    accountExpenses: 'Расходы на аккаунт',
    accountAd: 'Реклама',
    accountStorage: 'Хранение и обработка',
    accountLogistics: 'Логистика',
    unknown: 'Неизвестно',
    rubric: '₽'
  },
  en: {
    title: 'Ozon Dashboard',
    update: 'Update',
    lightMode: 'Light mode',
    darkMode: 'Dark mode',
    productsSold: 'Products sold',
    positions: 'positions',
    revenue: 'Revenue',
    expenses: 'Expenses',
    netProfit: 'Net profit',
    product: 'Product',
    sold: 'Sold',
    commission: 'Commission',
    delivery: 'Delivery',
    returns: 'Returns',
    services: 'Services',
    profit: 'Profit',
    entries: 'entries',
    posting: 'Posting',
    date: 'Date',
    price: 'Price',
    net: 'Net',
    noData: 'No data for this period',
    noDataHint: 'No sales found for the selected month. Try another period.',
    loading: 'Loading data...',
    loadError: 'Loading error',
    retry: 'Retry',
    dataSource: 'Data source',
    previousMonth: 'Previous month',
    nextMonth: 'Next month',
    total: 'Total',
    expenseBreakdown: 'Expense breakdown',
    commissionLabel: 'Ozon commission',
    deliveryLabel: 'Delivery',
    returnLabel: 'Returns',
    adExpense: 'Advertising',
    storageFees: 'Storage',
    logisticsFees: 'Logistics',
    otherServices: 'Other services',
    compensation: 'Compensations',
    acquiring: 'Acquiring',
    accountExpenses: 'Account expenses',
    accountAd: 'Advertising',
    accountStorage: 'Storage & handling',
    accountLogistics: 'Logistics',
    unknown: 'Unknown',
    rubric: 'RUB'
  }
}

function t(key) {
  return messages[locale.value]?.[key] ?? key
}

watch(locale, (val) => {
  localStorage.setItem(LOCALE_KEY, val)
})

export function useI18n() {
  function toggle() {
    locale.value = locale.value === 'ru' ? 'en' : 'ru'
  }
  return { locale, t, toggle }
}
