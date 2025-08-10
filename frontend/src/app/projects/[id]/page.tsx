import './page.scss';
import { Service, ServiceGroup } from "@/types";
import { notFound } from "next/navigation";
import ServiceSubscriberList from "./ServiceSubscriberList";
import ServiceAPIKeyList from "./ServiceAPIKeyList";
import ServiceGroupList from "./ServiceGroupList";
import { getService, getServiceGroups } from '@/helpers/service';
import { getUser } from '@/helpers/server';

interface ProjectPageParams {
  params: Promise<{ id: string }>
}

export default async function ProjectPage ({ params }: ProjectPageParams) {
  const serviceId = (await params).id

  const service = await getService(serviceId)
  const groups: ServiceGroup[] = await getServiceGroups(serviceId).then(x => x ?? [])

  if (!service) return notFound()

  return <div className="service-info-wrapper">
    <ServiceSubscriberList />
    <ServiceAPIKeyList />
    <ServiceGroupList serviceId={serviceId} groups={groups} />
  </div>
}
