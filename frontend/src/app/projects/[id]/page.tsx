import './page.scss';
import { Service } from "@/types";
import { notFound } from "next/navigation";
import ServiceSubscriberList from "./ServiceSubscriberList";
import ServiceAPIKeyList from "./ServiceAPIKeyList";
import ServiceGroupList from "./ServiceGroupList";
import { getService } from '@/helpers/service';
import { getUser } from '@/helpers/server';

interface ProjectPageParams {
  params: Promise<{ id: string }>
}

export default async function ProjectPage ({ params }: ProjectPageParams) {
  const user = await getUser()
  const serviceId = (await params).id

  const service = await getService(serviceId)
  console.log(user, service)

  if (!service) return notFound()

  return <div className="service-info-wrapper">
    <ServiceSubscriberList />
    <ServiceAPIKeyList />
    <ServiceGroupList serviceId={serviceId} />
  </div>
}
