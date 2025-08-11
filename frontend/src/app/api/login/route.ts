import config from "@/env"
import { cookies } from "next/headers"
import { NextResponse } from "next/server"

export async function GET () {
  const url = config.BACKEND_URL + '/oauth/start/goog'
  const result = await fetch(url)
    .catch(err => console.error(url, err))
  if (!result) return NextResponse.json(null, { status: 500 })
  const stateCookie = result.headers.getSetCookie().find(c => c.startsWith('oauth_state'))!
  const cookieStore = await cookies()
  cookieStore.set('oauth_state', stateCookie)
  const redirectUrl = await result.text()
  return NextResponse.redirect(redirectUrl)
}
