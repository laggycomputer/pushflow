'use server';
import { cookies } from "next/headers";

export async function getSessionHeaders () {
  const cookieStore = await cookies()
  const sessionCookie = cookieStore.get('session')
  console.log('sessionCookie', sessionCookie)

  if (!sessionCookie) return null
  return { Cookie: sessionCookie.value.replace(/;.*/, ';') } as Record<string, string>
}

