import { Button, Dialog, DialogActions, DialogContentText } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { closeDialog } from "@/store/slices/dialogSlice";

import { DialogName } from '@/helpers/dialog';
import { removeGroup } from "@/store/slices/serviceSlice";
import { useState } from "react";
import { deleteGroup } from "@/helpers/service-group";

export default function DeleteGroupDialog () {
  const dispatch = useAppDispatch()
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.DeleteServiceGroupPopup)
  const serviceId = useAppSelector(state => state.service.currentServiceId)
  const groupId = useAppSelector(state => state.dialog.key)
  const group = useAppSelector(state => state.service.groups.find(g => g.group_id === groupId))
  
  const [submitting, setSubmitting] = useState(false)
  
  if (!serviceId || !group) return null

  const handleClose = () => dispatch(closeDialog())

  const handleDelete = async () => {
    setSubmitting(true)
    const success = await deleteGroup(serviceId, group.group_id)
    setSubmitting(false)
    if (success) {
      dispatch(closeDialog())
      dispatch(removeGroup(group))
    } else {
      console.error('Unable to delete group')
    }
  }

  
  return <Dialog open={isOpen} onClose={handleClose} id="show-api-key-dialog">
    <Card>
      <CardHeader text="Deleting Group" />
      <DialogContentText>
        You are about to DELETE your Group &quot;{group.name}&quot;.
        Be sure, as this action cannot be undone!
      </DialogContentText>
      <DialogActions>
        <Button onClick={handleClose} disabled={submitting}>Close</Button>
        <Button onClick={handleDelete} disabled={submitting} loading={submitting} color="error">Delete</Button>
      </DialogActions>
    </Card>
  </Dialog>
}