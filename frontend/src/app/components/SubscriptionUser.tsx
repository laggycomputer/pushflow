'use client';
import { Button, ButtonGroup, Divider } from "@mui/material";
import IconWrapper from "./IconWrapper";

import DataRow, { DataRowInformation, DataRowStatItem } from "./DataRow";

import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import PersonAddIcon from '@mui/icons-material/PersonAdd';
import BuildIcon from '@mui/icons-material/Build';
import GroupIcon from '@mui/icons-material/Group';
import { pluralize } from "@/helpers/util";

interface SubscriptionUserProps {
  displayName: string;
  createdAt: Date;
  groupCount: number;
}

export default function SubscriptionUser (props: SubscriptionUserProps) {
  const groupText = pluralize(props.groupCount, 'groups', 'group')

  return <DataRow>
    <IconWrapper flatShadow><PersonAddIcon /></IconWrapper>
    <DataRowInformation title={props.displayName}>
      <DataRowStatItem icon={<BuildIcon/>} text={`Created ${props.createdAt.toLocaleDateString()}`} />
      <DataRowStatItem icon={<GroupIcon/>} text={groupText} />
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
