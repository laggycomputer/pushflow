'use client';
import { Button, Dialog, DialogActions, TextField } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { useState } from "react";
import { useRouter } from "next/navigation";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { DialogName } from "@/helpers/dialog";
import { closeDialog } from "@/store/slices/dialogSlice";
import { createGroup } from "@/helpers/service-group";
import { ServiceGroup } from "@/types";

interface CreateGroupDialogProps {
  serviceId: string;
  onCreate: (group: ServiceGroup) => void;
}

export default function CreateGroupDialog ({ serviceId, onCreate }: CreateGroupDialogProps) {
  const dispatch = useAppDispatch()
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.NewServiceGroupPopup)
  
  const [submitting, setSubmitting] = useState(false)
  const [groupName, setGroupName] = useState('')
  
  const handleClose = () => dispatch(closeDialog())

  const handleSubmit = async (event: React.FormEvent) => {
    setSubmitting(true)
    event.preventDefault()
    const group = await createGroup(serviceId, groupName)
    setSubmitting(false)
    if (!group) return console.error('There was an error creating the group')
    onCreate(group)
    handleClose()
  }

  return <Dialog open={isOpen} onClose={handleClose}>
    <Card>
      <CardHeader text="Create Group" />
      <form onSubmit={handleSubmit} id="create-group-form">
        <TextField
          autoFocus
          required
          margin="dense"
          name="group_name"
          label="Group Name"
          fullWidth
          onChange={e => setGroupName(e.target.value)}
          disabled={submitting}
        />
      </form>
      <DialogActions>
        <Button onClick={handleClose} disabled={submitting}>Cancel</Button>
        <Button form="create-group-form" type="submit" disabled={submitting} loading={submitting}>
          Create
        </Button>
      </DialogActions>
    </Card>
  </Dialog>
}
