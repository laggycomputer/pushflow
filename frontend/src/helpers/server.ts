'use server';
import config from "@/env";
import { cookies } from "next/headers";

export async function getSessionHeaders () {
  const cookieStore = await cookies()
  const sessionCookie = cookieStore.get('session')
  console.log('sessionCookie', sessionCookie)

  if (!sessionCookie) return null
  return { Cookie: sessionCookie.value.replace(/;.*/, ';') } as Record<string, string>
}

export async function getUser () {
  const headers = await getSessionHeaders()
  if (!headers) return null

  const url = `${config.BACKEND_URL}/gated/me`
  const response = await fetch(url, { headers })

  await response.json().catch(() => null)
}
