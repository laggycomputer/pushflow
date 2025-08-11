'use client';
import Card, { CardHeader } from "@/app/components/Card";
import DataList from "@/app/components/DataList";
import SubscriptionUser from "@/app/components/SubscriptionUser";
import { pluralize } from "@/helpers/util";
import { useAppSelector } from "@/store/hooks";
import { ServiceSubscriber } from "@/types";

interface ServiceSubscriberListProps {
  serviceId: string;
  subscribers: ServiceSubscriber[];
}

export default function ServiceSubscriberList ({ serviceId, subscribers: initialSubs }: ServiceSubscriberListProps) {
  const subscribers = useAppSelector(state => state.service.currentServiceId === serviceId ? state.service.subscribers : initialSubs)
  const titleText = pluralize(subscribers.length, 'Subscribers', 'Subscriber')

  return <Card>
    <CardHeader text={titleText} />
    <DataList>
      {subscribers.map(s => <SubscriptionUser
        key={s.subscriber_id}
        displayName={s.name ?? s.subscriber_id}
        createdAt={new Date(s.created_at ?? Date.now())}
        groupCount={s.groups.length}
      />)}
    </DataList>
  </Card>
}
