const applicationServerKey = process.env.NEXT_PUBLIC_VAPID_PUBLIC_KEY
const testApiKey = process.env.NEXT_PUBLIC_TEST_API_KEY

export async function subscribeToNotifications (serviceId: string, groups: string[]) {
  const worker = await navigator.serviceWorker.getRegistration()
  const status = await Notification.requestPermission()
  if (!worker || status !== 'granted') return console.error('Permission not granted')

  // TEMP: unsubscribe if needed
  const subscription = await worker.pushManager.getSubscription()
  if (subscription) await subscription.unsubscribe()

  console.log(applicationServerKey, subscription)

  const headers = { 'Content-Type': 'application/json' }
  const details = await worker.pushManager.subscribe({ userVisibleOnly: true, applicationServerKey })
  const body = JSON.stringify({
    subscription: details.toJSON(),
    apiKey: testApiKey,
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
