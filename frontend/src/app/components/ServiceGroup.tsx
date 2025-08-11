'use client';
import { Button, ButtonGroup, Divider } from "@mui/material";
import IconWrapper from "./IconWrapper";

import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import DataRow, { DataRowInformation, DataRowStatItem } from "./DataRow";

import GroupIcon from '@mui/icons-material/Group';
import WatchLaterIcon from '@mui/icons-material/WatchLater';
import WarningIcon from '@mui/icons-material/Warning';
import { pluralize } from "@/helpers/util";
import { useAppDispatch } from "@/store/hooks";
import { openDialog, openDialogWithKey } from "@/store/slices/dialogSlice";
import { DialogName } from "@/helpers/dialog";

interface BaseServiceGroupProps {
  name: string;
  userCount?: number;
  isService?: boolean;
  lastNotified: Date;
  groupId?: string;
}

type ServiceGroupProps = BaseServiceGroupProps &
  ({ isService: true; userCount?: never; groupId?: never } | { userCount: number; groupId: string; })

export default function ServiceGroup (props: ServiceGroupProps) {
  const dispatch = useAppDispatch()
  const lastNotified = props.lastNotified.toLocaleDateString()
  const userCountText = props.isService ? 'All users' : pluralize(props.userCount, 'users', 'user')
  const icon = props.isService ? <WarningIcon /> : <GroupIcon />

  const openDeleteDialog = () => {
    dispatch(openDialogWithKey({ name: DialogName.DeleteServiceGroupPopup, key: props.groupId! }))
  }

  const openEditGroupDialog = () => {
    if (!props.groupId) return
    dispatch(openDialogWithKey({ name: DialogName.NewServiceGroupPopup, key: props.groupId }))
  }

  return <DataRow>
    <IconWrapper flatShadow>{icon}</IconWrapper>
    <DataRowInformation title={props.name}>
      <DataRowStatItem icon={<GroupIcon />} text={userCountText} />
      <DataRowStatItem icon={<WatchLaterIcon/>} text={'Used ' + lastNotified} />
    </DataRowInformation>
    <ButtonGroup>
      <Button variant="text" size="small" onClick={openEditGroupDialog}><EditIcon /></Button>
      <Divider/>
      <Button variant="text" size="small" disabled={props.isService} onClick={openDeleteDialog}><DeleteIcon /></Button>
    </ButtonGroup>
  </DataRow>
}