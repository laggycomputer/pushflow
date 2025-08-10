'use client';
import Card, { CardHeader } from "@/app/components/Card";
import DataList from "@/app/components/DataList";
import Group from "@/app/components/ServiceGroup";
import { ServiceGroup } from "@/types";
import { IconButton } from "@mui/material";

import AddIcon from "@mui/icons-material/Add";
import { pluralize } from "@/helpers/util";
import CreateGroupDialog from "@/app/components/dialogs/CreateGroupDialog";
import { setActiveDialog } from "@/store/slices/dialogSlice";
import { DialogName } from "@/helpers/dialog";
import { useAppDispatch } from "@/store/hooks";
import { useState } from "react";

interface ServiceGroupListProps {
  serviceId: string;
  groups: ServiceGroup[];
}
export default function ServiceGroupList ({ serviceId, groups }: ServiceGroupListProps) {
  const dispatch = useAppDispatch()

  /** @todo put in redux store */
  const [storedGroups, setStoredGroups] = useState(groups)

  const openCreateGroupDialog = () => dispatch(setActiveDialog(DialogName.NewServiceGroupPopup))
  const handleNewGroup = (group: ServiceGroup) => {
    setStoredGroups([...storedGroups, group])
  }
  
  const groupHeaderText = pluralize(groups.length + 1, 'Groups', 'Group')

  return <Card>
    <CardHeader text={groupHeaderText}>
      <IconButton size="small" onClick={openCreateGroupDialog}>
        <AddIcon />
      </IconButton>
    </CardHeader>
    <DataList>
      <Group
        name="Service Alerts"
        lastNotified={new Date()}
        isService
      />
      {storedGroups.map(g => <Group
        key={g.group_id}
        name={g.name}
        userCount={2}
        lastNotified={new Date()}
      />)}
    </DataList>
    <CreateGroupDialog serviceId={serviceId} onCreate={handleNewGroup} />
  </Card>
}
