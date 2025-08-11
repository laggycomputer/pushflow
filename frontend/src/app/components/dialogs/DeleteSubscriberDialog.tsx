import { Button, Dialog, DialogActions, DialogContentText } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { closeDialog } from "@/store/slices/dialogSlice";

import { DialogName } from '@/helpers/dialog';
import { useState } from "react";
import { removeSubscriber } from "@/store/slices/serviceSlice";
import { deleteSubscriber } from "@/helpers/service-subscriber";

export default function DeleteSubscriberDialog () {
  const dispatch = useAppDispatch()
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.DeleteServiceSubscriberPopup)
  const serviceId = useAppSelector(state => state.service.currentServiceId)
  const subscriberId = useAppSelector(state => state.dialog.key)
  const subscriber = useAppSelector(state => state.service.subscribers.find(s => s.subscriber_id === subscriberId))
  
  const [submitting, setSubmitting] = useState(false)
  
  if (!serviceId || !subscriber) return null

  const handleClose = () => dispatch(closeDialog())

  const handleDelete = async () => {
    setSubmitting(true)
    const success = await deleteSubscriber(serviceId, subscriber.subscriber_id)
    setSubmitting(false)
    if (success) {
      dispatch(closeDialog())
      dispatch(removeSubscriber(subscriber))
    } else {
      console.error('Unable to delete subscriber')
    }
  }

  
  return <Dialog open={isOpen} onClose={handleClose} id="delete-subscriber-dialog">
    <Card>
      <CardHeader text="Deleting Subscriber" />
      <DialogContentText>
        You are about to DELETE subscriber &quot;{subscriber.name ?? subscriber.subscriber_id}&quot;.
        Be sure, as this action cannot be undone!
      </DialogContentText>
      <DialogActions>
        <Button onClick={handleClose} disabled={submitting}>Close</Button>
        <Button onClick={handleDelete} disabled={submitting} loading={submitting} color="error">Delete</Button>
      </DialogActions>
    </Card>
  </Dialog>
}