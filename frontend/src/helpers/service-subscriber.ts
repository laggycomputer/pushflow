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
