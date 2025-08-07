import { Button, ButtonGroup, Divider } from "@mui/material";
import IconWrapper from "./IconWrapper";

import PersonAddIcon from '@mui/icons-material/PersonAdd';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import DataRow, { DataRowInformation, DataRowStatItem } from "./DataRow";

import BuildIcon from '@mui/icons-material/Build';
import GroupIcon from '@mui/icons-material/Group';

export default function SubscriptionUser () {
  return <DataRow>
    <IconWrapper flatShadow><PersonAddIcon/></IconWrapper>
    <DataRowInformation title="This is a username or id">
      <DataRowStatItem icon={<BuildIcon/>} text="Created 3y ago" />
      <DataRowStatItem icon={<GroupIcon/>} text="2 groups" />
    </DataRowInformation>
    <ButtonGroup>
      <Button variant="text" size="small"><EditIcon/></Button>
      <Divider/>
      <Button variant="text" size="small"><DeleteIcon/></Button>
      <Divider/>
      <Button variant="text" size="small"><ExpandMoreIcon/></Button>
    </ButtonGroup>
  </DataRow>
}
