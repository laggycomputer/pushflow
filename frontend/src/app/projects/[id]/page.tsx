import './page.scss';
import { Service, ServiceGroup } from "@/types";
import { notFound } from "next/navigation";
import ServiceSubscriberList from "./ServiceSubscriberList";
import ServiceAPIKeyList from "./ServiceAPIKeyList";
import ServiceGroupList from "./ServiceGroupList";
import { getService } from '@/helpers/service';
import { getServiceGroups } from '@/helpers/service-group';
import { getServiceApiKeys } from '@/helpers/service-api-key';
import ServiceStateLoader from './ServiceStateLoader';

interface ProjectPageParams {
  params: Promise<{ id: string }>
}

export default async function ProjectPage ({ params }: ProjectPageParams) {
  const serviceId = (await params).id

  const service = await getService(serviceId)
  const groups = await getServiceGroups(serviceId).then(x => x ?? [])
  const apiKeys = await getServiceApiKeys(serviceId).then(x => x ?? [])

  if (!service) return notFound()

  return <div className="service-info-wrapper">
    <ServiceStateLoader service={service} groups={groups} apiKeys={apiKeys} subscribers={[]} />
    <ServiceSubscriberList />
    <ServiceAPIKeyList serviceId={serviceId} apiKeys={apiKeys} />
    <ServiceGroupList serviceId={serviceId} groups={groups} />
  </div>
}
