import Card, { CardHeader } from "@/app/components/Card";
import DataList from "@/app/components/DataList";
import SubscriptionUser from "@/app/components/SubscriptionUser";
import { ServiceSubscriber } from "@/types";

export default function ServiceSubscriberList () {
  const subscribers: ServiceSubscriber[] = [
    {
      service_id: "something",
      subscriber_id: "ef20e72efd98e7e",
      name: 'user 1 or something',
      groups: ["test", 'tes4t', 'test2']
    },
    {
      service_id: "something",
      subscriber_id: "da5cef31890299",
      name: 'A second user who has a name',
      groups: ['service']
    }
  ]
  return <Card>
    <CardHeader text="20 Subscribers" />
    <DataList>
      {subscribers.map(s => <SubscriptionUser
        key={s.subscriber_id}
        displayName={s.name ?? s.subscriber_id}
        createdAt={new Date()}
        groupCount={s.groups.length}
      />)}
    </DataList>
  </Card>
}