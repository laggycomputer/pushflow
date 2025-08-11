'use client';
import { Button, Dialog, DialogActions, TextField } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { useState } from "react";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { DialogName } from "@/helpers/dialog";
import { closeDialog } from "@/store/slices/dialogSlice";
import { createGroup, updateGroup } from "@/helpers/service-group";
import { ServiceGroup } from "@/types";
import { editGroup } from "@/store/slices/serviceSlice";

interface CreateGroupDialogProps {
  serviceId: string;
  onCreate: (group: ServiceGroup) => void;
}

export default function CreateGroupDialog ({ serviceId, onCreate }: CreateGroupDialogProps) {
  const dispatch = useAppDispatch()
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.NewServiceGroupPopup)
  const groupId = useAppSelector(state => state.dialog.key)
  const editingGroup = useAppSelector(state => state.service.groups.find(g => g.group_id === groupId))
  
  const [submitting, setSubmitting] = useState(false)
  const [groupName, setGroupName] = useState('')
  
  const handleClose = () => dispatch(closeDialog())

  const handleSubmit = async (event: React.FormEvent) => {
    setSubmitting(true)
    event.preventDefault()
    if (editingGroup) {
      await updateGroup(serviceId, groupId, groupName)
      setSubmitting(false)
      handleClose()
      dispatch(editGroup({ ...editingGroup, name: groupName }))
      return
    }

    const group = await createGroup(serviceId, groupName)
    setSubmitting(false)
    if (!group) return console.error('There was an error creating the group')
    onCreate(group)
    handleClose()
  }

  if (groupId && !editingGroup) return null

  return <Dialog open={isOpen} onClose={handleClose}>
    <Card>
      <CardHeader text={editingGroup ? 'Edit Group' : 'Create Group'} />
      <form onSubmit={handleSubmit} id="create-group-form">
        <TextField
          autoFocus
          required
          margin="dense"
          name="group_name"
          label="Group Name"
          fullWidth
          defaultValue={editingGroup?.name ?? ''}
          onChange={e => setGroupName(e.target.value)}
          disabled={submitting}
        />
      </form>
      <DialogActions>
        <Button onClick={handleClose} disabled={submitting}>Cancel</Button>
        <Button form="create-group-form" type="submit" disabled={submitting} loading={submitting}>
          {editingGroup ? 'Save' : 'Create'}
        </Button>
      </DialogActions>
    </Card>
  </Dialog>
}
