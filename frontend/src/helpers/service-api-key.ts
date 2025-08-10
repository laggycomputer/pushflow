'use server';
import config from "@/env";
import { getSessionHeaders } from "./server";
import { ServiceApiKey } from "@/types";

export async function createServiceApiKey (serviceId: string, name: string, selectedScopes: string[]): Promise<ServiceApiKey | null> {
  const headers = await getSessionHeaders()
  if (!headers) return null

  headers['Content-Type'] = 'application/json'

  const url = `${config.BACKEND_URL}/gated/service/${serviceId}/key`
  const scopes = selectedScopes.map(s => ({ scope: s }))
  const body = JSON.stringify({ name, scopes })

  const response = await fetch(url, { headers, method: 'POST', body })
    .then(x => x.json())
    .catch(err => console.error(err))
  console.log(response)
  return response
}

export async function getServiceApiKeys (serviceId: string): Promise<ServiceApiKey[] | null> {
  const headers = await getSessionHeaders()
  if (!headers) return null

  const url = `${config.BACKEND_URL}/gated/service/${encodeURIComponent(serviceId)}/key`
  const response = await fetch(url, { headers })
    .then(x => x.json())
    .catch(err => console.error(err))
  
  return response
}
