'use server';
import config from "@/env";
import { getSessionHeaders } from "./server";
import { ServiceSubscriber } from "@/types";

export async function getServiceSubscribers (serviceId: string): Promise<ServiceSubscriber[] | null> {
  const headers = await getSessionHeaders()
  if (!headers) return null

  const url = `${config.BACKEND_URL}/gated/service/${encodeURIComponent(serviceId)}/subscriber`
  const response = await fetch(url, { headers })
    .then(x => x.json())
    .catch(err => console.error(err))

  return response
}

export async function deleteSubscriber (serviceId: string, subscriberId: string): Promise<boolean> {
  const headers = await getSessionHeaders()
  if (!headers) return false
  
  const url = `${config.BACKEND_URL}/gated/service/${encodeURIComponent(serviceId)}/subscriber/${encodeURIComponent(subscriberId)}`
  await fetch(url, { headers, method: 'DELETE' })
    .then(x => x.text())
    .catch(err => console.error(err))

  return true
}

export async function updateSubscriber (serviceId: string, subscriberId: string, name: string): Promise<boolean> {
  const headers = await getSessionHeaders()
  if (!headers) return false
  
  headers['Content-Type'] = 'application/json'

  const url = `${config.BACKEND_URL}/gated/service/${encodeURIComponent(serviceId)}/subscriber/${encodeURIComponent(subscriberId)}`
  const body = JSON.stringify({ name })

  const response = await fetch(url, { headers, body, method: 'PATCH' })
    .then(x => x.text())
    .catch(err => console.error(err))
  console.log(response)

  return true
}
