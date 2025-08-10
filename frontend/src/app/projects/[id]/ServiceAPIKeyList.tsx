import Card, { CardHeader } from "@/app/components/Card";
import DataList from "@/app/components/DataList";
import ApiKey from "@/app/components/ServiceApiKey";
import { ServiceApiKey } from "@/types";
import { IconButton } from "@mui/material";

import AddIcon from '@mui/icons-material/Add';

export default function ServiceAPIKeyList () {
  const keys: ServiceApiKey[] = [
    {
      service_id: "something",
      key_uuid: "cdafef0d7f2325784",
      last_used: "2025-02-02T10:00:00Z",
      created_at: "2025-02-02T10:00:00Z",
      scopes: ['subscriber.add']
    },
    {
      service_id: "something",
      key_uuid: "234862789fadef6c",
      last_used: "2025-02-02T10:00:00Z",
      created_at: "2025-02-02T10:00:00Z",
      scopes: ['subscriber.add', 'subscriber.edit', 'group.edit']
    }
  ]
  return <Card>
    <CardHeader text="API Keys">
      <IconButton size="small">
        <AddIcon />
      </IconButton>
    </CardHeader>
    <DataList>
      {keys.map(k => <ApiKey
        key={k.key_uuid}
        keyUuid={k.key_uuid}
        lastUsed={new Date()}
        scopes={k.scopes}
      />)}
    </DataList>
  </Card>
}
