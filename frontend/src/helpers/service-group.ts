'use server';
import config from "@/env";
import { getSessionHeaders } from "./server";
import { ServiceGroup } from "@/types";

export async function createGroup (serviceId: string, name: string): Promise<ServiceGroup | null> {
  const headers = await getSessionHeaders()
  if (!headers) return null

  headers['Content-Type'] = 'application/json'

  const url = `${config.BACKEND_URL}/gated/service/${serviceId}/group`
  const body = JSON.stringify({ name })

  return await fetch(url, { headers, method: 'POST', body })
    .then(x => x.json())
    .catch(err => console.error(err))
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

export async function deleteGroup (serviceId: string, groupId: string): Promise<boolean> {
  const headers = await getSessionHeaders()
  if (!headers) return false
  
  const url = `${config.BACKEND_URL}/gated/service/${encodeURIComponent(serviceId)}/group/${encodeURIComponent(groupId)}`
  await fetch(url, { headers, method: 'DELETE' })
    .then(x => x.text())
    .catch(err => console.error(err))

  return true
}

export async function updateGroup (serviceId: string, groupId: string, name: string): Promise<boolean> {
  const headers = await getSessionHeaders()
  if (!headers) return false
  
  headers['Content-Type'] = 'application/json'

  const url = `${config.BACKEND_URL}/gated/service/${encodeURIComponent(serviceId)}/group/${encodeURIComponent(groupId)}`
  const body = JSON.stringify({ name })

  await fetch(url, { headers, body, method: 'PATCH' })
    .then(x => x.json())
    .catch(err => console.error(err))

  return true
}
