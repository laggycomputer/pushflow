import { cookies } from "next/headers"
import { NextResponse } from "next/server"

const BASE_URL = process.env.BACKEND_BASE_URL!

export async function GET () {
  const result = await fetch(BASE_URL + '/oauth/start/goog')
  const stateCookie = result.headers.getSetCookie().find(c => c.startsWith('oauth_state'))!
  const cookieStore = await cookies()
  cookieStore.set('oauth_state', stateCookie)
  const redirectUrl = await result.text()
  console.log(redirectUrl)
  return NextResponse.redirect(redirectUrl)
}
