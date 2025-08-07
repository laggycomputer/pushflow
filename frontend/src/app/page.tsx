import Image from "next/image";
import "./page.scss";
import Card, { CardHeader } from "./components/Card";
import { IconButton } from "@mui/material";
import AddIcon from '@mui/icons-material/Add';

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
          <Image
            className="logo"
            src="/next.svg"
            alt="Next.js logo"
            width={180}
            height={38}
            priority
          />
          <ol>
            <li>
              Get started by editing <code>src/app/page.tsx</code>.
            </li>
            <li>Save and see your changes instantly.</li>
          </ol>
        </Card>

      </main>
    </div>
  );
}
