'use client';
import { useAppDispatch } from "@/store/hooks";
import { resetState, setInitialData } from "@/store/slices/serviceSlice";
import { Service, ServiceApiKey, ServiceGroup, ServiceSubscriber } from "@/types";
import { useEffect } from "react";

interface ServiceStateLoaderProps {
  service: Service;
  subscribers: ServiceSubscriber[];
  groups: ServiceGroup[];
  apiKeys: ServiceApiKey[];
}

export default function ServiceStateLoader ({ service, subscribers, groups, apiKeys }: ServiceStateLoaderProps) {
  const dispatch = useAppDispatch()

  const onUnmount = () => {
    dispatch(resetState())
  }

  const onMount = () => {
    dispatch(setInitialData({
      currentServiceId: service.service_id,
      name: service.name,
      vapidPublic: service.vapid_public,
      subscribers,
      apiKeys,
      groups
    }))
  }

  useEffect(() => {
    onMount()
    return onUnmount
  }, [])
  return null
}