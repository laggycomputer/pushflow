import { Button, ButtonGroup, Divider } from "@mui/material";
import IconWrapper from "./IconWrapper";

import PersonAddIcon from '@mui/icons-material/PersonAdd';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import DataRow, { DataRowInformation, DataRowStatItem } from "./DataRow";

import BuildIcon from '@mui/icons-material/Build';
import WatchLaterIcon from '@mui/icons-material/WatchLater';
import CodeIcon from '@mui/icons-material/Code';

interface ServiceApiKeyProps {
  name?: string;
  keyUuid: string;
  scopes: string[];
  lastUsed: Date;
}

export default function ServiceApiKey (props: ServiceApiKeyProps) {
  const title = `${props.name ? props.name + ' • ' : ''}${props.keyUuid.slice(0, 8)}...`
  const lastUsed = props.lastUsed.toLocaleDateString()

  return <DataRow>
    <IconWrapper flatShadow><PersonAddIcon /></IconWrapper>
    <DataRowInformation title={title}>
      <DataRowStatItem icon={<BuildIcon/>} text="Add Subscription" />
      <DataRowStatItem icon={<WatchLaterIcon/>} text={'Used ' + lastUsed} />
    </DataRowInformation>
    <ButtonGroup>
      <Button variant="text" size="small"><EditIcon /></Button>
      <Divider/>
      <Button variant="text" size="small"><DeleteIcon /></Button>
      <Divider/>
      <Button variant="text" size="small"><CodeIcon /></Button>
    </ButtonGroup>
  </DataRow>
}