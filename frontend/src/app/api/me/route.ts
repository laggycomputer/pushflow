import config from "@/env"
import { getSessionHeaders } from "@/helpers/server"
import { NextResponse } from "next/server"

export async function GET () {
  return await getUser()
}

export async function getUser () {
  const headers = await getSessionHeaders()
  if (!headers) return NextResponse.json(null, { status: 401 })

  const url = `${config.BACKEND_URL}/gated/me`
  const response = await fetch(url, { headers })

  return NextResponse.json(await response.json().catch(() => null))
}
