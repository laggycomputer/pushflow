import config from "@/env"

const swBaseCode = `
/// <reference lib="webworker" />
/** @type {ServiceWorkerGlobalScope} */
const worker = self

worker.addEventListener('push', event => {
  // Replace this with event.data.json() if you prefer to parse JSON from the payload received
  const payload = event.data.text()

  const respond = async () => {
    await worker.registration.showNotification("Hello from $$SERVICE$$", { body: payload })
  }

  event.waitUntil(respond())
})
`.trim()

export function getServiceWorkerBaseCode (serviceName: string) {
  return swBaseCode.replace('$$SERVICE$$', serviceName)
}

const registerSwBaseCode = `
export async function registerWorker () {
  if (!('serviceWorker' in navigator)) return console.error('Cannot register service worker')
  const worker = await navigator.serviceWorker.register('/service-worker.js')
  return worker
}
`.trim()

export function getServiceWorkerRegistrationCode () {
  return registerSwBaseCode
}

const subscribeBaseCode = `
async function subscribeToNotifications (serviceId: string, groups: string[], forceResubscribe: boolean) {
  const baseUrl = '$$APPLICATION_DOMAIN$$'
  const applicationServerKey = '$$APPLICATION_SERVER_KEY$$'
  const serviceId = '$$APPLICATION_SERVICE_KEY$$'

  const apiKey = '$$APPLICATION_API_KEY$$'

  const worker = await navigator.serviceWorker.getRegistration()
  const status = await Notification.requestPermission()
  if (!worker || status !== 'granted') throw new Error('Notification Permission not granted')

  const subscription = await worker.pushManager.getSubscription()
  if (subscription && forceResubscribe) await subscription.unsubscribe()

  const headers = { 'Content-Type': 'application/json' }
  const details = await worker.pushManager.subscribe({ userVisibleOnly: true, applicationServerKey })
  const body = JSON.stringify({ subscription: details.toJSON(), apiKey, groups })

  const url = \`\${baseUrl}/api/services/\${serviceId}/subscribe\`
  await fetch(url, { headers, body, method: 'POST' })
}
`.trim()

export function getSubscribeCode (serviceId: string, apiKey: string, vapidPublicKey: string) {
  return subscribeBaseCode
    .replace('$$APPLICATION_DOMAIN$$', config.NEXT_PUBLIC_APP_DOMAIN)
    .replace('$$APPLICATION_SERVER_KEY$$', vapidPublicKey)
    .replace('$$APPLICATION_API_KEY$$', apiKey)
    .replace('$$APPLICATION_SERVICE_KEY$$', serviceId)
}
