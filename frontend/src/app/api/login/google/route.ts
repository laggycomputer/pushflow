import { cookies } from "next/headers"
import { NextRequest, NextResponse } from "next/server"

/** @todo refactor into constant config file */
const BASE_URL = process.env.BACKEND_BASE_URL!

export async function GET (request: NextRequest) {
  const cookieStore = await cookies()
  const oauthState = cookieStore.get('oauth_state')
  if (!oauthState) return NextResponse.json(null, { status: 400 })

  const url = `${BASE_URL}/oauth/cb/goog${request.nextUrl.search}`
  const headers = { Cookie: oauthState.value }
  const oauthResponse = await fetch(url, { headers })
  console.log(headers, oauthResponse)
  return NextResponse.json(await oauthResponse.text())
}
