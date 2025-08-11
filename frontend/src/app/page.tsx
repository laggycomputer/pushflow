import "./page.scss";
import Card, { CardHeader } from "./components/Card";
import NewServiceDialog from "./components/dialogs/NewServiceDialog";
import { getAllServices } from "@/helpers/service";
import { Service } from "@/types";
import Link from "next/link";
import { EmptyListMessage } from "./components/DataList";

function ServiceCard ({ data: { name, service_id: id } }: { data: Service }) {
  return <Link href={'/projects/' + id}>
    <Card>
      <CardHeader text={name} />
      <p>This is a service!</p>
    </Card>
  </Link>
}

export default async function Home() {
  const userServices = await getAllServices()
  if (!userServices) return null

  return (
    <>
      <div className="services-grid">
        {userServices.map(s => 
          <ServiceCard key={s.service_id} data={s} />
        )}
        <EmptyListMessage list={userServices} message="You don't have any services yet!" />
      </div>
      <NewServiceDialog />
    </>
  );
}
