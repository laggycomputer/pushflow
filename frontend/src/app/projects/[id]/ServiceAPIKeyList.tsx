'use client';
import Card, { CardHeader } from "@/app/components/Card";
import DataList, { EmptyListMessage } from "@/app/components/DataList";
import ApiKey from "@/app/components/ServiceApiKey";
import { ServiceApiKey } from "@/types";
import { Button, IconButton } from "@mui/material";

import AddIcon from '@mui/icons-material/Add';
import { DialogName } from "@/helpers/dialog";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import CreateApiKeyDialog from "@/app/components/dialogs/CreateApiKeyDialog";
import { useState } from "react";
import ShowApiKeyDialog from "@/app/components/dialogs/ShowApiKeyDialog";
import { registerWorker, subscribeToNotifications } from "@/helpers/webpush-test";
import { prependApiKey } from "@/store/slices/serviceSlice";
import DeleteApiKeyDialog from "@/app/components/dialogs/DeleteApiKeyDialog";
import { openDialog } from "@/store/slices/dialogSlice";
import ApiKeyCodeSnippetDialog from "@/app/components/dialogs/ApiKeyCodeSnippetDialog";

interface ServiceAPIKeyListProps {
  serviceId: string;
  apiKeys: ServiceApiKey[]
}
export default function ServiceAPIKeyList ({ serviceId, apiKeys: initialKeys }: ServiceAPIKeyListProps) {
  const dispatch = useAppDispatch()

  const apiKeys = useAppSelector(state => state.service.currentServiceId === serviceId ? state.service.apiKeys : initialKeys)

  const [createdApiKey, setCreatedApiKey] = useState<ServiceApiKey>()

  const openCreateApiKeyDialog = () => dispatch(openDialog(DialogName.NewServiceApiKeyPopup))
  const handleCreateKey = (apiKey: ServiceApiKey) => {
    const truncated = apiKey.key_preview.replace(/^.*-/, '')
    const dataWithTruncation = { ...apiKey, key_preview: truncated }

    dispatch(prependApiKey(dataWithTruncation))

    setCreatedApiKey(apiKey)
    dispatch(openDialog(DialogName.ShowServiceApiKeyPopup))
  }

  const handleTestClick = async () => {
    await registerWorker()
    subscribeToNotifications(serviceId, ['0198972d-6fa0-7ff0-b3b2-86a46ecfbb73'])
  }

  return <Card className="list-container">
    <CardHeader text="API Keys">
      <IconButton size="small" onClick={openCreateApiKeyDialog}>
        <AddIcon />
      </IconButton>
    </CardHeader>
    <DataList>
      {apiKeys.map(k => <ApiKey key={k.key_preview} data={k} />)}
      <EmptyListMessage list={apiKeys} message="You have no API Keys yet!"/>
    </DataList>
    <CreateApiKeyDialog serviceId={serviceId} onCreate={handleCreateKey} />
    <ShowApiKeyDialog apiKey={createdApiKey} />
    <DeleteApiKeyDialog />
    <ApiKeyCodeSnippetDialog />
  </Card>
}
