import { Button, ButtonGroup, Divider } from "@mui/material";
import IconWrapper from "./IconWrapper";

import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import DataRow, { DataRowInformation, DataRowStatItem } from "./DataRow";

import GroupIcon from '@mui/icons-material/Group';
import WatchLaterIcon from '@mui/icons-material/WatchLater';
import WarningIcon from '@mui/icons-material/Warning';
import { pluralize } from "@/helpers/util";

interface BaseServiceGroupProps {
  name: string;
  userCount?: number;
  isService?: boolean;
  lastNotified: Date;
}

type ServiceGroupProps = BaseServiceGroupProps &
  ({ isService: true; userCount?: never; } | { userCount: number })

export default function ServiceGroup (props: ServiceGroupProps) {
  const lastNotified = props.lastNotified.toLocaleDateString()
  const userCountText = props.isService ? 'All users' : pluralize(props.userCount, 'users', 'user')
  const icon = props.isService ? <WarningIcon /> : <GroupIcon />

  return <DataRow>
    <IconWrapper flatShadow>{icon}</IconWrapper>
    <DataRowInformation title={props.name}>
      <DataRowStatItem icon={<GroupIcon />} text={userCountText} />
      <DataRowStatItem icon={<WatchLaterIcon/>} text={'Used ' + lastNotified} />
    </DataRowInformation>
    <ButtonGroup>
      <Button variant="text" size="small"><EditIcon /></Button>
      <Divider/>
      <Button variant="text" size="small" disabled={props.isService}><DeleteIcon /></Button>
    </ButtonGroup>
  </DataRow>
}