'use client';
import { Button, Dialog, DialogActions, TextField } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { useState } from "react";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { DialogName } from "@/helpers/dialog";
import { closeDialog } from "@/store/slices/dialogSlice";
import { editSubscriber } from "@/store/slices/serviceSlice";
import { updateSubscriber } from "@/helpers/service-subscriber";

export default function EditSubscriberDialog () {
  const dispatch = useAppDispatch()
  const serviceId = useAppSelector(state => state.service.currentServiceId)
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.EditServiceSubscriberPopup)
  const subId = useAppSelector(state => state.dialog.key)
  const editingSub = useAppSelector(state => state.service.subscribers.find(s => s.subscriber_id === subId))
  
  const [submitting, setSubmitting] = useState(false)
  const [subName, setSubName] = useState('')
  
  const handleClose = () => dispatch(closeDialog())

  if (!serviceId || !editingSub) return null

  const handleSubmit = async (event: React.FormEvent) => {
    setSubmitting(true)
    event.preventDefault()
      await updateSubscriber(serviceId, subId, subName)
      setSubmitting(false)
      handleClose()
      dispatch(editSubscriber({ ...editingSub, name: subName }))
  }

  return <Dialog open={isOpen} onClose={handleClose}>
    <Card>
      <CardHeader text="Editing Subscriber" />
      <form onSubmit={handleSubmit} id="edit-subscriber-form">
        <TextField
          autoFocus
          required
          margin="dense"
          name="sub_name"
          label="Subscriber Name"
          fullWidth
          defaultValue={editingSub?.name ?? ''}
          onChange={e => setSubName(e.target.value)}
          disabled={submitting}
        />
      </form>
      <DialogActions>
        <Button onClick={handleClose} disabled={submitting}>Cancel</Button>
        <Button form="edit-subscriber-form" type="submit" disabled={submitting} loading={submitting}>
          Save
        </Button>
      </DialogActions>
    </Card>
  </Dialog>
}
