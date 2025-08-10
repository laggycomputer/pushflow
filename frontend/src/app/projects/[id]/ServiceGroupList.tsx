import Card, { CardHeader } from "@/app/components/Card";
import DataList from "@/app/components/DataList";
import Group from "@/app/components/ServiceGroup";
import { ServiceGroup } from "@/types";
import { IconButton } from "@mui/material";

import AddIcon from "@mui/icons-material/Add";
import { pluralize } from "@/helpers/util";
import { getServiceGroups } from "@/helpers/service";

export default async function ServiceGroupList ({ serviceId }: { serviceId: string; }) {
  const groups: ServiceGroup[] = await getServiceGroups(serviceId).then(x => x ?? [])

  const groupHeaderText = pluralize(groups.length + 1, 'Groups', 'Group')

  return <Card>
    <CardHeader text={groupHeaderText}>
      <IconButton size="small">
        <AddIcon />
      </IconButton>
    </CardHeader>
    <DataList>
      <Group
        name="Service Alerts"
        lastNotified={new Date()}
        isService
      />
      {groups.map(g => <Group
        key={g.group_id}
        name={"Test Group Name"}
        userCount={2}
        lastNotified={new Date()}
      />)}
    </DataList>
  </Card>
}
