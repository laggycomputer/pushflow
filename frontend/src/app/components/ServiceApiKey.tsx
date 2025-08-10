import { Button, ButtonGroup, Divider } from "@mui/material";
import IconWrapper from "./IconWrapper";

import PersonAddIcon from '@mui/icons-material/PersonAdd';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import DataRow, { DataRowInformation, DataRowStatItem } from "./DataRow";

import BuildIcon from '@mui/icons-material/Build';
import WatchLaterIcon from '@mui/icons-material/WatchLater';
import CodeIcon from '@mui/icons-material/Code';
import { ServiceApiKey as ApiKey } from "@/types";

interface ServiceApiKeyProps {
  data: ApiKey
}

export default function ServiceApiKey ({ data: apiKey }: ServiceApiKeyProps) {
  const title = `${apiKey.name ? apiKey.name + ' • ' : ''}...${apiKey.key_preview}`
  const lastUsed = apiKey.last_used
    ? 'Used ' + new Date(apiKey.last_used).toLocaleDateString()
    : 'Never used'

  const scopeNames: Record<string, string> = {
    sub: 'Subscriptions',
    notify: 'Notifications',
    group: 'Groups'
  }
  const scopesText = apiKey.scopes
    .map(s => scopeNames[s.scope])
    .filter(t => !!t)
    .join(', ')
  console.log(apiKey.scopes)

  return <DataRow>
    <IconWrapper flatShadow><PersonAddIcon /></IconWrapper>
    <DataRowInformation title={title}>
      <DataRowStatItem icon={<BuildIcon/>} text={scopesText} />
      <DataRowStatItem icon={<WatchLaterIcon/>} text={lastUsed} />
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