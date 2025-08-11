export async function subscribeToNotifications (serviceId: string, groups: string[], applicationServerKey: string, apiKey: string) {
  const worker = await navigator.serviceWorker.getRegistration()
  const status = await Notification.requestPermission()
  if (!worker || status !== 'granted') return console.error('Permission not granted')

  const subscription = await worker.pushManager.getSubscription()
  if (subscription) await subscription.unsubscribe()

  const headers = { 'Content-Type': 'application/json' }
  const details = await worker.pushManager.subscribe({ userVisibleOnly: true, applicationServerKey })
  const body = JSON.stringify({
    subscription: details.toJSON(),
    apiKey,
    groups
  })
  const url = `/api/services/${serviceId}/subscribe`
  const response = await fetch(url, { headers, body, method: 'POST' })

  console.log('Subscribed to notifications', response)
}

export async function registerWorker () {
  if (!('serviceWorker' in navigator)) return console.error('Cannot register service worker')
  const worker = await navigator.serviceWorker.register('/service-worker.js')

  if (!('Notification' in window)) return console.error('Notification not supported')
  if (await worker.pushManager.getSubscription()) return console.log('Already subscribed')
}
