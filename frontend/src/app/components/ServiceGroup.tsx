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
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { openDialogWithKey } from "@/store/slices/dialogSlice";
import { DialogName } from "@/helpers/dialog";

interface BaseServiceGroupProps {
  name: string;
  isService?: boolean;
  lastNotified: string;
  groupId?: string;
}

type ServiceGroupProps = BaseServiceGroupProps &
  ({ isService: true; groupId?: never } | { groupId: string; })

export default function ServiceGroup (props: ServiceGroupProps) {
  const dispatch = useAppDispatch()
  const userCount = useAppSelector(state => props.isService
    ? -1
    : state.service.subscribers.filter(s => s.groups.includes(props.groupId!)).length
  )
  const userCountText = props.isService ? 'All users' : pluralize(userCount, 'users', 'user')
  const notifiedText = props.lastNotified
    ? 'Notified ' + new Date(props.lastNotified).toLocaleDateString()
    : 'Never Notified'

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
      <DataRowStatItem icon={<WatchLaterIcon/>} text={notifiedText} />
    </DataRowInformation>
    <ButtonGroup>
      <Button variant="text" size="small" onClick={openEditGroupDialog}><EditIcon /></Button>
      <Divider/>
      <Button variant="text" size="small" disabled={props.isService} onClick={openDeleteDialog}><DeleteIcon /></Button>
    </ButtonGroup>
  </DataRow>
}