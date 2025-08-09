import config from "@/env"
import { cookies } from "next/headers"
import { NextRequest, NextResponse } from "next/server"

export async function GET (request: NextRequest) {
  const cookieStore = await cookies()
  const oauthState = cookieStore.get('oauth_state')
  if (!oauthState) return NextResponse.json(null, { status: 400 })

  const url = `${config.BACKEND_URL}/oauth/cb/goog${request.nextUrl.search}`
  const headers = { Cookie: oauthState.value }
  const oauthResponse = await fetch(url, { headers })

  const sidCookie = oauthResponse.headers.getSetCookie().find(c => c.startsWith('session='))!
  cookieStore.set("session", sidCookie)

  return NextResponse.json(await oauthResponse.text())
}
