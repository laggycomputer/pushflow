import config from "@/env"
import { getSessionHeaders } from "@/helpers/server"
import { cookies } from "next/headers"
import { NextResponse } from "next/server"

export async function POST () {
  const headers = await getSessionHeaders()
  if (!headers) return NextResponse.json(null, { status: 401 })

  await fetch(`${config.BACKEND_URL}/gated/logout`, { headers, method: 'POST' })
  
  const cookieStore = await cookies()
  cookieStore.delete('session')

  return NextResponse.json(null)
}
