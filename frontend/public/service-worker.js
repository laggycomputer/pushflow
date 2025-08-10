/// <reference lib="webworker" />
/** @type {ServiceWorkerGlobalScope} */
const worker = self

worker.addEventListener('push', event => {
  const { title, body } = event.data.json()

  const respond = async () => {
    await worker.registration.showNotification(title, { body })
  }

  event.waitUntil(respond())
})
