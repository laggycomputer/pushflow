import config from "@/env"
import { cookies } from "next/headers"
import { NextResponse } from "next/server"

export async function GET () {
  const result = await fetch(config.BACKEND_BASE_URL + '/oauth/start/goog')
  const stateCookie = result.headers.getSetCookie().find(c => c.startsWith('oauth_state'))!
  const cookieStore = await cookies()
  cookieStore.set('oauth_state', stateCookie)
  const redirectUrl = await result.text()
  return NextResponse.redirect(redirectUrl)
}
