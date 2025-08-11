import { Button, Dialog, DialogActions, DialogContentText } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { closeDialog } from "@/store/slices/dialogSlice";

import { DialogName } from '@/helpers/dialog';
import { removeApiKey } from "@/store/slices/serviceSlice";
import { deleteApiKey } from "@/helpers/service-api-key";
import { useState } from "react";

export default function DeleteApiKeyDialog () {
  const dispatch = useAppDispatch()
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.DeleteServiceApiKeyPopup)
  const serviceId = useAppSelector(state => state.service.currentServiceId)
  const keyId = useAppSelector(state => state.dialog.key)
  const apiKey = useAppSelector(state => state.service.apiKeys.find(k => k.key_id === keyId))
  
  const [submitting, setSubmitting] = useState(false)
  
  if (!serviceId || !apiKey) return null

  const handleClose = () => {
    dispatch(closeDialog())
  }

  const handleDelete = async () => {
    setSubmitting(true)
    const success = await deleteApiKey(serviceId, apiKey.key_id)
    setSubmitting(false)
    if (success) {
      dispatch(closeDialog())
      dispatch(removeApiKey(apiKey))
    } else {
      console.error('Unable to delete API Key')
    }
  }

  
  return <Dialog open={isOpen} onClose={handleClose} id="show-api-key-dialog">
    <Card>
      <CardHeader text="Deleting API Key" />
      <DialogContentText>
        You are about to DELETE your API Key &quot;{apiKey.name}&quot; (starting with <code>
          {apiKey.key_preview}
        </code>). Be sure, as this action cannot be undone!
      </DialogContentText>
      <DialogActions>
        <Button onClick={handleClose} disabled={submitting}>Close</Button>
        <Button onClick={handleDelete} disabled={submitting} loading={submitting} color="error">Delete</Button>
      </DialogActions>
    </Card>
  </Dialog>
}