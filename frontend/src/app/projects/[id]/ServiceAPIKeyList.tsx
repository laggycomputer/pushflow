'use client';
import Card, { CardHeader } from "@/app/components/Card";
import DataList, { EmptyListMessage } from "@/app/components/DataList";
import ApiKey from "@/app/components/ServiceApiKey";
import { ServiceApiKey } from "@/types";
import { Button, IconButton } from "@mui/material";

import AddIcon from '@mui/icons-material/Add';
import { setActiveDialog } from "@/store/slices/dialogSlice";
import { DialogName } from "@/helpers/dialog";
import { useAppDispatch } from "@/store/hooks";
import CreateApiKeyDialog from "@/app/components/dialogs/CreateApiKeyDialog";
import { useState } from "react";
import ShowApiKeyDialog from "@/app/components/dialogs/ShowApiKeyDialog";
import { registerWorker, subscribeToNotifications } from "@/helpers/webpush-test";

interface ServiceAPIKeyListProps {
  serviceId: string;
  apiKeys: ServiceApiKey[]
}
export default function ServiceAPIKeyList ({ serviceId, apiKeys }: ServiceAPIKeyListProps) {
  const dispatch = useAppDispatch()

  /** @todo put in redux store */
  const [shownKeys, setShownKeys] = useState(apiKeys)
  const [createdApiKey, setCreatedApiKey] = useState<ServiceApiKey>()

  const openCreateApiKeyDialog = () => dispatch(setActiveDialog(DialogName.NewServiceApiKeyPopup))
  const handleCreateKey = (apiKey: ServiceApiKey) => {
    const truncated = apiKey.key_preview.replace(/-.*$/, '')
    const dataWithTruncation = { ...apiKey, key_preview: truncated }
    setShownKeys([dataWithTruncation, ...shownKeys])

    setCreatedApiKey(apiKey)
    dispatch(setActiveDialog(DialogName.ShowServiceApiKeyPopup))
  }

  const handleTestClick = async () => {
    await registerWorker()
    subscribeToNotifications(serviceId, [])
  }

  return <Card>
    <CardHeader text="API Keys">
      <IconButton size="small" onClick={openCreateApiKeyDialog}>
        <AddIcon />
      </IconButton>
    </CardHeader>
    <DataList>
      {shownKeys.map(k => <ApiKey
        key={k.key_preview}
        data={k}
      />)}
      <EmptyListMessage list={shownKeys} message="You have no API Keys yet!"/>
    </DataList>
    <CreateApiKeyDialog serviceId={serviceId} onCreate={handleCreateKey} />
    <ShowApiKeyDialog apiKey={createdApiKey} />
    <Button onClick={handleTestClick}>
      {process.env.NEXT_PUBLIC_VAPID_PUBLIC_KEY}<br/>{process.env.NEXT_PUBLIC_TEST_API_KEY}
    </Button>
  </Card>
}
