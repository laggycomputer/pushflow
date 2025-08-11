import config from "@/env";
import { NextRequest, NextResponse } from "next/server";

export async function POST (request: NextRequest, { params }: { params: Promise<{ id: string, groupId: string }> }) {
  const query = await params
  const serviceId = encodeURIComponent(query.id)
  const input = await request.json() ?? {}

  const { apiKey, payload } = input

  const url = `${config.BACKEND_URL}/keyed/service/${serviceId}/group/${query.groupId}/notify`
  const body = JSON.stringify({ apiKey, payload })
  const headers = { 'Content-Type': 'application/json', 'Authorization': 'Bearer ' + apiKey }

  const response = await fetch(url, { method: 'POST', headers, body }).then(x => x.text())
  return NextResponse.json(response)
}
