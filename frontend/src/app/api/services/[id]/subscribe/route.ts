import config from "@/env";
import { NextRequest, NextResponse } from "next/server";

export async function POST (request: NextRequest, { params }: { params: Promise<{ id: string }> }) {
  const serviceId = encodeURIComponent((await params).id)
  const input = await request.json() ?? {}

  /**
   * @todo the Rust backend won't accept invalid values, but we should validate here and
   * give more descriptive errors in the future
   * */
  const { subscription, apiKey, groups } = input

  const url = `${config.BACKEND_URL}/keyed/service/${serviceId}/subscribe`
  const body = JSON.stringify({ subscription, groups })
  const headers = { 'Content-Type': 'application/json', 'Authorization': 'Bearer ' + apiKey }

  const response = await fetch(url, { method: 'POST', headers, body }).then(x => x.json())
  return NextResponse.json(response)
}
