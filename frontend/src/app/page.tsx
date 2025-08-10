import "./page.scss";
import Card, { CardHeader } from "./components/Card";
import { IconButton } from "@mui/material";

import AddIcon from '@mui/icons-material/Add';
import NewServicePopup from "./NewServicePopup";
import { getAllServices } from "@/helpers/service";
import { Service } from "@/types";
import Link from "next/link";

function ServiceCard ({ data: { name, service_id: id } }: { data: Service }) {
  return <Link href={'/projects/' + id}>
    <Card>
      <CardHeader text={name} />
      <p>some text, some text</p>
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
      </div>
      <NewServicePopup />
    </>
  );
}
