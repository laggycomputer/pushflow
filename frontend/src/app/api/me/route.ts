import { getUser } from "@/helpers/server"
import { NextResponse } from "next/server"

export async function GET () {
  const user = await getUser()
  if (!user) return NextResponse.json(null, { status: 401 })
  return NextResponse.json(user)
}
