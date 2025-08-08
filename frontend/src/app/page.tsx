import Image from "next/image";
import "./page.scss";
import Card, { CardHeader } from "./components/Card";
import { IconButton } from "@mui/material";

import AddIcon from '@mui/icons-material/Add';
import SubscriptionUser from "./components/SubscriptionUser";
import DataList from "./components/DataList";

export default function Home() {
  return (
    <div className="page">
      <main className="main">
        <Card>
          <CardHeader text="20 Subscribers">
            <IconButton size="small">
              <AddIcon/>
            </IconButton>
          </CardHeader>
          <DataList>
            <SubscriptionUser/>
            <SubscriptionUser/>
            <SubscriptionUser/>
          </DataList>
        </Card>

      </main>
    </div>
  );
}
