'use client';
import Card, { CardHeader } from "@/app/components/Card";
import DataList from "@/app/components/DataList";
import Group from "@/app/components/ServiceGroup";
import { ServiceGroup } from "@/types";
import { IconButton } from "@mui/material";

import AddIcon from "@mui/icons-material/Add";
import { pluralize } from "@/helpers/util";
import CreateGroupDialog from "@/app/components/dialogs/CreateGroupDialog";
import { DialogName } from "@/helpers/dialog";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { addGroup } from "@/store/slices/serviceSlice";
import { openDialog } from "@/store/slices/dialogSlice";
import DeleteGroupDialog from "@/app/components/dialogs/DeleteGroupDialog";

interface ServiceGroupListProps {
  serviceId: string;
  groups: ServiceGroup[];
}
export default function ServiceGroupList ({ serviceId, groups: intialGroups }: ServiceGroupListProps) {
  const dispatch = useAppDispatch()

  const groups = useAppSelector(state => state.service.currentServiceId === serviceId ? state.service.groups : intialGroups)

  const openCreateGroupDialog = () => dispatch(openDialog(DialogName.NewServiceGroupPopup))

  const handleNewGroup = (group: ServiceGroup) => dispatch(addGroup(group))
  
  /** @todo add service alerts */
  const groupHeaderText = pluralize(groups.length /* + 1 */, 'Groups', 'Group')

  return <Card>
    <CardHeader text={groupHeaderText}>
      <IconButton size="small" onClick={openCreateGroupDialog}>
        <AddIcon />
      </IconButton>
    </CardHeader>
    <DataList>
      {/* <Group
        name="Service Alerts"
        lastNotified={new Date()}
        isService
      /> */}
      {groups.map(g => <Group
        key={g.group_id}
        groupId={g.group_id}
        name={g.name}
        userCount={2}
        lastNotified={new Date()}
      />)}
    </DataList>
    <CreateGroupDialog serviceId={serviceId} onCreate={handleNewGroup} />
    <DeleteGroupDialog />
  </Card>
}
