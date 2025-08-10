'use server';
import { Service, ServiceGroup } from "@/types"
import { getSessionHeaders } from "./server"
import config from "@/env"

export async function getAllServices (): Promise<Service[] | null> {
  const headers = await getSessionHeaders()
  if (!headers) return null

  const url = `${config.BACKEND_URL}/gated/service`
  const response = await fetch(url, { headers })
    .then(x => x.json())
    .catch(err => console.error(err))

  return response
}

export async function getService (id: string): Promise<Service | null> {
  const headers = await getSessionHeaders()
  if (!headers) return null

  const url = `${config.BACKEND_URL}/gated/service/${encodeURIComponent(id)}`
  const response = await fetch(url, { headers })
    .then(x => x.json())
    .catch(err => console.error(err))

  return response
}

export async function getServiceGroups (serviceId: string): Promise<ServiceGroup[] | null> {
  const headers = await getSessionHeaders()
  if (!headers) return null

  const url = `${config.BACKEND_URL}/gated/service/${encodeURIComponent(serviceId)}/group`
  const response = await fetch(url, { headers })
    .then(x => x.json())
    .catch(err => console.error(err))
  
  return response
}

export async function createService (name: string): Promise<Service | null> {
  const headers = await getSessionHeaders()
  if (!headers) return null
  
  headers['Content-Type'] = 'application/json'

  const url = `${config.BACKEND_URL}/gated/service`
  const body = JSON.stringify({ name })

  console.log({ headers, method: 'POST', body })
  const response = await fetch(url, { headers, method: 'POST', body })
    .then(x => x.json())
    .catch(err => console.error(err))

  return response
}
